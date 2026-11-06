//! AEK Resource Profiles (LIGHT, ENGINEERING, FULL).

use crate::error::ProfileError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Resource and capability profile tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProfileKind {
    /// Light tier: minimal resource footprint, core CLI, basic validation.
    #[default]
    Light,
    /// Engineering tier: multi-language tooling, MCP configuration, project validation.
    Engineering,
    /// Full tier: engineering features + prompt optimization + security scanning.
    Full,
}

impl ProfileKind {
    /// Returns slice of all available profile tiers.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[Self::Light, Self::Engineering, Self::Full]
    }

    /// String identifier for the profile.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Engineering => "engineering",
            Self::Full => "full",
        }
    }
}

impl fmt::Display for ProfileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ProfileKind {
    type Err = ProfileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "light" => Ok(Self::Light),
            "engineering" => Ok(Self::Engineering),
            "full" => Ok(Self::Full),
            other => Err(ProfileError::UnknownProfile(other.to_string())),
        }
    }
}

/// Metadata and feature capabilities describing a profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDescriptor {
    /// Profile identifier.
    pub kind: ProfileKind,
    /// Human-friendly name.
    pub name: String,
    /// Detailed description.
    pub description: String,
    /// Target hardware and developer personas.
    pub target_audience: String,
    /// Enabled feature list.
    pub features: Vec<String>,
    /// Default concurrency limit.
    pub default_concurrency: usize,
    /// Default command execution timeout in seconds.
    pub default_timeout_seconds: u64,
    /// Whether MCP server orchestration is enabled.
    pub mcp_enabled: bool,
    /// Whether prompt optimization engine is enabled.
    pub prompt_optimization_enabled: bool,
    /// Whether security vulnerability scanning is enabled.
    pub security_scanning_enabled: bool,
}

impl From<ProfileKind> for ProfileDescriptor {
    fn from(kind: ProfileKind) -> Self {
        match kind {
            ProfileKind::Light => Self {
                kind,
                name: "Light Profile".to_string(),
                description: "Minimal resource footprint without persistent background daemons. \
                              Provides core CLI, configuration management, and basic diagnostics."
                    .to_string(),
                target_audience: "Low-resource machines, 8 GB RAM laptops, CI runners, and minimal containers."
                    .to_string(),
                features: vec![
                    "Core CLI operations".to_string(),
                    "Configuration management".to_string(),
                    "Basic environment diagnostics".to_string(),
                    "Zero background memory usage".to_string(),
                ],
                default_concurrency: 2,
                default_timeout_seconds: 30,
                mcp_enabled: false,
                prompt_optimization_enabled: false,
                security_scanning_enabled: false,
            },
            ProfileKind::Engineering => Self {
                kind,
                name: "Engineering Profile".to_string(),
                description: "Full multi-language developer tooling, MCP server orchestration, \
                              and automated compiler/lint validation."
                    .to_string(),
                target_audience: "Standard development workstations and daily engineering workflows."
                    .to_string(),
                features: vec![
                    "All Light features".to_string(),
                    "Multi-language toolchains (Rust, Go, C/C++, Zig, Python, TS)".to_string(),
                    "Model Context Protocol (MCP) server management".to_string(),
                    "Automated compiler and lint validation".to_string(),
                ],
                default_concurrency: 4,
                default_timeout_seconds: 60,
                mcp_enabled: true,
                prompt_optimization_enabled: false,
                security_scanning_enabled: false,
            },
            ProfileKind::Full => Self {
                kind,
                name: "Full Profile".to_string(),
                description: "Comprehensive AI engineering stack with DSPy programmatic prompts, \
                              Promptfoo red-teaming, and deep security static analysis."
                    .to_string(),
                target_audience: "High-spec workstations with 16 GB+ RAM for full AI agent lifecycle engineering."
                    .to_string(),
                features: vec![
                    "All Engineering features".to_string(),
                    "Prompt Optimizer integration".to_string(),
                    "DSPy programmatic prompt optimization".to_string(),
                    "Promptfoo LLM evaluation and red-teaming".to_string(),
                    "Security auditing and secret scanning".to_string(),
                ],
                default_concurrency: 8,
                default_timeout_seconds: 120,
                mcp_enabled: true,
                prompt_optimization_enabled: true,
                security_scanning_enabled: true,
            },
        }
    }
}
