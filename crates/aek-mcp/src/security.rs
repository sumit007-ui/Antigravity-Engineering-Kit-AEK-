//! Security classification, permission boundaries, and secret sanitization for MCP.

use crate::errors::McpError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Classification of actions and tools by risk profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolPermission {
    /// Safe operations that only read data (documentation, status checks, listing files).
    ReadOnly = 1,
    /// Operations modifying files, directories, or local storage.
    FilesystemWrite = 2,
    /// Operations transmitting or modifying state over the network.
    NetworkWrite = 3,
    /// High-risk operations (shell execution, credential exposure, system configuration).
    Sensitive = 4,
    /// Unclassified or ambiguous tools. Handled with conservative policies.
    Unknown = 5,
}

impl std::fmt::Display for ToolPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadOnly => write!(f, "READ_ONLY"),
            Self::FilesystemWrite => write!(f, "FILESYSTEM_WRITE"),
            Self::NetworkWrite => write!(f, "NETWORK_WRITE"),
            Self::Sensitive => write!(f, "SENSITIVE"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

/// Heuristically classifies a tool's permission risk level based on identifier and description.
///
/// # Security Note
/// This classification is an informational, advisory risk label intended for UX warnings
/// and prompt-level policy checks. It does NOT constitute a kernel sandbox or hardware isolation
/// boundary. External tools must run in least-privilege containers or OS-level sandboxes
/// if untrusted.
#[must_use]
pub fn classify_tool_permission(name: &str, description: &str) -> ToolPermission {
    let lower_name = name.to_ascii_lowercase();
    let lower_desc = description.to_ascii_lowercase();

    // 1. High-risk / Sensitive commands
    if lower_name.contains("exec")
        || lower_name.contains("shell")
        || lower_name.contains("command")
        || lower_name.contains("terminal")
        || lower_name.contains("sudo")
        || lower_name.contains("eval")
        || lower_name.contains("run_code_unsafe")
        || lower_desc.contains("execute command")
        || lower_desc.contains("run code")
        || lower_desc.contains("shell")
    {
        return ToolPermission::Sensitive;
    }

    // 2. Filesystem mutation
    if lower_name.contains("write")
        || lower_name.contains("edit")
        || lower_name.contains("delete")
        || lower_name.contains("remove")
        || lower_name.contains("create")
        || lower_name.contains("move")
        || lower_name.contains("mkdir")
        || lower_name.contains("touch")
        || lower_desc.contains("write")
        || lower_desc.contains("modify file")
        || lower_desc.contains("create directory")
    {
        return ToolPermission::FilesystemWrite;
    }

    // 3. Network mutations
    if lower_name.contains("push")
        || lower_name.contains("post")
        || lower_name.contains("publish")
        || lower_name.contains("deploy")
        || lower_name.contains("merge")
        || lower_name.contains("pull_request")
        || lower_name.contains("issue")
        || lower_name.contains("comment")
        || lower_desc.contains("create pull request")
        || lower_desc.contains("push")
    {
        return ToolPermission::NetworkWrite;
    }

    // 4. Read-only queries
    if lower_name.contains("read")
        || lower_name.contains("get")
        || lower_name.contains("list")
        || lower_name.contains("search")
        || lower_name.contains("find")
        || lower_name.contains("query")
        || lower_name.contains("doc")
        || lower_name.contains("inspect")
        || lower_name.contains("fetch")
        || lower_name.contains("view")
        || lower_desc.contains("read")
        || lower_desc.contains("search")
        || lower_desc.contains("query")
    {
        return ToolPermission::ReadOnly;
    }

    ToolPermission::Unknown
}

/// Redacts sensitive credential patterns (bearer tokens, passwords, GitHub PATs, API keys).
#[must_use]
pub fn redact_secrets(input: &str) -> String {
    let mut output = input.to_string();

    // Redact Bearer tokens
    let bearer_regex = [
        ("Bearer [a-zA-Z0-9_.-]{8,}", "Bearer [REDACTED]"),
        ("bearer [a-zA-Z0-9_.-]{8,}", "bearer [REDACTED]"),
        ("ghp_[a-zA-Z0-9]{20,}", "ghp_[REDACTED]"),
        ("github_pat_[a-zA-Z0-9_]{20,}", "github_pat_[REDACTED]"),
    ];

    for (target_prefix, _replacement) in &bearer_regex {
        if target_prefix.starts_with("Bearer ") {
            if let Some(pos) = output.find("Bearer ") {
                let rest = &output[pos + 7..];
                let end = rest
                    .find(|c: char| c.is_whitespace() || c == '"' || c == '\'')
                    .unwrap_or(rest.len());
                output.replace_range(pos + 7..pos + 7 + end, "[REDACTED]");
            }
        } else if target_prefix.starts_with("ghp_") {
            while let Some(pos) = output.find("ghp_") {
                let rest = &output[pos..];
                let end = rest
                    .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == '\n')
                    .unwrap_or(rest.len());
                output.replace_range(pos..pos + end, "ghp_[REDACTED]");
            }
        }
    }

    // Redact password=... or token=... lines
    let patterns = ["password=", "secret=", "token=", "key="];
    for p in &patterns {
        let mut start_idx = 0;
        while let Some(pos) = output[start_idx..].to_ascii_lowercase().find(p) {
            let actual_pos = start_idx + pos + p.len();
            let rest = &output[actual_pos..];
            let end = rest
                .find(|c: char| c.is_whitespace() || c == '&' || c == ';' || c == '"')
                .unwrap_or(rest.len());
            if end > 0 {
                output.replace_range(actual_pos..actual_pos + end, "[REDACTED]");
            }
            start_idx = actual_pos + 10;
            if start_idx >= output.len() {
                break;
            }
        }
    }

    output
}

/// Validates that a requested path is strictly contained within at least one allowed root directory.
pub fn validate_path_containment(
    target: &Path,
    allowed_roots: &[PathBuf],
) -> Result<PathBuf, McpError> {
    if allowed_roots.is_empty() {
        return Err(McpError::PermissionDenied(
            target.display().to_string(),
            "No allowed roots configured for filesystem operations".to_string(),
        ));
    }

    // Reject explicit path traversal tokens
    let target_str = target.to_string_lossy();
    if target_str.contains("..") {
        return Err(McpError::PermissionDenied(
            target.display().to_string(),
            "Path contains illegal '..' traversal sequence".to_string(),
        ));
    }

    let target_canonical = target
        .canonicalize()
        .unwrap_or_else(|_| target.to_path_buf());

    for root in allowed_roots {
        let root_canonical = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
        if target_canonical.starts_with(&root_canonical) {
            return Ok(target_canonical);
        }
    }

    Err(McpError::PermissionDenied(
        target.display().to_string(),
        format!("Path is outside allowed directories: {:?}", allowed_roots),
    ))
}
