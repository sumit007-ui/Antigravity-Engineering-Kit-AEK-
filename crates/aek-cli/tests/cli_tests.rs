// Integration tests for aek CLI binary
#[test]
fn test_cli_version_flag() {
    // Assert that --version or -V is handled
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .arg("--version")
        .output()
        .expect("failed to run aek --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aek"));
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_cli_doctor_command() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .arg("doctor")
        .output()
        .expect("failed to run aek doctor");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Antigravity Engineering Kit (AEK) Doctor"));
    assert!(stdout.contains("Rust Toolchain") || stdout.contains("rustc"));
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
    assert!(parsed.get("platform").is_some());
}

#[test]
fn test_cli_config_validate_command() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["config", "validate"])
        .output()
        .expect("failed to run aek config validate");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Even if no config file is found, it informs the user cleanly
    assert!(
        output.status.success()
            || stdout.contains("No configuration file found")
            || stdout.contains("Configuration")
    );
}

#[test]
fn test_cli_config_init_and_validate_workflow() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let config_path = temp_dir.path().join("aek.toml");

    // Initialize config
    let init_output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["config", "init", "--path", config_path.to_str().unwrap()])
        .output()
        .expect("failed to run aek config init");

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
    assert!(stdout.contains("Configuration schema is valid"));
    assert!(stdout.contains("ruthless_precision") || stdout.contains("RuthlessPrecision"));
}
