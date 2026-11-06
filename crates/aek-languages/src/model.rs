//! Core models and data structures for language toolchains, projects, and diagnostics.

use aek_platform::OsKind;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

/// Supported programming languages in AEK.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LanguageId {
    Rust,
    Go,
    Cpp,
    Zig,
    Python,
    Javascript,
    Flutter,
    Ros2,
    Custom(String),
}

impl LanguageId {
    /// Returns the canonical string identifier for the language.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Rust => "rust",
            Self::Go => "go",
            Self::Cpp => "cpp",
            Self::Zig => "zig",
            Self::Python => "python",
            Self::Javascript => "javascript",
            Self::Flutter => "flutter",
            Self::Ros2 => "ros2",
            Self::Custom(s) => s.as_str(),
        }
    }
}

impl fmt::Display for LanguageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for LanguageId {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_ascii_lowercase();
        Ok(match normalized.as_str() {
            "rust" | "rs" => Self::Rust,
            "go" | "golang" => Self::Go,
            "cpp" | "c++" | "c" => Self::Cpp,
            "zig" => Self::Zig,
            "python" | "py" => Self::Python,
            "javascript" | "typescript" | "js" | "ts" | "node" => Self::Javascript,
            "flutter" | "dart" => Self::Flutter,
            "ros2" | "ros" => Self::Ros2,
            other => Self::Custom(other.to_string()),
        })
    }
}

/// Tool requirement level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolRequirement {
    Required,
    Optional,
    Recommended,
}

impl fmt::Display for ToolRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Required => write!(f, "required"),
            Self::Optional => write!(f, "optional"),
            Self::Recommended => write!(f, "recommended"),
        }
    }
}

/// Tool category for classification in diagnostics and reports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolCategory {
    Compiler,
    PackageManager,
    Linter,
    Formatter,
    LanguageServer,
    Debugger,
    BuildSystem,
    Sdk,
    Other(String),
}

/// Typed definition for a toolchain executable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub id: String,
    pub name: String,
    pub language: LanguageId,
    pub binary: String,
    pub version_args: Vec<String>,
    pub supported_os: Vec<OsKind>,
    pub requirement: ToolRequirement,
    pub category: ToolCategory,
    pub doc_url: String,
    pub install_guidance: String,
}

/// Tool discovery result after probing host system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredTool {
    pub definition: ToolDefinition,
    pub path: Option<PathBuf>,
    pub version: Option<String>,
    pub is_available: bool,
}

/// Evidence that identified a project on the filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEvidence {
    pub file: String,
    pub matched_content: Option<String>,
    pub description: String,
}

/// Represents a detected software project on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedProject {
    pub path: PathBuf,
    pub language: LanguageId,
    pub name: String,
    pub evidence: Vec<ProjectEvidence>,
    pub monorepo_root: Option<PathBuf>,
    pub is_nested: bool,
}

/// Diagnostic compatibility classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompatibilityLevel {
    Compatible,
    Warning,
    Incompatible,
    Unknown,
}

impl fmt::Display for CompatibilityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compatible => write!(f, "COMPATIBLE"),
            Self::Warning => write!(f, "WARNING"),
            Self::Incompatible => write!(f, "INCOMPATIBLE"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

/// Detailed compatibility check result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityItem {
    pub name: String,
    pub level: CompatibilityLevel,
    pub message: String,
    pub remediation: Option<String>,
}

/// Full compatibility report for a project or language toolchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityReport {
    pub language: LanguageId,
    pub overall_level: CompatibilityLevel,
    pub items: Vec<CompatibilityItem>,
    pub remediation_steps: Vec<String>,
}

/// Safe toolchain installation plan with actionable commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupPlan {
    pub language: LanguageId,
    pub missing_required: Vec<ToolDefinition>,
    pub missing_optional: Vec<ToolDefinition>,
    pub suggested_commands: Vec<String>,
    pub instructions: Vec<String>,
}
