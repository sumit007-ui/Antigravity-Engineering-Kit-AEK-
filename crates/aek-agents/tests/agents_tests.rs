use aek_agents::adapter::AgentAdapter;
use aek_agents::antigravity::AntigravityAdapter;
use aek_rules::model::{Rule, RuleScope};
use aek_skills::model::SkillBundle;
use tempfile::tempdir;

#[test]
fn test_antigravity_adapter_capabilities() {
    let adapter = AntigravityAdapter::new();
    assert_eq!(adapter.name(), "Google Antigravity");

    let caps = adapter.capabilities();
    assert!(caps.supports_global_rules);
    assert!(caps.supports_workspace_rules);
    assert!(caps.supports_skills);
    assert!(caps.supports_mcp);
    assert_eq!(caps.max_rule_bytes, 24_000);
    assert_eq!(caps.rule_token_budget, 20_000);
}

#[test]
fn test_antigravity_install_rules_and_skills() {
    let dir = tempdir().expect("tempdir");
    let workspace = dir.path();
    let adapter = AntigravityAdapter::new();

    let rules = vec![Rule::new(
        "engineering-standards",
        "Engineering Standards",
        "Standards",
        RuleScope::Workspace,
        100,
        "# Engineering Standards Content",
    )];

    let target_rules_dir = adapter.workspace_rules_dir(workspace);

    // 1. Dry run rules install
    let dry_rules = adapter
        .install_rules(&rules, &target_rules_dir, true)
        .expect("dry run rules");
    assert_eq!(dry_rules.installed_count, 1);
    assert!(!target_rules_dir.join("engineering-standards.md").exists());

    // 2. Actual rules install
    let act_rules = adapter
        .install_rules(&rules, &target_rules_dir, false)
        .expect("actual rules");
    assert_eq!(act_rules.installed_count, 1);
    assert!(target_rules_dir.join("engineering-standards.md").exists());

    // 3. Skills install
    let skills = vec![SkillBundle::new_inline(
        "rust-systems-engineer",
        "Expert Rust systems engineer",
        "# Rust Systems Engineer Instructions",
    )];
    let target_skills_dir = adapter.workspace_skills_dir(workspace);

    let act_skills = adapter
        .install_skills(&skills, &target_skills_dir, false)
        .expect("actual skills");
    assert_eq!(act_skills.installed_count, 1);
    assert!(
        target_skills_dir
            .join("rust-systems-engineer")
            .join("SKILL.md")
            .exists()
    );
}
