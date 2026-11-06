//! Operating system and architecture detection, plus binary discovery.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Identified operating system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OsKind {
    Windows,
    Linux,
    MacOS,
    Unknown(String),
}

impl std::fmt::Display for OsKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Windows => write!(f, "windows"),
            Self::Linux => write!(f, "linux"),
            Self::MacOS => write!(f, "macos"),
            Self::Unknown(s) => write!(f, "unknown ({s})"),
        }
    }
}

/// Detects the running operating system.
#[must_use]
pub fn detect_os() -> OsKind {
    if cfg!(target_os = "windows") {
        OsKind::Windows
    } else if cfg!(target_os = "linux") {
        OsKind::Linux
    } else if cfg!(target_os = "macos") {
        OsKind::MacOS
    } else {
        OsKind::Unknown(std::env::consts::OS.to_string())
    }
}

/// Detects the target CPU architecture.
#[must_use]
pub fn detect_arch() -> String {
    std::env::consts::ARCH.to_string()
}

/// If running on Linux, attempts to extract distribution details (e.g. "Ubuntu").
#[must_use]
pub fn detect_linux_distro() -> Option<String> {
    if !cfg!(target_os = "linux") {
        return None;
    }

    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(name) = line.strip_prefix("PRETTY_NAME=") {
                return Some(name.trim_matches('"').to_string());
            }
        }
    }
    None
}

/// Searches the system PATH for the given binary name.
/// Handles platform differences (e.g., automatically probing `.exe` on Windows).
#[must_use]
pub fn find_executable_in_path(binary: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    let paths = std::env::split_paths(&path_var);

    let candidates: Vec<String> = if cfg!(windows) {
        if binary.ends_with(".exe") || binary.ends_with(".cmd") || binary.ends_with(".bat") {
            vec![binary.to_string()]
        } else {
            vec![
                format!("{binary}.exe"),
                format!("{binary}.cmd"),
                format!("{binary}.bat"),
                binary.to_string(),
            ]
        }
    } else {
        vec![binary.to_string()]
    };

    for dir in paths {
        for candidate in &candidates {
            let candidate_path = dir.join(candidate);
            if candidate_path.is_file() {
                return Some(candidate_path);
            }
        }
    }

    None
}
