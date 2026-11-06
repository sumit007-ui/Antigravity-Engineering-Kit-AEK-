//! Release manifest data structures and artifact lookup.

use crate::errors::DistributionError;
use crate::model::PlatformSpec;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Specification of an individual downloadable release artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseArtifact {
    pub filename: String,
    pub target_triple: String,
    pub os: String,
    pub arch: String,
    pub size_bytes: u64,
    pub sha256: String,
    pub download_url: String,
}

/// Machine-readable release manifest detailing an official AEK release.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseManifest {
    pub schema_version: String,
    pub version: String,
    pub release_channel: String,
    pub timestamp: String,
    pub artifacts: Vec<ReleaseArtifact>,
}

impl ReleaseManifest {
    /// Parses a release manifest from a JSON string.
    pub fn from_json(json_str: &str) -> Result<Self, DistributionError> {
        let manifest: Self = serde_json::from_str(json_str)?;
        if manifest.artifacts.is_empty() {
            return Err(DistributionError::Manifest(
                "Release manifest contains no release artifacts".to_string(),
            ));
        }
        Ok(manifest)
    }

    /// Loads and parses a release manifest from a file.
    pub fn load_from_file(path: &Path) -> Result<Self, DistributionError> {
        let content = fs::read_to_string(path)?;
        Self::from_json(&content)
    }

    /// Finds a matching artifact for the specified platform.
    #[must_use]
    pub fn find_artifact(&self, platform: &PlatformSpec) -> Option<&ReleaseArtifact> {
        // 1. Try exact target_triple match
        if let Some(artifact) = self.artifacts.iter().find(|a| {
            a.target_triple
                .eq_ignore_ascii_case(&platform.target_triple)
        }) {
            return Some(artifact);
        }

        // 2. Fallback to OS and Arch match
        self.artifacts.iter().find(|a| {
            a.os.eq_ignore_ascii_case(&platform.os) && a.arch.eq_ignore_ascii_case(&platform.arch)
        })
    }
}
