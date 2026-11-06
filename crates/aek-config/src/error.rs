//! Typed error definitions for `aek-config`.

use std::path::PathBuf;
use thiserror::Error;

/// Configuration subsystem errors.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// File not found.
    #[error("Configuration file not found at '{0}'")]
    NotFound(PathBuf),

    /// Standard I/O errors.
    #[error("I/O error during configuration operation: {0}")]
    Io(#[from] std::io::Error),

    /// Deserialization errors from TOML format.
    #[error("Failed to parse TOML configuration: {0}")]
    TomlDeserialization(#[from] toml::de::Error),

    /// Serialization errors to TOML format.
    #[error("Failed to serialize configuration to TOML: {0}")]
    TomlSerialization(#[from] toml::ser::Error),

    /// Configuration constraint violation.
    #[error("Configuration validation error: {0}")]
    Validation(String),

    /// Atomic write failure.
    #[error("Atomic configuration write failed: {0}")]
    AtomicWrite(String),
}
