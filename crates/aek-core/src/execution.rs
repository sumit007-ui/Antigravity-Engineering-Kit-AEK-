//! Subprocess execution engine with argument array safety, secret redaction, and timeouts.

pub use crate::error::ExecutionError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// Result of executing a system command.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandResult {
    /// Exit code of the process if terminated normally.
    pub status_code: Option<i32>,
    /// Standard output parsed as UTF-8 string.
    pub stdout: String,
    /// Standard error parsed as UTF-8 string.
    pub stderr: String,
    /// True if the process exited with code 0.
    pub success: bool,
}

/// Subprocess execution builder that guarantees argument array passing (no shell interpolation).
#[derive(Debug, Clone)]
pub struct SafeCommand {
    program: String,
    args: Vec<String>,
    current_dir: Option<PathBuf>,
    envs: HashMap<String, String>,
    env_clear: bool,
    timeout: Option<Duration>,
    dry_run: bool,
}

impl SafeCommand {
    /// Creates a new `SafeCommand` for the specified executable binary.
    #[must_use]
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            current_dir: None,
            envs: HashMap::new(),
            env_clear: false,
            timeout: Some(Duration::from_secs(60)),
            dry_run: false,
        }
    }

    /// Appends a single argument.
    #[must_use]
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Appends multiple arguments.
    #[must_use]
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    /// Sets the working directory for the process.
    #[must_use]
    pub fn current_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.current_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Adds an environment variable.
    #[must_use]
    pub fn env(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.envs.insert(key.into(), val.into());
        self
    }

    /// Adds multiple environment variables.
    #[must_use]
    pub fn envs<I, K, V>(mut self, envs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (k, v) in envs {
            self.envs.insert(k.into(), v.into());
        }
        self
    }

    /// Clears inherited environment variables, isolating the process from host secrets.
    #[must_use]
    pub fn env_clear(mut self) -> Self {
        self.env_clear = true;
        self
    }

    /// Configures execution timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Configures dry-run mode. If true, prints intended execution and returns mock success.
    #[must_use]
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Executes the command asynchronously with timeout enforcement and child cleanup.
    ///
    /// # Errors
    /// Returns `ExecutionError::Timeout` if execution exceeds duration,
    /// or `ExecutionError::SpawnFailed` if binary could not be invoked.
    pub async fn execute(&self) -> Result<CommandResult, ExecutionError> {
        if self.dry_run {
            let msg = format!(
                "[DRY-RUN] Would execute: {} {}",
                self.program,
                self.args.join(" ")
            );
            return Ok(CommandResult {
                status_code: Some(0),
                stdout: msg,
                stderr: String::new(),
                success: true,
            });
        }

        let mut cmd = Command::new(&self.program);
        cmd.args(&self.args);
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        cmd.kill_on_drop(true);

        if self.env_clear {
            cmd.env_clear();
            // Preserve essential system variables for execution
            for (k, v) in std::env::vars() {
                let upper = k.to_ascii_uppercase();
                if matches!(
                    upper.as_str(),
                    "PATH"
                        | "PATHEXT"
                        | "SYSTEMROOT"
                        | "WINDIR"
                        | "COMSPEC"
                        | "TEMP"
                        | "TMP"
                        | "HOME"
                        | "USER"
                        | "USERPROFILE"
                        | "SHELL"
                        | "TERM"
                        | "LANG"
                ) {
                    cmd.env(k, v);
                }
            }
        }

        if let Some(ref dir) = self.current_dir {
            cmd.current_dir(dir);
        }

        for (k, v) in &self.envs {
            cmd.env(k, v);
        }

        let timeout_duration = self.timeout.unwrap_or(Duration::from_secs(60));

        let child = cmd.spawn().map_err(|e| ExecutionError::SpawnFailed {
            command: self.program.clone(),
            source: e,
        })?;

        match timeout(timeout_duration, child.wait_with_output()).await {
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
            Ok(Err(e)) => Err(ExecutionError::SpawnFailed {
                command: self.program.clone(),
                source: e,
            }),
            Err(_) => Err(ExecutionError::Timeout {
                command: format!("{} {}", self.program, self.args.join(" ")),
                timeout: timeout_duration,
            }),
        }
    }
}

/// Sanitizes environment variable values that may contain sensitive credentials.
#[must_use]
pub fn redact_secrets(envs: &[(String, String)]) -> HashMap<String, String> {
    let sensitive_keys = [
        "KEY",
        "SECRET",
        "TOKEN",
        "PASSWORD",
        "PASSWD",
        "AUTH",
        "CREDENTIAL",
        "BEARER",
        "PRIVATE",
    ];

    let mut sanitized = HashMap::new();
    for (k, v) in envs {
        let is_sensitive = sensitive_keys
            .iter()
            .any(|needle| k.to_uppercase().contains(needle));
        if is_sensitive {
            sanitized.insert(k.clone(), "********".to_string());
        } else {
            sanitized.insert(k.clone(), v.clone());
        }
    }
    sanitized
}
