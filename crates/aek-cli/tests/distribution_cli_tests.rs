use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
fn test_cli_install_info() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["install", "info"])
        .output()
        .expect("failed to run aek install info");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AEK Installation Information"));
    assert!(stdout.contains("Version:"));
    assert!(stdout.contains("Executable Path:"));
    assert!(stdout.contains("Host Target Triple:"));
}

#[test]
fn test_cli_install_doctor() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["install", "doctor"])
        .output()
        .expect("failed to run aek install doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AEK Installation Diagnostics Doctor"));
    assert!(stdout.contains("Executable Binary"));
    assert!(stdout.contains("Platform Support"));
}

#[test]
fn test_cli_install_verify() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["install", "verify"])
        .output()
        .expect("failed to run aek install verify");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AEK Installation Integrity Verification"));
    assert!(stdout.contains("[VERIFIED]"));
    assert!(stdout.contains("SHA256:"));
}

#[test]
fn test_cli_update_check_and_dry_run() {
    let check_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["update", "check"])
        .output()
        .expect("failed to run aek update check");

    assert!(check_output.status.success());
    let check_stdout = String::from_utf8_lossy(&check_output.stdout);
    assert!(check_stdout.contains("AEK Update Manager"));

    let dry_run_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["update", "--dry-run"])
        .output()
        .expect("failed to run aek update --dry-run");

    assert!(dry_run_output.status.success());
    let dry_run_stdout = String::from_utf8_lossy(&dry_run_output.stdout);
    assert!(dry_run_stdout.contains("[DRY RUN]") || dry_run_stdout.contains("Update Plan"));
}

#[test]
fn test_cli_rollback_list_and_dry_run() {
    let list_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["rollback", "--list"])
        .output()
        .expect("failed to run aek rollback --list");

    assert!(list_output.status.success());
    let list_stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(list_stdout.contains("AEK Rollback Manager"));
    assert!(list_stdout.contains("VERSION"));

    let dry_run_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["rollback", "--dry-run"])
        .output()
        .expect("failed to run aek rollback --dry-run");

    let dry_run_stdout = String::from_utf8_lossy(&dry_run_output.stdout);
    assert!(
        dry_run_stdout.contains("AEK Rollback Manager")
            || dry_run_stdout.contains("[DRY RUN]")
            || dry_run_stdout.contains("No previous versions found")
    );
}

#[test]
fn test_cli_uninstall_dry_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["uninstall", "--dry-run"])
        .output()
        .expect("failed to run aek uninstall --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AEK Safe Uninstallation"));
    assert!(stdout.contains("[DRY RUN]"));
    assert!(stdout.contains("No files were removed"));
}

#[test]
fn test_cli_setup_antigravity_dry_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["setup", "antigravity", "--dry-run"])
        .output()
        .expect("failed to run aek setup antigravity --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AEK Antigravity Setup Engine"));
    assert!(stdout.contains("[DRY RUN]"));
    assert!(stdout.contains("No changes were written to disk"));
}

#[test]
fn test_cli_config_migrate_check_and_dry_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["config", "migrate", "--check"])
        .output()
        .expect("failed to run aek config migrate --check");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Configuration Migration Status"));
    assert!(stdout.contains("Current Schema Version:"));

    let dry_run_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["config", "migrate", "--dry-run"])
        .output()
        .expect("failed to run aek config migrate --dry-run");

    assert!(dry_run_output.status.success());
    let dry_run_stdout = String::from_utf8_lossy(&dry_run_output.stdout);
    assert!(dry_run_stdout.contains("[DRY RUN]"));
}
