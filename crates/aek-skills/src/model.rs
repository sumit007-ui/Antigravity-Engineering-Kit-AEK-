//! Skill metadata and bundle representations.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Metadata header describing an agent skill (parsed from YAML frontmatter).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillMetadata {
    /// Unique identifier for the skill (e.g. `rust-systems-engineer`).
    pub name: String,
    /// Detailed description of what the skill provides and when to activate.
    pub description: String,
    /// Semantic version of the skill.
    #[serde(default = "default_version")]
    pub version: String,
    /// Domain tags for categorization.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Heuristic trigger keywords.
    #[serde(default)]
    pub triggers: Vec<String>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

/// A complete, packaged skill bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillBundle {
    /// Parsed frontmatter metadata.
    pub metadata: SkillMetadata,
    /// Root directory path of the skill on disk.
    pub root_dir: PathBuf,
    /// Path to the primary `SKILL.md` instruction file.
    pub instructions_path: PathBuf,
    /// Complete Markdown instructions body (excluding frontmatter).
    pub body: String,
}

impl SkillBundle {
    /// Creates an inline `SkillBundle` representation.
    #[must_use]
    pub fn new_inline(
        name: impl Into<String>,
        description: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        let name_str = name.into();
        Self {
            metadata: SkillMetadata {
                name: name_str.clone(),
                description: description.into(),
                version: "1.0.0".to_string(),
                tags: Vec::new(),
                triggers: Vec::new(),
            },
            root_dir: PathBuf::from(&name_str),
            instructions_path: PathBuf::from(&name_str).join("SKILL.md"),
            body: body.into(),
        }
    }
}
