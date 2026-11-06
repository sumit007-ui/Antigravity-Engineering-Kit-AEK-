//! Antigravity Engineering Kit (AEK) Distribution & Lifecycle Engine.
//!
//! Provides cross-platform installation management, release manifest verification,
//! cryptographic checksum enforcement, rollback restoration, configuration migration,
//! and safe uninstallation for Windows and Linux systems.

pub mod errors;
pub mod manifest;
pub mod migration;
pub mod model;
pub mod paths;
pub mod rollback;
pub mod uninstall;
pub mod updates;
pub mod verification;
pub mod versions;

pub use errors::DistributionError;
pub use manifest::{ReleaseArtifact, ReleaseManifest};
pub use migration::{ConfigMigrationStep, ConfigMigrator, MigrationPlan};
pub use model::{
    InstallationManifest, InstallationMethod, InstallationScope, InstalledFile, PlatformSpec,
    VerificationStatus,
};
pub use paths::DistributionPaths;
pub use rollback::{RollbackEntry, RollbackManager, RollbackPlan};
pub use uninstall::{UninstallPlan, Uninstaller};
pub use updates::{UpdateChecker, UpdatePlan};
pub use verification::{InstallationVerifier, compute_file_sha256, verify_file_checksum};
pub use versions::{ReleaseChannel, VersionManager};
