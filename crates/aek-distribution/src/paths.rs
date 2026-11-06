//! Platform installation directory resolution and path containment defenses.

use crate::errors::DistributionError;
use std::env;
use std::path::{Component, Path, PathBuf};

/// Manages resolution and security validation for installation and version directories.
#[derive(Debug, Clone)]
pub struct DistributionPaths {
    base_dir: PathBuf,
}

impl DistributionPaths {
    /// Constructs paths relative to an explicitly specified base directory.
    #[must_use]
    pub fn with_base(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Resolves the default user-level installation paths for the current platform.
    #[must_use]
    pub fn user_default() -> Self {
        if cfg!(target_os = "windows") {
            let base = if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
                PathBuf::from(local_app_data).join("Programs").join("aek")
            } else if let Ok(user_profile) = env::var("USERPROFILE") {
                PathBuf::from(user_profile)
                    .join("AppData")
                    .join("Local")
                    .join("Programs")
                    .join("aek")
            } else {
                PathBuf::from("C:\\Program Files\\aek")
            };
            Self { base_dir: base }
        } else {
            // Linux / Unix / macOS
            let base = if let Ok(home) = env::var("HOME") {
                PathBuf::from(home).join(".local")
            } else {
                PathBuf::from("/usr/local")
            };
            Self { base_dir: base }
        }
    }

    /// Returns the base directory of the installation.
    #[must_use]
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    /// Returns the bin directory containing the executable.
    #[must_use]
    pub fn bin_dir(&self) -> PathBuf {
        self.base_dir.join("bin")
    }

    /// Returns the path to the executable binary.
    #[must_use]
    pub fn binary_path(&self) -> PathBuf {
        let exe_name = if cfg!(target_os = "windows") {
            "aek.exe"
        } else {
            "aek"
        };
        self.bin_dir().join(exe_name)
    }

    /// Returns the versions directory storing rollback binaries.
    #[must_use]
    pub fn versions_dir(&self) -> PathBuf {
        self.base_dir.join("versions")
    }

    /// Returns the path to the installation manifest file.
    #[must_use]
    pub fn manifest_path(&self) -> PathBuf {
        self.base_dir.join("installation.json")
    }

    /// Normalizes a path by evaluating `.` and `..` components lexically without touching the disk.
    #[must_use]
    pub fn normalize_lexically(path: &Path) -> PathBuf {
        let mut normalized = PathBuf::new();
        for component in path.components() {
            match component {
                Component::ParentDir => {
                    normalized.pop();
                }
                Component::CurDir => {}
                c => normalized.push(c),
            }
        }
        normalized
    }

    /// Validates that `target` is strictly contained within `base_dir`.
    /// Rejects any path traversal escaping the designated base.
    pub fn validate_containment(target: &Path, base: &Path) -> Result<PathBuf, DistributionError> {
        let norm_base = Self::normalize_lexically(base);
        let norm_target = Self::normalize_lexically(target);

        if !norm_target.starts_with(&norm_base) {
            return Err(DistributionError::PathTraversal {
                path: target.to_path_buf(),
                base: base.to_path_buf(),
            });
        }

        Ok(norm_target)
    }
}

