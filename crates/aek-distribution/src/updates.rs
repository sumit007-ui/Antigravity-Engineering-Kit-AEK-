//! Update planning, version resolution, and staging strategies.

use crate::errors::DistributionError;
use crate::manifest::{ReleaseArtifact, ReleaseManifest};
use crate::model::PlatformSpec;
use crate::versions::VersionManager;

/// Plan detailing available update or downgrade actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePlan {
    pub current_version: String,
    pub target_version: String,
    pub is_update_available: bool,
    pub is_downgrade: bool,
    pub artifact: ReleaseArtifact,
}

/// Evaluates release manifests against the installed environment to generate an update plan.
#[derive(Debug, Clone)]
pub struct UpdateChecker {
    current_version: String,
    platform: PlatformSpec,
}

impl UpdateChecker {
    /// Constructs an update checker for the current version and platform.
    #[must_use]
    pub fn new(current_version: &str, platform: PlatformSpec) -> Self {
        Self {
            current_version: current_version.to_string(),
            platform,
        }
    }

    /// Evaluates a release manifest and determines if an update (or downgrade) is applicable.
    pub fn create_plan(
        &self,
        manifest: &ReleaseManifest,
        allow_downgrade: bool,
    ) -> Result<UpdatePlan, DistributionError> {
        let artifact = manifest
            .find_artifact(&self.platform)
            .ok_or_else(|| DistributionError::IncompatiblePlatform {
                target_triple: self.platform.target_triple.clone(),
            })?
            .clone();

        let vm = VersionManager::new(&self.current_version);
        let is_upgrade = vm.is_upgrade(&manifest.version)?;
        let is_downgrade = vm.is_downgrade(&manifest.version)?;

        if is_downgrade && !allow_downgrade {
            return Err(DistributionError::Version(format!(
                "Target version '{}' is older than installed version '{}'. Pass --force to allow downgrade.",
                manifest.version, self.current_version
            )));
        }

        Ok(UpdatePlan {
            current_version: self.current_version.clone(),
            target_version: manifest.version.clone(),
            is_update_available: is_upgrade,
            is_downgrade,
            artifact,
        })
    }
}

