//! Typed error definitions for `aek-config`.

use std::path::PathBuf;
use thiserror::Error;

/// Configuration subsystem errors.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// File not found at the specified path.
    #[error("Configuration file not found at '{0}'")]
    NotFound(PathBuf),

    /// Standard I/O error during filesystem operation.
    #[error("I/O error during configuration operation: {0}")]
    Io(#[from] std::io::Error),

    /// Deserialization error from TOML format.
    #[error("Failed to parse TOML configuration: {0}")]
    TomlDeserialization(#[from] toml::de::Error),

    /// Serialization error to TOML format.
    #[error("Failed to serialize configuration to TOML: {0}")]
    TomlSerialization(#[from] toml::ser::Error),

    /// Configuration constraint violation.
    #[error("Configuration validation error: {0}")]
    Validation(String),

    /// Atomic write failure.
    #[error("Atomic configuration write failed: {0}")]
    AtomicWrite(String),

    /// Invalid or malformed backup identifier.
    #[error("Invalid backup identifier '{0}'. Expected alphanumeric with hyphens.")]
    InvalidBackupId(String),

    /// Corrupted backup file detected.
    #[error("Backup file '{id}' is corrupted: {details}")]
    CorruptedBackup { id: String, details: String },

    /// Security violation detected (e.g. path traversal or symlink escape).
    #[error("Security violation in configuration engine: {0}")]
    SecurityViolation(String),
}
