//! `aek restore` command implementation.

use crate::cli::RestoreArgs;
use aek_config::backup::BackupEngine;
use aek_config::loader::resolve_config_file;
use aek_platform::paths::PlatformPaths;
use std::process::ExitCode;

/// Executes the `aek restore` command.
pub fn execute_restore(args: &RestoreArgs) -> ExitCode {
    let current_dir = std::env::current_dir().ok();
    let paths = PlatformPaths::resolve();

    let backup_dir = args.backup_dir.clone().unwrap_or(paths.backup_dir);
    let target_config_file = resolve_config_file(args.config.as_deref(), current_dir.as_deref());

    if let Err(e) = BackupEngine::validate_backup_id(&args.backup_id) {
        eprintln!("[FAIL] Security validation error: {e}");
        return ExitCode::FAILURE;
    }

    if args.dry_run {
        println!("[DRY-RUN] Restoration preview:");
        println!("  Backup identifier:   {}", args.backup_id);
        println!("  Backup directory:    {}", backup_dir.display());
        println!("  Target destination:  {}", target_config_file.display());
        println!("  (No files will be modified in dry-run mode)");
        return ExitCode::SUCCESS;
    }

    println!(
        "Restoring configuration from backup '{}'...",
        args.backup_id
    );
    println!("Target destination: {}", target_config_file.display());

    match BackupEngine::restore_backup(
        &args.backup_id,
        &backup_dir,
        &target_config_file,
        args.dry_run,
    ) {
        Ok(res) => {
            println!("\n[OK] Configuration successfully restored.");
            if let Some(safety_id) = res.safety_backup_id {
                println!("  Safety backup of previous config: {safety_id}");
            }
            println!("  Active config file: {}", res.target_path.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("\n[FAIL] Restoration failed: {e}");
            ExitCode::FAILURE
        }
    }
}
