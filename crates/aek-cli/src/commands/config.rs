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
        ConfigSubcommand::Migrate {
            check,
            dry_run,
            path,
        } => {
            let target = resolve_config_file(path.as_deref(), current_dir.as_deref());
            if !target.exists() {
                eprintln!(
                    "[FAIL] Configuration file not found at: {}",
                    target.display()
                );
                return ExitCode::FAILURE;
            }

            let mut migrator = aek_distribution::migration::ConfigMigrator::new();
            // Default migration step for schema v1 -> v2
            migrator.register_step(aek_distribution::migration::ConfigMigrationStep {
                from_version: 1,
                to_version: 2,
                description: "Ensure schema_version is tracked explicitly".to_string(),
                migrate_fn: Box::new(|mut val| {
                    if let toml::Value::Table(ref mut map) = val {
                        map.insert("schema_version".to_string(), toml::Value::Integer(2));
                    }
                    Ok(val)
                }),
            });

            match migrator.plan_migration(&target, 2) {
                Ok(plan) => {
                    println!("Configuration Migration Status for '{}':", target.display());
                    println!("  Current Schema Version: {}", plan.from_version);
                    println!("  Target Schema Version:  {}", plan.to_version);
                    println!("  Pending Migration Steps: {}", plan.steps.len());

                    if *check {
                        if plan.steps.is_empty() {
                            println!("[PASS] Configuration is up-to-date. No migrations required.");
                            return ExitCode::SUCCESS;
                        } else {
                            println!("[INFO] Migrations are pending.");
                            return ExitCode::SUCCESS;
                        }
                    }

                    if *dry_run {
                        println!(
                            "\n[DRY RUN] Would execute {} migration step(s).",
                            plan.steps.len()
                        );
                        println!(
                            "[DRY RUN] Safety backup would be created at: {}.bak",
                            target.display()
                        );
                        return ExitCode::SUCCESS;
                    }

                    if plan.steps.is_empty() {
                        println!("Configuration is already on the latest schema version.");
                        return ExitCode::SUCCESS;
                    }

                    match migrator.execute_migration(&target, &plan) {
                        Ok(()) => {
                            println!(
                                "[PASS] Successfully migrated configuration to schema version {}.",
                                plan.to_version
                            );
                            ExitCode::SUCCESS
                        }
                        Err(e) => {
                            eprintln!("[FAIL] Configuration migration failed: {e}");
                            ExitCode::FAILURE
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[FAIL] Failed to plan configuration migration: {e}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}
