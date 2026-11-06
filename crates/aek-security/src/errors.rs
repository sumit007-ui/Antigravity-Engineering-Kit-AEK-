//! Error types for the `aek-security` crate.

use std::path::PathBuf;
use thiserror::Error;

/// Typed errors encountered during security scanning and dependency audits.
#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Subprocess execution error: {0}")]
    Execution(String),

    #[error("Scanner '{0}' is not installed or available on PATH")]
    ScannerNotAvailable(String),

    #[error("Scanner timeout exceeded for '{0}'")]
    Timeout(String),

    #[error("Path traversal rejected: target path '{0}' escapes allowed boundary")]
    PathTraversal(PathBuf),

    #[error("Failed to parse scanner output: {0}")]
    OutputParse(String),

    #[error("Security policy violation: {0}")]
    PolicyViolation(String),
}
