//! Safe rule installation, containment validation, and removal.

use crate::model::Rule;
use crate::validator::validate_rule;
use aek_platform::paths::PlatformPaths;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors arising during rule installation.
#[derive(Error, Debug)]
pub enum RuleInstallError {
    #[error("I/O error during rule installation: {0}")]
    Io(#[from] std::io::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Security violation: path '{}' escapes destination directory", .0.display())]
    PathTraversal(PathBuf),

    #[error("Rule file '{0}' not found for removal")]
    NotFound(PathBuf),
}

/// Result of an install or uninstall operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleInstallResult {
    /// Rule identifier.
    pub id: String,
    /// Destination path on disk.
    pub target_path: PathBuf,
    /// Whether operation executed in dry-run mode.
    pub dry_run: bool,
}

/// Installs a validated rule to the destination directory.
pub fn install_rule(
    rule: &Rule,
    target_dir: &Path,
    dry_run: bool,
) -> Result<RuleInstallResult, RuleInstallError> {
    validate_rule(rule).map_err(|e| RuleInstallError::Validation(e.to_string()))?;

    let destination_file = target_dir.join(format!("{}.md", rule.id));

    if !PlatformPaths::is_safe_subpath(target_dir, &destination_file) {
        return Err(RuleInstallError::PathTraversal(destination_file));
    }

    if dry_run {
        return Ok(RuleInstallResult {
            id: rule.id.clone(),
            target_path: destination_file,
            dry_run: true,
        });
    }

    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
    }

    let mut temp = tempfile::Builder::new()
        .prefix(".aek-rule-tmp-")
        .suffix(".md")
        .tempfile_in(target_dir)?;

    temp.write_all(rule.content.as_bytes())?;
    temp.flush()?;
    temp.as_file().sync_all()?;
    temp.persist(&destination_file).map_err(|e| {
        RuleInstallError::Io(std::io::Error::other(format!(
            "Failed to atomically persist rule: {e}"
        )))
    })?;

    Ok(RuleInstallResult {
        id: rule.id.clone(),
        target_path: destination_file,
        dry_run: false,
    })
}

/// Removes an installed rule file from the target directory.
pub fn remove_rule(
    rule_id: &str,
    target_dir: &Path,
    dry_run: bool,
) -> Result<RuleInstallResult, RuleInstallError> {
    let target_file = target_dir.join(format!("{rule_id}.md"));

    if !PlatformPaths::is_safe_subpath(target_dir, &target_file) {
        return Err(RuleInstallError::PathTraversal(target_file));
    }

    if !target_file.exists() {
        return Err(RuleInstallError::NotFound(target_file));
    }

    if dry_run {
        return Ok(RuleInstallResult {
            id: rule_id.to_string(),
            target_path: target_file,
            dry_run: true,
        });
    }

    fs::remove_file(&target_file)?;

    Ok(RuleInstallResult {
        id: rule_id.to_string(),
        target_path: target_file,
        dry_run: false,
    })
}
