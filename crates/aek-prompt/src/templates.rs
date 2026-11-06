//! Template variable extraction, validation, and safe rendering.

use crate::errors::PromptError;
use crate::security::sanitize_template_variable;
use std::collections::{HashMap, HashSet};

/// A prompt containing parameter placeholders in the form `{{variable}}`.
#[derive(Debug, Clone)]
pub struct PromptTemplate {
    raw: String,
    variables: Vec<String>,
}

impl PromptTemplate {
    /// Parses a prompt template string and extracts variable placeholders.
    pub fn parse(raw: &str) -> Result<Self, PromptError> {
        let mut variables = Vec::new();
        let mut seen = HashSet::new();

        let mut pos = 0;
        while let Some(start) = raw[pos..].find("{{") {
            let actual_start = pos + start;
            if let Some(end) = raw[actual_start + 2..].find("}}") {
                let actual_end = actual_start + 2 + end;
                let var_name = raw[actual_start + 2..actual_end].trim();

                if var_name.is_empty() {
                    return Err(PromptError::TemplateSyntax(
                        raw.to_string(),
                        "Empty variable placeholder '{{}}'".to_string(),
                    ));
                }

                if seen.insert(var_name.to_string()) {
                    variables.push(var_name.to_string());
                }

                pos = actual_end + 2;
            } else {
                return Err(PromptError::TemplateSyntax(
                    raw.to_string(),
                    "Unclosed placeholder '{{'".to_string(),
                ));
            }
        }

        Ok(Self {
            raw: raw.to_string(),
            variables,
        })
    }

    /// Returns the unique variable names required by this template.
    #[must_use]
    pub fn variables(&self) -> Vec<&str> {
        self.variables.iter().map(|s| s.as_str()).collect()
    }

    /// Renders the template using the provided variable values with security sanitization.
    pub fn render(&self, values: &HashMap<String, String>) -> Result<String, PromptError> {
        let mut output = self.raw.clone();

        for var in &self.variables {
            let val = values.get(var).ok_or_else(|| {
                PromptError::MissingVariable(format!(
                    "Required template variable '{}' not provided",
                    var
                ))
            })?;

            let sanitized = sanitize_template_variable(val);
            let pattern = format!("{{{{{var}}}}}");
            output = output.replace(&pattern, &sanitized);
        }

        Ok(output)
    }

    /// Returns the raw template string.
    #[must_use]
    pub fn raw(&self) -> &str {
        &self.raw
    }
}
