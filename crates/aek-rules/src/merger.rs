//! Merging resolved rules into consolidated agent instruction documents.

use crate::model::Rule;

/// Maximum aggregate byte limit (approx 20,000 token budget) across inlined rules.
pub const MAX_AGGREGATE_BYTES: usize = 80_000;

/// Consolidates active rules into a unified Markdown document.
pub struct RuleMerger;

impl RuleMerger {
    /// Combines rules into a single string formatted with clear boundaries.
    #[must_use]
    pub fn merge(rules: &[Rule]) -> String {
        let mut document = String::new();
        document.push_str("# AEK Consolidated Engineering Invariants & Rules\n\n");

        let mut current_bytes = document.len();

        for rule in rules {
            let section_header = format!(
                "\n<!-- RULE: {} (scope: {}, priority: {}) -->\n",
                rule.id, rule.scope, rule.priority
            );
            let section_content = format!("{}\n\n", rule.content.trim());
            let additional_bytes = section_header.len() + section_content.len();

            if current_bytes + additional_bytes > MAX_AGGREGATE_BYTES {
                document.push_str(&format!(
                    "\n> [!NOTE] Rule '{}' was demoted to an on-demand reference to conserve the 20,000-token context budget.\n",
                    rule.id
                ));
                continue;
            }

            document.push_str(&section_header);
            document.push_str(&section_content);
            current_bytes += additional_bytes;
        }

        document
    }
}
