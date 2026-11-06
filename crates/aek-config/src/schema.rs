//! AEK configuration models and validation logic.

use crate::error::ConfigError;
use aek_core::ExecutionMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Top-level AEK configuration file schema (`aek.toml`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AekConfig {
    /// Schema format version (semver).
    #[serde(default = "default_schema_version")]
    pub schema_version: String,

    /// Execution policy mode.
    #[serde(default)]
    pub execution_mode: ExecutionMode,

    /// General operational settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Antigravity platform integration options.
    #[serde(default)]
    pub antigravity: AntigravityConfig,

    /// Toolchain paths and configurations.
    #[serde(default)]
    pub tools: ToolsConfig,
}

fn default_schema_version() -> String {
    "1.0.0".to_string()
}

impl Default for AekConfig {
    fn default() -> Self {
        Self {
            schema_version: default_schema_version(),
            execution_mode: ExecutionMode::RuthlessPrecision,
            general: GeneralConfig::default(),
            antigravity: AntigravityConfig::default(),
            tools: ToolsConfig::default(),
        }
    }
}

impl AekConfig {
    /// Validates configuration values and constraints.
    ///
    /// # Errors
    /// Returns `ConfigError::Validation` if any setting violates invariant bounds.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.schema_version.trim().is_empty() {
            return Err(ConfigError::Validation(
                "schema_version cannot be empty".to_string(),
            ));
        }

        self.general.validate()?;
        self.antigravity.validate()?;
        self.tools.validate()?;

        Ok(())
    }
}

/// General tool settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Logging level (trace, debug, info, warn, error).
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Subprocess execution timeout in seconds.
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,

    /// Colorized terminal output.
    #[serde(default = "default_true")]
    pub color: bool,
}

fn default_log_level() -> String {
    "info".to_string()
}

const fn default_timeout_seconds() -> u64 {
    30
}

const fn default_true() -> bool {
    true
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            timeout_seconds: default_timeout_seconds(),
            color: default_true(),
        }
    }
}

impl GeneralConfig {
    /// Validates general settings.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.timeout_seconds == 0 {
            return Err(ConfigError::Validation(
                "timeout_seconds must be greater than zero".to_string(),
            ));
        }
        if self.timeout_seconds > 3600 {
            return Err(ConfigError::Validation(
                "timeout_seconds cannot exceed 3600 seconds (1 hour)".to_string(),
            ));
        }

        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.log_level.to_lowercase().as_str()) {
            return Err(ConfigError::Validation(format!(
                "Invalid log_level '{}'. Expected one of: trace, debug, info, warn, error",
                self.log_level
            )));
        }

        Ok(())
    }
}

/// Antigravity platform integration settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AntigravityConfig {
    /// Automatically synchronize generated rules to `.agents/rules/` and `AGENTS.md`.
    #[serde(default = "default_true")]
    pub auto_sync_rules: bool,

    /// Enable Model Context Protocol (MCP) server management.
    #[serde(default = "default_true")]
    pub mcp_enabled: bool,

    /// Enable skill registry and discovery.
    #[serde(default = "default_true")]
    pub skills_enabled: bool,

    /// Optional explicit path to global Antigravity config directory.
    pub global_config_dir: Option<PathBuf>,
}

impl Default for AntigravityConfig {
    fn default() -> Self {
        Self {
            auto_sync_rules: default_true(),
            mcp_enabled: default_true(),
            skills_enabled: default_true(),
            global_config_dir: None,
        }
    }
}

impl AntigravityConfig {
    /// Validates Antigravity configuration.
    pub const fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// Tools and toolchain paths configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ToolsConfig {
    /// Custom path to rustc binary if not in PATH.
    pub rustc_path: Option<PathBuf>,

    /// Custom path to cargo binary if not in PATH.
    pub cargo_path: Option<PathBuf>,

    /// Custom path to git binary if not in PATH.
    pub git_path: Option<PathBuf>,

    /// Custom path to node binary if not in PATH.
    pub node_path: Option<PathBuf>,

    /// Custom path to python binary if not in PATH.
    pub python_path: Option<PathBuf>,
}

impl ToolsConfig {
    /// Validates tool paths if provided.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if let Some(ref p) = self.rustc_path {
            if !p.exists() {
                return Err(ConfigError::Validation(format!(
                    "Configured rustc_path '{}' does not exist",
                    p.display()
                )));
            }
        }
        if let Some(ref p) = self.cargo_path {
            if !p.exists() {
                return Err(ConfigError::Validation(format!(
                    "Configured cargo_path '{}' does not exist",
                    p.display()
                )));
            }
        }
        if let Some(ref p) = self.git_path {
            if !p.exists() {
                return Err(ConfigError::Validation(format!(
                    "Configured git_path '{}' does not exist",
                    p.display()
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tools_config_validation() {
        let tools = ToolsConfig::default();
        assert!(tools.validate().is_ok());

        let invalid_tools = ToolsConfig {
            rustc_path: Some(PathBuf::from("/definitely/nonexistent/path/rustc")),
            ..Default::default()
        };
        assert!(invalid_tools.validate().is_err());
    }

    #[test]
    fn test_general_config_upper_bound() {
        let general = GeneralConfig {
            timeout_seconds: 4000,
            ..Default::default()
        };
        assert!(general.validate().is_err());
    }
}
