//! CLI command handler for `aek security`.

use crate::cli::{SecurityArgs, SecurityScanArgs, SecuritySubcommand};
use aek_reporting::sarif::SarifReportGenerator;
use aek_security::doctor::SecurityDoctor;
use aek_security::findings::Severity;
use aek_security::policy::SecurityPolicy;
use aek_security::scanner::SecurityScanner;
use aek_validation::report::ValidationReport;
use anyhow::{Context, Result, bail};
use std::env;
use std::process::ExitCode;

/// Entry point from `main` returning `ExitCode`.
pub fn execute_security(args: &SecurityArgs) -> ExitCode {
    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error initializing async runtime: {e}");
            return ExitCode::FAILURE;
        }
    };

    match rt.block_on(handle_security(args.clone())) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}

/// Executes security subcommands.
pub async fn handle_security(args: SecurityArgs) -> Result<()> {
    match args.action {
        SecuritySubcommand::Scan(scan_args) => handle_scan(scan_args).await,
        SecuritySubcommand::Doctor => handle_doctor().await,
    }
}

async fn handle_scan(args: SecurityScanArgs) -> Result<()> {
    let scan_dir = match args.path {
        Some(p) => p,
        None => env::current_dir().context("Failed to resolve current directory")?,
    };

    // If none of the scanner flags are explicitly passed, enable all three by default
    let (run_semgrep, run_trivy, run_deps) = if !args.semgrep && !args.trivy && !args.dependencies {
        (true, true, true)
    } else {
        (args.semgrep, args.trivy, args.dependencies)
    };

    let policy = SecurityPolicy::default();
    let scanner = SecurityScanner::with_policy(policy);

    let report = scanner
        .scan(&scan_dir, run_semgrep, run_trivy, run_deps, 120)
        .await?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        if report.policy_violated {
            bail!("Security policy violated by one or more findings.");
        }
        return Ok(());
    }

    if args.sarif {
        let dummy_val_report = ValidationReport {
            project_name: "security-scan".to_string(),
            language: "multi".to_string(),
            dry_run: false,
            overall_success: !report.policy_violated,
            passed_count: 0,
            failed_count: report.total_findings,
            skipped_count: 0,
            total_duration_ms: 0,
            results: Vec::new(),
        };
        let sarif = SarifReportGenerator::generate(&dummy_val_report, &report.findings)?;
        println!("{sarif}");
        if report.policy_violated {
            bail!("Security policy violated by one or more findings.");
        }
        return Ok(());
    }

    // Terminal reporting
    println!("\n=== AEK Security Scan Summary ===");
    println!("Target Directory:   {}", scan_dir.display());
    println!("Total Findings:     {}", report.total_findings);
    println!("Critical Severity:  {}", report.critical_count);
    println!("High Severity:      {}", report.high_count);
    println!("Medium Severity:    {}", report.medium_count);
    println!("Low Severity:       {}", report.low_count);
    println!("{:-<60}", "");

    if report.findings.is_empty() {
        println!("No security vulnerabilities or policy violations detected.");
    } else {
        for f in &report.findings {
            let sev_badge = match f.severity {
                Severity::Critical => "[CRITICAL]",
                Severity::High => "[HIGH]",
                Severity::Medium => "[MEDIUM]",
                Severity::Low => "[LOW]",
                _ => "[INFO]",
            };
            println!("{} {} ({})", sev_badge, f.rule_id, f.scanner);
            println!("  {}", f.description);
            if let Some(ref loc) = f.file_path {
                if let Some(line) = f.line_number {
                    println!("  Location: {}:{}", loc.display(), line);
                } else {
                    println!("  Location: {}", loc.display());
                }
            }
            if let Some(ref rem) = f.remediation {
                println!("  Remediation: {}", rem);
            }
        }
    }

    // Report skipped scanners — always print on stderr so CI sees it even when piping.
    if report.incomplete {
        eprintln!(
            "\n[WARN] The following scanners were requested but not found on PATH: {}",
            report.skipped_scanners.join(", ")
        );
        eprintln!(
            "[WARN] Install the missing tools or set `allow_missing_scanners = true` in your security policy to suppress this error."
        );
    }

    println!("{:-<60}\n", "");

    if report.policy_violated {
        bail!("Security scan failed: policy threshold exceeded or scanners missing.");
    }

    Ok(())
}

async fn handle_doctor() -> Result<()> {
    println!("\nAEK Security Toolchain Doctor");
    println!("{:=<75}", "");
    println!(
        "{:<18} {:<12} {:<24} {:<20}",
        "TOOL", "STATUS", "PATH", "PURPOSE"
    );
    println!("{:-<75}", "");

    let doctor = SecurityDoctor::new();
    let report = doctor.audit().await;

    for tool in &report.tools {
        let status_str = if tool.installed { "[PASS]" } else { "[INFO]" };
        let path_display = if let Some(ref p) = tool.path {
            if p.len() > 22 {
                format!("...{}", &p[p.len() - 19..])
            } else {
                p.clone()
            }
        } else {
            "Not on PATH".to_string()
        };

        let ver_suffix = tool.version.as_deref().unwrap_or("");
        println!(
            "{:<18} {:<12} {:<24} {:<20}",
            format!("{} {}", tool.name, ver_suffix),
            status_str,
            path_display,
            if tool.required_for.len() > 20 {
                format!("{}...", &tool.required_for[..17])
            } else {
                tool.required_for.clone()
            }
        );
    }
    println!("{:=<75}\n", "");

    Ok(())
}
