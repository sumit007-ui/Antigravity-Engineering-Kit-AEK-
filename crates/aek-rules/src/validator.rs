//! Rule validation and budget enforcement.

use crate::model::Rule;
use thiserror::Error;

/// Maximum allowable size for a single rule file under Antigravity limits (24,000 bytes).
pub const MAX_RULE_BYTES: usize = 24_000;

/// Errors arising from rule validation.
#[derive(Error, Debug)]
pub enum RuleValidationError {
    #[error("Rule ID '{0}' is invalid. Must be lowercase alphanumeric with hyphens.")]
    InvalidId(String),

    #[error("Rule name cannot be empty")]
    EmptyName,

    #[error("Rule '{id}' content size ({bytes} bytes) exceeds the 24,000 byte limit")]
    OversizedContent { id: String, bytes: usize },
}

/// Validates that a rule meets identification and size invariants.
pub fn validate_rule(rule: &Rule) -> Result<(), RuleValidationError> {
    let id = rule.id.trim();
    if id.is_empty()
        || !id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(RuleValidationError::InvalidId(rule.id.clone()));
    }

    if rule.name.trim().is_empty() {
        return Err(RuleValidationError::EmptyName);
    }

    let byte_len = rule.content.len();
    if byte_len > MAX_RULE_BYTES {
        return Err(RuleValidationError::OversizedContent {
            id: rule.id.clone(),
            bytes: byte_len,
        });
    }

    Ok(())
}
