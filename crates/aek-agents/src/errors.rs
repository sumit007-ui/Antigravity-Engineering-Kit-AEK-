//! Typed errors for agent operations, translation, and synchronization.

use thiserror::Error;

/// Errors arising during agent adapter execution, translation, or synchronization.
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("I/O error during agent operation: {0}")]
    Io(#[from] std::io::Error),

    #[error("Rule installation error: {0}")]
    RuleInstall(#[from] aek_rules::installer::RuleInstallError),

    #[error("Skill installation error: {0}")]
    SkillInstall(#[from] aek_skills::installer::SkillInstallError),

    #[error("MCP configuration error: {0}")]
    Mcp(#[from] aek_mcp::errors::McpError),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Target agent '{0}' is not installed or detectable")]
    AgentNotDetected(String),

    #[error("Security violation: {0}")]
    Security(String),

    #[error("Configuration conflict detected: {0}")]
    Conflict(String),

    #[error("Agent '{agent}' does not support capability '{capability}'")]
    UnsupportedCapability { agent: String, capability: String },

    #[error("Agent profile error: {0}")]
    InvalidProfile(String),

    #[error("Agent validation failure: {0}")]
    Validation(String),
}
