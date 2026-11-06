//! # aek-config
//!
//! Configuration models, TOML parsing, validation, and atomic filesystem persistence
//! for the Antigravity Engineering Kit (AEK).

pub mod error;
pub mod schema;
pub mod storage;

pub use error::ConfigError;
pub use schema::{AekConfig, AntigravityConfig, GeneralConfig, ToolsConfig};
