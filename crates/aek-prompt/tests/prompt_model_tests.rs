use aek_prompt::cache::PromptCache;
use aek_prompt::datasets::{DatasetManager, EvaluationCase};
use aek_prompt::evaluator::EvaluationMetricCalculator;
use aek_prompt::model::{PromptRecord, PromptStatus, compute_sha256};
use aek_prompt::templates::PromptTemplate;
use aek_prompt::versions::PromptVersionManager;
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_prompt_record_creation_and_hashing() {
    let original = "Generate a robust concurrent LRU cache in Rust.";
    let prompt = PromptRecord::new(
        "rust-lru-cache",
        "Rust LRU Cache Prompt",
        "Generates a thread-safe LRU cache using Tokio RwLock",
        original,
    );

    assert_eq!(prompt.id, "rust-lru-cache");
    assert_eq!(prompt.version, "1.0.0");
    assert_eq!(prompt.status, PromptStatus::Draft);
    assert_eq!(prompt.content_hash, compute_sha256(original));
}

#[test]
fn test_prompt_template_variable_substitution() {
    let raw = "Write a {{role}} module in {{language}} implementing {{feature}}.";
    let template = PromptTemplate::parse(raw).unwrap();

    let vars = template.variables();
    assert_eq!(vars, vec!["role", "language", "feature"]);

    let mut values = HashMap::new();
    values.insert("role".to_string(), "backend service".to_string());
    values.insert("language".to_string(), "Rust".to_string());
    values.insert("feature".to_string(), "graceful shutdown".to_string());

    let rendered = template.render(&values).unwrap();
    assert_eq!(
        rendered,
        "Write a backend service module in Rust implementing graceful shutdown."
    );
}

#[test]
fn test_prompt_versioning_and_rollback() {
    let dir = tempdir().unwrap();
    let manager = PromptVersionManager::new(dir.path().to_path_buf());

    let mut record = PromptRecord::new(
        "test-prompt",
        "Test Prompt",
        "Description",
        "Version 1 content",
    );

    // Save v1
    let v1_hash = manager.save_version(&record).unwrap();
    assert_eq!(record.version, "1.0.0");

    // Update to v2
    record.version = "1.1.0".to_string();
    record.optimized_content = Some("Version 2 optimized content".to_string());
    let v2_hash = manager.save_version(&record).unwrap();
    assert_ne!(v1_hash, v2_hash);

    let versions = manager.list_versions("test-prompt").unwrap();
    assert_eq!(versions.len(), 2);

    // Rollback to v1
    let rolled_back = manager.rollback("test-prompt", "1.0.0").unwrap();
    assert_eq!(rolled_back.version, "1.0.0");
    assert_eq!(rolled_back.original_content, "Version 1 content");
}

#[test]
fn test_prompt_cache_operations() {
    let cache = PromptCache::new(50);
    let key = "cache-key-sha256";
    let optimized = "Optimized prompt result";

    assert!(cache.get(key).is_none());
    cache.insert(key.to_string(), optimized.to_string());
    assert_eq!(cache.get(key).as_deref(), Some(optimized));

    cache.clear();
    assert!(cache.get(key).is_none());
}

#[test]
fn test_dataset_validation_and_splitting() {
    let mut cases = Vec::new();
    for i in 0..10 {
        cases.push(EvaluationCase {
            id: format!("case-{i}"),
            input_prompt: format!("Prompt {i}"),
            context: Some("System context".to_string()),
            expected_properties: vec!["compiles".to_string()],
            constraints: vec!["no unsafe".to_string()],
            evaluation_criteria: "Checks correctness".to_string(),
            reference_output: Some(format!("Output {i}")),
            tags: vec!["rust".to_string()],
        });
    }

    let manager = DatasetManager::from_cases("test-dataset", cases).unwrap();
    let (train, val, test) = manager.split(0.6, 0.2, 0.2).unwrap();

    assert_eq!(train.len(), 6);
    assert_eq!(val.len(), 2);
    assert_eq!(test.len(), 2);
}

#[test]
fn test_evaluation_metric_calculation() {
    let output = r#"{"name": "test", "status": "ok"}"#;
    let expected_keywords = vec!["name", "status"];
    let negative_constraints = vec!["error", "panic"];

    let calc = EvaluationMetricCalculator::new();
    let score = calc.evaluate_constraints(output, &expected_keywords, &negative_constraints);
    assert_eq!(score.constraint_score, 1.0);
    assert!(score.schema_valid);
}
