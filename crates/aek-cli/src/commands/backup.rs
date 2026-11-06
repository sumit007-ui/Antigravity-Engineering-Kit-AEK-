//! `aek backup` command implementation.

use crate::cli::{BackupArgs, BackupSubcommand};
use aek_config::backup::BackupEngine;
use aek_config::loader::resolve_config_file;
use aek_platform::paths::PlatformPaths;
use std::process::ExitCode;

/// Executes the `aek backup` command.
pub fn execute_backup(args: &BackupArgs) -> ExitCode {
    let current_dir = std::env::current_dir().ok();
    let paths = PlatformPaths::resolve();

    let backup_dir = args.backup_dir.clone().unwrap_or(paths.backup_dir);
    let config_file = resolve_config_file(args.config.as_deref(), current_dir.as_deref());

    match &args.action {
        Some(BackupSubcommand::List) => {
            println!("Listing backups in: {}\n", backup_dir.display());
            match BackupEngine::list_backups(&backup_dir) {
                Ok(backups) if backups.is_empty() => {
                    println!("No backups found in directory.");
                    ExitCode::SUCCESS
                }
                Ok(backups) => {
                    println!(
                        "{:<36}  {:<24}  {:<12}  {:<10}",
                        "BACKUP ID", "CREATED AT", "PROFILE", "SIZE"
                    );
                    println!("{:-<36}  {:-<24}  {:-<12}  {:-<10}", "", "", "", "");
                    for b in backups {
                        println!(
                            "{:<36}  {:<24}  {:<12}  {:<10}",
                            b.id,
                            b.created_at,
                            b.profile,
                            format!("{} B", b.file_size)
                        );
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("[FAIL] Failed to list backups: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Some(BackupSubcommand::Create { reason }) => {
            do_create_backup(&config_file, &backup_dir, reason.as_deref())
        }
        None => {
            // Default `aek backup` creates a new backup
            do_create_backup(&config_file, &backup_dir, None)
        }
    }
}

fn do_create_backup(
    config_file: &std::path::Path,
    backup_dir: &std::path::Path,
    reason: Option<&str>,
) -> ExitCode {
    if !config_file.exists() {
        eprintln!(
            "[FAIL] Cannot create backup: configuration file does not exist at '{}'",
            config_file.display()
        );
        eprintln!("Run `aek init` first to initialize AEK configuration.");
        return ExitCode::FAILURE;
    }

    println!("Creating backup of: {}", config_file.display());
    match BackupEngine::create_backup(config_file, backup_dir, reason) {
        Ok(meta) => {
            println!("[OK] Backup created successfully.");
            println!("  Backup ID:   {}", meta.id);
            println!("  Created At:  {}", meta.created_at);
            println!("  Profile:     {}", meta.profile);
            println!("  File Size:   {} bytes", meta.file_size);
            println!("  Location:    {}", meta.path.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[FAIL] Backup creation failed: {e}");
            ExitCode::FAILURE
        }
    }
}
