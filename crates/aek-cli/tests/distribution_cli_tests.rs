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

    // Without an expected hash or manifest, verification cannot pass and exits with non-zero
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AEK Installation Integrity Verification"));
    assert!(stdout.contains("[UNVERIFIED]"));
    assert!(stdout.contains("SHA256:"));
}

#[test]
fn test_cli_install_verify_with_manifest_matching_and_mismatch() {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let manifest_file = temp_dir.path().join("installation-manifest.json");
    let exe_path = PathBuf::from(env!("CARGO_BIN_EXE_aek"));
    let actual_hash = aek_distribution::verification::compute_file_sha256(&exe_path)
        .expect("failed to compute binary hash");

    // Test with matching hash in manifest
    let valid_manifest = serde_json::json!({
        "version": "0.2.0",
        "scope": "user",
        "install_dir": temp_dir.path().to_string_lossy(),
        "binary_path": exe_path.to_string_lossy(),
        "binary_checksum": actual_hash,
        "installed_at": "2026-11-07T12:00:00Z",
        "method": "installer_script",
        "platform": {
            "target_triple": "x86_64-pc-windows-msvc",
            "os": "windows",
            "arch": "x86_64"
        },
        "managed_files": [exe_path.to_string_lossy()],
        "previous_version": null,
        "verification_status": "verified"
    });
    std::fs::write(&manifest_file, valid_manifest.to_string()).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args([
            "install",
            "verify",
            "--manifest",
            manifest_file.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run aek install verify with manifest");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "expected success but got stdout: {stdout}"
    );
    assert!(stdout.contains("[VERIFIED]"));

    // Test with mismatched hash in manifest
    let invalid_manifest = serde_json::json!({
        "version": "0.2.0",
        "scope": "user",
        "install_dir": temp_dir.path().to_string_lossy(),
        "binary_path": exe_path.to_string_lossy(),
        "binary_checksum": "1111111111111111111111111111111111111111111111111111111111111111",
        "installed_at": "2026-11-07T12:00:00Z",
        "method": "installer_script",
        "platform": {
            "target_triple": "x86_64-pc-windows-msvc",
            "os": "windows",
            "arch": "x86_64"
        },
        "managed_files": [exe_path.to_string_lossy()],
        "previous_version": null,
        "verification_status": "verified"
    });
    std::fs::write(&manifest_file, invalid_manifest.to_string()).unwrap();

    let output_bad = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args([
            "install",
            "verify",
            "--manifest",
            manifest_file.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run aek install verify with bad manifest");

    assert!(!output_bad.status.success());
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output_bad.stdout),
        String::from_utf8_lossy(&output_bad.stderr)
    );
    assert!(combined.contains("[FAILED]") || combined.contains("mismatch"));
}

#[test]
fn test_cli_install_verify_with_matching_expected_hash() {
    let exe_path = PathBuf::from(env!("CARGO_BIN_EXE_aek"));
    let expected_hash = aek_distribution::verification::compute_file_sha256(&exe_path)
        .expect("failed to compute binary hash");

    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["install", "verify", "--expected-hash", &expected_hash])
        .output()
        .expect("failed to run aek install verify with expected hash");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[VERIFIED]"));
    assert!(stdout.contains("Checksum verified"));
}

#[test]
fn test_cli_install_verify_with_mismatch_hash_fails() {
    let wrong_hash = "0000000000000000000000000000000000000000000000000000000000000000";

    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args(["install", "verify", "--expected-hash", wrong_hash])
        .output()
        .expect("failed to run aek install verify with wrong hash");

    assert!(
        !output.status.success(),
        "expected failure on hash mismatch"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{stdout}\n{stderr}");
    assert!(combined.contains("[FAILED]") || combined.contains("Checksum mismatch"));
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
fn test_cli_update_with_custom_manifest() {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let manifest_file = temp_dir.path().join("test-manifest.json");

    let manifest_json = r#"{
        "schema_version": "1.0.0",
        "version": "0.3.0",
        "release_channel": "stable",
        "timestamp": "2026-11-07T12:00:00Z",
        "artifacts": [
            {
                "filename": "aek-v0.3.0-test.zip",
                "target_triple": "x86_64-pc-windows-msvc",
                "os": "windows",
                "arch": "x86_64",
                "size_bytes": 10485760,
                "sha256": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                "download_url": "https://github.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/releases/download/v0.3.0/aek-v0.3.0-test.zip"
            },
            {
                "filename": "aek-v0.3.0-test.tar.gz",
                "target_triple": "x86_64-unknown-linux-gnu",
                "os": "linux",
                "arch": "x86_64",
                "size_bytes": 10485760,
                "sha256": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                "download_url": "https://github.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/releases/download/v0.3.0/aek-v0.3.0-test.tar.gz"
            },
            {
                "filename": "aek-v0.3.0-macos-x86_64.tar.gz",
                "target_triple": "x86_64-apple-darwin",
                "os": "macos",
                "arch": "x86_64",
                "size_bytes": 10485760,
                "sha256": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                "download_url": "https://github.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/releases/download/v0.3.0/aek-v0.3.0-macos-x86_64.tar.gz"
            },
            {
                "filename": "aek-v0.3.0-macos-aarch64.tar.gz",
                "target_triple": "aarch64-apple-darwin",
                "os": "macos",
                "arch": "aarch64",
                "size_bytes": 10485760,
                "sha256": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                "download_url": "https://github.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/releases/download/v0.3.0/aek-v0.3.0-macos-aarch64.tar.gz"
            }
        ]
    }"#;
    std::fs::write(&manifest_file, manifest_json).expect("failed to write test manifest");

    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args([
            "update",
            "check",
            "--manifest",
            manifest_file.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run aek update check with custom manifest");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.3.0"));
    assert!(stdout.contains("Update available") || stdout.contains("UPDATE AVAILABLE"));
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
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let config_path = temp_dir.path().join("config.toml");
    std::fs::write(&config_path, "schema_version = 1\n").expect("write test config");

    let output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args([
            "config",
            "migrate",
            "--path",
            config_path.to_str().unwrap(),
            "--check",
        ])
        .output()
        .expect("failed to run aek config migrate --check");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Configuration Migration Status"));
    assert!(stdout.contains("Current Schema Version:"));

    let dry_run_output = Command::new(env!("CARGO_BIN_EXE_aek"))
        .current_dir(workspace_root())
        .args([
            "config",
            "migrate",
            "--path",
            config_path.to_str().unwrap(),
            "--dry-run",
        ])
        .output()
        .expect("failed to run aek config migrate --dry-run");

    assert!(dry_run_output.status.success());
    let dry_run_stdout = String::from_utf8_lossy(&dry_run_output.stdout);
    assert!(dry_run_stdout.contains("[DRY RUN]"));
}
