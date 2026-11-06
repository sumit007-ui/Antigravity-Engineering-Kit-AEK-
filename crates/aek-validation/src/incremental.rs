//! Git-aware incremental change detection and affected project resolution.

use crate::errors::ValidationError;
use aek_languages::model::DetectedProject;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Detector tracking modified files via Git and mapping changes to projects.
#[derive(Debug, Default, Clone)]
pub struct GitChangeDetector;

impl GitChangeDetector {
    /// Creates a new `GitChangeDetector`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Checks if a directory is inside a Git working tree.
    #[must_use]
    pub fn is_git_repo(&self, path: &Path) -> bool {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .current_dir(path)
            .output();

        matches!(output, Ok(out) if out.status.success() && String::from_utf8_lossy(&out.stdout).trim() == "true")
    }

    /// Detects all uncommitted changed or untracked files relative to HEAD.
    pub fn detect_changed_files(&self, path: &Path) -> Result<Vec<PathBuf>, ValidationError> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(path)
            .output()
            .map_err(|e| ValidationError::Command(format!("Failed to invoke git: {e}")))?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut changed = Vec::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.len() > 3 {
                let file_part = &trimmed[3..].trim();
                let clean_file = file_part.trim_matches('"');
                changed.push(PathBuf::from(clean_file));
            }
        }

        Ok(changed)
    }

    /// Detects changed files relative to a specific Git base reference (e.g. `main` or `origin/main`).
    pub fn detect_changed_files_with_base(
        &self,
        path: &Path,
        base_ref: &str,
    ) -> Result<Vec<PathBuf>, ValidationError> {
        let output = Command::new("git")
            .args(["diff", "--name-only", base_ref])
            .current_dir(path)
            .output()
            .map_err(|e| ValidationError::Command(format!("Failed to invoke git diff: {e}")))?;

        if !output.status.success() {
            // Fall back to uncommitted changes if base ref is unavailable
            return self.detect_changed_files(path);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut changed = Vec::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                changed.push(PathBuf::from(trimmed));
            }
        }

        // Also merge in uncommitted changes
        if let Ok(uncommitted) = self.detect_changed_files(path) {
            for f in uncommitted {
                if !changed.contains(&f) {
                    changed.push(f);
                }
            }
        }

        Ok(changed)
    }

    /// Filters detected projects down to those containing or affected by changed files.
    pub fn filter_affected_projects(
        &self,
        projects: &[DetectedProject],
        changed_files: &[PathBuf],
    ) -> Vec<DetectedProject> {
        if changed_files.is_empty() {
            return Vec::new();
        }

        // Check if root build manifest or global files changed
        let root_files = ["Cargo.toml", "Cargo.lock", "go.work", "package.json"];
        let root_changed = changed_files.iter().any(|f| {
            let s = f.to_string_lossy();
            root_files.iter().any(|rf| s == *rf)
        });

        if root_changed {
            // Conservative expansion: root manifest changed, validate all projects
            return projects.to_vec();
        }

        let mut affected = Vec::new();
        for project in projects {
            let project_path_str = project.path.to_string_lossy().replace('\\', "/");
            let is_affected = changed_files.iter().any(|f| {
                let f_str = f.to_string_lossy().replace('\\', "/");
                if project_path_str == "." {
                    !f_str.contains('/') || f_str.starts_with("src/")
                } else {
                    f_str.starts_with(&project_path_str)
                }
            });

            if is_affected {
                affected.push(project.clone());
            }
        }

        affected
    }
}
