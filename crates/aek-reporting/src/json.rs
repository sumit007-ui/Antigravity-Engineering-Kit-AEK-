//! JSON format report serializer.

use aek_security::findings::SecurityFinding;
use aek_validation::report::ValidationReport;
use chrono::Utc;
use serde_json::json;

/// Serializer generating machine-readable JSON validation reports.
#[derive(Debug, Default, Clone)]
pub struct JsonReportGenerator;

impl JsonReportGenerator {
    /// Generates structured JSON report string combining validation and security findings.
    pub fn generate(
        report: &ValidationReport,
        findings: &[SecurityFinding],
    ) -> Result<String, serde_json::Error> {
        let payload = json!({
            "version": "1.0.0",
            "timestamp": Utc::now().to_rfc3339(),
            "project_name": report.project_name,
            "language": report.language,
            "dry_run": report.dry_run,
            "overall_success": report.overall_success,
            "passed_count": report.passed_count,
            "failed_count": report.failed_count,
            "skipped_count": report.skipped_count,
            "total_duration_ms": report.total_duration_ms,
            "steps": report.results,
            "security_findings": findings,
        });

        serde_json::to_string_pretty(&payload)
    }
}

