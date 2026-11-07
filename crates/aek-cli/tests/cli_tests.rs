// Integration tests for aek CLI binary

#[test]
fn test_cli_version_flag() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .arg("--version")
        .output()
        .expect("failed to run aek --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aek"));
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_doctor_command() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .arg("doctor")
        .output()
        .expect("failed to run aek doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Antigravity Engineering Kit (AEK) Doctor"));
    assert!(stdout.contains("Core Toolchain") || stdout.contains("rustc"));
}

#[test]
fn test_cli_doctor_json_output() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["doctor", "--json"])
        .output()
        .expect("failed to run aek doctor --json");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("doctor --json must output valid JSON");
    assert!(parsed.get("checks").is_some());
    assert!(parsed.get("os").is_some());
}

#[test]
fn test_cli_config_validate_command() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["config", "validate"])
        .output()
        .expect("failed to run aek config validate");

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.status.success()
            || combined.contains("not found")
            || combined.contains("Configuration")
    );
}

#[test]
fn test_cli_init_and_validate_workflow() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let config_path = temp_dir.path().join("config.toml");

    // Initialize config
    let init_output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["init", "--path", config_path.to_str().unwrap()])
        .output()
        .expect("failed to run aek init");

    assert!(init_output.status.success());
    assert!(config_path.exists());

    // Validate generated config
    let val_output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "config",
            "validate",
            "--path",
            config_path.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run aek config validate");

    assert!(val_output.status.success());
    let stdout = String::from_utf8_lossy(&val_output.stdout);
    assert!(stdout.contains("Configuration is valid"));
    assert!(stdout.contains("light"));
}
