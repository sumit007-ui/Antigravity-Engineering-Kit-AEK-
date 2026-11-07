//! Safe subprocess execution and system diagnostic utilities.

use crate::error::AekCoreError;
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

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

/// Executes a binary safely with an explicit bounded timeout.
///
/// # Errors
/// Returns `AekCoreError::ProcessExecutionFailed` if spawning fails or the timeout is exceeded.
pub fn execute_safe_with_timeout(
    cmd: &str,
    args: &[&str],
    timeout: Duration,
) -> Result<CommandResult, AekCoreError> {
    let mut command = Command::new(cmd);
    command.args(args);

    let (tx, rx) = channel();
    thread::spawn(move || {
        let res = command.output();
        let _ = tx.send(res);
    });

    match rx.recv_timeout(timeout) {
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Ok(CommandResult {
                status_code: output.status.code(),
                stdout,
                stderr,
                success: output.status.success(),
            })
        }
        Ok(Err(e)) => Err(AekCoreError::ProcessExecutionFailed(format!(
            "Failed to execute '{cmd}': {e}"
        ))),
        Err(_) => Err(AekCoreError::ProcessExecutionFailed(format!(
            "Process '{cmd}' timed out after {} seconds",
            timeout.as_secs()
        ))),
    }
}

/// Executes a binary safely without shell wrapping with a default 30-second timeout.
///
/// # Errors
/// Returns `AekCoreError::ProcessExecutionFailed` if spawning or executing fails.
pub fn execute_safe(cmd: &str, args: &[&str]) -> Result<CommandResult, AekCoreError> {
    execute_safe_with_timeout(cmd, args, Duration::from_secs(30))
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
