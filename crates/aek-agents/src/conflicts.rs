//! Conflict detection and resolution strategies for multi-agent synchronization.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Strategy to resolve configuration collisions during synchronization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConflictStrategy {
    /// Keep existing file intact, do not overwrite, mark as preserved.
    KeepExisting,
    /// Overwrite existing file only if it is managed by AEK (or create backup).
    ReplaceManaged,
    /// Skip conflicting file and continue with others.
    Skip,
    /// Abort or prompt for manual resolution.
    ManualResolution,
}

impl ConflictStrategy {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::KeepExisting => "keep-existing",
            Self::ReplaceManaged => "replace-managed",
            Self::Skip => "skip",
            Self::ManualResolution => "manual-resolution",
        }
    }
}

impl fmt::Display for ConflictStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ConflictStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase().replace('_', "-");
        match normalized.as_str() {
            "keep-existing" | "keep" => Ok(Self::KeepExisting),
            "replace-managed" | "replace" | "overwrite" => Ok(Self::ReplaceManaged),
            "skip" => Ok(Self::Skip),
            "manual-resolution" | "manual" => Ok(Self::ManualResolution),
            other => Err(format!(
                "Invalid conflict strategy '{other}'. Options: keep-existing, replace-managed, skip, manual-resolution"
            )),
        }
    }
}

/// Outcome of a conflict evaluation for a single managed file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    Preserved,
    Overwritten,
    Skipped,
    RequiresManual,
}
