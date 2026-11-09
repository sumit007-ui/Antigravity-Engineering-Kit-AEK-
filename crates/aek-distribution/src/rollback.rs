//! Safe rollback engine restoring previously archived AEK binaries.

use crate::errors::DistributionError;
use crate::paths::DistributionPaths;
use semver::Version;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

/// Record of an archived version available for rollback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackEntry {
    pub version: String,
    pub binary_path: PathBuf,
}

/// Execution plan for reverting to a previous version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackPlan {
    pub target_version: String,
    pub source_binary: PathBuf,
    pub destination_binary: PathBuf,
}

/// Manager coordinating binary archival and safe restoration.
#[derive(Debug, Clone)]
pub struct RollbackManager {
    paths: DistributionPaths,
}

impl RollbackManager {
    /// Constructs a rollback manager for the given paths.
    #[must_use]
    pub fn new(paths: DistributionPaths) -> Self {
        Self { paths }
    }

    /// Validates version string component: only [A-Za-z0-9._+-], max 64 chars, not "." or "..".
    fn validate_version_component(version: &str) -> Result<(), DistributionError> {
        let v = version.trim();
        if v.is_empty()
            || v.len() > 64
            || v == "."
            || v == ".."
            || !v
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '+' || c == '-')
        {
            return Err(DistributionError::Rollback(format!(
                "Invalid version identifier '{version}': must match [A-Za-z0-9._+-], max 64 chars, not '.' or '..'"
            )));
        }
        Ok(())
    }

    /// Archives an existing binary before an upgrade into the versions directory.
    /// Also records a sha256 checksum file to detect tampering before rollback restore.
    pub fn archive_version(
        &self,
        version: &str,
        current_binary: &Path,
    ) -> Result<PathBuf, DistributionError> {
        Self::validate_version_component(version)?;

        let versions_dir = self.paths.versions_dir();
        let target_dir = versions_dir.join(version);
        fs::create_dir_all(&target_dir)?;

        let exe_name = current_binary
            .file_name()
            .ok_or_else(|| DistributionError::Rollback("Invalid binary filename".to_string()))?;
        let archived_bin = target_dir.join(exe_name);

        fs::copy(current_binary, &archived_bin)?;

        // Compute and store SHA-256 for integrity verification during restore
        let mut file = File::open(&archived_bin)?;
        let mut hasher = Sha256::new();
        let mut buf = [0u8; 8192];
        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }
        let checksum = format!("{:x}", hasher.finalize());
        fs::write(target_dir.join("checksum.sha256"), checksum)?;

        Ok(archived_bin)
    }

    /// Lists all archived versions available for rollback.
    pub fn list_rollbacks(&self) -> Result<Vec<RollbackEntry>, DistributionError> {
        let versions_dir = self.paths.versions_dir();
        if !versions_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        for dir_entry in fs::read_dir(&versions_dir)? {
            let dir_entry = dir_entry?;
            let path = dir_entry.path();
            if let Some(version) = path
                .file_name()
                .and_then(|n| n.to_str())
                .filter(|_| path.is_dir())
            {
                let exe_name = if cfg!(target_os = "windows") {
                    "aek.exe"
                } else {
                    "aek"
                };
                let bin_path = path.join(exe_name);
                if bin_path.is_file() {
                    entries.push(RollbackEntry {
                        version: version.to_string(),
                        binary_path: bin_path,
                    });
                }
            }
        }

        entries.sort_by(|a, b| {
            let ver_a = Version::parse(&a.version);
            let ver_b = Version::parse(&b.version);
            match (ver_a, ver_b) {
                // Both valid semver: sort newest first (b compared to a)
                (Ok(va), Ok(vb)) => vb.cmp(&va),
                // Valid semver comes before invalid
                (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                // Neither valid: sort alphabetically descending
                (Err(_), Err(_)) => b.version.cmp(&a.version),
            }
        });
        Ok(entries)
    }

    /// Plans a rollback to the specified version, or the most recent archived version.
    pub fn plan_rollback(
        &self,
        target_version: Option<&str>,
    ) -> Result<RollbackPlan, DistributionError> {
        if let Some(v) = target_version {
            Self::validate_version_component(v)?;
        }

        let rollbacks = self.list_rollbacks()?;
        if rollbacks.is_empty() {
            return Err(DistributionError::Rollback(
                "No previous versions found in archive for rollback".to_string(),
            ));
        }

        let selected = match target_version {
            Some(v) => rollbacks
                .into_iter()
                .find(|r| r.version == v)
                .ok_or_else(|| {
                    DistributionError::Rollback(format!("Archived version '{v}' not found"))
                })?,
            None => rollbacks.into_iter().next().ok_or_else(|| {
                DistributionError::Rollback("No archived versions found".to_string())
            })?,
        };

        Ok(RollbackPlan {
            target_version: selected.version,
            source_binary: selected.binary_path,
            destination_binary: self.paths.binary_path(),
        })
    }

    /// Executes the rollback restoration plan safely, verifying stored hash if available.
    pub fn execute_rollback(&self, plan: &RollbackPlan) -> Result<(), DistributionError> {
        if !plan.source_binary.is_file() {
            return Err(DistributionError::Rollback(format!(
                "Source binary '{}' does not exist",
                plan.source_binary.display()
            )));
        }

        // Verify hash against archived checksum.sha256 if present
        if let Some(parent) = plan.source_binary.parent() {
            let checksum_file = parent.join("checksum.sha256");
            if checksum_file.is_file() {
                let expected = fs::read_to_string(&checksum_file)?.trim().to_string();
                let mut file = File::open(&plan.source_binary)?;
                let mut hasher = Sha256::new();
                let mut buf = [0u8; 8192];
                loop {
                    let n = file.read(&mut buf)?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buf[..n]);
                }
                let actual = format!("{:x}", hasher.finalize());
                if !actual.eq_ignore_ascii_case(&expected) {
                    return Err(DistributionError::Rollback(format!(
                        "Archived binary integrity check failed! Expected sha256 {expected}, found {actual}"
                    )));
                }
            }
        }

        if let Some(parent) = plan.destination_binary.parent() {
            fs::create_dir_all(parent)?;
        }

        // On Windows, if destination executable is active, standard copy will fail.
        // Copy to temporary file in same folder and atomic rename if possible.
        let temp_dest = plan.destination_binary.with_extension("tmp");
        fs::copy(&plan.source_binary, &temp_dest)?;
        fs::rename(&temp_dest, &plan.destination_binary)?;

        Ok(())
    }
}
