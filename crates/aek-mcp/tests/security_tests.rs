// Security and permission tests for AEK MCP

use aek_mcp::security::{ToolPermission, classify_tool_permission, redact_secrets};

#[test]
fn test_tool_permission_classification() {
    assert_eq!(
        classify_tool_permission("read_file", "Read text file content"),
        ToolPermission::ReadOnly
    );
    assert_eq!(
        classify_tool_permission("get_documentation", "Fetch docs"),
        ToolPermission::ReadOnly
    );
    assert_eq!(
        classify_tool_permission("write_file", "Save file to disk"),
        ToolPermission::FilesystemWrite
    );
    assert_eq!(
        classify_tool_permission("create_directory", "New dir"),
        ToolPermission::FilesystemWrite
    );
    assert_eq!(
        classify_tool_permission("push_commit", "Git push commits"),
        ToolPermission::NetworkWrite
    );
    assert_eq!(
        classify_tool_permission("execute_command", "Run bash shell"),
        ToolPermission::Sensitive
    );
    assert_eq!(
        classify_tool_permission("obscure_tool_xyz", "Does something"),
        ToolPermission::Unknown
    );
}

#[test]
fn test_secret_redaction() {
    let raw = "Authorization: Bearer ghp_secretToken1234567890\npassword=supersecret99";
    let redacted = redact_secrets(raw);
    assert!(!redacted.contains("ghp_secretToken1234567890"));
    assert!(!redacted.contains("supersecret99"));
    assert!(redacted.contains("[REDACTED]"));
}
