use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_language_list() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["language", "list"])
        .output()
        .expect("failed to run aek language list");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("go"));
    assert!(stdout.contains("cpp"));
    assert!(stdout.contains("zig"));
    assert!(stdout.contains("python"));
    assert!(stdout.contains("javascript"));
    assert!(stdout.contains("flutter"));
    assert!(stdout.contains("ros2"));
}

#[test]
fn test_cli_language_detect_isolated_dir() {
    let dir = tempdir().unwrap();
    let cargo_toml = dir.path().join("Cargo.toml");
    fs::write(
        &cargo_toml,
        r#"[package]
name = "isolated-test"
version = "0.1.0"
"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["language", "detect", "--path", dir.path().to_str().unwrap()])
        .output()
        .expect("failed to run aek language detect");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("isolated-test"));
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("rust-systems-engineer"));
}

#[test]
fn test_cli_language_tools_rust() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["language", "tools", "rust"])
        .output()
        .expect("failed to run aek language tools rust");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cargo") || stdout.contains("cargo"));
    assert!(stdout.contains("rustc"));
}

#[test]
fn test_cli_language_doctor_rust() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["language", "doctor", "rust"])
        .output()
        .expect("failed to run aek language doctor rust");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rust"));
    assert!(stdout.contains("cargo"));
}

#[test]
fn test_cli_language_setup_dry_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["language", "setup", "rust", "--dry-run"])
        .output()
        .expect("failed to run aek language setup rust --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Toolchain Setup Plan") || stdout.contains("installed"));
}

#[test]
fn test_cli_validate_dry_run() {
    let dir = tempdir().unwrap();
    let cargo_toml = dir.path().join("Cargo.toml");
    fs::write(
        &cargo_toml,
        r#"[package]
name = "dry-run-test"
version = "0.1.0"
"#,
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "validate",
            "--path",
            dir.path().to_str().unwrap(),
            "--dry-run",
        ])
        .output()
        .expect("failed to run aek validate --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DRY RUN"));
    assert!(stdout.contains("dry-run-test"));
}
