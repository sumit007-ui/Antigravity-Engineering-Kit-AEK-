// Integration tests for `aek mcp` CLI subcommands
use tempfile::tempdir;

#[test]
fn test_cli_mcp_list() {
    let tmp = tempdir().expect("tempdir");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .env("AEK_CONFIG_DIR", tmp.path())
        .args(["mcp", "list"])
        .output()
        .expect("failed to run aek mcp list");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available MCP Servers"));
    assert!(stdout.contains("context7"));
    assert!(stdout.contains("github"));
    assert!(stdout.contains("playwright"));
}

#[test]
fn test_cli_mcp_list_json() {
    let tmp = tempdir().expect("tempdir");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .env("AEK_CONFIG_DIR", tmp.path())
        .args(["mcp", "list", "--json"])
        .output()
        .expect("failed to run aek mcp list --json");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("mcp list --json must output valid JSON");
    assert!(parsed.as_array().is_some());
}

#[test]
fn test_cli_mcp_show() {
    let tmp = tempdir().expect("tempdir");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .env("AEK_CONFIG_DIR", tmp.path())
        .args(["mcp", "show", "context7"])
        .output()
        .expect("failed to run aek mcp show context7");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Server: context7"));
    assert!(stdout.contains("Transport:"));
}

#[test]
fn test_cli_mcp_doctor() {
    let tmp = tempdir().expect("tempdir");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .env("AEK_CONFIG_DIR", tmp.path())
        .args(["mcp", "doctor"])
        .output()
        .expect("failed to run aek mcp doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("MCP Server Diagnostics"));
    assert!(stdout.contains("context7"));
}

#[test]
fn test_cli_mcp_export_antigravity_dry_run() {
    let tmp = tempdir().expect("tempdir");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .env("AEK_CONFIG_DIR", tmp.path())
        .args(["mcp", "export", "--agent", "antigravity", "--dry-run"])
        .output()
        .expect("failed to run aek mcp export --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dry run") || stdout.contains("mcpServers"));
}

#[test]
fn test_cli_mcp_enable_disable() {
    let tmp = tempdir().expect("tempdir");
    let output1 = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .env("AEK_CONFIG_DIR", tmp.path())
        .args(["mcp", "disable", "context7"])
        .output()
        .expect("failed to run aek mcp disable");
    assert!(output1.status.success());

    let output2 = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .env("AEK_CONFIG_DIR", tmp.path())
        .args(["mcp", "enable", "context7"])
        .output()
        .expect("failed to run aek mcp enable");
    assert!(output2.status.success());
}
