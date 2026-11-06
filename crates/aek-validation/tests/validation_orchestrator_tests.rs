use aek_languages::model::{DetectedProject, LanguageId};
use aek_security::findings::{ScannerKind, SecurityFinding, Severity};
use aek_validation::incremental::GitChangeDetector;
use aek_validation::planner::ValidationPlanner;
use aek_validation::policy::ValidationPolicy;
use aek_validation::quality_gate::QualityGateEngine;
use aek_validation::report::{StepResult, StepStatus, ValidationReport};
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_validation_planner_creates_ordered_steps() {
    let project = DetectedProject {
        name: "test-crate".to_string(),
        language: LanguageId::Rust,
        path: PathBuf::from("."),
        evidence: Vec::new(),
        monorepo_root: None,
        is_nested: false,
    };

    let planner = ValidationPlanner::new();
    let plan = planner.create_plan(&[project], &ValidationPolicy::default(), false);

    assert_eq!(plan.len(), 1);
    assert_eq!(plan[0].project_name, "test-crate");
    assert!(!plan[0].steps.is_empty());
}

#[test]
fn test_quality_gate_evaluation() {
    let report = ValidationReport {
        project_name: "test-crate".to_string(),
        language: "rust".to_string(),
        dry_run: false,
        overall_success: true,
        passed_count: 3,
        failed_count: 0,
        skipped_count: 0,
        total_duration_ms: 1000,
        results: vec![StepResult {
            name: "test".to_string(),
            stage: aek_validation::pipeline::ValidationStage::UnitTesting,
            status: StepStatus::Pass,
            duration_ms: 200,
            exit_code: Some(0),
            stdout: "ok".to_string(),
            stderr: String::new(),
        }],
    };

    let findings = vec![SecurityFinding {
        id: "f-01".to_string(),
        scanner: ScannerKind::Trivy,
        rule_id: "CVE-2024-0001".to_string(),
        severity: Severity::Critical,
        category: "vuln".to_string(),
        description: "Critical bug".to_string(),
        file_path: Some(PathBuf::from("Cargo.lock")),
        line_number: None,
        dependency: Some("old-crate".to_string()),
        advisory_id: None,
        remediation: None,
        confidence: None,
        metadata: HashMap::new(),
    }];

    let gate = QualityGateEngine::default();
    let result = gate.evaluate(&report, &findings);

    assert!(
        !result.passed,
        "Quality gate should fail on critical security finding"
    );
    assert!(!result.failure_reasons.is_empty());
}

#[test]
fn test_git_incremental_detector() {
    let detector = GitChangeDetector::new();
    // Path detection in current repo
    let status = detector.detect_changed_files(&PathBuf::from("."));
    // Under git repository, should return a list (empty or with modified files) without crashing
    assert!(status.is_ok());
}
