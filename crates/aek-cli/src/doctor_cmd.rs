//! Doctor subcommand execution and report rendering.

use crate::args::DoctorArgs;
use aek_core::doctor::{CheckStatus, DoctorCategory, DoctorReport, DoctorRunner};
use std::path::Path;
use std::process::ExitCode;

/// Executes the `aek doctor` command.
pub fn execute_doctor(args: &DoctorArgs) -> ExitCode {
    let current_dir = std::env::current_dir().ok();
    let workspace_ref = args.workspace.as_deref().or(current_dir.as_deref());

    let report = DoctorRunner::run_all(workspace_ref);

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
                eprintln!("Error serializing doctor report to JSON: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    render_human_report(&report, workspace_ref);

    if report.is_healthy() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn render_human_report(report: &DoctorReport, workspace: Option<&Path>) {
    println!("============================================================");
    println!(
        "     Antigravity Engineering Kit (AEK) Doctor (v{})",
        env!("CARGO_PKG_VERSION")
    );
    println!("============================================================");

    if let Some(ws) = workspace {
        println!("Workspace Root: {}", ws.display());
    }
    println!(
        "Host Platform:  {}-{} ({} cores)",
        report.platform.os, report.platform.arch, report.platform.logical_cores
    );
    println!("------------------------------------------------------------");

    let categories = [
        (DoctorCategory::Platform, "System & Platform"),
        (DoctorCategory::Toolchain, "Core Toolchain"),
        (DoctorCategory::VersionControl, "Version Control"),
        (DoctorCategory::Antigravity, "Antigravity Environment"),
        (DoctorCategory::Runtime, "Optional Runtimes"),
        (DoctorCategory::Configuration, "Configuration"),
    ];

    for (cat, title) in categories {
        let cat_checks: Vec<_> = report.checks.iter().filter(|c| c.category == cat).collect();
        if cat_checks.is_empty() {
            continue;
        }

        println!("\n[{title}]");
        for check in cat_checks {
            match &check.status {
                CheckStatus::Ok => {
                    let details_str = check
                        .details
                        .as_deref()
                        .map(|d| format!(" - {d}"))
                        .unwrap_or_default();
                    println!("  [OK]   {: <28} {}", check.name, details_str);
                }
                CheckStatus::Warning { message } => {
                    println!("  [WARN] {: <28} - {message}", check.name);
                }
                CheckStatus::Failed {
                    reason,
                    remediation,
                } => {
                    println!("  [FAIL] {: <28} - {reason}", check.name);
                    if let Some(rem) = remediation {
                        println!("         -> Remediation: {rem}");
                    }
                }
            }
        }
    }

    println!("\n============================================================");
    let total = report.total_count();
    let ok = report.ok_count();
    let warn = report.warning_count();
    let fail = report.failed_count();

    println!("Summary: {total} checks evaluated: {ok} passed, {warn} warnings, {fail} failed.");

    if report.is_healthy() {
        if warn > 0 {
            println!("Status: HEALTHY (with non-blocking warnings)");
        } else {
            println!("Status: HEALTHY - Environment is ready for ruthless precision engineering.");
        }
    } else {
        println!("Status: UNHEALTHY - Please remediate the failed checks above.");
    }
    println!("============================================================");
}
