//! Semantic versioning, channel detection, and comparison mechanics.

use crate::errors::DistributionError;
use semver::Version;
use serde::{Deserialize, Serialize};

/// Release channel categorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseChannel {
    Stable,
    Beta,
    Nightly,
}

impl std::fmt::Display for ReleaseChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stable => write!(f, "stable"),
            Self::Beta => write!(f, "beta"),
            Self::Nightly => write!(f, "nightly"),
        }
    }
}

/// Version manager providing semver comparison and downgrade guards.
#[derive(Debug, Clone)]
pub struct VersionManager {
    current_version: String,
}

impl VersionManager {
    /// Constructs a version manager for the specified version string.
    #[must_use]
    pub fn new(current_version: &str) -> Self {
        Self {
            current_version: current_version.to_string(),
        }
    }

    /// Returns the active current version string.
    #[must_use]
    pub fn current_version(&self) -> &str {
        &self.current_version
    }

    /// Determines whether `target` is an upgrade relative to current version.
    pub fn is_upgrade(&self, target: &str) -> Result<bool, DistributionError> {
        let cur = Version::parse(self.clean_version(&self.current_version))
            .map_err(|e| DistributionError::Version(format!("Invalid current version: {e}")))?;
        let tgt = Version::parse(self.clean_version(target))
            .map_err(|e| DistributionError::Version(format!("Invalid target version: {e}")))?;

        Ok(tgt > cur)
    }

    /// Determines whether `target` is a downgrade relative to current version.
    pub fn is_downgrade(&self, target: &str) -> Result<bool, DistributionError> {
        let cur = Version::parse(self.clean_version(&self.current_version))
            .map_err(|e| DistributionError::Version(format!("Invalid current version: {e}")))?;
        let tgt = Version::parse(self.clean_version(target))
            .map_err(|e| DistributionError::Version(format!("Invalid target version: {e}")))?;

        Ok(tgt < cur)
    }

    /// Detects the release channel for a version string.
    #[must_use]
    pub fn channel(&self, version: &str) -> ReleaseChannel {
        let lower = version.to_lowercase();
        if lower.contains("nightly") {
            ReleaseChannel::Nightly
        } else if lower.contains("beta") || lower.contains("rc") || lower.contains("alpha") {
            ReleaseChannel::Beta
        } else {
            ReleaseChannel::Stable
        }
    }

    fn clean_version<'a>(&self, v: &'a str) -> &'a str {
        v.trim_start_matches('v')
    }
}

