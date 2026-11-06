use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_profile_list_and_show() {
    let list_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["profile", "list"])
        .output()
        .expect("aek profile list");

    assert!(list_output.status.success());
    let list_stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(list_stdout.contains("light"));
    assert!(list_stdout.contains("engineering"));
    assert!(list_stdout.contains("full"));

    let show_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["profile", "show", "engineering"])
        .output()
        .expect("aek profile show engineering");

    assert!(show_output.status.success());
    let show_stdout = String::from_utf8_lossy(&show_output.stdout);
    assert!(show_stdout.contains("Engineering"));
    assert!(show_stdout.contains("MCP"));
}

#[test]
fn test_cli_profile_apply_dry_run_and_commit() {
    let dir = tempdir().expect("tempdir");
    let config_file = dir.path().join("config.toml");

    // Initialize default Light config
    Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["init", "--path", config_file.to_str().unwrap()])
        .output()
        .expect("aek init");

    // Apply engineering profile in dry-run mode
    let dry_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "profile",
            "apply",
            "engineering",
            "--dry-run",
            "--config",
            config_file.to_str().unwrap(),
        ])
        .output()
        .expect("aek profile apply dry-run");

    assert!(dry_output.status.success());
    let dry_stdout = String::from_utf8_lossy(&dry_output.stdout);
    assert!(dry_stdout.contains("[DRY-RUN]"));

    // Verify file still has Light profile
    let content1 = std::fs::read_to_string(&config_file).expect("read");
    assert!(content1.contains("profile = \"light\""));

    // Apply engineering profile committed
    let commit_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "profile",
            "apply",
            "engineering",
            "--config",
            config_file.to_str().unwrap(),
        ])
        .output()
        .expect("aek profile apply");

    assert!(commit_output.status.success());
    let content2 = std::fs::read_to_string(&config_file).expect("read");
    assert!(content2.contains("profile = \"engineering\""));
    assert!(content2.contains("mcp_enabled = true"));
}
