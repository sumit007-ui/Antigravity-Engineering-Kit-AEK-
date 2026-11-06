//! Strongly-typed configuration schema and profile application.

use crate::error::ConfigError;
use aek_core::profile::{ProfileDescriptor, ProfileKind};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Primary AEK system configuration (`config.toml`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct AekConfig {
    /// General core metadata and active profile.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Logging preferences.
    #[serde(default)]
    pub logging: LoggingConfig,

    /// Subprocess and task execution policies.
    #[serde(default)]
    pub execution: ExecutionConfig,

    /// Operational safety and permissions.
    #[serde(default)]
    pub security: SecurityConfig,

    /// Modular integration flags.
    #[serde(default)]
    pub integrations: IntegrationsConfig,

    /// Optional toolchain executable paths.
    #[serde(default)]
    pub tools: ToolsConfig,
}

impl AekConfig {
    /// Applies a profile tier while preserving unrelated user preferences.
    pub fn apply_profile(&mut self, profile: ProfileKind) {
        let desc = ProfileDescriptor::from(profile);
        self.general.profile = profile;
        self.integrations.mcp_enabled = desc.mcp_enabled;
        self.integrations.prompt_optimization_enabled = desc.prompt_optimization_enabled;
        self.integrations.security_scanning_enabled = desc.security_scanning_enabled;
        self.execution.max_concurrent_tasks = desc.default_concurrency;
        self.execution.default_timeout_seconds = desc.default_timeout_seconds;
    }

    /// Validates the configuration model invariants and constraints.
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.general.validate()?;
        self.logging.validate()?;
        self.execution.validate()?;
        self.security.validate()?;
        self.integrations.validate()?;
        self.tools.validate()?;
        Ok(())
    }

    /// Loads and validates configuration from a TOML string.
    pub fn load_from_str(content: &str) -> Result<Self, ConfigError> {
        crate::loader::load_config_from_str(content)
    }

    /// Loads and validates configuration from a file path.
    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> {
        crate::loader::load_config_from_file(path)
    }

    /// Saves configuration atomically to disk.
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), ConfigError> {
        crate::writer::save_config_atomic(path, self)
    }

    /// Resolves configuration path if it exists on disk.
    #[must_use]
    pub fn find_config_path(
        explicit_path: Option<&std::path::Path>,
        workspace_dir: Option<&std::path::Path>,
    ) -> Option<PathBuf> {
        let p = crate::loader::resolve_config_file(explicit_path, workspace_dir);
        if p.exists() { Some(p) } else { None }
    }
}

/// General project settings and schema version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneralConfig {
    /// Configuration name or project identifier.
    #[serde(default = "default_name")]
    pub name: String,

    /// Configuration schema version (must be 1 for current spec).
    #[serde(default = "default_config_version")]
    pub config_version: u32,

    /// Active resource profile tier.
    #[serde(default)]
    pub profile: ProfileKind,
}

fn default_name() -> String {
    "AEK".to_string()
}

const fn default_config_version() -> u32 {
    1
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            name: default_name(),
            config_version: default_config_version(),
            profile: ProfileKind::Light,
        }
    }
}

impl GeneralConfig {
    /// Validates general section.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.name.trim().is_empty() {
            return Err(ConfigError::Validation(
                "general.name cannot be empty".to_string(),
            ));
        }
        if self.config_version != 1 {
            return Err(ConfigError::Validation(format!(
                "Unsupported config_version {}. Expected version 1.",
                self.config_version
            )));
        }
        Ok(())
    }
}

/// Logging and diagnostic output settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingConfig {
    /// Log verbosity level: trace, debug, info, warn, error.
    #[serde(default = "default_level")]
    pub level: String,

    /// Output format: text or json.
    #[serde(default = "default_format")]
    pub format: String,
}

fn default_level() -> String {
    "info".to_string()
}

fn default_format() -> String {
    "text".to_string()
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_level(),
            format: default_format(),
        }
    }
}

impl LoggingConfig {
    /// Validates logging section.
    pub fn validate(&self) -> Result<(), ConfigError> {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.level.to_lowercase().as_str()) {
            return Err(ConfigError::Validation(format!(
                "Invalid logging.level '{}'. Expected one of: trace, debug, info, warn, error",
                self.level
            )));
        }

        let valid_formats = ["text", "json"];
        if !valid_formats.contains(&self.format.to_lowercase().as_str()) {
            return Err(ConfigError::Validation(format!(
                "Invalid logging.format '{}'. Expected 'text' or 'json'",
                self.format
            )));
        }

        Ok(())
    }
}

/// Process execution constraints and limits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionConfig {
    /// Dry run mode: print actions without executing.
    #[serde(default)]
    pub dry_run: bool,

    /// Maximum concurrent background/build tasks (1 to 32).
    #[serde(default = "default_max_concurrent_tasks")]
    pub max_concurrent_tasks: usize,

    /// Default command timeout in seconds (1 to 3600).
    #[serde(default = "default_timeout")]
    pub default_timeout_seconds: u64,
}

const fn default_max_concurrent_tasks() -> usize {
    2
}

const fn default_timeout() -> u64 {
    30
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            dry_run: false,
            max_concurrent_tasks: default_max_concurrent_tasks(),
            default_timeout_seconds: default_timeout(),
        }
    }
}

impl ExecutionConfig {
    /// Validates execution bounds.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_concurrent_tasks == 0 || self.max_concurrent_tasks > 32 {
            return Err(ConfigError::Validation(
                "execution.max_concurrent_tasks must be between 1 and 32".to_string(),
            ));
        }
        if self.default_timeout_seconds == 0 || self.default_timeout_seconds > 3600 {
            return Err(ConfigError::Validation(
                "execution.default_timeout_seconds must be between 1 and 3600".to_string(),
            ));
        }
        Ok(())
    }
}

/// Safety invariants and confirmation requirements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityConfig {
    /// Allow operations that delete or replace files.
    #[serde(default)]
    pub allow_destructive_operations: bool,

    /// Require interactive prompt confirmation for irreversible modifications.
    #[serde(default = "default_true")]
    pub require_confirmation: bool,
}

const fn default_true() -> bool {
    true
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allow_destructive_operations: false,
            require_confirmation: true,
        }
    }
}

impl SecurityConfig {
    /// Validates security section.
    pub const fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// Integrations configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct IntegrationsConfig {
    /// Model Context Protocol (MCP) server management.
    #[serde(default)]
    pub mcp_enabled: bool,

    /// Prompt optimization engine (Prompt Optimizer / DSPy).
    #[serde(default)]
    pub prompt_optimization_enabled: bool,

    /// Automated security auditing and vulnerability scanning.
    #[serde(default)]
    pub security_scanning_enabled: bool,
}

impl IntegrationsConfig {
    /// Validates integrations section.
    pub const fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// Optional paths for external toolchains.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ToolsConfig {
    pub rustc_path: Option<PathBuf>,
    pub cargo_path: Option<PathBuf>,
    pub git_path: Option<PathBuf>,
    pub node_path: Option<PathBuf>,
    pub python_path: Option<PathBuf>,
}

impl ToolsConfig {
    /// Validates configured tool paths.
    pub fn validate(&self) -> Result<(), ConfigError> {
        let paths = [
            ("rustc_path", &self.rustc_path),
            ("cargo_path", &self.cargo_path),
            ("git_path", &self.git_path),
            ("node_path", &self.node_path),
            ("python_path", &self.python_path),
        ];

        for (name, opt) in paths {
            if let Some(path) = opt {
                if !path.exists() {
                    return Err(ConfigError::Validation(format!(
                        "Configured tools.{name} '{path}' does not exist on disk",
                        path = path.display()
                    )));
                }
            }
        }
        Ok(())
    }
}
