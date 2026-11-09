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

    // Parse as combined JSON report
    let val: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("File '{}' is not valid JSON", path.display()))?;

    let findings: Vec<SecurityFinding> = val
        .get("security_findings")
        .and_then(|f| f.as_array())
        .and_then(|arr| serde_json::from_value(serde_json::Value::Array(arr.clone())).ok())
        .unwrap_or_default();

    // Must have valid ValidationReport fields, or fail honestly with file path and reason
    let val_report: ValidationReport = serde_json::from_str(&content).with_context(|| {
        format!(
            "File '{}' does not match AEK ValidationReport schema (missing or invalid report fields)",
            path.display()
        )
    })?;

    Ok((val_report, findings))
}
