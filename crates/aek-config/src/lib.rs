//! # aek-config
//!
//! Strongly-typed configuration schema, validation, profile tier application,
//! atomic file writes, and secure backup/restore engine for AEK.

pub mod backup;
pub mod error;
pub mod loader;
pub mod model;
pub mod validator;
pub mod writer;

pub use backup::{BackupEngine, BackupMetadata, RestoreResult};
pub use error::ConfigError;
pub use loader::{load_config_from_file, load_config_from_str, resolve_config_file};
pub use model::{
    AekConfig, ExecutionConfig, GeneralConfig, IntegrationsConfig, LoggingConfig, SecurityConfig,
    ToolsConfig,
};
pub use validator::validate_config;
pub use writer::save_config_atomic;
