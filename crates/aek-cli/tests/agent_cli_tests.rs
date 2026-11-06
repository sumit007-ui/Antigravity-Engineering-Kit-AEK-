// Integration tests for `aek agent` CLI commands

#[test]
fn test_agent_detect() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "detect"])
        .output()
        .expect("failed to run aek agent detect");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Detected Agent") || stdout.contains("Google Antigravity"));
}

#[test]
fn test_agent_capabilities() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "capabilities"])
        .output()
        .expect("failed to run aek agent capabilities");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Capabilities") || stdout.contains("Google Antigravity"));
}

#[test]
fn test_agent_doctor() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "doctor"])
        .output()
        .expect("failed to run aek agent doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Antigravity Agent Diagnostics") || stdout.contains("Agent Doctor"));
}

#[test]
fn test_agent_install_rules_dry_run() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "install-rules", "--dry-run"])
        .output()
        .expect("failed to run aek agent install-rules --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Dry run")
            || stdout.contains("dry-run")
            || stdout.contains("Would install")
    );
}

#[test]
fn test_agent_install_skills_dry_run() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["agent", "install-skills", "--dry-run"])
        .output()
        .expect("failed to run aek agent install-skills --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Dry run")
            || stdout.contains("dry-run")
            || stdout.contains("Would install")
    );
}
