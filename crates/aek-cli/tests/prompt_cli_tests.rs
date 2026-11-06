use std::process::Command;

#[test]
fn test_cli_prompt_list() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["prompt", "list"])
        .output()
        .expect("failed to run aek prompt list");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rust") || stdout.contains("Registered Prompts"));
}

#[test]
fn test_cli_prompt_show() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["prompt", "show", "rust_lru"])
        .output()
        .expect("failed to run aek prompt show");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("LRU"));
}

#[test]
fn test_cli_prompt_adapters() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["prompt", "adapters"])
        .output()
        .expect("failed to run aek prompt adapters");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("prompt-optimizer"));
    assert!(stdout.contains("dspy"));
    assert!(stdout.contains("promptfoo"));
}

#[test]
fn test_cli_prompt_doctor() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["prompt", "doctor"])
        .output()
        .expect("failed to run aek prompt doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AEK Prompt Optimization & Evaluation Doctor"));
}

#[test]
fn test_cli_prompt_optimize_dry_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "prompt",
            "optimize",
            "rust_lru",
            "--adapter",
            "prompt-optimizer",
            "--dry-run",
        ])
        .output()
        .expect("failed to run aek prompt optimize");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Optimizing prompt"));
    assert!(stdout.contains("Role: Principal Rust Systems Engineer"));
}

#[test]
fn test_cli_prompt_evaluate() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "prompt",
            "evaluate",
            "rust_lru",
            "--dataset",
            "datasets/examples/rust.jsonl",
        ])
        .output()
        .expect("failed to run aek prompt evaluate");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Evaluation Report"));
    assert!(stdout.contains("Avg Constraint Score"));
}

#[test]
fn test_cli_prompt_export_dry_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["prompt", "export", "rust_lru", "--dry-run"])
        .output()
        .expect("failed to run aek prompt export");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dry Run") || stdout.contains("Would export"));
}
