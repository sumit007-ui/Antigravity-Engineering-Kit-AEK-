use aek_reporting::json::JsonReportGenerator;
use aek_reporting::sarif::SarifReportGenerator;
use aek_reporting::summary::MarkdownSummaryGenerator;
use aek_reporting::terminal::TerminalReportGenerator;
use aek_security::findings::{ScannerKind, SecurityFinding, Severity};
use aek_validation::pipeline::ValidationStage;
use aek_validation::report::{StepResult, StepStatus, ValidationReport};
use std::collections::HashMap;
use std::path::PathBuf;

fn sample_validation_report() -> ValidationReport {
    ValidationReport {
        project_name: "sample-project".to_string(),
        language: "rust".to_string(),
        dry_run: false,
        overall_success: true,
        passed_count: 2,
        failed_count: 0,
        skipped_count: 0,
        total_duration_ms: 1500,
        results: vec![
            StepResult {
                name: "cargo fmt".to_string(),
                stage: ValidationStage::Formatting,
                status: StepStatus::Pass,
                duration_ms: 500,
                exit_code: Some(0),
                stdout: "fmt clean".to_string(),
                stderr: String::new(),
            },
            StepResult {
                name: "cargo clippy".to_string(),
                stage: ValidationStage::StaticAnalysis,
                status: StepStatus::Pass,
                duration_ms: 1000,
                exit_code: Some(0),
                stdout: "clippy clean".to_string(),
                stderr: String::new(),
            },
        ],
    }
}

fn sample_findings() -> Vec<SecurityFinding> {
    vec![SecurityFinding {
        id: "sec-01".to_string(),
        scanner: ScannerKind::Semgrep,
        rule_id: "rust.security.insecure-rng".to_string(),
        severity: Severity::High,
        category: "cryptography".to_string(),
        description: "Insecure PRNG used for security-sensitive operation".to_string(),
        file_path: Some(PathBuf::from("src/crypto.rs")),
        line_number: Some(25),
        dependency: None,
        advisory_id: None,
        remediation: Some("Use rand::rngs::OsRng".to_string()),
        confidence: Some("HIGH".to_string()),
        metadata: HashMap::new(),
    }]
}

#[test]
fn test_sarif_v210_generation() {
    let report = sample_validation_report();
    let findings = sample_findings();

    let sarif_json =
        SarifReportGenerator::generate(&report, &findings).expect("generate sarif json");

    assert!(sarif_json.contains("\"version\": \"2.1.0\""));
    assert!(sarif_json.contains("https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json"));
    assert!(sarif_json.contains("rust.security.insecure-rng"));
    assert!(sarif_json.contains("src/crypto.rs"));
    assert!(sarif_json.contains("\"startLine\": 25"));
}

#[test]
fn test_json_report_generation() {
    let report = sample_validation_report();
    let findings = sample_findings();

    let json_output =
        JsonReportGenerator::generate(&report, &findings).expect("generate json report");

    let parsed: serde_json::Value = serde_json::from_str(&json_output).expect("parse json");
    assert_eq!(parsed["project_name"], "sample-project");
    assert_eq!(parsed["overall_success"], true);
    assert_eq!(parsed["security_findings"].as_array().unwrap().len(), 1);
}

#[test]
fn test_markdown_summary_and_terminal_generation() {
    let report = sample_validation_report();
    let findings = sample_findings();

    let md = MarkdownSummaryGenerator::generate(&report, &findings);
    assert!(md.contains("# AEK Validation & Security Summary"));
    assert!(md.contains("sample-project"));
    assert!(md.contains("rust.security.insecure-rng"));

    let term = TerminalReportGenerator::format(&report, &findings);
    assert!(term.contains("sample-project"));
    assert!(term.contains("cargo fmt"));
}
