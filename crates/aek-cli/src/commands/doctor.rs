//! `aek doctor` command implementation.

use crate::cli::DoctorArgs;
use aek_config::loader::load_config_from_file;
use aek_core::diagnostics::{
    DiagnosticCheck, DiagnosticReport, DiagnosticStatus, DiagnosticsRunner,
};
use aek_platform::paths::PlatformPaths;
use std::path::Path;
use std::process::ExitCode;

/// Executes the `aek doctor` command.
pub fn execute_doctor(args: &DoctorArgs, verbose: bool) -> ExitCode {
    let current_dir = std::env::current_dir().ok();
    let workspace_ref = args.workspace.as_deref().or(current_dir.as_deref());

    let paths = PlatformPaths::resolve();
    let config_valid = if paths.config_file.exists() {
        Some(load_config_from_file(&paths.config_file).is_ok())
    } else {
        None
    };

    let report = DiagnosticsRunner::run_all(workspace_ref, config_valid);

    if args.json {
        match serde_json::to_string_pretty(&report) {
            Ok(json_output) => {
                println!("{json_output}");
                return if report.is_healthy() {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                };
            }
            Err(e) => {
                eprintln!("Error serializing diagnostic report to JSON: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    render_human_report(&report, workspace_ref, verbose);

    if report.is_healthy() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn render_human_report(report: &DiagnosticReport, workspace: Option<&Path>, verbose: bool) {
    println!("============================================================");
    println!(
        "     Antigravity Engineering Kit (AEK) Doctor (v{})",
        env!("CARGO_PKG_VERSION")
    );
    println!("============================================================");

    if let Some(ws) = workspace {
        println!("Workspace: {}", ws.display());
    }
    println!("Platform:  {} ({})", report.os, report.arch);
    println!("------------------------------------------------------------");

    let categories = [
        "Platform",
        "Core Toolchain",
        "Optional Runtimes",
        "Configuration",
        "Environment",
    ];

    for cat in categories {
        let cat_checks: Vec<&DiagnosticCheck> =
            report.checks.iter().filter(|c| c.category == cat).collect();
        if cat_checks.is_empty() {
            continue;
        }

        println!("\n[{cat}]");
        for check in cat_checks {
            match &check.status {
                DiagnosticStatus::Pass => {
                    let details_str = check
                        .details
                        .as_deref()
                        .map(|d| format!(" - {d}"))
                        .unwrap_or_default();
                    println!("  [PASS] {: <24} {}", check.name, details_str);
                }
                DiagnosticStatus::Warn { message } => {
                    println!("  [WARN] {: <24} - {message}", check.name);
                    if verbose {
                        if let Some(ref details) = check.details {
                            println!("         -> Details: {details}");
                        }
                    }
                }
                DiagnosticStatus::Fail {
                    reason,
                    remediation,
                } => {
                    println!("  [FAIL] {: <24} - {reason}", check.name);
                    if let Some(rem) = remediation {
                        println!("         -> Remediation: {rem}");
                    }
                }
                DiagnosticStatus::Skip { reason } => {
                    println!("  [SKIP] {: <24} - {reason}", check.name);
                }
            }
        }
    }

    println!("\n============================================================");
    let total = report.total_count();
    let pass = report.pass_count();
    let warn = report.warn_count();
    let fail = report.fail_count();
    let skip = report.skip_count();

    println!(
        "Summary: {total} checks: {pass} passed, {warn} warnings, {fail} failed, {skip} skipped."
    );

    if report.is_healthy() {
        if warn > 0 {
            println!("Status:  HEALTHY (with non-blocking warnings)");
        } else {
            println!("Status:  HEALTHY - Environment is ready.");
        }
    } else {
        println!("Status:  UNHEALTHY - Please address the failed checks above.");
    }
    println!("============================================================");
}
