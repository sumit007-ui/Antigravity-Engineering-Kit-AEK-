use aek_core::execution::{ExecutionError, SafeCommand, redact_secrets};
use std::time::Duration;

#[tokio::test]
async fn test_safe_command_execution() {
    let output = SafeCommand::new("cargo")
        .arg("--version")
        .execute()
        .await
        .expect("cargo --version should execute successfully");

    assert!(output.success);
    assert_eq!(output.status_code, Some(0));
    assert!(output.stdout.contains("cargo"));
}

#[tokio::test]
async fn test_safe_command_dry_run() {
    let output = SafeCommand::new("rm")
        .args(["-rf", "/some/path"])
        .dry_run(true)
        .execute()
        .await
        .expect("dry run should succeed without executing");

    assert!(output.success);
    assert!(output.stdout.contains("[DRY-RUN]"));
}

#[tokio::test]
async fn test_safe_command_timeout() {
    // Ping with large count or sleep will trigger timeout
    #[cfg(windows)]
    let cmd = SafeCommand::new("ping")
        .args(["-n", "10", "127.0.0.1"])
        .timeout(Duration::from_millis(50));

    #[cfg(not(windows))]
    let cmd = SafeCommand::new("sleep")
        .arg("10")
        .timeout(Duration::from_millis(50));

    let result = cmd.execute().await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, ExecutionError::Timeout { .. }));
}

#[test]
fn test_secret_redaction() {
    let envs = vec![
        ("API_KEY".to_string(), "secret_token_12345".to_string()),
        (
            "AWS_SECRET_ACCESS_KEY".to_string(),
            "super_secret".to_string(),
        ),
        ("APP_ENV".to_string(), "production".to_string()),
    ];

    let redacted = redact_secrets(&envs);
    assert_eq!(redacted.get("API_KEY").unwrap(), "********");
    assert_eq!(redacted.get("AWS_SECRET_ACCESS_KEY").unwrap(), "********");
    assert_eq!(redacted.get("APP_ENV").unwrap(), "production");
}
