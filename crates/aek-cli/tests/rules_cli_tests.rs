// Integration tests for `aek rules` CLI commands

#[test]
fn test_rules_list() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["rules", "list"])
        .output()
        .expect("failed to run aek rules list");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available Rules"));
    assert!(stdout.contains("engineering"));
    assert!(stdout.contains("security"));
}

#[test]
fn test_rules_show() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["rules", "show", "engineering"])
        .output()
        .expect("failed to run aek rules show engineering");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rule: engineering"));
    assert!(stdout.contains("Scope: Global"));
}

#[test]
fn test_rules_validate() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["rules", "validate"])
        .output()
        .expect("failed to run aek rules validate");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All") || stdout.contains("Validated"));
}

#[test]
fn test_rules_install_dry_run() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["rules", "install", "engineering", "--dry-run"])
        .output()
        .expect("failed to run aek rules install --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Dry run")
            || stdout.contains("dry-run")
            || stdout.contains("Would install")
    );
}
