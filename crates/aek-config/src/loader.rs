//! Configuration loading, TOML parsing, and path resolution.

use crate::error::ConfigError;
use crate::model::AekConfig;
use aek_platform::paths::PlatformPaths;
use std::fs;
use std::path::{Path, PathBuf};

/// Parses and validates an `AekConfig` from a TOML formatted string.
///
/// # Errors
/// Returns `ConfigError::TomlDeserialization` if syntax is invalid,
/// or `ConfigError::Validation` if constraints are violated.
pub fn load_config_from_str(content: &str) -> Result<AekConfig, ConfigError> {
    let config: AekConfig = toml::from_str(content)?;
    config.validate()?;
    Ok(config)
}

/// Reads, parses, and validates configuration from a file on disk.
///
/// # Errors
/// Returns `ConfigError::NotFound` if path does not exist,
/// or corresponding I/O, deserialization, or validation errors.
pub fn load_config_from_file(path: impl AsRef<Path>) -> Result<AekConfig, ConfigError> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(ConfigError::NotFound(path.to_path_buf()));
    }

    let content = fs::read_to_string(path)?;
    load_config_from_str(&content)
}

/// Resolves the effective configuration file path based on precedence:
/// 1. Explicitly provided path.
/// 2. Workspace root `aek.toml` or `.aek/config.toml`.
/// 3. Platform standard user configuration file.
#[must_use]
pub fn resolve_config_file(explicit_path: Option<&Path>, workspace_dir: Option<&Path>) -> PathBuf {
    if let Some(explicit) = explicit_path {
        return explicit.to_path_buf();
    }

    if let Some(ws) = workspace_dir {
        let ws_aek = ws.join("aek.toml");
        if ws_aek.exists() {
            return ws_aek;
        }
        let dot_aek = ws.join(".aek").join("config.toml");
        if dot_aek.exists() {
            return dot_aek;
        }
    }

    let platform = PlatformPaths::resolve();
    platform.config_file
}
