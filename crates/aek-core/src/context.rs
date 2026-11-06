//! AEK execution context and runtime policies.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Execution mode for AEK commands and operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMode {
    /// Invariant: ruthless precision. Maximum verification, strict checks, no hallucinations.
    #[default]
    RuthlessPrecision,
    /// Standard execution mode.
    Standard,
}

impl ExecutionMode {
    /// Returns true if running in ruthless precision mode.
    #[must_use]
    pub const fn is_ruthless(&self) -> bool {
        matches!(self, Self::RuthlessPrecision)
    }
}

/// Global context passed across AEK subsystems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AekContext {
    /// Active workspace root directory.
    pub workspace_root: PathBuf,
    /// Execution mode policy.
    pub execution_mode: ExecutionMode,
    /// Verbose diagnostic output flag.
    pub verbose: bool,
    /// Detected or configured Antigravity home directory (`~/.gemini`).
    pub antigravity_home: PathBuf,
}

impl AekContext {
    /// Creates a new `AekContext` with default environment-detected paths.
    #[must_use]
    pub fn new(workspace_root: impl AsRef<Path>) -> Self {
        let antigravity_home = directories::BaseDirs::new()
            .map(|dirs| dirs.home_dir().join(".gemini"))
            .unwrap_or_else(|| PathBuf::from(".gemini"));

        Self {
            workspace_root: workspace_root.as_ref().to_path_buf(),
            execution_mode: ExecutionMode::RuthlessPrecision,
            verbose: false,
            antigravity_home,
        }
    }

    /// Sets the execution mode.
    #[must_use]
    pub fn with_execution_mode(mut self, mode: ExecutionMode) -> Self {
        self.execution_mode = mode;
        self
    }

    /// Sets verbosity flag.
    #[must_use]
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_mode() {
        let default_mode = ExecutionMode::default();
        assert_eq!(default_mode, ExecutionMode::RuthlessPrecision);
        assert!(default_mode.is_ruthless());

        let standard = ExecutionMode::Standard;
        assert!(!standard.is_ruthless());
    }

    #[test]
    fn test_aek_context_builder() {
        let ctx = AekContext::new("my-workspace")
            .with_execution_mode(ExecutionMode::Standard)
            .with_verbose(true);

        assert_eq!(ctx.workspace_root, PathBuf::from("my-workspace"));
        assert_eq!(ctx.execution_mode, ExecutionMode::Standard);
        assert!(ctx.verbose);
        assert!(!ctx.antigravity_home.as_os_str().is_empty());
    }
}
