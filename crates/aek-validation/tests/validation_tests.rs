use aek_languages::model::{DetectedProject, LanguageId, ProjectEvidence};
use aek_validation::executor::ValidationExecutor;
use aek_validation::pipeline::{ValidationPipeline, ValidationStage, ValidationStep};
use aek_validation::policy::ValidationPolicy;
use aek_validation::report::StepStatus;
use tempfile::tempdir;

#[tokio::test]
async fn test_validation_pipeline_dry_run() {
    let dir = tempdir().unwrap();
    let project = DetectedProject {
        path: dir.path().to_path_buf(),
        language: LanguageId::Rust,
        name: "test-crate".to_string(),
        evidence: vec![ProjectEvidence {
            file: "Cargo.toml".to_string(),
            matched_content: None,
            description: "Cargo manifest".to_string(),
        }],
        monorepo_root: None,
        is_nested: false,
    };

    let pipeline = ValidationPipeline::for_project(&project);
    assert!(!pipeline.steps.is_empty());

    let policy = ValidationPolicy {
        dry_run: true,
        ..Default::default()
    };

    let executor = ValidationExecutor::new(policy);
    let report = executor.execute(&pipeline).await.unwrap();

    assert_eq!(report.project_name, "test-crate");
    assert!(report.dry_run);
    // In dry-run, all executed planned steps report as Skip or Pass without running processes
    assert!(
        report
            .results
            .iter()
            .all(|r| r.status == StepStatus::Skip || r.status == StepStatus::Pass)
    );
}

#[tokio::test]
async fn test_validation_report_serialization() {
    let dir = tempdir().unwrap();
    let step = ValidationStep {
        name: "mock-check".to_string(),
        stage: ValidationStage::Environment,
        program: "cargo".to_string(),
        args: vec!["--version".to_string()],
        working_dir: dir.path().to_path_buf(),
        is_optional: false,
        timeout_secs: 5,
    };

    let pipeline = ValidationPipeline {
        project_name: "demo".to_string(),
        language: LanguageId::Rust,
        steps: vec![step],
    };

    let policy = ValidationPolicy {
        dry_run: true,
        ..Default::default()
    };

    let executor = ValidationExecutor::new(policy);
    let report = executor.execute(&pipeline).await.unwrap();
    let json = serde_json::to_string_pretty(&report).unwrap();

    assert!(json.contains("demo"));
    assert!(json.contains("mock-check"));
}
