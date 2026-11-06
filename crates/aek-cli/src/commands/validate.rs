//! CLI command handler for `aek validate`.

use crate::cli::{ValidateArgs, ValidatePolicySubcommand, ValidateSubcommand};
use aek_languages::detection::ProjectDetector;
use aek_reporting::terminal::TerminalReportGenerator;
use aek_security::findings::SecurityFinding;
use aek_validation::executor::ValidationExecutor;
use aek_validation::incremental::GitChangeDetector;
use aek_validation::planner::ValidationPlanner;
use aek_validation::policy::ValidationPolicy;
use aek_validation::quality_gate::QualityGateEngine;
use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Entry point from `main` returning `ExitCode`.
pub fn execute_validate(args: &ValidateArgs) -> ExitCode {
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error initializing async runtime: {e}");
            return ExitCode::FAILURE;
        }
    };

    match rt.block_on(handle_validate(args.clone())) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}

/// Executes project-aware validation pipelines across detected projects.
pub async fn handle_validate(args: ValidateArgs) -> Result<()> {
    // Handle subcommands if specified (e.g. `aek validate policy`)
    if let Some(subcmd) = args.action {
        match subcmd {
            ValidateSubcommand::Policy { action } => return handle_policy_subcommand(action),
        }
    }

    let scan_dir = match args.path {
        Some(p) => p,
        None => env::current_dir().context("Failed to resolve current directory")?,
    };

    let detector = ProjectDetector::new();
    let detected_projects = detector.detect(&scan_dir)?;

    if detected_projects.is_empty() {
        if args.json {
            println!("[]");
        } else {
            println!(
                "\nNo recognized programming language projects found in '{}'.",
                scan_dir.display()
            );
            println!(
                "Supported markers: Cargo.toml, go.mod, CMakeLists.txt, build.zig, pyproject.toml, package.json, pubspec.yaml, package.xml\n"
            );
        }
        return Ok(());
    }

    // Incremental validation filtering if `--changed` is active
    let projects = if args.changed {
        let git_detector = GitChangeDetector::new();
        if !git_detector.is_git_repo(&scan_dir) {
            println!(
                "\n[INFO] Directory '{}' is not a Git repository; running full scan.",
                scan_dir.display()
            );
            detected_projects
        } else {
            let changed_files = if let Some(ref base) = args.base {
                git_detector.detect_changed_files_with_base(&scan_dir, base)?
            } else {
                git_detector.detect_changed_files(&scan_dir)?
            };

            if changed_files.is_empty() {
                if args.json {
                    println!("[]");
                } else {
                    println!(
                        "\nNo modified files detected relative to Git HEAD. Validation passed (0 checks required).\n"
                    );
                }
                return Ok(());
            }

            let filtered =
                git_detector.filter_affected_projects(&detected_projects, &changed_files);
            if filtered.is_empty() {
                if args.json {
                    println!("[]");
                } else {
                    println!(
                        "\nChanged files do not affect any detected language projects. Validation skipped.\n"
                    );
                }
                return Ok(());
            }
            filtered
        }
    } else {
        detected_projects
    };

    // Resolve validation policy
    let mut policy = resolve_policy(args.policy.as_deref())?;
    policy.dry_run = args.dry_run;
    if args.full {
        policy.full_validation = true;
    }

    let planner = ValidationPlanner::new();
    let pipelines = planner.create_plan(&projects, &policy, args.changed);

    let executor = ValidationExecutor::new(policy.clone());
    let gate_engine = QualityGateEngine::new(policy.clone());

    let mut reports = Vec::new();
    let mut quality_failures = Vec::new();

    for pipeline in &pipelines {
        let report = executor.execute(pipeline).await?;
        let gate_result = gate_engine.evaluate(&report, &[]);
        if !gate_result.passed {
            quality_failures.push((pipeline.project_name.clone(), gate_result.failure_reasons));
        }
        reports.push(report);
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&reports)?);
    } else {
        for report in &reports {
            let empty_findings: [SecurityFinding; 0] = [];
            print!(
                "{}",
                TerminalReportGenerator::format(report, &empty_findings)
            );
        }
    }

    if !quality_failures.is_empty() && !args.dry_run {
        println!("\n❌ Quality Gate Violations:");
        for (proj, reasons) in &quality_failures {
            println!("  Project '{}':", proj);
            for r in reasons {
                println!("    - {r}");
            }
        }
        println!();
        bail!("Validation failed quality gate checks.");
    }

    Ok(())
}

fn resolve_policy(policy_name_or_path: Option<&str>) -> Result<ValidationPolicy> {
    match policy_name_or_path {
        Some("strict") => Ok(ValidationPolicy::strict()),
        Some("ci") => Ok(ValidationPolicy::ci()),
        Some("default") | None => Ok(ValidationPolicy::default()),
        Some(custom) => {
            let path = PathBuf::from(custom);
            if path.is_file() {
                ValidationPolicy::load_from_file(&path)
                    .with_context(|| format!("Failed to load policy file '{}'", path.display()))
            } else {
                // Check in config/validation/
                let candidate = Path::new("config/validation").join(format!("{custom}.toml"));
                if candidate.is_file() {
                    ValidationPolicy::load_from_file(&candidate).with_context(|| {
                        format!("Failed to load policy file '{}'", candidate.display())
                    })
                } else {
                    bail!("Policy '{custom}' not found as preset or valid file path");
                }
            }
        }
    }
}

fn handle_policy_subcommand(action: ValidatePolicySubcommand) -> Result<()> {
    match action {
        ValidatePolicySubcommand::List => {
            println!("\nAvailable Validation Policies:");
            println!("  - default  : Standard developer policy (balanced timeouts, fail on error)");
            println!(
                "  - strict   : Strict release policy (fail-fast, full validation, fail on medium)"
            );
            println!("  - ci       : Automated CI policy (extended timeouts, full matrix)\n");
        }
        ValidatePolicySubcommand::Show { name } => match name.as_str() {
            "default" => {
                let toml = toml::to_string_pretty(&ValidationPolicy::default())?;
                println!("\n=== Policy: default ===\n{toml}");
            }
            "strict" => {
                let toml = toml::to_string_pretty(&ValidationPolicy::strict())?;
                println!("\n=== Policy: strict ===\n{toml}");
            }
            "ci" => {
                let toml = toml::to_string_pretty(&ValidationPolicy::ci())?;
                println!("\n=== Policy: ci ===\n{toml}");
            }
            other => {
                let path = Path::new("config/validation").join(format!("{other}.toml"));
                if path.is_file() {
                    let content = fs::read_to_string(&path)?;
                    println!("\n=== Policy: {other} ===\n{content}");
                } else {
                    bail!("Policy '{other}' not found.");
                }
            }
        },
        ValidatePolicySubcommand::Check { file } => {
            if !file.is_file() {
                bail!("File '{}' does not exist.", file.display());
            }
            let policy = ValidationPolicy::load_from_file(&file)?;
            println!("\n[PASS] Policy file '{}' is valid.", file.display());
            println!("  Default Timeout:  {}s", policy.default_timeout_secs);
            println!("  Fail Fast:        {}", policy.fail_fast);
            println!("  Full Validation:  {}", policy.full_validation);
            println!("  Max Concurrency:  {}\n", policy.max_concurrent_tasks);
        }
    }
    Ok(())
}
