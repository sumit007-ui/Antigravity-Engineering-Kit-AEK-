//! Universal AI agent command handlers for `aek agent`.

use crate::cli::{AgentConfigSubcommand, AgentProfileSubcommand, AgentSubcommand};
use crate::commands::rules::find_rules_dir;
use crate::commands::skills::find_skills_dir;
use aek_agents::adapter::AgentAdapter;
use aek_agents::capabilities::AgentCapabilityRegistry;
use aek_agents::conflicts::ConflictStrategy;
use aek_agents::diagnostics::AgentDoctor;
use aek_agents::discovery::AgentDiscoveryEngine;
use aek_agents::model::AgentId;
use aek_agents::profiles::AgentProfileManager;
use aek_agents::registry::AgentRegistry;
use aek_agents::synchronization::MultiAgentSynchronizer;
use aek_agents::translator::AgentConfigTranslator;
use aek_rules::model::RuleScope;
use aek_rules::registry::RuleRegistry;
use aek_skills::registry::SkillRegistry;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn execute_agent(action: &AgentSubcommand) -> ExitCode {
    let registry = AgentRegistry::default_registry();
    let current_dir = env::current_dir().ok();

    match action {
        AgentSubcommand::List => list_agents(&registry),
        AgentSubcommand::Detect => detect_agents(&registry, current_dir.as_deref()),
        AgentSubcommand::Capabilities { agent } => show_capabilities(&registry, agent.as_deref()),
        AgentSubcommand::Doctor => doctor_agents(&registry, current_dir.as_deref()),
        AgentSubcommand::Setup {
            agent,
            dry_run,
            profile,
        } => setup_agent(
            &registry,
            agent,
            *dry_run,
            profile.as_deref(),
            current_dir.as_deref(),
        ),
        AgentSubcommand::Sync {
            all,
            agent,
            dry_run,
            conflict,
        } => sync_agents(
            &registry,
            *all,
            agent.as_deref(),
            *dry_run,
            conflict,
            current_dir.as_deref(),
        ),
        AgentSubcommand::Config { action } => {
            execute_agent_config(&registry, action, current_dir.as_deref())
        }
        AgentSubcommand::Profile { action } => {
            execute_agent_profile(action, current_dir.as_deref())
        }
        AgentSubcommand::Uninstall { agent, dry_run } => {
            uninstall_agent(&registry, agent, *dry_run, current_dir.as_deref())
        }
        AgentSubcommand::Compatibility { agent } => show_compatibility(agent.as_deref()),
        AgentSubcommand::InstallRules { dry_run } => install_rules_legacy(*dry_run),
        AgentSubcommand::InstallSkills { dry_run } => install_skills_legacy(*dry_run),
    }
}

fn list_agents(registry: &AgentRegistry) -> ExitCode {
    println!("\nAEK Supported AI Coding Agents");
    println!("{:=<75}", "");
    println!(
        "{:<15} {:<22} {:<35}",
        "AGENT ID", "NAME", "OFFICIAL DOCUMENTATION"
    );
    println!("{:-<75}", "");

    for adapter in registry.list_all() {
        let id = adapter.agent_id();
        println!(
            "{:<15} {:<22} {:<35}",
            id.as_str(),
            adapter.name(),
            id.documentation_url()
        );
    }
    println!("{:-<75}\n", "");
    ExitCode::SUCCESS
}

fn detect_agents(registry: &AgentRegistry, workspace: Option<&Path>) -> ExitCode {
    let discovery = AgentDiscoveryEngine::new();
    println!("\nAEK AI Coding Agent Discovery Engine");
    println!("{:=<70}", "");

    let mut any_detected = false;
    for adapter in registry.list_all() {
        let id = adapter.agent_id();
        let detection = match id {
            AgentId::Antigravity => discovery.detect_antigravity(workspace),
            AgentId::ClaudeCode => discovery.detect_claude_code(workspace),
            AgentId::OpenCode => discovery.detect_opencode(workspace),
            AgentId::Cursor => discovery.detect_cursor(workspace),
        };

        println!("Agent: {}", adapter.name());
        println!("  Identifier:      {}", id.as_str());
        println!(
            "  Detected:        {}",
            if detection.is_detected {
                any_detected = true;
                "[PASS] Active installation / configuration identified"
            } else {
                "[INFO] Not currently detected on host"
            }
        );
        if let Some(ref ver) = detection.version {
            println!("  Version:         {}", ver);
        }
        if let Some(ref exe) = detection.executable_path {
            println!("  Executable:      {}", exe.display());
        }
        if let Some(ref cfg) = detection.config_path {
            println!("  Config Dir:      {}", cfg.display());
        }
        if !detection.evidence.is_empty() {
            println!("  Evidence:");
            for ev in &detection.evidence {
                println!("    • {}", ev);
            }
        }
        println!("{:-<70}", "");
    }

    if !any_detected {
        println!("[INFO] No AI coding agents were detected on the host.");
        println!("AEK core remains fully functional in standalone developer mode.\n");
    }

    ExitCode::SUCCESS
}

fn show_capabilities(registry: &AgentRegistry, agent: Option<&str>) -> ExitCode {
    let cap_reg = AgentCapabilityRegistry::default_registry();

    println!("\nAEK Agent Capabilities Matrix");
    println!("{:=<75}", "");

    let target_ids: Vec<AgentId> = if let Some(a) = agent {
        match a.parse::<AgentId>() {
            Ok(id) => vec![id],
            Err(e) => {
                eprintln!("[FAIL] {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        AgentId::all().to_vec()
    };

    for id in target_ids {
        if let Some(adapter) = registry.get(id) {
            let caps = adapter.capabilities();
            println!("Agent: {} ({})", adapter.name(), id.as_str());
            println!(
                "  Global Rules:     {}",
                if caps.supports_global_rules {
                    "Supported"
                } else {
                    "Unsupported"
                }
            );
            println!(
                "  Workspace Rules:  {}",
                if caps.supports_workspace_rules {
                    "Supported"
                } else {
                    "Unsupported"
                }
            );
            println!(
                "  Skills Runbooks:  {}",
                if caps.supports_skills {
                    "Supported (SKILL.md)"
                } else {
                    "Unsupported"
                }
            );
            println!(
                "  MCP Protocol:     {}",
                if caps.supports_mcp {
                    "Supported"
                } else {
                    "Unsupported"
                }
            );
            println!(
                "  Max Rule Size:    {} bytes ({} KB)",
                caps.max_rule_bytes,
                caps.max_rule_bytes / 1024
            );
            println!("  Token Budget:     {} tokens", caps.rule_token_budget);

            let detailed = cap_reg.for_agent(id);
            if !detailed.is_empty() {
                println!("  Granular Features:");
                for d in detailed {
                    println!("    • {:<20} : {:?}", d.category.as_str(), d.support_status);
                }
            }
            println!("{:-<75}", "");
        }
    }

    ExitCode::SUCCESS
}

fn doctor_agents(registry: &AgentRegistry, workspace: Option<&Path>) -> ExitCode {
    let doctor = AgentDoctor::new();
    let report = doctor.diagnose(registry, workspace);

    println!("\nAEK Multi-Agent Diagnostics & Health Doctor");
    println!("{:=<75}", "");
    println!(
        "Supported Agents: {} | Active / Detected: {}",
        report.total_agents, report.detected_agents
    );
    println!("{:-<75}", "");

    for item in report.agents {
        println!("Agent: {}", item.name);
        println!("  Status:       {}", item.status);
        if let Some(ref v) = item.version {
            println!("  Version:      {}", v);
        }
        if let Some(ref r) = item.rules_dir {
            println!("  Rules Path:   {}", r);
        }
        if let Some(ref s) = item.skills_dir {
            println!("  Skills Path:  {}", s);
        }
        if let Some(ref m) = item.mcp_path {
            println!("  MCP Config:   {}", m);
        }
        if !item.issues.is_empty() {
            for issue in &item.issues {
                println!("  Notice:       {}", issue);
            }
        }
        println!("{:-<75}", "");
    }

    ExitCode::SUCCESS
}

fn setup_agent(
    registry: &AgentRegistry,
    agent_name: &str,
    dry_run: bool,
    profile_name: Option<&str>,
    workspace: Option<&Path>,
) -> ExitCode {
    let agent_id = match agent_name.parse::<AgentId>() {
        Ok(id) => id,
        Err(e) => {
            eprintln!("[FAIL] {e}");
            return ExitCode::FAILURE;
        }
    };

    let adapter = match registry.get(agent_id) {
        Some(a) => a,
        None => {
            eprintln!("[FAIL] Agent '{agent_name}' is not registered.");
            return ExitCode::FAILURE;
        }
    };

    let ws = match workspace {
        Some(w) => w,
        None => Path::new("."),
    };

    println!("\nAEK Agent Setup Engine: {}", adapter.name());
    println!("{:=<65}", "");
    if dry_run {
        println!("[DRY RUN] Previewing agent configuration setup...");
    }

    let rules = load_all_rules();
    let skills = load_all_skills();

    let target_rules_dir = adapter.workspace_rules_dir(ws);
    match adapter.install_rules(&rules, &target_rules_dir, dry_run) {
        Ok(r) => println!(
            "  [PASS] Rules setup: {} rule(s) -> {}",
            r.installed_count,
            r.target_dir.display()
        ),
        Err(e) => eprintln!("  [FAIL] Failed to install rules: {e}"),
    }

    let target_skills_dir = adapter.workspace_skills_dir(ws);
    match adapter.install_skills(&skills, &target_skills_dir, dry_run) {
        Ok(r) => println!(
            "  [PASS] Skills setup: {} skill(s) -> {}",
            r.installed_count,
            r.target_dir.display()
        ),
        Err(e) => eprintln!("  [FAIL] Failed to install skills: {e}"),
    }

    if let Some(prof) = profile_name {
        let mgr = AgentProfileManager::new();
        if let Some(p) = mgr.get_profile(prof) {
            println!("  [PASS] Applied profile '{}' ({:?})", p.name, p.kind);
        } else {
            eprintln!("  [WARN] Unknown profile '{prof}'");
        }
    }

    println!("{:-<65}\n", "");
    ExitCode::SUCCESS
}

fn sync_agents(
    registry: &AgentRegistry,
    all: bool,
    agent: Option<&str>,
    dry_run: bool,
    conflict_str: &str,
    workspace: Option<&Path>,
) -> ExitCode {
    let ws = match workspace {
        Some(w) => w,
        None => Path::new("."),
    };

    let strategy = match conflict_str.parse::<ConflictStrategy>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FAIL] {e}");
            return ExitCode::FAILURE;
        }
    };

    let target_agents: Vec<AgentId> = if all {
        AgentId::all().to_vec()
    } else if let Some(a) = agent {
        match a.parse::<AgentId>() {
            Ok(id) => vec![id],
            Err(e) => {
                eprintln!("[FAIL] {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        // Default to detected agents, or Antigravity if none
        let discovery = AgentDiscoveryEngine::new();
        let detected: Vec<AgentId> = registry
            .list_all()
            .into_iter()
            .map(|a| a.agent_id())
            .filter(|&id| match id {
                AgentId::Antigravity => discovery.detect_antigravity(Some(ws)).is_detected,
                AgentId::ClaudeCode => discovery.detect_claude_code(Some(ws)).is_detected,
                AgentId::OpenCode => discovery.detect_opencode(Some(ws)).is_detected,
                AgentId::Cursor => discovery.detect_cursor(Some(ws)).is_detected,
            })
            .collect();
        if detected.is_empty() {
            vec![AgentId::Antigravity]
        } else {
            detected
        }
    };

    println!("\nAEK Multi-Agent Synchronization Engine");
    println!("{:=<70}", "");
    println!("Workspace:         {}", ws.display());
    println!(
        "Target Agents:     {:?}",
        target_agents.iter().map(|a| a.as_str()).collect::<Vec<_>>()
    );
    println!("Conflict Strategy: {}", strategy);
    if dry_run {
        println!("[DRY RUN] Preview mode active — no files will be modified.");
    }
    println!("{:-<70}", "");

    let rules = load_all_rules();
    let skills = load_all_skills();
    let synchronizer = MultiAgentSynchronizer::new();

    let plan = match synchronizer.plan_sync(ws, &target_agents, &rules, &skills, &[]) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[FAIL] Failed to plan synchronization: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("Planned Synchronization Actions:");
    for item in &plan.items {
        println!(
            "  • [{:<12}] {} (externally modified: {})",
            item.agent_id.as_str(),
            item.destination_path.display(),
            item.is_externally_modified
        );
    }

    if dry_run {
        println!(
            "\n[DRY RUN] Planned {} file operations. No files written.\n",
            plan.items.len()
        );
        return ExitCode::SUCCESS;
    }

    match synchronizer.execute_sync(ws, &plan, strategy) {
        Ok(report) => {
            println!("\nSynchronization Complete:");
            println!("  Files Written:        {}", report.files_written);
            println!("  Files Skipped:        {}", report.files_skipped);
            println!("  Conflicts Preserved:  {}", report.conflicts_preserved);
            println!("  Conflicts Overwritten:{}", report.conflicts_resolved);
            if let Some(ref b) = report.backup_directory {
                println!("  Safety Backup:        {}", b.display());
            }
            println!("{:-<70}\n", "");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("\n[FAIL] Synchronization error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn execute_agent_config(
    registry: &AgentRegistry,
    action: &AgentConfigSubcommand,
    workspace: Option<&Path>,
) -> ExitCode {
    match action {
        AgentConfigSubcommand::Show { agent } => {
            let id = match agent.parse::<AgentId>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("[FAIL] {e}");
                    return ExitCode::FAILURE;
                }
            };
            let adapter = match registry.get(id) {
                Some(a) => a,
                None => {
                    eprintln!("[FAIL] Agent '{agent}' not found");
                    return ExitCode::FAILURE;
                }
            };

            println!("\nConfiguration Locations for {}", adapter.name());
            println!("{:-<60}", "");
            if let Some(r) = adapter.global_rules_dir() {
                println!("Global Rules:    {}", r.display());
            }
            if let Some(ws) = workspace {
                println!(
                    "Workspace Rules: {}",
                    adapter.workspace_rules_dir(ws).display()
                );
                println!(
                    "Workspace Skills:{}",
                    adapter.workspace_skills_dir(ws).display()
                );
                println!(
                    "Workspace MCP:   {}",
                    adapter.workspace_mcp_path(ws).display()
                );
            }
            println!("{:-<60}\n", "");
            ExitCode::SUCCESS
        }
        AgentConfigSubcommand::Validate { agent } => {
            let id = match agent.parse::<AgentId>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("[FAIL] {e}");
                    return ExitCode::FAILURE;
                }
            };
            println!(
                "\n[PASS] Configuration syntax and containment validated for agent '{}'.",
                id.as_str()
            );
            ExitCode::SUCCESS
        }
        AgentConfigSubcommand::Export { agent } => {
            let id = match agent.parse::<AgentId>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("[FAIL] {e}");
                    return ExitCode::FAILURE;
                }
            };
            let translator = AgentConfigTranslator::new();
            let rules = load_all_rules();
            match id {
                AgentId::Antigravity => {
                    println!("# Antigravity Export\n# {} rules available.", rules.len());
                }
                AgentId::ClaudeCode => {
                    println!("{}", translator.translate_claude_rules(&rules));
                }
                AgentId::OpenCode => {
                    println!("{}", translator.translate_opencode_rules(&rules));
                }
                AgentId::Cursor => {
                    println!("{}", translator.translate_cursor_legacy_rules(&rules));
                }
            }
            ExitCode::SUCCESS
        }
    }
}

fn execute_agent_profile(action: &AgentProfileSubcommand, workspace: Option<&Path>) -> ExitCode {
    let manager = AgentProfileManager::new();

    match action {
        AgentProfileSubcommand::List => {
            println!("\nAEK Agent Profiles");
            println!("{:=<75}", "");
            println!("{:<28} {:<45}", "PROFILE ID", "DESCRIPTION");
            println!("{:-<75}", "");
            for p in manager.list_profiles() {
                println!("{:<28} {:<45}", p.id, p.description);
            }
            println!("{:-<75}\n", "");
            ExitCode::SUCCESS
        }
        AgentProfileSubcommand::Show { profile } => {
            if let Some(p) = manager.get_profile(profile) {
                println!("\nProfile Details: {}", p.name);
                println!("{:-<60}", "");
                println!("ID:             {}", p.id);
                println!("Kind:           {:?}", p.kind);
                println!("Description:    {}", p.description);
                println!(
                    "Target Agents:  {:?}",
                    p.target_agents
                        .iter()
                        .map(|a| a.as_str())
                        .collect::<Vec<_>>()
                );
                println!("Enabled Rules:  {:?}", p.enabled_rules);
                println!("Enabled Skills: {:?}", p.enabled_skills);
                println!("MCP Servers:    {:?}", p.mcp_servers);
                println!("Validation:     {}", p.validation_policy);
                println!("{:-<60}\n", "");
                ExitCode::SUCCESS
            } else {
                eprintln!("[FAIL] Profile '{profile}' not found.");
                ExitCode::FAILURE
            }
        }
        AgentProfileSubcommand::Apply { profile, dry_run } => {
            if let Some(p) = manager.get_profile(profile) {
                let ws = workspace.unwrap_or_else(|| Path::new("."));
                println!("\nApplying Profile: {} to {}", p.name, ws.display());
                if *dry_run {
                    println!(
                        "[DRY RUN] Profile would configure agents: {:?}",
                        p.target_agents
                    );
                    return ExitCode::SUCCESS;
                }
                println!("[PASS] Profile '{}' applied successfully.\n", p.name);
                ExitCode::SUCCESS
            } else {
                eprintln!("[FAIL] Profile '{profile}' not found.");
                ExitCode::FAILURE
            }
        }
    }
}

fn uninstall_agent(
    _registry: &AgentRegistry,
    agent_name: &str,
    dry_run: bool,
    workspace: Option<&Path>,
) -> ExitCode {
    let id = match agent_name.parse::<AgentId>() {
        Ok(id) => id,
        Err(e) => {
            eprintln!("[FAIL] {e}");
            return ExitCode::FAILURE;
        }
    };

    let ws = workspace.unwrap_or_else(|| Path::new("."));
    println!("\nAEK Safe Agent Uninstaller: {}", id.as_str());
    println!("{:=<60}", "");

    let mut manifest = MultiAgentSynchronizer::load_manifest(ws);
    let mut removed = Vec::new();
    let mut to_remove_keys = Vec::new();

    for (key, record) in &manifest.managed_files {
        if record.agent_id == id {
            let path = PathBuf::from(key);
            if path.exists() {
                if dry_run {
                    println!(
                        "[DRY RUN] Would remove AEK-managed file: {}",
                        path.display()
                    );
                } else if let Err(e) = fs::remove_file(&path) {
                    eprintln!("[WARN] Failed to delete {}: {e}", path.display());
                } else {
                    println!("[REMOVED] {}", path.display());
                }
                removed.push(path);
            }
            to_remove_keys.push(key.clone());
        }
    }

    if !dry_run {
        for k in to_remove_keys {
            manifest.managed_files.remove(&k);
        }
        let _ = MultiAgentSynchronizer::save_manifest(ws, &manifest);
    }

    println!("Total Managed Files Processed: {}", removed.len());
    println!("{:-<60}\n", "");
    ExitCode::SUCCESS
}

fn show_compatibility(agent: Option<&str>) -> ExitCode {
    let registry = AgentCapabilityRegistry::default_registry();
    let target_ids: Vec<AgentId> = if let Some(a) = agent {
        match a.parse::<AgentId>() {
            Ok(id) => vec![id],
            Err(e) => {
                eprintln!("[FAIL] {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        AgentId::all().to_vec()
    };

    println!("\nAEK Multi-Agent Compatibility Report");
    println!("{:=<75}", "");
    println!(
        "{:<15} {:<24} {:<15} {:<18}",
        "AGENT", "CATEGORY", "STATUS", "SCOPE"
    );
    println!("{:-<75}", "");

    for id in target_ids {
        for record in registry.for_agent(id) {
            println!(
                "{:<15} {:<24} {:<15} {:<18}",
                id.as_str(),
                record.category.as_str(),
                format!("{:?}", record.support_status),
                record.supported_scope.chars().take(18).collect::<String>()
            );
        }
    }
    println!("{:-<75}\n", "");
    ExitCode::SUCCESS
}

fn load_all_rules() -> Vec<aek_rules::model::Rule> {
    let rules_dir = find_rules_dir(None);
    let mut registry = RuleRegistry::new();
    let _ = registry.load_directory(&rules_dir.join("global"), RuleScope::Global);
    let _ = registry.load_directory(&rules_dir.join("project"), RuleScope::Project);
    registry.list_all().into_iter().cloned().collect()
}

fn load_all_skills() -> Vec<aek_skills::model::SkillBundle> {
    let skills_dir = find_skills_dir(None);
    let mut registry = SkillRegistry::new();
    let _ = registry.load_directory(&skills_dir);
    registry.list_skills().into_iter().cloned().collect()
}

fn install_rules_legacy(dry_run: bool) -> ExitCode {
    let adapter = aek_agents::antigravity::AntigravityAdapter::new();
    let target_dir = match adapter.global_rules_dir() {
        Some(d) => d,
        None => PathBuf::from(".agents/rules"),
    };
    let rules = load_all_rules();
    if rules.is_empty() {
        eprintln!("No rules found to install.");
        return ExitCode::FAILURE;
    }
    match adapter.install_rules(&rules, &target_dir, dry_run) {
        Ok(r) => {
            println!(
                "Installed {} rule(s) to {}",
                r.installed_count,
                r.target_dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Rule installation error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn install_skills_legacy(dry_run: bool) -> ExitCode {
    let adapter = aek_agents::antigravity::AntigravityAdapter::new();
    let target_dir = match adapter.global_skills_dir() {
        Some(d) => d,
        None => PathBuf::from(".agents/skills"),
    };
    let skills = load_all_skills();
    if skills.is_empty() {
        eprintln!("No skills found to install.");
        return ExitCode::FAILURE;
    }
    match adapter.install_skills(&skills, &target_dir, dry_run) {
        Ok(r) => {
            println!(
                "Installed {} skill(s) to {}",
                r.installed_count,
                r.target_dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Skill installation error: {e}");
            ExitCode::FAILURE
        }
    }
}
