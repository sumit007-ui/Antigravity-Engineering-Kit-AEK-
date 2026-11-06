// Integration tests for `aek skills` CLI commands

#[test]
fn test_skills_list() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["skills", "list"])
        .output()
        .expect("failed to run aek skills list");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available Skills"));
    assert!(stdout.contains("rust-systems-engineer"));
    assert!(stdout.contains("devops-engineer"));
}

#[test]
fn test_skills_show() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["skills", "show", "rust-systems-engineer"])
        .output()
        .expect("failed to run aek skills show rust-systems-engineer");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Skill: rust-systems-engineer"));
}

#[test]
fn test_skills_validate() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["skills", "validate"])
        .output()
        .expect("failed to run aek skills validate");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("All") || stdout.contains("Validated"));
}

#[test]
fn test_skills_recommend() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["skills", "recommend"])
        .output()
        .expect("failed to run aek skills recommend");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rust-systems-engineer"));
}

#[test]
fn test_skills_install_dry_run() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_aek"))
        .args(["skills", "install", "rust-systems-engineer", "--dry-run"])
        .output()
        .expect("failed to run aek skills install --dry-run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Dry run")
            || stdout.contains("dry-run")
            || stdout.contains("Would install")
    );
}
