//! Config subcommand execution.

use crate::args::ConfigSubcommand;
use aek_config::AekConfig;
use std::process::ExitCode;

/// Executes configuration subcommands.
pub fn execute_config(action: &ConfigSubcommand) -> ExitCode {
    match action {
        ConfigSubcommand::Validate { path } => {
            let cwd = std::env::current_dir().ok();
            let config_path = AekConfig::find_config_path(path.as_deref(), cwd.as_deref());

            let path_to_load = match (path, config_path) {
                (Some(explicit), _) => explicit.clone(),
                (None, Some(discovered)) => discovered,
                (None, None) => {
                    println!(
                        "No configuration file found in workspace or standard global locations."
                    );
                    println!("Run `aek config init` to generate a new `aek.toml` configuration.");
                    return ExitCode::SUCCESS;
                }
            };

            println!("Validating configuration file: {}", path_to_load.display());
            match AekConfig::load_from_file(&path_to_load) {
                Ok(config) => {
                    println!("[OK] Configuration schema is valid.");
                    println!("     Schema Version:  {}", config.schema_version);
                    println!("     Execution Mode:  {:?}", config.execution_mode);
                    println!("     Log Level:       {}", config.general.log_level);
                    println!("     Timeout:         {}s", config.general.timeout_seconds);
                    println!(
                        "     Auto-Sync Rules: {}",
                        config.antigravity.auto_sync_rules
                    );
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("[FAIL] Configuration validation failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        ConfigSubcommand::Init { path, force } => {
            if path.exists() && !force {
                eprintln!(
                    "Error: Configuration file already exists at '{}'. Use `--force` to overwrite.",
                    path.display()
                );
                return ExitCode::FAILURE;
            }

            let default_config = AekConfig::default();
            match default_config.save_to_file(path) {
                Ok(()) => {
                    println!(
                        "Successfully initialized configuration at: {}",
                        path.display()
                    );
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Failed to write configuration file: {e}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}
