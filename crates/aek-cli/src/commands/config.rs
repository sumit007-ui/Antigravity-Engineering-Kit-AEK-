//! `aek config` command implementation.

use crate::cli::ConfigSubcommand;
use aek_config::loader::{load_config_from_file, resolve_config_file};
use aek_config::validator::validate_config;
use std::process::ExitCode;

/// Executes the `aek config` commands.
pub fn execute_config(action: &ConfigSubcommand) -> ExitCode {
    let current_dir = std::env::current_dir().ok();

    match action {
        ConfigSubcommand::Show { path } => {
            let target = resolve_config_file(path.as_deref(), current_dir.as_deref());
            if !target.exists() {
                eprintln!(
                    "[FAIL] Configuration file not found at: {}",
                    target.display()
                );
                eprintln!("Run `aek init` to initialize default configuration.");
                return ExitCode::FAILURE;
            }

            match std::fs::read_to_string(&target) {
                Ok(content) => {
                    println!("# Active configuration: {}", target.display());
                    println!("{content}");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!(
                        "[FAIL] Failed to read configuration at '{}': {e}",
                        target.display()
                    );
                    ExitCode::FAILURE
                }
            }
        }
        ConfigSubcommand::Validate { path } => {
            let target = resolve_config_file(path.as_deref(), current_dir.as_deref());
            if !target.exists() {
                eprintln!(
                    "[FAIL] Configuration file not found at: {}",
                    target.display()
                );
                eprintln!("Run `aek init` to generate a valid configuration.");
                return ExitCode::FAILURE;
            }

            println!("Validating configuration file: {}", target.display());
            match load_config_from_file(&target) {
                Ok(config) => match validate_config(&config) {
                    Ok(()) => {
                        println!("[PASS] Configuration is valid.");
                        println!("  Profile:     {}", config.general.profile);
                        println!("  Version:     {}", config.general.config_version);
                        println!("  Log Level:   {}", config.logging.level);
                        println!(
                            "  Timeout:     {}s",
                            config.execution.default_timeout_seconds
                        );
                        println!("  Concurrency: {}", config.execution.max_concurrent_tasks);
                        println!("  MCP Enabled: {}", config.integrations.mcp_enabled);
                        ExitCode::SUCCESS
                    }
                    Err(e) => {
                        eprintln!("[FAIL] Configuration validation constraint error: {e}");
                        ExitCode::FAILURE
                    }
                },
                Err(e) => {
                    eprintln!("[FAIL] Configuration syntax or parsing error: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        ConfigSubcommand::Path { path } => {
            let target = resolve_config_file(path.as_deref(), current_dir.as_deref());
            println!("{}", target.display());
            if target.exists() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
    }
}
