//! Command handler for `aek agent`.

use crate::cli::AgentSubcommand;
use crate::commands::rules::find_rules_dir;
use crate::commands::skills::find_skills_dir;
use aek_agents::adapter::AgentAdapter;
use aek_agents::antigravity::AntigravityAdapter;
use aek_rules::model::RuleScope;
use aek_rules::registry::RuleRegistry;
use aek_skills::registry::SkillRegistry;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn execute_agent(action: &AgentSubcommand) -> ExitCode {
    match action {
        AgentSubcommand::Detect => detect_agent(),
        AgentSubcommand::Capabilities => show_capabilities(),
        AgentSubcommand::InstallRules { dry_run } => install_rules(*dry_run),
        AgentSubcommand::InstallSkills { dry_run } => install_skills(*dry_run),
        AgentSubcommand::Doctor => agent_doctor(),
    }
}

fn detect_agent() -> ExitCode {
    let adapter = AntigravityAdapter::new();
    let detection = adapter.detect();

    println!();
    println!("Detected Agent: {}", detection.agent_name);
    println!("{:-<60}", "");
    println!(
        "Active / Installed: {}",
        if detection.is_detected {
            "YES"
        } else {
            "NO (Will initialize upon installation)"
        }
    );
    if !detection.evidence.is_empty() {
        println!("Evidence:");
        for ev in &detection.evidence {
            println!("  • {}", ev);
        }
    }
    if let Some(rules_dir) = adapter.global_rules_dir() {
        println!("Global Rules Directory:  {}", rules_dir.display());
    }
    if let Some(skills_dir) = adapter.global_skills_dir() {
        println!("Global Skills Directory: {}", skills_dir.display());
    }
    println!("{:-<60}", "");

    ExitCode::SUCCESS
}

fn show_capabilities() -> ExitCode {
    let adapter = AntigravityAdapter::new();
    let caps = adapter.capabilities();

    println!();
    println!("Agent Capabilities: {}", adapter.name());
    println!("{:-<65}", "");
    println!(
        "Supports Global Rules:    {}",
        if caps.supports_global_rules {
            "YES"
        } else {
            "NO"
        }
    );
    println!(
        "Supports Workspace Rules: {}",
        if caps.supports_workspace_rules {
            "YES"
        } else {
            "NO"
        }
    );
    println!(
        "Supports Skills:          {}",
        if caps.supports_skills {
            "YES (Progressive disclosure via SKILL.md)"
        } else {
            "NO"
        }
    );
    println!(
        "Supports MCP:             {}",
        if caps.supports_mcp {
            "YES (Model Context Protocol)"
        } else {
            "NO"
        }
    );
    println!(
        "Max Rule Size (per file): {} bytes ({} KB)",
        caps.max_rule_bytes,
        caps.max_rule_bytes / 1024
    );
    println!(
        "Max Rule Token Budget:    {} tokens",
        caps.rule_token_budget
    );
    println!("{:-<65}", "");

    ExitCode::SUCCESS
}

fn install_rules(dry_run: bool) -> ExitCode {
    let adapter = AntigravityAdapter::new();
    let target_dir = match adapter.global_rules_dir() {
        Some(d) => d,
        None => PathBuf::from(".agents/rules"),
    };

    let rules_dir = find_rules_dir(None);
    let mut registry = RuleRegistry::new();

    let _ = registry.load_directory(&rules_dir.join("global"), RuleScope::Global);
    let _ = registry.load_directory(&rules_dir.join("project"), RuleScope::Project);

    let rules_refs = registry.list_all();
    if rules_refs.is_empty() {
        eprintln!("No rules found to install from {}", rules_dir.display());
        return ExitCode::FAILURE;
    }

    let rules: Vec<_> = rules_refs.into_iter().cloned().collect();

    if dry_run {
        println!();
        println!(
            "[Dry run] Would install {} rules into {}:",
            rules.len(),
            target_dir.display()
        );
        for r in &rules {
            println!("  • {} ({} B)", r.name, r.content.len());
        }
        return ExitCode::SUCCESS;
    }

    match adapter.install_rules(&rules, &target_dir, false) {
        Ok(report) => {
            println!();
            println!(
                "Successfully installed {} rules to {}.",
                report.installed_count,
                report.target_dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to install rules into agent: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn install_skills(dry_run: bool) -> ExitCode {
    let adapter = AntigravityAdapter::new();
    let target_dir = match adapter.global_skills_dir() {
        Some(d) => d,
        None => PathBuf::from(".agents/skills"),
    };

    let skills_dir = find_skills_dir(None);
    let mut registry = SkillRegistry::new();
    let _ = registry.load_directory(&skills_dir);

    let skills_refs = registry.list_skills();
    if skills_refs.is_empty() {
        eprintln!("No skills found to install from {}", skills_dir.display());
        return ExitCode::FAILURE;
    }

    let skills: Vec<_> = skills_refs.into_iter().cloned().collect();

    if dry_run {
        println!();
        println!(
            "[Dry run] Would install {} skills into {}:",
            skills.len(),
            target_dir.display()
        );
        for s in &skills {
            println!("  • {}", s.metadata.name);
        }
        return ExitCode::SUCCESS;
    }

    match adapter.install_skills(&skills, &target_dir, false) {
        Ok(report) => {
            println!();
            println!(
                "Successfully installed {} skills to {}.",
                report.installed_count,
                report.target_dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to install skills into agent: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn agent_doctor() -> ExitCode {
    let adapter = AntigravityAdapter::new();
    println!();
    println!("Antigravity Agent Diagnostics & Health Check");
    println!("{:-<65}", "");

    let detection = adapter.detect();
    println!("Agent Platform:         {}", adapter.name());
    println!(
        "Detection Status:       {}",
        if detection.is_detected {
            "PASS"
        } else {
            "INITIALIZING"
        }
    );

    if let Some(rules_dir) = adapter.global_rules_dir() {
        let exists = rules_dir.exists();
        println!(
            "Rules Directory:        {} [{}]",
            rules_dir.display(),
            if exists { "PASS" } else { "NOT CREATED" }
        );
    }

    if let Some(skills_dir) = adapter.global_skills_dir() {
        let exists = skills_dir.exists();
        println!(
            "Skills Directory:       {} [{}]",
            skills_dir.display(),
            if exists { "PASS" } else { "NOT CREATED" }
        );
    }

    println!("{:-<65}", "");
    println!("Agent Status: READY");
    ExitCode::SUCCESS
}
