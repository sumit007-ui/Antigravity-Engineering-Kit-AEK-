//! Rule models, scopes, and representation.

use serde::{Deserialize, Serialize};

/// Target application scope for an engineering rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleScope {
    /// Lowest baseline precedence: global default rules.
    Global = 1,
    /// Medium precedence: team or repository workspace rules (`.agents/rules/`).
    Workspace = 2,
    /// Highest precedence: project or language specific overrides.
    Project = 3,
}

impl std::fmt::Display for RuleScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Global => write!(f, "global"),
            Self::Workspace => write!(f, "workspace"),
            Self::Project => write!(f, "project"),
        }
    }
}

/// An engineering guideline or instruction rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    /// Unique identifier (lowercase alphanumeric with hyphens, e.g. `engineering-workflow`).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Short description of the rule's purpose.
    pub description: String,
    /// Hierarchy scope.
    pub scope: RuleScope,
    /// Numerical priority within scope (higher numbers indicate higher precedence).
    pub priority: u32,
    /// Whether the rule is currently active.
    pub enabled: bool,
    /// Source origin (e.g. `builtin`, `local`, or file path).
    pub source: String,
    /// Markdown rule content.
    pub content: String,
    /// Version string.
    pub version: String,
}

impl Rule {
    /// Creates a new `Rule` with default enabled state and version 1.0.0.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        scope: RuleScope,
        priority: u32,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            scope,
            priority,
            enabled: true,
            source: "builtin".to_string(),
            content: content.into(),
            version: "1.0.0".to_string(),
        }
    }
}

