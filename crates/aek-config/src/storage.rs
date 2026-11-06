//! Configuration filesystem persistence, atomic writes, and discovery.

use crate::error::ConfigError;
use crate::schema::AekConfig;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

impl AekConfig {
    /// Loads and parses configuration from a TOML string.
    ///
    /// # Errors
    /// Returns `ConfigError::TomlDeserialization` or `ConfigError::Validation`.
    pub fn load_from_str(content: &str) -> Result<Self, ConfigError> {
        let config: Self = toml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }

    /// Loads and parses configuration from a file path.
    ///
    /// # Errors
    /// Returns `ConfigError::NotFound` if the file does not exist,
    /// or `ConfigError::Io`, `ConfigError::TomlDeserialization`, or `ConfigError::Validation`.
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }

        let content = fs::read_to_string(path)?;
        Self::load_from_str(&content)
    }

    /// Saves configuration to the specified destination path atomically.
    ///
    /// Writes content to a temporary file in the same parent directory first,
    /// flushes to disk, and performs an atomic rename to prevent corruption.
    ///
    /// # Errors
    /// Returns `ConfigError::Validation`, `ConfigError::TomlSerialization`, or `ConfigError::AtomicWrite`.
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        self.validate()?;
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

        let serialized = toml::to_string_pretty(self)?;

        // Create temporary file in the same directory to guarantee same filesystem partition
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

        // Write content
        temp_file.write_all(serialized.as_bytes()).map_err(|e| {
            ConfigError::AtomicWrite(format!(
                "Failed to write configuration to temporary file: {e}"
            ))
        })?;

        temp_file.flush().map_err(|e| {
            ConfigError::AtomicWrite(format!("Failed to flush temporary configuration file: {e}"))
        })?;

        // Synchronize data to disk before renaming
        temp_file.as_file().sync_all().map_err(|e| {
            ConfigError::AtomicWrite(format!(
                "Failed to sync temporary configuration to disk: {e}"
            ))
        })?;

        // Atomically replace target file
        temp_file.persist(path).map_err(|e| {
            ConfigError::AtomicWrite(format!(
                "Failed to persist configuration to destination '{}': {e}",
                path.display()
            ))
        })?;

        Ok(())
    }

    /// Resolves the effective configuration file path.
    ///
    /// Precedence:
    /// 1. Explicit path (if provided)
    /// 2. Workspace `.aek/config.toml` or `aek.toml`
    /// 3. Global user configuration `~/.config/aek/config.toml` (or platform equivalent)
    #[must_use]
    pub fn find_config_path(
        explicit_path: Option<&Path>,
        workspace_root: Option<&Path>,
    ) -> Option<PathBuf> {
        if let Some(explicit) = explicit_path {
            if explicit.exists() {
                return Some(explicit.to_path_buf());
            }
        }

        if let Some(root) = workspace_root {
            let local_toml = root.join("aek.toml");
            if local_toml.exists() {
                return Some(local_toml);
            }
            let dot_aek = root.join(".aek").join("config.toml");
            if dot_aek.exists() {
                return Some(dot_aek);
            }
        }

        let global_dir = directories::ProjectDirs::from("com", "antigravity", "aek")
            .map(|dirs| dirs.config_dir().join("config.toml"));

        if let Some(global) = global_dir {
            if global.exists() {
                return Some(global);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_find_config_path_workspace() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();

        // Initially none
        assert!(AekConfig::find_config_path(None, Some(root)).is_none());

        // Create aek.toml
        let local_toml = root.join("aek.toml");
        fs::write(&local_toml, "").expect("write");
        assert_eq!(
            AekConfig::find_config_path(None, Some(root)),
            Some(local_toml)
        );
    }
}
