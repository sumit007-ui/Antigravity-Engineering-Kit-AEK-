//! Markdown summary report generator for CI/CD step summaries and agent contexts.

use aek_security::findings::{SecurityFinding, Severity};
use aek_validation::report::{StepStatus, ValidationReport};

/// Formatter generating GitHub Flavored Markdown summaries.
#[derive(Debug, Default, Clone)]
pub struct MarkdownSummaryGenerator;

impl MarkdownSummaryGenerator {
    /// Generates Markdown summary report.
    #[must_use]
    pub fn generate(report: &ValidationReport, findings: &[SecurityFinding]) -> String {
        let mut md = String::new();

        let status_badge = if report.overall_success {
            "✅ **PASSED**"
        } else {
            "❌ **FAILED**"
        };

        md.push_str(&format!(
            "# AEK Validation & Security Summary: {}\n\n",
            report.project_name
        ));
        md.push_str(&format!("- **Language**: `{}`\n", report.language));
        md.push_str(&format!("- **Overall Result**: {}\n", status_badge));
        md.push_str(&format!(
            "- **Checks Executed**: {} (Passed: {}, Failed: {}, Skipped: {})\n",
            report.results.len(),
            report.passed_count,
            report.failed_count,
            report.skipped_count
        ));
        md.push_str(&format!(
            "- **Total Duration**: {} ms\n\n",
            report.total_duration_ms
        ));

        md.push_str("## Validation Steps\n\n");
        md.push_str("| Step | Stage | Status | Duration |\n");
        md.push_str("|---|---|---|---|\n");

        for step in &report.results {
            let status_icon = match step.status {
                StepStatus::Pass => "✅ Pass",
                StepStatus::Fail => "❌ Fail",
                StepStatus::Warn => "⚠️ Warn",
                StepStatus::Skip => "⏭️ Skip",
                StepStatus::Error => "💥 Error",
            };
            md.push_str(&format!(
                "| `{}` | {} | {} | {} ms |\n",
                step.name, step.stage, status_icon, step.duration_ms
            ));
        }

        if !findings.is_empty() {
            md.push_str(&format!(
                "\n## Security Findings ({} total)\n\n",
                findings.len()
            ));
            md.push_str("| Severity | Rule / Advisory | Description | Location |\n");
            md.push_str("|---|---|---|---|\n");

            for f in findings {
                let sev_badge = match f.severity {
                    Severity::Critical => "🔴 Critical",
                    Severity::High => "🟠 High",
                    Severity::Medium => "🟡 Medium",
                    Severity::Low => "🔵 Low",
                    _ => "⚪ Info",
                };
                let loc = match (&f.file_path, f.line_number) {
                    (Some(p), Some(l)) => format!("`{}:{}`", p.display(), l),
                    (Some(p), None) => format!("`{}`", p.display()),
                    _ => "-".to_string(),
                };
                md.push_str(&format!(
                    "| {} | `{}` | {} | {} |\n",
                    sev_badge, f.rule_id, f.description, loc
                ));
            }
        }

        md
    }
}
