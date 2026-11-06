//! Data models, hashing, and status representations for prompts.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

/// Computes a hex-encoded SHA-256 hash of the input string.
#[must_use]
pub fn compute_sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Lifecycle status of a prompt artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptStatus {
    Draft,
    Optimized,
    Evaluated,
    Approved,
    Deprecated,
}

impl fmt::Display for PromptStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Optimized => write!(f, "optimized"),
            Self::Evaluated => write!(f, "evaluated"),
            Self::Approved => write!(f, "approved"),
            Self::Deprecated => write!(f, "deprecated"),
        }
    }
}

/// A versioned prompt artifact with provenance and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRecord {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub original_content: String,
    pub optimized_content: Option<String>,
    pub template_variables: Vec<String>,
    pub constraints: Vec<String>,
    pub status: PromptStatus,
    pub created_at: String,
    pub content_hash: String,
    pub provenance: String,
}

impl PromptRecord {
    /// Creates a new draft prompt record.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        let original_content = content.into();
        let content_hash = compute_sha256(&original_content);
        let now = Utc::now().to_rfc3339();

        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            version: "1.0.0".to_string(),
            original_content,
            optimized_content: None,
            template_variables: Vec::new(),
            constraints: Vec::new(),
            status: PromptStatus::Draft,
            created_at: now,
            content_hash,
            provenance: "manual".to_string(),
        }
    }

    /// Returns the active prompt content (preferring optimized if approved/available).
    #[must_use]
    pub fn active_content(&self) -> &str {
        self.optimized_content
            .as_deref()
            .unwrap_or(&self.original_content)
    }
}

/// Diff comparison between two prompt versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptComparison {
    pub prompt_id: String,
    pub version_a: String,
    pub version_b: String,
    pub hash_a: String,
    pub hash_b: String,
    pub original_diff_lines: Vec<String>,
    pub optimized_diff_lines: Vec<String>,
}
