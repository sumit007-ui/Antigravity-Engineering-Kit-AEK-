//! CLI command handler for `aek validate`.

use crate::cli::ValidateArgs;
use crate::commands::language::print_validation_report;
use aek_languages::detection::ProjectDetector;
use aek_validation::executor::ValidationExecutor;
use aek_validation::pipeline::ValidationPipeline;
use aek_validation::policy::ValidationPolicy;
use anyhow::{Context, Result};
use std::env;

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
    let scan_dir = match args.path {
        Some(p) => p,
        None => env::current_dir().context("Failed to resolve current directory")?,
    };

    let detector = ProjectDetector::new();
    let projects = detector.detect(&scan_dir)?;

    if projects.is_empty() {
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

    let policy = ValidationPolicy {
        dry_run: args.dry_run,
        full_validation: args.full,
        ..Default::default()
    };

    let executor = ValidationExecutor::new(policy);
    let mut reports = Vec::new();

    for project in &projects {
        let pipeline = ValidationPipeline::for_project(project);
        let report = executor.execute(&pipeline).await?;
        reports.push(report);
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&reports)?);
        return Ok(());
    }

    for report in &reports {
        print_validation_report(report);
    }

    let all_succeeded = reports.iter().all(|r| r.overall_success);
    if !all_succeeded && !args.dry_run {
        anyhow::bail!("One or more validation steps failed.");
    }

    Ok(())
}
