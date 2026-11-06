//! Safe subprocess execution and system diagnostic utilities.

use crate::error::AekCoreError;
use std::process::{Command, Output};

/// Result of executing a system command.
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Exit code of the process.
    pub status_code: Option<i32>,
    /// Standard output parsed as UTF-8 string.
    pub stdout: String,
    /// Standard error parsed as UTF-8 string.
    pub stderr: String,
    /// True if the process exited with code 0.
    pub success: bool,
}

/// Executes a binary safely without shell wrapping to eliminate command injection risk.
///
/// # Errors
/// Returns `AekCoreError::ProcessExecutionFailed` if spawning or executing fails.
pub fn execute_safe(cmd: &str, args: &[&str]) -> Result<CommandResult, AekCoreError> {
    let output: Output = Command::new(cmd).args(args).output().map_err(|e| {
        AekCoreError::ProcessExecutionFailed(format!("Failed to execute '{cmd}': {e}"))
    })?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    Ok(CommandResult {
        status_code: output.status.code(),
        stdout,
        stderr,
        success: output.status.success(),
    })
}

/// System platform information.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlatformInfo {
    /// Operating system family (windows, linux, macos).
    pub os: String,
    /// CPU target architecture (x86_64, aarch64, etc.).
    pub arch: String,
    /// Available logical cores.
    pub logical_cores: usize,
}

impl PlatformInfo {
    /// Collects platform information from the current host.
    #[must_use]
    pub fn current() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            logical_cores: std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_info_current() {
        let plat = PlatformInfo::current();
        assert!(!plat.os.is_empty());
        assert!(!plat.arch.is_empty());
        assert!(plat.logical_cores >= 1);
    }

    #[test]
    fn test_execute_safe_success() {
        let res = execute_safe("cargo", &["--version"]).expect("cargo must execute");
        assert!(res.success);
        assert_eq!(res.status_code, Some(0));
        assert!(res.stdout.contains("cargo"));
    }

    #[test]
    fn test_execute_safe_nonexistent() {
        let res = execute_safe("nonexistent_binary_xyz_123456", &[]);
        assert!(res.is_err());
    }
}
