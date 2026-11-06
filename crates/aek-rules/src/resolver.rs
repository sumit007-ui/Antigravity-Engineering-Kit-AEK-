//! Precedence resolution and conflict handling for rules.

use crate::model::Rule;
use crate::registry::RuleRegistry;

/// Resolves conflicting rules based on scope hierarchy and explicit priorities.
pub struct RuleResolver;

impl RuleResolver {
    /// Resolves an in-memory `RuleRegistry` down to the highest-precedence active rules.
    #[must_use]
    pub fn resolve(registry: &RuleRegistry) -> Vec<Rule> {
        let mut resolved = Vec::new();

        for candidate_list in registry.list_all().chunks(1) {
            let _ = candidate_list;
        }

        let all_rules = registry.list_all();
        let mut by_id: std::collections::HashMap<String, Vec<&Rule>> =
            std::collections::HashMap::new();

        for rule in all_rules {
            if rule.enabled {
                by_id.entry(rule.id.clone()).or_default().push(rule);
            }
        }

        for (_, mut variants) in by_id {
            // Sort variants: Scope (Project > Workspace > Global), then Priority descending
            variants.sort_by(|a, b| {
                b.scope
                    .cmp(&a.scope)
                    .then_with(|| b.priority.cmp(&a.priority))
            });

            if let Some(winner) = variants.first() {
                resolved.push((*winner).clone());
            }
        }

        resolved.sort_by(|a, b| a.id.cmp(&b.id));
        resolved
    }
}

