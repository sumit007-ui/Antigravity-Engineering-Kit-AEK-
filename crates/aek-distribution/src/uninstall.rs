//! Safe uninstallation engine preserving user data and managing purge modes.

use crate::errors::DistributionError;
use crate::model::InstallationManifest;
use std::fs;
use std::path::PathBuf;

/// Complete inventory of files and directories scheduled for deletion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UninstallPlan {
    pub files_to_remove: Vec<PathBuf>,
    pub dirs_to_remove: Vec<PathBuf>,
    pub is_purge: bool,
}

/// Uninstaller managing safe deletion of AEK-managed assets.
#[derive(Debug, Clone)]
pub struct Uninstaller {
    manifest: InstallationManifest,
    config_dir: PathBuf,
}

impl Uninstaller {
    /// Constructs an uninstaller for an active installation.
    #[must_use]
    pub fn new(manifest: InstallationManifest, config_dir: PathBuf) -> Self {
        Self {
            manifest,
            config_dir,
        }
    }

    /// Plans uninstallation steps, preserving configuration unless `purge` is true.
    #[must_use]
    pub fn plan_uninstall(&self, purge: bool) -> UninstallPlan {
        let mut files = Vec::new();

        // 1. Executable binary
        files.push(self.manifest.binary_path.clone());

        // 2. Managed files tracked in manifest
        for f in &self.manifest.managed_files {
            if !files.contains(f) {
                files.push(f.clone());
            }
        }

        let mut dirs = Vec::new();
        if purge {
            // In purge mode, also mark user configuration directory for removal
            dirs.push(self.config_dir.clone());
            dirs.push(self.manifest.install_dir.clone());
        }

        UninstallPlan {
            files_to_remove: files,
            dirs_to_remove: dirs,
            is_purge: purge,
        }
    }

    /// Executes the removal plan safely.
    pub fn execute(&self, plan: &UninstallPlan) -> Result<(), DistributionError> {
        // 1. Delete scheduled files
        for file in &plan.files_to_remove {
            if file.is_file() {
                fs::remove_file(file).map_err(|e| {
                    DistributionError::Uninstall(format!(
                        "Failed to remove file '{}': {e}",
                        file.display()
                    ))
                })?;
            }
        }

        // 2. Delete scheduled directories (only in purge mode)
        for dir in &plan.dirs_to_remove {
            if dir.is_dir() {
                fs::remove_dir_all(dir).map_err(|e| {
                    DistributionError::Uninstall(format!(
                        "Failed to remove directory '{}': {e}",
                        dir.display()
                    ))
                })?;
            }
        }

        Ok(())
    }
}

