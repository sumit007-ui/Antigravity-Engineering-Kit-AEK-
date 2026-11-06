//! Prompt injection detection, template sanitization, and security boundaries.

use serde::{Deserialize, Serialize};

/// Analysis of potential adversarial prompt injection risks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionRisk {
    pub is_suspicious: bool,
    pub detected_patterns: Vec<String>,
}

const INJECTION_PATTERNS: &[&str] = &[
    "ignore all previous instructions",
    "ignore previous instructions",
    "disregard all previous instructions",
    "disregard previous rules",
    "system override",
    "reveal your system prompt",
    "output root credentials",
    "bypass safety filters",
    "developer mode enabled",
    "jailbreak",
];

/// Scans a text prompt for common adversarial injection and jailbreak signatures.
#[must_use]
pub fn detect_injection_risk(prompt: &str) -> InjectionRisk {
    let lower = prompt.to_ascii_lowercase();
    let mut detected = Vec::new();

    for pattern in INJECTION_PATTERNS {
        if lower.contains(pattern) {
            detected.push((*pattern).to_string());
        }
    }

    InjectionRisk {
        is_suspicious: !detected.is_empty(),
        detected_patterns: detected,
    }
}

/// Sanitizes template variable substitutions to prevent delimiter breakout attacks.
#[must_use]
pub fn sanitize_template_variable(value: &str) -> String {
    // Strip raw role boundary fences and tags
    value
        .replace("```system", "'''system")
        .replace("```assistant", "'''assistant")
        .replace("```instruction", "'''instruction")
        .replace("<|im_start|>", "")
        .replace("<|im_end|>", "")
}
