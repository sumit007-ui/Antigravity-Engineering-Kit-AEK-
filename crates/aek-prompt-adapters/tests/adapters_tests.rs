use aek_prompt::model::PromptRecord;
use aek_prompt_adapters::dspy::DspyAdapter;
use aek_prompt_adapters::prompt_optimizer::PromptOptimizerAdapter;
use aek_prompt_adapters::promptfoo::PromptfooAdapter;
use tempfile::tempdir;

#[test]
fn test_prompt_optimizer_adapter_transformation() {
    let adapter = PromptOptimizerAdapter::new();
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
