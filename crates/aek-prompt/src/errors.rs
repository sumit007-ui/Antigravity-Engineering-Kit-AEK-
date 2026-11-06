//! Typed errors for the prompt optimization and evaluation engine.

use thiserror::Error;

/// Prompt engine errors.
#[derive(Error, Debug)]
pub enum PromptError {
    #[error("I/O error during prompt operations: {0}")]
    Io(#[from] std::io::Error),

    #[error("Prompt not found with ID: {0}")]
    NotFound(String),

    #[error("Version not found for prompt '{0}': {1}")]
    VersionNotFound(String, String),

    #[error("Template syntax error in '{0}': {1}")]
    TemplateSyntax(String, String),

    #[error("Missing required template variable: {0}")]
    MissingVariable(String),

    #[error("Dataset error: {0}")]
    Dataset(String),

    #[error("Evaluation error: {0}")]
    Evaluation(String),

    #[error("Optimization adapter error: {0}")]
    Adapter(String),

    #[error("Security violation: {0}")]
    Security(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
