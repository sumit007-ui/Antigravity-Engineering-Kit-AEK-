use aek_rules::model::{Rule, RuleScope};
use aek_rules::registry::RuleRegistry;
use aek_rules::resolver::RuleResolver;
use aek_rules::validator::validate_rule;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_rule_creation_and_validation() {
    let rule = Rule::new(
        "test-rule",
        "Test Rule",
        "Description of test rule",
        RuleScope::Global,
        100,
        "# Test Rule Content",
    );

    assert_eq!(rule.id, "test-rule");
    assert_eq!(rule.scope, RuleScope::Global);
    assert_eq!(rule.priority, 100);
    assert!(rule.enabled);
    assert!(validate_rule(&rule).is_ok());

    // Invalid ID (contains uppercase and spaces)
    let mut invalid_rule = rule.clone();
    invalid_rule.id = "Invalid Rule ID".to_string();
    assert!(validate_rule(&invalid_rule).is_err());

    // Exceeding 24 KB limit
    let huge_content = "a".repeat(25000);
    let mut oversized_rule = rule.clone();
    oversized_rule.content = huge_content;
    assert!(validate_rule(&oversized_rule).is_err());
}

#[test]
fn test_rule_registry_and_precedence_resolution() {
    let mut registry = RuleRegistry::new();

    // Global rule
    registry.add_rule(Rule::new(
        "security-policy",
        "Global Security",
        "Default security",
        RuleScope::Global,
        10,
        "# Global Security Invariants",
    ));

    // Project rule with same or higher priority
    registry.add_rule(Rule::new(
        "security-policy",
        "Project Security",
        "Custom security",
        RuleScope::Project,
        20,
        "# Project Custom Security Invariants",
    ));

    // Non-conflicting rule
    registry.add_rule(Rule::new(
        "testing-policy",
        "Testing Policy",
        "TDD rules",
        RuleScope::Workspace,
        15,
        "# TDD Testing Rules",
    ));

    let resolved = RuleResolver::resolve(&registry);
    assert_eq!(resolved.len(), 2);

    // Project scope must win over Global scope for the same rule id
    let sec_rule = resolved.iter().find(|r| r.id == "security-policy").unwrap();
    assert_eq!(sec_rule.scope, RuleScope::Project);
    assert!(sec_rule.content.contains("Project Custom Security"));
}

#[test]
fn test_rule_installer_and_dry_run() {
    let dir = tempdir().expect("tempdir");
    let target_dir = dir.path().join(".agents").join("rules");

    let rule = Rule::new(
        "rust-guidelines",
        "Rust Guidelines",
        "Rust standards",
        RuleScope::Workspace,
        50,
        "# Rust Coding Standards",
    );

    // Dry-run install
    let dry_res =
        aek_rules::installer::install_rule(&rule, &target_dir, true).expect("dry run install");
    assert!(dry_res.dry_run);
    assert!(!target_dir.join("rust-guidelines.md").exists());

    // Actual install
    let actual_res =
        aek_rules::installer::install_rule(&rule, &target_dir, false).expect("actual install");
    assert!(!actual_res.dry_run);
    let installed_file = target_dir.join("rust-guidelines.md");
    assert!(installed_file.exists());
    let content = fs::read_to_string(&installed_file).expect("read");
    assert!(content.contains("# Rust Coding Standards"));

    // Uninstall
    aek_rules::installer::remove_rule("rust-guidelines", &target_dir, false).expect("remove rule");
    assert!(!installed_file.exists());
}
