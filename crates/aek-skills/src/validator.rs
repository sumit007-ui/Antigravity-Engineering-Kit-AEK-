//! Frontmatter parsing and skill validation against specification.

use crate::model::SkillMetadata;
use thiserror::Error;

/// Errors arising during skill parsing and validation.
#[derive(Error, Debug)]
pub enum SkillValidationError {
    #[error("Missing YAML frontmatter delimiters ('---')")]
    MissingFrontmatter,

    #[error("Failed to parse YAML frontmatter: {0}")]
    YamlParse(String),

    #[error("Skill name '{0}' is invalid. Must be lowercase alphanumeric with hyphens.")]
    InvalidName(String),

    #[error("Skill description is too short (minimum 10 characters required)")]
    DescriptionTooShort,
}

/// Parses the YAML frontmatter block from a `SKILL.md` string.
pub fn parse_skill_markdown(content: &str) -> Result<SkillMetadata, SkillValidationError> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Err(SkillValidationError::MissingFrontmatter);
    }

    let after_first = &trimmed[3..];
    let end_index = after_first
        .find("\n---")
        .ok_or(SkillValidationError::MissingFrontmatter)?;

    let yaml_str = &after_first[..end_index].trim();

    // Simple robust line-based YAML extractor for `name` and `description` to avoid heavy YAML engine
    let mut name = None;
    let mut description_lines = Vec::new();
    let mut in_description = false;

    for line in yaml_str.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("name:") {
            in_description = false;
            let val = trimmed_line.trim_start_matches("name:").trim();
            name = Some(val.trim_matches('"').trim_matches('\'').to_string());
        } else if trimmed_line.starts_with("description:") {
            in_description = true;
            let val = trimmed_line.trim_start_matches("description:").trim();
            if !val.is_empty() && val != ">-" && val != "|" {
                description_lines.push(val.trim_matches('"').trim_matches('\'').to_string());
            }
        } else if in_description {
            if trimmed_line.starts_with("version:")
                || trimmed_line.starts_with("tags:")
                || trimmed_line.starts_with("triggers:")
            {
                in_description = false;
            } else {
                description_lines.push(trimmed_line.to_string());
            }
        }
    }

    let skill_name = name.ok_or_else(|| {
        SkillValidationError::YamlParse("Missing 'name' field in frontmatter".to_string())
    })?;

    let description = description_lines.join(" ").trim().to_string();
    if description.len() < 10 {
        return Err(SkillValidationError::DescriptionTooShort);
    }

    // Validate name format: lowercase alphanumeric with hyphens
    if !skill_name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(SkillValidationError::InvalidName(skill_name));
    }

    Ok(SkillMetadata {
        name: skill_name,
        description,
        version: "1.0.0".to_string(),
        tags: Vec::new(),
        triggers: Vec::new(),
    })
}
