//! OASIS Static Analysis Results Interchange Format (SARIF) v2.1.0 report generator.

use aek_security::findings::{SecurityFinding, Severity};
use aek_validation::report::ValidationReport;
use serde_json::json;

/// Serializer generating OASIS SARIF v2.1.0 documents.
#[derive(Debug, Default, Clone)]
pub struct SarifReportGenerator;

impl SarifReportGenerator {
    /// Generates standard SARIF v2.1.0 JSON representing static analysis and security findings.
    pub fn generate(
        report: &ValidationReport,
        findings: &[SecurityFinding],
    ) -> Result<String, serde_json::Error> {
        let mut rules = Vec::new();
        let mut results = Vec::new();

        for f in findings {
            // Rule definition
            rules.push(json!({
                "id": f.rule_id,
                "name": f.rule_id.replace(['.', '-', '_'], " "),
                "shortDescription": { "text": f.description },
                "help": { "text": f.remediation.as_deref().unwrap_or("Inspect code against security best practices") }
            }));

            // Level mapping
            let level = match f.severity {
                Severity::Critical | Severity::High => "error",
                Severity::Medium | Severity::Low => "warning",
                _ => "note",
            };

            let mut location_json = serde_json::Map::new();
            if let Some(ref path) = f.file_path {
                let uri = path.to_string_lossy().replace('\\', "/");
                let mut region_json = serde_json::Map::new();
                if let Some(line) = f.line_number {
                    region_json.insert("startLine".to_string(), json!(line));
                }

                location_json.insert(
                    "physicalLocation".to_string(),
                    json!({
                        "artifactLocation": {
                            "uri": uri
                        },
                        "region": region_json
                    }),
                );
            }

            let mut result_obj = json!({
                "ruleId": f.rule_id,
                "level": level,
                "message": { "text": f.description }
            });

            if !location_json.is_empty() {
                result_obj
                    .as_object_mut()
                    .unwrap()
                    .insert("locations".to_string(), json!([location_json]));
            }

            results.push(result_obj);
        }

        let sarif_payload = json!({
            "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            "version": "2.1.0",
            "runs": [
                {
                    "tool": {
                        "driver": {
                            "name": "aek-security",
                            "semanticVersion": "0.1.0",
                            "informationUri": "https://github.com/google-antigravity/aek",
                            "rules": rules
                        }
                    },
                    "automationDetails": {
                        "id": format!("{}-validation", report.project_name)
                    },
                    "results": results
                }
            ]
        });

        serde_json::to_string_pretty(&sarif_payload)
    }
}
