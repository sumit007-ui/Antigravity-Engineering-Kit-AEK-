use aek_agents::AgentAdapter;
use aek_agents::adapters::{AntigravityAdapter, ClaudeCodeAdapter, CursorAdapter, OpenCodeAdapter};
use aek_agents::capabilities::{
    AgentCapabilityCategory, AgentCapabilityRegistry, CapabilityStatus,
};
use aek_agents::conflicts::ConflictStrategy;
use aek_agents::model::{AgentId, AgentProfileKind};
use aek_agents::profiles::AgentProfileManager;
use aek_agents::registry::AgentRegistry;
use aek_agents::synchronization::MultiAgentSynchronizer;
use aek_agents::translator::AgentConfigTranslator;
use aek_mcp::config::{McpServerConfig, McpTransportConfig};
use aek_rules::model::{Rule, RuleScope};
use aek_skills::model::SkillBundle;
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_agent_registry_all_four_agents() {
    let mut registry = AgentRegistry::new();
    registry.register(Box::new(AntigravityAdapter::new()));
    registry.register(Box::new(ClaudeCodeAdapter::new()));
    registry.register(Box::new(OpenCodeAdapter::new()));
    registry.register(Box::new(CursorAdapter::new()));

    assert_eq!(registry.len(), 4);
    assert!(registry.get(AgentId::Antigravity).is_some());
    assert!(registry.get(AgentId::ClaudeCode).is_some());
    assert!(registry.get(AgentId::OpenCode).is_some());
    assert!(registry.get(AgentId::Cursor).is_some());

    let list = registry.list_all();
    assert_eq!(list.len(), 4);
}

#[test]
fn test_claude_code_adapter_and_translation() {
    let dir = tempdir().expect("tempdir");
    let workspace = dir.path();
    let adapter = ClaudeCodeAdapter::new();

    assert_eq!(adapter.name(), "Claude Code");
    assert_eq!(adapter.agent_id(), AgentId::ClaudeCode);

    let rules = vec![Rule::new(
        "tdd-mandate",
        "Mandatory TDD",
        "TDD",
        RuleScope::Workspace,
        100,
        "# Test Driven Development Invariants",
    )];

    let report = adapter
        .install_rules(&rules, workspace, false)
        .expect("install claude rules");
    assert_eq!(report.installed_count, 1);

    // Claude Code translates instructions into CLAUDE.md
    let claude_md = workspace.join("CLAUDE.md");
    assert!(claude_md.exists());
    let content = std::fs::read_to_string(&claude_md).expect("read CLAUDE.md");
    assert!(content.contains("Mandatory TDD"));
    assert!(content.contains("Test Driven Development Invariants"));

    // Claude Code MCP configuration
    let servers = vec![McpServerConfig {
        id: "test-server".to_string(),
        name: "Test Server".to_string(),
        description: "Test MCP server".to_string(),
        transport: McpTransportConfig::Stdio {
            command: "npx".to_string(),
            args: vec!["-y".to_string(), "test-server".to_string()],
            env: HashMap::from([("API_KEY".to_string(), "secret123".to_string())]),
            working_dir: None,
        },
        enabled: true,
        timeout_seconds: 30,
        requires_auth: false,
        auth_env_vars: vec![],
        required_dependencies: vec![],
        permissions: Default::default(),
        version: "1.0.0".to_string(),
    }];

    let mcp_report = adapter
        .install_mcp(&servers, Some(workspace), false)
        .expect("install mcp");
    assert_eq!(mcp_report.merged_count, 1);
    let claude_mcp = workspace.join(".claude").join("mcp.json");
    assert!(claude_mcp.exists());
    let mcp_json = std::fs::read_to_string(&claude_mcp).expect("read mcp.json");
    assert!(mcp_json.contains("test-server"));
}

#[test]
fn test_cursor_adapter_and_mdc_rules() {
    let dir = tempdir().expect("tempdir");
    let workspace = dir.path();
    let adapter = CursorAdapter::new();

    assert_eq!(adapter.name(), "Cursor");
    assert_eq!(adapter.agent_id(), AgentId::Cursor);

    let rules = vec![Rule::new(
        "precision-rule",
        "Ruthless Precision",
        "Precision",
        RuleScope::Workspace,
        90,
        "Never hallucinate or guess APIs.",
    )];

    let report = adapter
        .install_rules(&rules, workspace, false)
        .expect("install cursor rules");
    assert_eq!(report.installed_count, 1);

    // Cursor rules installed as .cursor/rules/*.mdc
    let mdc_rule = workspace
        .join(".cursor")
        .join("rules")
        .join("precision-rule.mdc");
    assert!(mdc_rule.exists());
    let mdc_content = std::fs::read_to_string(&mdc_rule).expect("read mdc");
    assert!(mdc_content.contains("---"));
    assert!(mdc_content.contains("description:"));
    assert!(mdc_content.contains("Never hallucinate or guess APIs."));

    // Legacy fallback .cursorrules also generated or preserved
    let legacy_rules = workspace.join(".cursorrules");
    assert!(legacy_rules.exists());
}

#[test]
fn test_opencode_adapter_and_translation() {
    let dir = tempdir().expect("tempdir");
    let workspace = dir.path();
    let adapter = OpenCodeAdapter::new();

    assert_eq!(adapter.name(), "OpenCode");
    assert_eq!(adapter.agent_id(), AgentId::OpenCode);

    let rules = vec![Rule::new(
        "security-rule",
        "Security Invariants",
        "Security",
        RuleScope::Workspace,
        95,
        "Strict path sanitization required.",
    )];

    let report = adapter
        .install_rules(&rules, workspace, false)
        .expect("install opencode rules");
    assert_eq!(report.installed_count, 1);

    let agents_md = workspace.join("AGENTS.md");
    assert!(agents_md.exists());
    let content = std::fs::read_to_string(&agents_md).expect("read AGENTS.md");
    assert!(content.contains("Security Invariants"));

    let config_file = workspace.join(".opencode").join("config.json");
    assert!(config_file.exists());
}

#[test]
fn test_agent_capability_matrix_and_unsupported_features() {
    let registry = AgentCapabilityRegistry::default_registry();

    // Check Antigravity MCP support
    let anti_mcp = registry.get_status(AgentId::Antigravity, AgentCapabilityCategory::Mcp);
    assert_eq!(anti_mcp, CapabilityStatus::Supported);

    // Check Claude Code hooks support with approval requirement
    let claude_hooks = registry.get_status(AgentId::ClaudeCode, AgentCapabilityCategory::Hooks);
    assert_eq!(claude_hooks, CapabilityStatus::Partial);

    // Check Cursor prompt templates support
    let cursor_templates =
        registry.get_status(AgentId::Cursor, AgentCapabilityCategory::PromptTemplates);
    assert_ne!(cursor_templates, CapabilityStatus::Unknown);
}

#[test]
fn test_multi_agent_synchronization_and_dry_run() {
    let dir = tempdir().expect("tempdir");
    let workspace = dir.path();

    let synchronizer = MultiAgentSynchronizer::new();
    let rules = vec![Rule::new(
        "universal-rule",
        "Universal Rule",
        "Rule",
        RuleScope::Workspace,
        100,
        "All agents must obey.",
    )];
    let skills = vec![SkillBundle::new_inline(
        "systems-engineer",
        "Systems engineer",
        "# Instructions",
    )];
    let servers = vec![];

    // Dry run: no files written
    let dry_plan = synchronizer
        .plan_sync(
            workspace,
            &[AgentId::ClaudeCode, AgentId::Cursor],
            &rules,
            &skills,
            &servers,
        )
        .expect("plan sync");
    assert!(!dry_plan.items.is_empty());
    assert!(!workspace.join("CLAUDE.md").exists());
    assert!(!workspace.join(".cursor").exists());

    // Actual sync
    let report = synchronizer
        .execute_sync(workspace, &dry_plan, ConflictStrategy::ReplaceManaged)
        .expect("execute sync");
    assert!(report.files_written > 0);
    assert!(workspace.join("CLAUDE.md").exists());
    assert!(workspace.join(".cursor").exists());

    // Ownership manifest tracked in .aek/managed-agents.toml
    let manifest_path = workspace.join(".aek").join("managed-agents.toml");
    assert!(manifest_path.exists());
}

#[test]
fn test_conflict_detection_and_strategies() {
    let dir = tempdir().expect("tempdir");
    let workspace = dir.path();
    let claude_md = workspace.join("CLAUDE.md");

    // Pre-existing user-authored file
    std::fs::write(&claude_md, "# Custom User Content Never Touch").expect("write file");

    let synchronizer = MultiAgentSynchronizer::new();
    let rules = vec![Rule::new(
        "rule1",
        "Rule 1",
        "R",
        RuleScope::Workspace,
        50,
        "New AEK Content",
    )];

    let plan = synchronizer
        .plan_sync(workspace, &[AgentId::ClaudeCode], &rules, &[], &[])
        .expect("plan sync");

    // 1. With KeepExisting strategy: user file preserved
    let report_keep = synchronizer
        .execute_sync(workspace, &plan, ConflictStrategy::KeepExisting)
        .expect("execute sync keep");
    assert_eq!(report_keep.conflicts_preserved, 1);
    let after_keep = std::fs::read_to_string(&claude_md).expect("read");
    assert_eq!(after_keep, "# Custom User Content Never Touch");

    // 2. With ReplaceManaged: user file updated with backup created
    let report_replace = synchronizer
        .execute_sync(workspace, &plan, ConflictStrategy::ReplaceManaged)
        .expect("execute sync replace");
    assert_eq!(report_replace.conflicts_resolved, 1);
    let after_replace = std::fs::read_to_string(&claude_md).expect("read");
    assert!(after_replace.contains("New AEK Content"));
    assert!(report_replace.backup_directory.is_some());
}

#[test]
fn test_agent_profiles_and_validation() {
    let manager = AgentProfileManager::new();
    let profiles = manager.list_profiles();
    assert!(profiles.len() >= 5);

    let anti_profile = manager
        .get_profile("antigravity-engineering")
        .expect("anti profile");
    assert_eq!(anti_profile.kind, AgentProfileKind::AntigravityEngineering);

    let multi_profile = manager
        .get_profile("multi-agent-engineering")
        .expect("multi profile");
    assert_eq!(multi_profile.target_agents.len(), 4);
}

#[test]
fn test_security_guards_path_traversal() {
    let translator = AgentConfigTranslator::new();
    let malicious_rule = Rule::new(
        "../../etc/cron.d/malicious",
        "Malicious",
        "Exploit",
        RuleScope::Workspace,
        100,
        "echo root",
    );

    let result = translator.sanitize_relative_path(&malicious_rule.id);
    assert!(result.is_err());
}
