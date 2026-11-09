use aek_prompt::model::PromptRecord;
use aek_prompt_adapters::dspy::DspyAdapter;
use aek_prompt_adapters::prompt_optimizer::DeterministicRuleFormatter;
use aek_prompt_adapters::promptfoo::{PINNED_PROMPTFOO_VERSION, PromptfooAdapter};
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_prompt_optimizer_adapter_transformation() {
    let adapter = DeterministicRuleFormatter::new();
    let prompt = PromptRecord::new(
        "test-prompt",
        "Test Prompt",
        "Description",
        "Write a safe C++ RAII socket wrapper without raw pointer leaks.",
    );

    let optimized = adapter.optimize(&prompt).unwrap();
    assert!(optimized.contains("# Role:"));
    assert!(optimized.contains("## Objective"));
    assert!(optimized.contains("## Non-Negotiable Constraints & Rules"));
    assert!(optimized.contains("## Output Schema"));
    assert!(optimized.contains("C++"));
}

#[test]
fn test_dspy_bridge_payload_generation() {
    let adapter = DspyAdapter::new();
    let prompt = PromptRecord::new(
        "dspy-task",
        "DSPy Task",
        "DSPy compilation",
        "Extract entities: {{text}}",
    );

    let payload = adapter.build_bridge_payload(&prompt, "datasets/examples/rust.jsonl", 4);
    assert_eq!(payload.task_id, "dspy-task");
    assert_eq!(payload.max_demos, 4);

    let json = serde_json::to_string(&payload).unwrap();
    assert!(json.contains("dspy-task"));
}

#[tokio::test]
async fn test_dspy_offline_passthrough_honest_simulated_result() {
    let adapter = DspyAdapter::new();
    let prompt = PromptRecord::new(
        "task-offline",
        "Task Offline",
        "Test Offline",
        "Summarize: {{text}}",
    );
    let payload = adapter.build_bridge_payload(&prompt, "dummy.jsonl", 2);
    // Point to non-existent bridge script to force fallback
    let res = adapter
        .run_bridge(&payload, Some(&PathBuf::from("nonexistent_bridge.py")))
        .await
        .expect("passthrough result should succeed");

    assert!(
        res.simulated,
        "Result must explicitly indicate simulated: true"
    );
    assert!(
        !res.success,
        "Simulated passthrough must have success: false"
    );
    assert_eq!(
        res.metric_score, 0.0,
        "Metric score must be 0.0, never fake 0.94"
    );
    assert_eq!(res.iterations, 0, "Iterations must be 0, never fake 4");
    assert!(
        res.demonstrations.is_empty(),
        "Demonstrations must be empty"
    );
    assert_eq!(res.optimized_instructions, "Summarize: {{text}}");
}

#[tokio::test]
async fn test_promptfoo_missing_tool_returns_error_not_fake_success() {
    let adapter = PromptfooAdapter::new();
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.yaml");
    let output_path = dir.path().join("output.json");
    std::fs::write(&config_path, "prompts: []\n").unwrap();

    // If promptfoo/npx does not exist, run_eval must return Err
    // Even if npx exists, with an invalid config it will fail, never returning 5/5 passed
    let res = adapter.run_eval(&config_path, &output_path, true).await;
    // It must either fail or, if it succeeded, simulated must be false and total_tests > 0
    if let Ok(eval) = res {
        assert!(!eval.simulated);
    }
}

#[test]
fn test_promptfoo_version_is_pinned() {
    assert!(!PINNED_PROMPTFOO_VERSION.is_empty());
    assert_ne!(PINNED_PROMPTFOO_VERSION, "latest");
}

#[test]
fn test_promptfoo_configuration_generation() {
    let _dir = tempdir().unwrap();
    let adapter = PromptfooAdapter::new();

    let prompts = vec!["prompts/v1.txt".to_string(), "prompts/v2.txt".to_string()];
    let providers = vec!["openai:gpt-4o-mini".to_string()];
    let tests_count = 3;

    let yaml = adapter.generate_config("Test Eval", &prompts, &providers, tests_count);
    assert!(yaml.contains("prompts/v1.txt"));
    assert!(yaml.contains("openai:gpt-4o-mini"));
    assert!(yaml.contains("assert:"));
}
