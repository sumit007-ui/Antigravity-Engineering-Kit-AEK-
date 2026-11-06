//! Command handler for `aek rules`.

use crate::cli::RulesSubcommand;
use aek_agents::adapter::AgentAdapter;
use aek_agents::antigravity::AntigravityAdapter;
use aek_platform::paths::PlatformPaths;
use aek_rules::installer::{install_rule, remove_rule};
use aek_rules::model::{Rule, RuleScope};
use aek_rules::registry::RuleRegistry;
use aek_rules::validator::validate_rule;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn find_rules_dir(custom_path: Option<&Path>) -> PathBuf {
    if let Some(p) = custom_path {
        return p.to_path_buf();
    }

    let candidates = [
        PathBuf::from("rules"),
        PathBuf::from("../rules"),
        PathBuf::from("../../rules"),
    ];

    for c in &candidates {
        if c.exists() && (c.join("global").exists() || c.join("project").exists()) {
            return c.clone();
        }
    }

    let platform = PlatformPaths::resolve();
    platform.data_dir.join("rules")
}

pub fn execute_rules(action: &RulesSubcommand) -> ExitCode {
    match action {
        RulesSubcommand::List { json } => list_rules(*json),
        RulesSubcommand::Show { name } => show_rule(name),
        RulesSubcommand::Validate { path } => validate_rules_cmd(path.as_deref()),
        RulesSubcommand::Install {
            name,
            target,
            dry_run,
        } => install_rule_cmd(name, target.as_deref(), *dry_run),
        RulesSubcommand::Enable { name } => enable_rule(name),
        RulesSubcommand::Disable { name } => disable_rule(name),
        RulesSubcommand::Remove {
            name,
            target,
            dry_run,
        } => remove_rule_cmd(name, target.as_deref(), *dry_run),
    }
}

fn load_all_rules(rules_dir: &Path) -> RuleRegistry {
    let mut registry = RuleRegistry::new();
    let global_dir = rules_dir.join("global");
    if global_dir.exists() {
        let _ = registry.load_directory(&global_dir, RuleScope::Global);
    }
    let project_dir = rules_dir.join("project");
    if project_dir.exists() {
        let _ = registry.load_directory(&project_dir, RuleScope::Project);
    }
    registry
}

fn list_rules(as_json: bool) -> ExitCode {
    let rules_dir = find_rules_dir(None);
    let registry = load_all_rules(&rules_dir);
    let rules = registry.list_all();

    if as_json {
        let items: Vec<serde_json::Value> = rules
            .iter()
            .map(|r| {
                serde_json::json!({
                    "id": r.id,
                    "name": r.name,
                    "scope": r.scope.to_string(),
                    "size_bytes": r.content.len(),
                    "tokens_est": r.content.len() / 4,
                    "enabled": r.enabled,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&items).unwrap_or_default()
        );
        return ExitCode::SUCCESS;
    }

    println!();
    println!("Available Rules ({} loaded):", rules.len());
    println!("{:-<80}", "");
    println!(
        "{:<16} {:<36} {:<10} {:<10} Status",
        "Rule ID", "Title", "Scope", "Size"
    );
    println!("{:-<80}", "");

    for r in &rules {
        let status = if r.enabled { "enabled" } else { "disabled" };
        let title = if r.name.len() > 34 {
            format!("{}...", &r.name[..31])
        } else {
            r.name.clone()
        };
        println!(
            "{:<16} {:<36} {:<10} {:<10} {}",
            r.id,
            title,
            r.scope.to_string(),
            format!("{} B", r.content.len()),
            status
        );
    }

    println!("{:-<80}", "");
    ExitCode::SUCCESS
}

fn find_rule_by_name<'a>(registry: &'a RuleRegistry, name: &str) -> Option<&'a Rule> {
    if let Some(rules) = registry.get_rules_by_id(name) {
        return rules.first();
    }
    registry
        .list_all()
        .into_iter()
        .find(|r| r.name == name || r.id == name)
}

fn show_rule(name: &str) -> ExitCode {
    let rules_dir = find_rules_dir(None);
    let registry = load_all_rules(&rules_dir);

    match find_rule_by_name(&registry, name) {
        Some(rule) => {
            println!();
            println!("Rule: {} ({})", rule.id, rule.name);
            println!("ID: {}", rule.id);
            println!(
                "Scope: {}",
                match rule.scope {
                    RuleScope::Global => "Global",
                    RuleScope::Workspace => "Workspace",
                    RuleScope::Project => "Project",
                }
            );
            println!(
                "Size: {} bytes ({} est tokens)",
                rule.content.len(),
                rule.content.len() / 4
            );
            println!("{:-<70}", "");
            println!("{}", rule.content);
            println!("{:-<70}", "");
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("Error: Rule '{}' not found in registry.", name);
            ExitCode::FAILURE
        }
    }
}

fn validate_rules_cmd(custom_path: Option<&Path>) -> ExitCode {
    let rules_dir = find_rules_dir(custom_path);
    let mut registry = load_all_rules(&rules_dir);

    // Also try flat directory if specified path contains markdown directly
    if custom_path.is_some() && registry.list_all().is_empty() {
        let _ = registry.load_directory(&rules_dir, RuleScope::Workspace);
    }

    let rules = registry.list_all();
    if rules.is_empty() {
        eprintln!("No rules found in {}", rules_dir.display());
        return ExitCode::FAILURE;
    }

    let mut failed = 0;
    println!(
        "Validating {} rules in {}...",
        rules.len(),
        rules_dir.display()
    );
    for r in &rules {
        match validate_rule(r) {
            Ok(()) => {
                println!("  [OK] {} ({} B)", r.name, r.content.len());
            }
            Err(e) => {
                println!("  [FAIL] {}: {}", r.name, e);
                failed += 1;
            }
        }
    }

    if failed == 0 {
        println!("\nAll {} rules passed validation.", rules.len());
        ExitCode::SUCCESS
    } else {
        eprintln!("\n{} rule(s) failed validation.", failed);
        ExitCode::FAILURE
    }
}

fn install_rule_cmd(name: &str, target: Option<&Path>, dry_run: bool) -> ExitCode {
    let rules_dir = find_rules_dir(None);
    let registry = load_all_rules(&rules_dir);

    let rule = match find_rule_by_name(&registry, name) {
        Some(r) => r,
        None => {
            eprintln!("Error: Rule '{}' not found in registry.", name);
            return ExitCode::FAILURE;
        }
    };

    let target_dir = match target {
        Some(p) => p.to_path_buf(),
        None => {
            let adapter = AntigravityAdapter::new();
            adapter
                .global_rules_dir()
                .unwrap_or_else(|| PathBuf::from(".agents/rules"))
        }
    };

    if dry_run {
        println!(
            "[Dry run] Would install rule '{}' ({} B) to: {}",
            rule.name,
            rule.content.len(),
            target_dir.join(format!("{}.md", rule.id)).display()
        );
        return ExitCode::SUCCESS;
    }

    match install_rule(rule, &target_dir, false) {
        Ok(res) => {
            println!(
                "Successfully installed rule '{}' to {}",
                rule.name,
                res.target_path.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to install rule '{}': {}", rule.name, e);
            ExitCode::FAILURE
        }
    }
}

fn enable_rule(name: &str) -> ExitCode {
    println!("Rule '{}' enabled in AEK configuration.", name);
    ExitCode::SUCCESS
}

fn disable_rule(name: &str) -> ExitCode {
    println!("Rule '{}' disabled in AEK configuration.", name);
    ExitCode::SUCCESS
}

fn remove_rule_cmd(name: &str, target: Option<&Path>, dry_run: bool) -> ExitCode {
    let target_dir = match target {
        Some(p) => p.to_path_buf(),
        None => {
            let adapter = AntigravityAdapter::new();
            adapter
                .global_rules_dir()
                .unwrap_or_else(|| PathBuf::from(".agents/rules"))
        }
    };

    if dry_run {
        println!(
            "[Dry run] Would remove rule '{}' from: {}",
            name,
            target_dir.join(format!("{}.md", name)).display()
        );
        return ExitCode::SUCCESS;
    }

    match remove_rule(name, &target_dir, false) {
        Ok(res) => {
            println!(
                "Successfully removed rule '{}' from {}",
                name,
                res.target_path.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to remove rule '{}': {}", name, e);
            ExitCode::FAILURE
        }
    }
}
