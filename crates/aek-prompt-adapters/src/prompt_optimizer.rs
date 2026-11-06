//! Prompt Optimizer adapter implementing the structured prompt engineering framework.

use aek_prompt::errors::PromptError;
use aek_prompt::model::PromptRecord;

/// Adapter implementing the structured prompt engineering framework based on linshenkx/prompt-optimizer.
#[derive(Debug, Default, Clone)]
pub struct PromptOptimizerAdapter;

impl PromptOptimizerAdapter {
    /// Creates a new `PromptOptimizerAdapter`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Transforms an unoptimized prompt into a structured, production-grade instruction.
    pub fn optimize(&self, prompt: &PromptRecord) -> Result<String, PromptError> {
        let content = prompt.active_content().trim();

        // Extract primary domain / role title
        let role = if content.contains("Rust") {
            "Principal Rust Systems Engineer"
        } else if content.contains("Go") {
            "Principal Go Backend Architect"
        } else if content.contains("C++") || content.contains("Cpp") {
            "Principal C++ Systems Engineer"
        } else if content.contains("Zig") {
            "Principal Zig Systems Programmer"
        } else if content.contains("Python") {
            "Principal Python Infrastructure Architect"
        } else if content.contains("Flutter") {
            "Principal Flutter & Mobile Architect"
        } else if content.contains("ROS") || content.contains("robotics") {
            "Principal Robotics & ROS 2 Systems Engineer"
        } else {
            "Principal Software Engineer & Technical Architect"
        };

        let mut output = String::new();
        output.push_str(&format!("# Role: {role}\n\n"));

        output.push_str("## Profile\n");
        output.push_str("- Focus: Production-grade engineering, deterministic behavior, and ruthless precision\n");
        output
            .push_str("- Style: Highly concise, technically rigorous, zero placeholder stubs\n\n");

        output.push_str("## Objective\n");
        output.push_str(content.lines().next().unwrap_or(content));
        output.push_str("\n\n");

        output.push_str("## Context & Inputs\n");
        output.push_str(content);
        output.push_str("\n\n");

        output.push_str("## Non-Negotiable Constraints & Rules\n");
        output.push_str(
            "1. Accuracy: Never hallucinate APIs, configurations, or hardware capabilities.\n",
        );
        output.push_str("2. Security: Strictly sanitize inputs, prevent injection attacks, and avoid shell interpolation.\n");
        output.push_str("3. Completeness: Implement complete, functional code without TODO placeholders or omissions.\n");
        output.push_str("4. Quality: Enforce Test-Driven Development (TDD) and verify all execution exit codes.\n\n");

        output.push_str("## Step-by-Step Workflow\n");
        output.push_str("1. Deconstruct task intent, inputs, and boundaries.\n");
        output.push_str("2. Formulate step-by-step technical solution adhering to strict memory and security invariants.\n");
        output.push_str("3. Verify code or instructions against constraints.\n");
        output.push_str("4. Deliver verified, production-ready output.\n\n");

        output.push_str("## Output Schema\n");
        output.push_str(
            "- Fenced code blocks with explicit language identifiers for all source code.\n",
        );
        output.push_str("- GitHub-flavored Markdown for technical documentation.\n");

        Ok(output)
    }
}
