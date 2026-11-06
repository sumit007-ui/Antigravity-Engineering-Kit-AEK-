//! Typed error definitions for `aek-core`.

use thiserror::Error;

/// Core errors encountered during AEK system operations and diagnostics.
#[derive(Error, Debug)]
pub enum AekCoreError {
    /// Subprocess execution error (e.g. command not found or spawned failed).
    #[error("Subprocess execution failed: {0}")]
    ProcessExecutionFailed(String),

    /// Process execution exceeded timeout bounds.
    #[error("Command '{command}' timed out after {timeout_secs} seconds")]
    ProcessTimeout { command: String, timeout_secs: u64 },

    /// Version string extraction or semver parsing failure.
    #[error("Failed to parse tool version from output: {0}")]
    VersionParseError(String),

    /// Standard I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization or deserialization error.
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Generic domain error.
    #[error("Core domain error: {0}")]
    Generic(String),
}
