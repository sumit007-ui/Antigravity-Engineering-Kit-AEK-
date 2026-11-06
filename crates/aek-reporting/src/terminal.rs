//! Terminal report formatter with human-readable status indicators.

use aek_security::findings::{SecurityFinding, Severity};
use aek_validation::report::{StepStatus, ValidationReport};

/// Formatter generating clean terminal output for validation steps and security findings.
#[derive(Debug, Default, Clone)]
pub struct TerminalReportGenerator;

impl TerminalReportGenerator {
    /// Formats validation outcomes and security findings into a terminal summary.
    #[must_use]
    pub fn format(report: &ValidationReport, findings: &[SecurityFinding]) -> String {
        let mut out = String::new();

        out.push_str(&format!(
            "\n=== AEK Validation: {} ({}) ===\n",
            report.project_name, report.language
        ));
        out.push_str(&format!(
            "Overall Status:     {}\n",
            if report.overall_success {
                "PASSED"
            } else {
                "FAILED"
            }
        ));
        if report.dry_run {
            out.push_str("Mode:               DRY RUN (no commands executed)\n");
        }
        out.push_str(&format!("Executed Checks:    {}\n", report.results.len()));
        out.push_str(&format!("Passed:             {}\n", report.passed_count));
        out.push_str(&format!("Failed:             {}\n", report.failed_count));
        out.push_str(&format!("Skipped/Warned:     {}\n", report.skipped_count));
        out.push_str(&format!(
            "Duration:           {} ms\n",
            report.total_duration_ms
        ));
        out.push_str(&format!("{:-<70}\n", ""));

        out.push_str(&format!(
            "{:<30} {:<18} {:<10} {:<10}\n",
            "STEP", "STAGE", "STATUS", "DURATION"
        ));
        out.push_str(&format!("{:-<70}\n", ""));

        for step in &report.results {
            let status_str = match step.status {
                StepStatus::Pass => "[PASS]",
                StepStatus::Fail => "[FAIL]",
                StepStatus::Warn => "[WARN]",
                StepStatus::Skip => "[SKIP]",
                StepStatus::Error => "[ERROR]",
            };
            out.push_str(&format!(
                "{:<30} {:<18} {:<10} {:>6} ms\n",
                if step.name.len() > 28 {
                    format!("{}...", &step.name[..25])
                } else {
                    step.name.clone()
                },
                step.stage.to_string(),
                status_str,
                step.duration_ms
            ));
        }
        out.push_str(&format!("{:-<70}\n", ""));

        if !findings.is_empty() {
            out.push_str(&format!(
                "\n=== Security Findings ({} total) ===\n",
                findings.len()
            ));
            for f in findings {
                let sev_prefix = match f.severity {
                    Severity::Critical => "[CRITICAL]",
                    Severity::High => "[HIGH]",
                    Severity::Medium => "[MEDIUM]",
                    Severity::Low => "[LOW]",
                    _ => "[INFO]",
                };

                let loc = match (&f.file_path, f.line_number) {
                    (Some(p), Some(l)) => format!(" ({}:{})", p.display(), l),
                    (Some(p), None) => format!(" ({})", p.display()),
                    _ => String::new(),
                };

                out.push_str(&format!("{} {}{}\n", sev_prefix, f.rule_id, loc));
                out.push_str(&format!("  {}\n", f.description));
                if let Some(ref rem) = f.remediation {
                    out.push_str(&format!("  Remediation: {}\n", rem));
                }
            }
            out.push('\n');
        }

        out
    }
}
