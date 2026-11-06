use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_init_dry_run() {
    let dir = tempdir().expect("tempdir");
    let config_file = dir.path().join("config.toml");

    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["init", "--dry-run", "--path", config_file.to_str().unwrap()])
        .output()
        .expect("aek init --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[DRY-RUN]"));
    // File must NOT exist in dry-run
    assert!(!config_file.exists());
}

#[test]
fn test_cli_init_and_idempotency() {
    let dir = tempdir().expect("tempdir");
    let config_file = dir.path().join("config.toml");

    // 1. Initial init
    let output1 = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["init", "--path", config_file.to_str().unwrap()])
        .output()
        .expect("aek init");

    assert!(output1.status.success());
    assert!(config_file.exists());

    // 2. Second init (idempotency without --force)
    let output2 = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["init", "--path", config_file.to_str().unwrap()])
        .output()
        .expect("aek init second time");

    assert!(output2.status.success());
    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    assert!(stdout2.contains("already initialized") || stdout2.contains("Existing"));

    // 3. Init with --force must create a backup and succeed
    let output3 = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["init", "--force", "--path", config_file.to_str().unwrap()])
        .output()
        .expect("aek init --force");

    assert!(output3.status.success());
    let stdout3 = String::from_utf8_lossy(&output3.stdout);
    assert!(
        stdout3.contains("backup") || stdout3.contains("Backup") || stdout3.contains("Initialized")
    );
}
