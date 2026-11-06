//! Error types for distribution, installation, updates, and rollback.

use std::path::PathBuf;
use thiserror::Error;

/// Specific error conditions encountered in distribution workflows.
#[derive(Debug, Error)]
pub enum DistributionError {
    #[error("Path traversal detected: path '{path}' escapes base directory '{base}'")]
    PathTraversal { path: PathBuf, base: PathBuf },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML serialization/deserialization error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Invalid release manifest: {0}")]
    Manifest(String),

    #[error("Checksum mismatch for '{path}': expected {expected}, calculated {actual}")]
    ChecksumMismatch {
        path: PathBuf,
        expected: String,
        actual: String,
    },

    #[error("Incompatible platform: target triple '{target_triple}' is not compatible with host")]
    IncompatiblePlatform { target_triple: String },

    #[error("Version error: {0}")]
    Version(String),

    #[error("Rollback failed: {0}")]
    Rollback(String),

    #[error("Configuration migration failed: {0}")]
    Migration(String),

    #[error("Uninstall failed: {0}")]
    Uninstall(String),

    #[error("Network error querying release source: {0}")]
    Network(String),
}
