//! Context engineering pipeline: intent extraction, constraint preservation, and context assembly.

use serde::{Deserialize, Serialize};

/// Assembled and enriched prompt context ready for optimization or execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssembledPrompt {
    pub intent: String,
    pub original_prompt: String,
    pub enriched_prompt: String,
    pub constraints: Vec<String>,
    pub rules_applied: Vec<String>,
    pub skills_referenced: Vec<String>,
}

/// Context assembly pipeline combining user inputs, repository context, and engineering rules.
#[derive(Debug, Default, Clone)]
pub struct ContextAssembler;

impl ContextAssembler {
    /// Creates a new context assembler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Assembles a prompt with relevant context, constraints, and engineering guidelines.
    pub fn assemble(
        &self,
        prompt: &str,
        project_context: Option<&str>,
        rules: &[String],
        skills: &[String],
    ) -> AssembledPrompt {
        let trimmed = prompt.trim();
        let intent = trimmed.lines().next().unwrap_or(trimmed).to_string();

        let mut constraints = Vec::new();
        // Parse any bullet points containing constraints
        for line in trimmed.lines() {
            let l = line.trim();
            if (l.starts_with("- ") || l.starts_with("* "))
                && (l.contains("not")
                    || l.contains("must")
                    || l.contains("never")
                    || l.contains("only"))
            {
                constraints.push(
                    l.trim_start_matches("- ")
                        .trim_start_matches("* ")
                        .to_string(),
                );
            }
        }

        let mut enriched = String::new();
        enriched.push_str("# Engineering Task\n\n");
        enriched.push_str("## Primary Intent\n");
        enriched.push_str(&intent);
        enriched.push_str("\n\n");

        if let Some(ctx) = project_context {
            enriched.push_str("## Project Context\n");
            enriched.push_str(ctx.trim());
            enriched.push_str("\n\n");
        }

        if !rules.is_empty() {
            enriched.push_str("## Applicable Engineering Rules\n");
            for r in rules {
                enriched.push_str(&format!("- {r}\n"));
            }
            enriched.push('\n');
        }

        if !skills.is_empty() {
            enriched.push_str("## Active Domain Skills\n");
            for s in skills {
                enriched.push_str(&format!("- {s}\n"));
            }
            enriched.push('\n');
        }

        enriched.push_str("## Task Instructions\n");
        enriched.push_str(trimmed);
        enriched.push('\n');

        AssembledPrompt {
            intent,
            original_prompt: trimmed.to_string(),
            enriched_prompt: enriched,
            constraints,
            rules_applied: rules.to_vec(),
            skills_referenced: skills.to_vec(),
        }
    }
}
