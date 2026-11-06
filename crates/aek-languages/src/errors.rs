//! Typed errors for the language toolchain and detection engine.

use thiserror::Error;

/// Language toolchain and project detection errors.
#[derive(Error, Debug)]
pub enum LanguageError {
    #[error("I/O error during language operations: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse project manifest at '{0}': {1}")]
    ManifestParse(String, String),

    #[error("Tool '{0}' not found on system PATH")]
    ToolNotFound(String),

    #[error("Tool execution failed: {0}")]
    CommandFailed(String),

    #[error("Invalid language configuration: {0}")]
    InvalidConfig(String),

    #[error("Unsupported platform for language '{0}': {1}")]
    UnsupportedPlatform(String, String),

    #[error("Security policy violation: {0}")]
    Security(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
