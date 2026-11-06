//! `aek init` command implementation.

use crate::cli::InitArgs;
use aek_config::backup::BackupEngine;
use aek_config::model::AekConfig;
use aek_config::writer::save_config_atomic;
use aek_platform::detection::detect_os;
use aek_platform::paths::PlatformPaths;
use std::fs;
use std::process::ExitCode;

/// Executes the `aek init` command.
pub fn execute_init(args: &InitArgs) -> ExitCode {
    let os = detect_os();
    let paths = PlatformPaths::resolve();

    let target_config_file = args.path.clone().unwrap_or(paths.config_file);
    let target_config_dir = target_config_file
        .parent()
        .map(std::path::Path::to_path_buf)
        .unwrap_or(paths.config_dir.clone());

    println!("Detected platform: {os}");
    println!("Configuration path: {}", target_config_file.display());

    if args.dry_run {
        println!("\n[DRY-RUN] The following actions would be performed:");
        println!(
            "  - Create configuration directory: {}",
            target_config_dir.display()
        );
        println!(
            "  - Create data directory:          {}",
            paths.data_dir.display()
        );
        println!(
            "  - Create backup directory:        {}",
            paths.backup_dir.display()
        );
        println!(
            "  - Create cache directory:         {}",
            paths.cache_dir.display()
        );
        println!(
            "  - Write default LIGHT profile to: {}",
            target_config_file.display()
        );
        return ExitCode::SUCCESS;
    }

    // Check if configuration already exists
    if target_config_file.exists() {
        if !args.force {
            println!(
                "\nAEK is already initialized at '{}'.",
                target_config_file.display()
            );
            println!(
                "Existing configuration has been preserved. Use `--force` to reinitialize with backup."
            );
            return ExitCode::SUCCESS;
        }

        // Force mode: MUST NOT silently destroy data; create verified backup first!
        println!(
            "\nExisting configuration found. Creating safety backup prior to forced reinitialization..."
        );
        match BackupEngine::create_backup(
            &target_config_file,
            &paths.backup_dir,
            Some("pre-init-force-backup"),
        ) {
            Ok(backup_meta) => {
                println!(
                    "  [OK] Safety backup created: {} ({})",
                    backup_meta.id,
                    backup_meta.path.display()
                );
            }
            Err(e) => {
                eprintln!("[FAIL] Could not create safety backup of existing configuration: {e}");
                eprintln!("Initialization aborted to prevent data loss.");
                return ExitCode::FAILURE;
            }
        }
    }

    // Create required directories
    let dirs_to_create = [
        &target_config_dir,
        &paths.data_dir,
        &paths.backup_dir,
        &paths.cache_dir,
    ];

    for dir in dirs_to_create {
        if !dir.exists() {
            if let Err(e) = fs::create_dir_all(dir) {
                eprintln!("[FAIL] Failed to create directory '{}': {e}", dir.display());
                return ExitCode::FAILURE;
            }
        }
    }

    // Generate default configuration (LIGHT profile)
    let default_config = AekConfig::default();
    if let Err(e) = save_config_atomic(&target_config_file, &default_config) {
        eprintln!("[FAIL] Failed to write configuration file: {e}");
        return ExitCode::FAILURE;
    }

    println!("\n[OK] AEK initialized successfully with default LIGHT profile.");
    println!("Configuration saved to: {}", target_config_file.display());
    println!("Run `aek doctor` to verify environment diagnostics.");

    ExitCode::SUCCESS
}
