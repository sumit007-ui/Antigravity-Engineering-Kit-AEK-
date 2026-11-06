//! `aek profile` command implementation.

use crate::cli::ProfileSubcommand;
use aek_config::backup::BackupEngine;
use aek_config::loader::{load_config_from_file, resolve_config_file};
use aek_config::writer::save_config_atomic;
use aek_core::profile::{ProfileDescriptor, ProfileKind};
use aek_platform::paths::PlatformPaths;
use std::process::ExitCode;

/// Executes the `aek profile` commands.
pub fn execute_profile(action: &ProfileSubcommand) -> ExitCode {
    let current_dir = std::env::current_dir().ok();

    match action {
        ProfileSubcommand::List { config } => {
            let target = resolve_config_file(config.as_deref(), current_dir.as_deref());
            let active_profile = load_config_from_file(&target)
                .map(|c| c.general.profile)
                .unwrap_or(ProfileKind::Light);

            println!("Available AEK Resource Profiles:\n");
            for kind in ProfileKind::all() {
                let desc = ProfileDescriptor::from(*kind);
                let marker = if *kind == active_profile { "*" } else { " " };
                println!("  {marker} {: <14} - {}", desc.kind.as_str(), desc.name);
                println!("    Target:       {}", desc.target_audience);
                println!(
                    "    Concurrency:  {} tasks | Timeout: {}s",
                    desc.default_concurrency, desc.default_timeout_seconds
                );
                println!(
                    "    MCP: {} | Prompt Opt: {} | Security Scan: {}\n",
                    desc.mcp_enabled,
                    desc.prompt_optimization_enabled,
                    desc.security_scanning_enabled
                );
            }
            if target.exists() {
                println!(
                    "(* indicates active profile configured in '{}')",
                    target.display()
                );
            } else {
                println!("(* indicates default profile; run `aek init` to generate configuration)");
            }
            ExitCode::SUCCESS
        }
        ProfileSubcommand::Show { name } => {
            let kind = match name.parse::<ProfileKind>() {
                Ok(k) => k,
                Err(_) => {
                    eprintln!(
                        "[FAIL] Unknown profile '{name}'. Choose from: light, engineering, full"
                    );
                    return ExitCode::FAILURE;
                }
            };

            let desc = ProfileDescriptor::from(kind);
            println!("============================================================");
            println!("     Profile: {} ({})", desc.name, desc.kind);
            println!("============================================================");
            println!("Description:     {}", desc.description);
            println!("Target Audience: {}", desc.target_audience);
            println!("Concurrency:     {} workers", desc.default_concurrency);
            println!("Default Timeout: {}s", desc.default_timeout_seconds);
            println!("MCP Enabled:     {}", desc.mcp_enabled);
            println!("Prompt Engine:   {}", desc.prompt_optimization_enabled);
            println!("Security Audit:  {}", desc.security_scanning_enabled);
            println!("\nFeatures:");
            for f in &desc.features {
                println!("  - {f}");
            }
            println!("============================================================");
            ExitCode::SUCCESS
        }
        ProfileSubcommand::Apply {
            name,
            dry_run,
            config,
        } => {
            let kind = match name.parse::<ProfileKind>() {
                Ok(k) => k,
                Err(_) => {
                    eprintln!(
                        "[FAIL] Unknown profile '{name}'. Choose from: light, engineering, full"
                    );
                    return ExitCode::FAILURE;
                }
            };

            let target = resolve_config_file(config.as_deref(), current_dir.as_deref());
            if !target.exists() {
                eprintln!(
                    "[FAIL] Configuration file does not exist at: {}",
                    target.display()
                );
                eprintln!("Run `aek init` first to initialize configuration.");
                return ExitCode::FAILURE;
            }

            let mut current_config = match load_config_from_file(&target) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[FAIL] Failed to read existing configuration: {e}");
                    return ExitCode::FAILURE;
                }
            };

            let previous_profile = current_config.general.profile;
            let desc = ProfileDescriptor::from(kind);

            if *dry_run {
                println!(
                    "[DRY-RUN] Would apply profile '{}' to '{}':",
                    kind,
                    target.display()
                );
                println!("  Previous profile:    {previous_profile}");
                println!("  Target profile:      {kind}");
                println!(
                    "  Max concurrency:     {} -> {}",
                    current_config.execution.max_concurrent_tasks, desc.default_concurrency
                );
                println!(
                    "  Default timeout:     {}s -> {}s",
                    current_config.execution.default_timeout_seconds, desc.default_timeout_seconds
                );
                println!(
                    "  MCP enabled:         {} -> {}",
                    current_config.integrations.mcp_enabled, desc.mcp_enabled
                );
                println!(
                    "  Prompt optimization: {} -> {}",
                    current_config.integrations.prompt_optimization_enabled,
                    desc.prompt_optimization_enabled
                );
                println!(
                    "  Security scanning:   {} -> {}",
                    current_config.integrations.security_scanning_enabled,
                    desc.security_scanning_enabled
                );
                println!(
                    "  (Unrelated settings like name, log levels, and custom paths will be preserved)"
                );
                return ExitCode::SUCCESS;
            }

            // Create safety backup before mutating
            let paths = PlatformPaths::resolve();
            match BackupEngine::create_backup(
                &target,
                &paths.backup_dir,
                Some(&format!("pre-apply-profile-{kind}")),
            ) {
                Ok(meta) => {
                    println!("[OK] Safety backup created: {}", meta.id);
                }
                Err(e) => {
                    eprintln!("[FAIL] Could not create safety backup before applying profile: {e}");
                    return ExitCode::FAILURE;
                }
            }

            // Apply profile updates (preserving unrelated user fields)
            current_config.apply_profile(kind);

            if let Err(e) = save_config_atomic(&target, &current_config) {
                eprintln!("[FAIL] Failed to save updated configuration: {e}");
                return ExitCode::FAILURE;
            }

            println!(
                "[OK] Successfully applied profile '{}' to configuration.",
                kind
            );
            println!("  Active Profile:      {}", current_config.general.profile);
            println!(
                "  Max Concurrency:     {}",
                current_config.execution.max_concurrent_tasks
            );
            println!(
                "  Default Timeout:     {}s",
                current_config.execution.default_timeout_seconds
            );
            println!(
                "  MCP Enabled:         {}",
                current_config.integrations.mcp_enabled
            );
            println!(
                "  Prompt Optimization: {}",
                current_config.integrations.prompt_optimization_enabled
            );
            println!(
                "  Security Scanning:   {}",
                current_config.integrations.security_scanning_enabled
            );

            ExitCode::SUCCESS
        }
    }
}
