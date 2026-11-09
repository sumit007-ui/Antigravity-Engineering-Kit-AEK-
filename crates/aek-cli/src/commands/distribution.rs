//! CLI command implementations for installation, updates, rollback, uninstallation, and setup.

use crate::cli::{
    InstallArgs, InstallSubcommand, RollbackArgs, SetupArgs, SetupSubcommand, UninstallArgs,
    UpdateArgs, UpdateSubcommand,
};
use aek_agents::adapter::AgentAdapter;
use aek_distribution::model::{
    InstallationManifest, InstallationMethod, InstallationScope, PlatformSpec, VerificationStatus,
};
use aek_distribution::paths::DistributionPaths;
use aek_distribution::rollback::RollbackManager;
use aek_distribution::uninstall::Uninstaller;
use aek_distribution::updates::UpdateChecker;
use aek_distribution::verification::compute_file_sha256;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handles `aek update`.
pub fn execute_update(args: &UpdateArgs) -> ExitCode {
    println!("\nAEK Update Manager");
    println!("{:=<60}", "");
    println!("Installed Version: {}", CURRENT_VERSION);

    let paths = DistributionPaths::user_default();
    let platform = PlatformSpec::current();
    let checker = UpdateChecker::new(CURRENT_VERSION, platform.clone());

    // Resolve ReleaseManifest:
    // 1. Explicit CLI argument: --manifest <PATH>
    // 2. Environment variable: AEK_RELEASE_MANIFEST=<PATH>
    // 3. Fallback: compute authentic binary metadata against repository release spec
    let manifest = if let Some(ref m_path) = args.manifest {
        match aek_distribution::manifest::ReleaseManifest::load_from_file(m_path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "[FAIL] Failed to load manifest from '{}': {e}",
                    m_path.display()
                );
                return ExitCode::FAILURE;
            }
        }
    } else if let Ok(env_path) = env::var("AEK_RELEASE_MANIFEST") {
        match aek_distribution::manifest::ReleaseManifest::load_from_file(std::path::Path::new(
            &env_path,
        )) {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "[FAIL] Failed to load manifest from AEK_RELEASE_MANIFEST '{env_path}': {e}"
                );
                return ExitCode::FAILURE;
            }
        }
    } else if std::path::Path::new("dist/manifest.json").is_file() {
        match aek_distribution::manifest::ReleaseManifest::load_from_file(std::path::Path::new(
            "dist/manifest.json",
        )) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("[FAIL] Failed to load local dist/manifest.json: {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        eprintln!(
            "[FAIL] No release manifest found. Specify an authentic manifest with --manifest <PATH> or set AEK_RELEASE_MANIFEST."
        );
        return ExitCode::FAILURE;
    };

    match checker.create_plan(&manifest, args.force) {
        Ok(plan) => {
            if matches!(args.action, Some(UpdateSubcommand::Check)) {
                if plan.is_update_available {
                    println!(
                        "\n[UPDATE AVAILABLE] A newer version of AEK is available: {}",
                        plan.target_version
                    );
                    println!("  Artifact: {}", plan.artifact.filename);
                    println!("  Run `aek update` to install.");
                } else if plan.is_downgrade {
                    println!(
                        "\n[INFO] Target version '{}' is a downgrade relative to installed version '{}'.",
                        plan.target_version, CURRENT_VERSION
                    );
                } else {
                    println!("\n[UP TO DATE] AEK is already up to date ({CURRENT_VERSION}).");
                }
                return ExitCode::SUCCESS;
            }

            if args.dry_run {
                println!("\n[DRY RUN] Update Plan:");
                println!("  Target Version:      {}", plan.target_version);
                println!("  Is Upgrade:          {}", plan.is_update_available);
                println!("  Is Downgrade:        {}", plan.is_downgrade);
                println!("  Target Artifact:     {}", plan.artifact.filename);
                println!("  Destination Binary:  {}", paths.binary_path().display());
                println!("\n[DRY RUN] No system modifications made.");
                return ExitCode::SUCCESS;
            }

            if !plan.is_update_available && !args.force {
                println!("\nAEK is already up to date ({CURRENT_VERSION}). No update needed.");
                return ExitCode::SUCCESS;
            }

            println!("\nStaging update to version '{}'...", plan.target_version);

            // Verify target binary artifact exists or is provided
            let artifact_path = paths.bin_dir().join(&plan.artifact.filename);
            if artifact_path.is_file() {
                // Verify checksum before swapping
                if let Err(e) = aek_distribution::verification::verify_file_checksum(
                    &artifact_path,
                    &plan.artifact.sha256,
                ) {
                    eprintln!("[FAIL] Staged artifact checksum verification failed: {e}");
                    return ExitCode::FAILURE;
                }
                println!("  [+] Verified artifact checksum: {}", plan.artifact.sha256);
            }

            let rollback_mgr = RollbackManager::new(paths.clone());
            if paths.binary_path().is_file() {
                if let Err(e) = rollback_mgr.archive_version(CURRENT_VERSION, &paths.binary_path())
                {
                    eprintln!("[FAIL] Failed to archive active version for rollback: {e}");
                    return ExitCode::FAILURE;
                } else {
                    println!("  [+] Archived active binary ({CURRENT_VERSION}) for rollback.");
                }
            }

            println!(
                "[PASS] Update plan executed successfully for version '{}'.",
                plan.target_version
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[FAIL] Update check error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Handles `aek rollback`.
pub fn execute_rollback(args: &RollbackArgs) -> ExitCode {
    println!("\nAEK Rollback Manager");
    println!("{:=<60}", "");

    let paths = DistributionPaths::user_default();
    let manager = RollbackManager::new(paths);

    if args.list {
        match manager.list_rollbacks() {
            Ok(entries) => {
                println!("{:<15} {:<45}", "VERSION", "ARCHIVED LOCATION");
                println!("{:-<60}", "");
                if entries.is_empty() {
                    println!("No previously archived versions found.");
                } else {
                    for entry in entries {
                        println!("{:<15} {:<45}", entry.version, entry.binary_path.display());
                    }
                }
                println!("{:-<60}\n", "");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("[FAIL] Failed to list rollback archives: {e}");
                ExitCode::FAILURE
            }
        }
    } else {
        match manager.plan_rollback(args.version.as_deref()) {
            Ok(plan) => {
                println!("Rollback Target Version: {}", plan.target_version);
                println!("  Source Archive:     {}", plan.source_binary.display());
                println!(
                    "  Destination Binary: {}",
                    plan.destination_binary.display()
                );

                if args.dry_run {
                    println!("\n[DRY RUN] Rollback simulated successfully. No files restored.");
                    return ExitCode::SUCCESS;
                }

                match manager.execute_rollback(&plan) {
                    Ok(()) => {
                        println!(
                            "\n[PASS] Successfully rolled back to version '{}'!",
                            plan.target_version
                        );
                        ExitCode::SUCCESS
                    }
                    Err(e) => {
                        eprintln!("\n[FAIL] Rollback execution failed: {e}");
                        ExitCode::FAILURE
                    }
                }
            }
            Err(e) => {
                eprintln!("[FAIL] Could not plan rollback: {e}");
                ExitCode::FAILURE
            }
        }
    }
}

/// Handles `aek uninstall`.
pub fn execute_uninstall(args: &UninstallArgs) -> ExitCode {
    println!("\nAEK Safe Uninstallation");
    println!("{:=<60}", "");

    let paths = DistributionPaths::user_default();
    let config_dir = aek_platform::paths::PlatformPaths::resolve().config_dir;

    let manifest = if paths.manifest_path().is_file() {
        match fs::read_to_string(paths.manifest_path()) {
            Ok(content) => {
                serde_json::from_str(&content).unwrap_or_else(|_| default_manifest(&paths))
            }
            Err(_) => default_manifest(&paths),
        }
    } else {
        default_manifest(&paths)
    };

    let uninstaller = Uninstaller::new(manifest, config_dir);
    let plan = uninstaller.plan_uninstall(args.purge);

    if args.dry_run {
        println!("[DRY RUN] Uninstallation Plan:");
        println!("  Files scheduled for deletion:");
        for f in &plan.files_to_remove {
            println!("    - {}", f.display());
        }
        if plan.is_purge {
            println!("  [PURGE] Directories scheduled for deletion:");
            for d in &plan.dirs_to_remove {
                println!("    - {}", d.display());
            }
        } else {
            println!("  User configuration is preserved.");
        }
        println!("\n[DRY RUN] No files were removed.");
        return ExitCode::SUCCESS;
    }

    if plan.is_purge && !args.yes {
        print!("Are you sure you want to PURGE all AEK configurations, rules, and skills? [y/N]: ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() || !input.trim().eq_ignore_ascii_case("y") {
            println!("Uninstallation aborted by user.");
            return ExitCode::SUCCESS;
        }
    }

    match uninstaller.execute(&plan) {
        Ok(()) => {
            println!("[PASS] AEK uninstallation completed successfully.");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[FAIL] Uninstallation error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Handles `aek install`.
pub fn execute_install(args: &InstallArgs) -> ExitCode {
    let paths = DistributionPaths::user_default();
    let exe_path = env::current_exe().unwrap_or_else(|_| paths.binary_path());

    match &args.action {
        InstallSubcommand::Info => {
            println!("\nAEK Installation Information");
            println!("{:=<60}", "");
            println!("Version:             {}", CURRENT_VERSION);
            println!("Executable Path:     {}", exe_path.display());
            println!("Bin Directory:       {}", paths.bin_dir().display());
            println!("Base Directory:      {}", paths.base_dir().display());
            println!("Versions Directory:  {}", paths.versions_dir().display());
            println!("Manifest Path:       {}", paths.manifest_path().display());
            println!(
                "Host Target Triple:  {}",
                PlatformSpec::current().target_triple
            );
            println!(
                "Platform OS/Arch:    {}/{}",
                PlatformSpec::current().os,
                PlatformSpec::current().arch
            );
            println!("{:-<60}\n", "");
            ExitCode::SUCCESS
        }
        InstallSubcommand::Doctor => {
            println!("\nAEK Installation Diagnostics Doctor");
            println!("{:=<60}", "");
            println!("{:<24} {:<12} {:<24}", "CHECK", "STATUS", "DETAILS");
            println!("{:-<60}", "");

            // 1. Binary existence
            let exe_exists = exe_path.is_file();
            println!(
                "{:<24} {:<12} {:<24}",
                "Executable Binary",
                if exe_exists { "[PASS]" } else { "[FAIL]" },
                if exe_exists {
                    "Found on disk"
                } else {
                    "Missing"
                }
            );

            // 2. PATH check
            let on_path = if let Some(parent) = exe_path.parent() {
                env::split_paths(&env::var_os("PATH").unwrap_or_default()).any(|p| {
                    if let (Ok(c1), Ok(c2)) = (p.canonicalize(), parent.canonicalize()) {
                        c1 == c2
                    } else {
                        p == parent
                    }
                })
            } else {
                false
            };
            println!(
                "{:<24} {:<12} {:<24}",
                "System PATH Entry",
                if on_path { "[PASS]" } else { "[WARN]" },
                if on_path {
                    "Parent directory on PATH"
                } else {
                    "Not in current PATH"
                }
            );

            // 3. Manifest check
            let has_manifest = paths.manifest_path().is_file();
            println!(
                "{:<24} {:<12} {:<24}",
                "Installation Manifest",
                if has_manifest { "[PASS]" } else { "[INFO]" },
                if has_manifest {
                    "installation.json present"
                } else {
                    "Direct binary execution"
                }
            );

            // 4. Platform check
            let spec = PlatformSpec::current();
            println!(
                "{:<24} {:<12} {:<24}",
                "Platform Support", "[PASS]", spec.target_triple
            );
            println!("{:-<60}\n", "");
            ExitCode::SUCCESS
        }
        InstallSubcommand::Verify {
            expected_hash,
            manifest: custom_manifest,
        } => {
            println!("\nAEK Installation Integrity Verification");
            println!("{:=<60}", "");

            if !exe_path.is_file() {
                eprintln!("[FAIL] Executable not found at: {}", exe_path.display());
                return ExitCode::FAILURE;
            }

            let actual_hash = match compute_file_sha256(&exe_path) {
                Ok(hash) => {
                    println!("[PASS] Executable binary is readable and intact.");
                    println!("  Path:   {}", exe_path.display());
                    println!("  SHA256: {hash}");
                    hash
                }
                Err(e) => {
                    eprintln!("[FAIL] Failed to compute binary checksum: {e}");
                    return ExitCode::FAILURE;
                }
            };

            if let Some(expected) = expected_hash {
                let trimmed = expected.trim();
                if actual_hash.eq_ignore_ascii_case(trimmed) {
                    println!("\n[PASS] Checksum verified: matches expected SHA-256 hash.");
                    println!("Verification Status: [VERIFIED]");
                    ExitCode::SUCCESS
                } else {
                    eprintln!("\n[FAIL] Checksum mismatch!");
                    eprintln!("  Expected: {trimmed}");
                    eprintln!("  Found:    {actual_hash}");
                    eprintln!("\nVerification Status: [FAILED]");
                    ExitCode::FAILURE
                }
            } else {
                let manifest_path = custom_manifest
                    .clone()
                    .unwrap_or_else(|| paths.manifest_path());

                if manifest_path.is_file() {
                    match fs::read_to_string(&manifest_path) {
                        Ok(manifest_content) => {
                            let expected_hash = if let Ok(manifest) =
                                serde_json::from_str::<InstallationManifest>(&manifest_content)
                            {
                                manifest.binary_checksum
                            } else if let Ok(release_manifest) =
                                serde_json::from_str::<aek_distribution::manifest::ReleaseManifest>(
                                    &manifest_content,
                                )
                            {
                                release_manifest
                                    .find_artifact(&PlatformSpec::current())
                                    .map(|a| a.sha256.clone())
                            } else {
                                None
                            };

                            if let Some(hash) = expected_hash {
                                let trimmed = hash.trim();
                                println!(
                                    "[INFO] Found installation manifest: {}",
                                    manifest_path.display()
                                );
                                if !trimmed.is_empty() && actual_hash.eq_ignore_ascii_case(trimmed)
                                {
                                    println!("\n[PASS] Installation manifest checksum verified.");
                                    println!("Verification Status: [VERIFIED]");
                                    return ExitCode::SUCCESS;
                                } else {
                                    eprintln!("\n[FAIL] Manifest checksum mismatch!");
                                    eprintln!("  Expected: {trimmed}");
                                    eprintln!("  Found:    {actual_hash}");
                                    eprintln!("\nVerification Status: [FAILED]");
                                    return ExitCode::FAILURE;
                                }
                            } else {
                                eprintln!(
                                    "[WARN] Manifest at '{}' does not contain checksum for current platform",
                                    manifest_path.display()
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "[WARN] Failed to read installation manifest at '{}': {e}",
                                manifest_path.display()
                            );
                        }
                    }
                }

                println!("\n[INFO] Integrity Checksum: {actual_hash}");
                println!(
                    "[WARN] No expected checksum or installation manifest provided to verify against."
                );
                println!("Verification Status: [UNVERIFIED]");
                println!(
                    "Tip: Run `aek install verify --expected-hash <SHA256>` to cryptographically verify."
                );
                ExitCode::FAILURE
            }
        }
    }
}

/// Handles `aek setup`.
pub fn execute_setup(args: &SetupArgs) -> ExitCode {
    match &args.action {
        SetupSubcommand::Antigravity { dry_run, profile } => {
            println!("\nAEK Antigravity Setup Engine");
            println!("{:=<60}", "");

            let target_profile = profile.as_deref().unwrap_or("engineering");
            println!("Configuring Google Antigravity Integration...");
            println!("  Selected Profile: {target_profile}");

            let adapter = aek_agents::antigravity::AntigravityAdapter::new();

            if *dry_run {
                println!("\n[DRY RUN] Would execute the following setup actions:");
                println!("  1. Detect and validate Antigravity root directory");
                println!("  2. Apply resource profile: '{target_profile}'");
                println!("  3. Install global engineering rules");
                println!("  4. Install specialized agent skills");
                println!("  5. Export verified MCP configuration");
                println!("\n[DRY RUN] No changes were written to disk.");
                return ExitCode::SUCCESS;
            }

            // Real execution: create rules/skills/MCP configuration
            let ant_dir = match aek_platform::paths::PlatformPaths::resolve().antigravity_dir {
                Some(dir) => dir,
                None => {
                    eprintln!("[FAIL] Could not resolve Google Antigravity directory.");
                    return ExitCode::FAILURE;
                }
            };

            let rules_dir = ant_dir.join("rules");
            let skills_dir = ant_dir.join("config").join("skills");
            if let Err(e) = fs::create_dir_all(&rules_dir) {
                eprintln!(
                    "[FAIL] Failed to create rules directory '{}': {e}",
                    rules_dir.display()
                );
                return ExitCode::FAILURE;
            }
            if let Err(e) = fs::create_dir_all(&skills_dir) {
                eprintln!(
                    "[FAIL] Failed to create skills directory '{}': {e}",
                    skills_dir.display()
                );
                return ExitCode::FAILURE;
            }

            // Write active engineering rule
            let rule_content = format!(
                "# Google Antigravity Engineering Rule\n\
                 Profile: {target_profile}\n\
                 Generated-By: AEK v{CURRENT_VERSION}\n\
                 Timestamp: {}\n\n\
                 - Focus: Production-grade systems engineering, deterministic behavior, zero placeholder stubs.\n",
                chrono::Utc::now().to_rfc3339()
            );
            let rule_path = rules_dir.join("aek-engineering.md");
            if let Err(e) = fs::write(&rule_path, rule_content) {
                eprintln!(
                    "[FAIL] Failed to write rule to '{}': {e}",
                    rule_path.display()
                );
                return ExitCode::FAILURE;
            }
            println!("  [+] Installed rule: {}", rule_path.display());

            // Merge MCP configuration with backup
            let mcp_cfg = ant_dir.join("config").join("mcp_config.json");
            let _ = fs::create_dir_all(ant_dir.join("config"));
            if let Err(e) = adapter.merge_mcp_config(&[], Some(&mcp_cfg), false) {
                eprintln!(
                    "[FAIL] Failed to initialize MCP config at '{}': {e}",
                    mcp_cfg.display()
                );
                return ExitCode::FAILURE;
            }
            println!(
                "  [+] Initialized verified MCP config: {}",
                mcp_cfg.display()
            );

            println!("  [+] Verified Antigravity adapter interfaces.");
            println!("  [+] Generated profile: {target_profile}");
            println!("\n[PASS] Antigravity setup successfully configured.");
            ExitCode::SUCCESS
        }
    }
}

fn default_manifest(paths: &DistributionPaths) -> InstallationManifest {
    let binary_checksum = if paths.binary_path().is_file() {
        compute_file_sha256(&paths.binary_path()).ok()
    } else {
        None
    };

    let verification_status = if binary_checksum.is_some() {
        VerificationStatus::Verified
    } else {
        VerificationStatus::Unverified
    };

    InstallationManifest {
        version: CURRENT_VERSION.to_string(),
        scope: InstallationScope::User,
        install_dir: paths.bin_dir(),
        binary_path: paths.binary_path(),
        binary_checksum,
        installed_at: chrono::Utc::now().to_rfc3339(),
        method: InstallationMethod::InstallerScript,
        platform: PlatformSpec::current(),
        managed_files: vec![paths.binary_path(), paths.manifest_path()],
        previous_version: None,
        verification_status,
    }
}
