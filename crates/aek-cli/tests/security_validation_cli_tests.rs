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
fn test_cli_validate_dry_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["validate", "--dry-run"])
        .output()
        .expect("failed to run aek validate --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("AEK Validation")
            || stdout.contains("PASSED")
            || stdout.contains("overall_success")
    );
}

#[test]
fn test_cli_validate_changed() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["validate", "--changed", "--dry-run"])
        .output()
        .expect("failed to run aek validate --changed");

    assert!(output.status.success());
}

#[test]
fn test_cli_validate_policy_list() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["validate", "policy", "list"])
        .output()
        .expect("failed to run aek validate policy list");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("default"));
    assert!(stdout.contains("strict"));
    assert!(stdout.contains("ci"));
}

#[test]
fn test_cli_validate_policy_show() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["validate", "policy", "show", "strict"])
        .output()
        .expect("failed to run aek validate policy show strict");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("strict"));
    assert!(stdout.contains("fail_fast = true"));
}

#[test]
fn test_cli_validate_policy_check() {
    let policy_path = workspace_root()
        .join("config")
        .join("validation")
        .join("default.toml");
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["validate", "policy", "check", policy_path.to_str().unwrap()])
        .output()
        .expect("failed to run aek validate policy check");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[PASS]"));
}

#[test]
fn test_cli_security_doctor() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["security", "doctor"])
        .output()
        .expect("failed to run aek security doctor");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AEK Security Toolchain Doctor"));
    assert!(stdout.contains("semgrep"));
    assert!(stdout.contains("trivy"));
    assert!(stdout.contains("cargo-audit"));
}

#[test]
fn test_cli_security_scan_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["security", "scan", "--json"])
        .output()
        .expect("failed to run aek security scan --json");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"target_dir\"") || stdout.contains("\"findings\""));
}

#[test]
fn test_cli_ci_generate_dry_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["ci", "generate", "github", "--dry-run"])
        .output()
        .expect("failed to run aek ci generate github --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dry Run") || stdout.contains("aek-ci.yml"));
}

#[test]
fn test_cli_ci_validate() {
    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["ci", "validate"])
        .output()
        .expect("failed to run aek ci validate");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Auditing CI Workflows"));
}
