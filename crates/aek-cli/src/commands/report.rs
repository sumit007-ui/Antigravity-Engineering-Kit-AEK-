//! CLI command handler for `aek report`.

use crate::cli::{ReportArgs, ReportSubcommand};
use aek_reporting::json::JsonReportGenerator;
use aek_reporting::sarif::SarifReportGenerator;
use aek_reporting::summary::MarkdownSummaryGenerator;
use aek_reporting::terminal::TerminalReportGenerator;
use aek_security::findings::SecurityFinding;
use aek_validation::report::ValidationReport;
use anyhow::{Context, Result, bail};
use std::fs;
use std::process::ExitCode;

/// Entry point from `main` returning `ExitCode`.
pub fn execute_report(args: &ReportArgs) -> ExitCode {
    match handle_report(args.clone()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}

/// Executes report subcommands.
pub fn handle_report(args: ReportArgs) -> Result<()> {
    match args.action {
        ReportSubcommand::Show { report } => {
            let (val_report, findings) = load_report(&report)?;
            print!(
                "{}",
                TerminalReportGenerator::format(&val_report, &findings)
            );
        }
        ReportSubcommand::Export {
            report,
            format,
            output,
        } => {
            let (val_report, findings) = load_report(&report)?;
            let exported = match format.to_ascii_lowercase().as_str() {
                "json" => JsonReportGenerator::generate(&val_report, &findings)?,
                "sarif" => SarifReportGenerator::generate(&val_report, &findings)?,
                "markdown" | "md" => MarkdownSummaryGenerator::generate(&val_report, &findings),
                other => bail!(
                    "Unsupported export format: '{other}'. Choose 'json', 'sarif', or 'markdown'."
                ),
            };

            if let Some(out_path) = output {
                fs::write(&out_path, &exported)
                    .with_context(|| format!("Failed to write to '{}'", out_path.display()))?;
                println!("Exported report to: {}", out_path.display());
            } else {
                println!("{exported}");
            }
        }
    }
    Ok(())
}

fn load_report(path: &std::path::Path) -> Result<(ValidationReport, Vec<SecurityFinding>)> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read report file '{}'", path.display()))?;

    // Try parsing as combined JSON report
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
        let findings: Vec<SecurityFinding> = val["security_findings"]
            .as_array()
            .and_then(|arr| serde_json::from_value(serde_json::Value::Array(arr.clone())).ok())
            .unwrap_or_default();

        let val_report: ValidationReport = if val.get("project_name").is_some() {
            serde_json::from_str(&content).unwrap_or_else(|_| dummy_report())
        } else {
            dummy_report()
        };

        return Ok((val_report, findings));
    }

    bail!("File '{}' is not valid AEK report JSON.", path.display());
}

fn dummy_report() -> ValidationReport {
    ValidationReport {
        project_name: "security-audit".to_string(),
        language: "multi".to_string(),
        dry_run: false,
        overall_success: true,
        passed_count: 0,
        failed_count: 0,
        skipped_count: 0,
        total_duration_ms: 0,
        results: Vec::new(),
    }
}
