// Integration tests for `aek agent` CLI commands

#[test]
fn test_agent_detect() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "detect"])
        .output()
        .expect("failed to run aek agent detect");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Discovery Engine") || stdout.contains("Gemini Antigravity"));
}

#[test]
fn test_agent_list() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "list"])
        .output()
        .expect("failed to run aek agent list");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("antigravity"));
    assert!(stdout.contains("claude-code"));
    assert!(stdout.contains("opencode"));
    assert!(stdout.contains("cursor"));
}

#[test]
fn test_agent_capabilities() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "capabilities"])
        .output()
        .expect("failed to run aek agent capabilities");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Capabilities") || stdout.contains("Antigravity"));
}

#[test]
fn test_agent_capabilities_filtered() {
    for agent in ["claude-code", "cursor", "opencode", "antigravity"] {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
            .args(["agent", "capabilities", agent])
            .output()
            .unwrap_or_else(|_| panic!("failed to run aek agent capabilities {agent}"));

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Capabilities"));
    }
}

#[test]
fn test_agent_doctor() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "doctor"])
        .output()
        .expect("failed to run aek agent doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Diagnostics") || stdout.contains("Supported Agents"));
}

#[test]
fn test_agent_compatibility() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "compatibility"])
        .output()
        .expect("failed to run aek agent compatibility");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Compatibility Report"));
    assert!(stdout.contains("RULES") || stdout.contains("MCP"));
}

#[test]
fn test_agent_sync_dry_run() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "sync", "--all", "--dry-run"])
        .output()
        .expect("failed to run aek agent sync --all --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Synchronization Engine"));
    assert!(stdout.contains("[DRY RUN]"));
}

#[test]
fn test_agent_profiles_list_and_show() {
    let list_output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "profile", "list"])
        .output()
        .expect("failed to run aek agent profile list");

    assert!(list_output.status.success());
    let list_stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(list_stdout.contains("multi-agent-engineering"));
    assert!(list_stdout.contains("antigravity-engineering"));

    let show_output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "profile", "show", "multi-agent-engineering"])
        .output()
        .expect("failed to run aek agent profile show");

    assert!(show_output.status.success());
    let show_stdout = String::from_utf8_lossy(&show_output.stdout);
    assert!(show_stdout.contains("Multi-Agent Universal Engineering"));
}

#[test]
fn test_agent_setup_dry_run() {
    for agent in ["claude-code", "cursor", "opencode"] {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
            .args(["agent", "setup", agent, "--dry-run"])
            .output()
            .unwrap_or_else(|_| panic!("failed to run aek agent setup {agent} --dry-run"));

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Setup Engine"));
        assert!(stdout.contains("[DRY RUN]"));
    }
}

#[test]
fn test_agent_uninstall_dry_run() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "uninstall", "cursor", "--dry-run"])
        .output()
        .expect("failed to run aek agent uninstall cursor --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Safe Agent Uninstaller"));
}

#[test]
fn test_agent_install_rules_dry_run() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "install-rules", "--dry-run"])
        .output()
        .expect("failed to run aek agent install-rules --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rule(s)") || stdout.contains("Installed"));
}

#[test]
fn test_agent_install_skills_dry_run() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "install-skills", "--dry-run"])
        .output()
        .expect("failed to run aek agent install-skills --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("skill(s)") || stdout.contains("Installed"));
}
