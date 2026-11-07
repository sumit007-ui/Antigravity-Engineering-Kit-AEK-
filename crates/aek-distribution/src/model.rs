//! Installation and distribution data models.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Scope of the AEK installation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallationScope {
    /// User-level installation (default: %LOCALAPPDATA%\Programs\aek on Windows, ~/.local on Linux).
    User,
    /// Custom isolated directory explicitly designated by the user.
    Custom(PathBuf),
}

/// Installation mechanism used to install AEK.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallationMethod {
    InstallerScript,
    DirectBinary,
    PackageManager,
    Cargo,
}

/// Platform specification identifying OS, architecture, and target triple.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformSpec {
    pub target_triple: String,
    pub os: String,
    pub arch: String,
}

impl PlatformSpec {
    /// Detects the current host platform specification.
    #[must_use]
    pub fn current() -> Self {
        let os = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            "unknown"
        };

        let arch = if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            "unknown"
        };

        let target_triple = if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
            "x86_64-pc-windows-msvc"
        } else if cfg!(all(target_os = "windows", target_arch = "aarch64")) {
            "aarch64-pc-windows-msvc"
        } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            "x86_64-unknown-linux-gnu"
        } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
            "aarch64-unknown-linux-gnu"
        } else {
            "unknown-target"
        };

        Self {
            target_triple: target_triple.to_string(),
            os: os.to_string(),
            arch: arch.to_string(),
        }
    }
}

/// Integrity verification status of an installed executable or artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Verified,
    Unverified,
    Mismatch,
    Unknown,
}

/// Details of an individual installed file tracked in the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstalledFile {
    pub path: PathBuf,
    pub sha256: String,
    pub size_bytes: u64,
}

/// Persistent installation manifest recording installed version, files, and state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallationManifest {
    pub version: String,
    pub scope: InstallationScope,
    pub install_dir: PathBuf,
    pub binary_path: PathBuf,
    #[serde(default, alias = "sha256")]
    pub binary_checksum: Option<String>,
    pub installed_at: String,
    pub method: InstallationMethod,
    pub platform: PlatformSpec,
    pub managed_files: Vec<PathBuf>,
    pub previous_version: Option<String>,
    pub verification_status: VerificationStatus,
}
