use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_backup_and_restore_workflow() {
    let dir = tempdir().expect("tempdir");
    let config_file = dir.path().join("config.toml");
    let backup_dir = dir.path().join("backups");

    // Initialize config
    Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["init", "--path", config_file.to_str().unwrap()])
        .output()
        .expect("aek init");

    // Create backup
    let backup_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "backup",
            "--config",
            config_file.to_str().unwrap(),
            "--backup-dir",
            backup_dir.to_str().unwrap(),
        ])
        .output()
        .expect("aek backup");

    assert!(backup_output.status.success());
    let backup_stdout = String::from_utf8_lossy(&backup_output.stdout);
    assert!(backup_stdout.contains("backup-"));

    // Extract backup ID from output line
    let backup_id = backup_stdout
        .lines()
        .find(|line| line.contains("backup-"))
        .and_then(|line| {
            line.split_whitespace()
                .find(|word| word.starts_with("backup-"))
        })
        .expect("should find backup ID");

    // List backups
    let list_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "backup",
            "list",
            "--backup-dir",
            backup_dir.to_str().unwrap(),
        ])
        .output()
        .expect("aek backup list");

    assert!(list_output.status.success());
    let list_stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(list_stdout.contains(backup_id));

    // Restore dry run
    let restore_dry = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "restore",
            backup_id,
            "--dry-run",
            "--config",
            config_file.to_str().unwrap(),
            "--backup-dir",
            backup_dir.to_str().unwrap(),
        ])
        .output()
        .expect("aek restore dry-run");

    assert!(restore_dry.status.success());
    let restore_dry_stdout = String::from_utf8_lossy(&restore_dry.stdout);
    assert!(restore_dry_stdout.contains("[DRY-RUN]"));

    // Restore actual
    let restore_actual = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "restore",
            backup_id,
            "--config",
            config_file.to_str().unwrap(),
            "--backup-dir",
            backup_dir.to_str().unwrap(),
        ])
        .output()
        .expect("aek restore actual");

    assert!(restore_actual.status.success());
}

#[test]
fn test_cli_restore_path_traversal_rejection() {
    let dir = tempdir().expect("tempdir");
    let config_file = dir.path().join("config.toml");
    let backup_dir = dir.path().join("backups");

    let evil_restore = Command::new(env!("CARGO_BIN_EXE_aek"))
        .args([
            "restore",
            "../../etc/passwd",
            "--config",
            config_file.to_str().unwrap(),
            "--backup-dir",
            backup_dir.to_str().unwrap(),
        ])
        .output()
        .expect("aek restore path traversal");

    assert!(!evil_restore.status.success());
    let stderr = String::from_utf8_lossy(&evil_restore.stderr);
    let stdout = String::from_utf8_lossy(&evil_restore.stdout);
    assert!(
        stderr.contains("Invalid")
            || stderr.contains("Security")
            || stdout.contains("Invalid")
            || stdout.contains("Security")
    );
}
