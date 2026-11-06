//! Typed error definitions for `aek-core`.

use std::time::Duration;
use thiserror::Error;

/// Core errors encountered during AEK system operations and diagnostics.
#[derive(Error, Debug)]
pub enum AekCoreError {
    /// Subprocess execution failure.
    #[error("Subprocess execution failed: {0}")]
    ProcessExecutionFailed(String),

    /// Process execution exceeded timeout bounds.
    #[error("Command '{command}' timed out after {timeout_secs} seconds")]
    ProcessTimeout { command: String, timeout_secs: u64 },

    /// Subprocess execution error.
    #[error(transparent)]
    Execution(#[from] ExecutionError),

    /// Profile resolution or application error.
    #[error(transparent)]
    Profile(#[from] ProfileError),

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

/// Errors specific to subprocess execution.
#[derive(Error, Debug)]
pub enum ExecutionError {
    /// Command execution timed out.
    #[error("Command '{command}' timed out after {timeout:?}")]
    Timeout { command: String, timeout: Duration },

    /// Command failed to spawn or execute.
    #[error("Failed to spawn process '{command}': {source}")]
    SpawnFailed {
        command: String,
        #[source]
        source: std::io::Error,
    },

    /// Process terminated abnormally or was killed.
    #[error("Process '{command}' was terminated unexpectedly")]
    Terminated { command: String },

    /// Invalid executable path or security restriction.
    #[error("Executable '{path}' is invalid or blocked by execution policy")]
    InvalidExecutable { path: String },
}

/// Errors specific to profile operations.
#[derive(Error, Debug)]
pub enum ProfileError {
    /// Unknown profile name specified.
    #[error("Unknown profile '{0}'. Available profiles: light, engineering, full")]
    UnknownProfile(String),

    /// Profile configuration validation failure.
    #[error("Profile validation failed: {0}")]
    Validation(String),
}
