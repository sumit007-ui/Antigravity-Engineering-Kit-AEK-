//! Safe skill installation, containment validation, and removal.

use crate::model::SkillBundle;
use aek_platform::paths::PlatformPaths;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors arising during skill installation.
#[derive(Error, Debug)]
pub enum SkillInstallError {
    #[error("I/O error during skill installation: {0}")]
    Io(#[from] std::io::Error),

    #[error("Security violation: path '{}' escapes authorized destination directory", .0.display())]
    PathTraversal(PathBuf),

    #[error("Skill '{0}' not found for removal")]
    NotFound(String),
}

/// Result of installing or removing a skill bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInstallResult {
    /// Skill name.
    pub name: String,
    /// Target directory where the skill was installed.
    pub target_dir: PathBuf,
    /// True if executed in dry-run mode.
    pub dry_run: bool,
}

/// Installs a skill bundle into the target directory (e.g. `.agents/skills/`).
pub fn install_skill(
    bundle: &SkillBundle,
    target_skills_dir: &Path,
    dry_run: bool,
) -> Result<SkillInstallResult, SkillInstallError> {
    let skill_dest_dir = target_skills_dir.join(&bundle.metadata.name);

    if !PlatformPaths::is_safe_subpath(target_skills_dir, &skill_dest_dir) {
        return Err(SkillInstallError::PathTraversal(skill_dest_dir));
    }

    if dry_run {
        return Ok(SkillInstallResult {
            name: bundle.metadata.name.clone(),
            target_dir: skill_dest_dir,
            dry_run: true,
        });
    }

    if !skill_dest_dir.exists() {
        fs::create_dir_all(&skill_dest_dir)?;
    }

    let skill_md_dest = skill_dest_dir.join("SKILL.md");

    // Atomically persist SKILL.md
    let mut temp = tempfile::Builder::new()
        .prefix(".aek-skill-tmp-")
        .suffix(".md")
        .tempfile_in(&skill_dest_dir)?;

    temp.write_all(bundle.body.as_bytes())?;
    temp.flush()?;
    temp.as_file().sync_all()?;
    temp.persist(&skill_md_dest).map_err(|e| {
        SkillInstallError::Io(std::io::Error::other(format!(
            "Failed to atomically persist SKILL.md: {e}"
        )))
    })?;

    Ok(SkillInstallResult {
        name: bundle.metadata.name.clone(),
        target_dir: skill_dest_dir,
        dry_run: false,
    })
}

/// Removes an installed skill directory.
pub fn remove_skill(
    skill_name: &str,
    target_skills_dir: &Path,
    dry_run: bool,
) -> Result<SkillInstallResult, SkillInstallError> {
    let skill_dest_dir = target_skills_dir.join(skill_name);

    if !PlatformPaths::is_safe_subpath(target_skills_dir, &skill_dest_dir) {
        return Err(SkillInstallError::PathTraversal(skill_dest_dir));
    }

    if !skill_dest_dir.exists() {
        return Err(SkillInstallError::NotFound(skill_name.to_string()));
    }

    if dry_run {
        return Ok(SkillInstallResult {
            name: skill_name.to_string(),
            target_dir: skill_dest_dir,
            dry_run: true,
        });
    }

    fs::remove_dir_all(&skill_dest_dir)?;

    Ok(SkillInstallResult {
        name: skill_name.to_string(),
        target_dir: skill_dest_dir,
        dry_run: false,
    })
}

