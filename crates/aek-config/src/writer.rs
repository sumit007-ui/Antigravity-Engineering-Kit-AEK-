//! Crash-resilient atomic configuration writing.

use crate::error::ConfigError;
use crate::model::AekConfig;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Serializes and writes configuration to destination using an atomic temporary file.
///
/// Prevents configuration corruption from power loss or abrupt process termination.
///
/// # Errors
/// Returns `ConfigError::Validation`, `ConfigError::TomlSerialization`, or `ConfigError::AtomicWrite`.
pub fn save_config_atomic(path: impl AsRef<Path>, config: &AekConfig) -> Result<(), ConfigError> {
    config.validate()?;
    let path = path.as_ref();

    let parent_dir = path.parent().unwrap_or_else(|| Path::new("."));
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir).map_err(|e| {
            ConfigError::AtomicWrite(format!(
                "Failed to create configuration directory '{}': {e}",
                parent_dir.display()
            ))
        })?;
    }

    let serialized = toml::to_string_pretty(config)?;

    // Create tempfile on the exact same partition/mount
    let mut temp_file = tempfile::Builder::new()
        .prefix(".aek-config-tmp-")
        .suffix(".toml")
        .tempfile_in(parent_dir)
        .map_err(|e| {
            ConfigError::AtomicWrite(format!(
                "Failed to create temporary file in '{}': {e}",
                parent_dir.display()
            ))
        })?;

    temp_file.write_all(serialized.as_bytes()).map_err(|e| {
        ConfigError::AtomicWrite(format!("Failed to write configuration content: {e}"))
    })?;

    temp_file.flush().map_err(|e| {
        ConfigError::AtomicWrite(format!("Failed to flush temporary configuration: {e}"))
    })?;

    temp_file.as_file().sync_all().map_err(|e| {
        ConfigError::AtomicWrite(format!("Failed to sync temporary configuration: {e}"))
    })?;

    temp_file.persist(path).map_err(|e| {
        ConfigError::AtomicWrite(format!(
            "Failed to atomically persist configuration to '{}': {e}",
            path.display()
        ))
    })?;

    Ok(())
}
