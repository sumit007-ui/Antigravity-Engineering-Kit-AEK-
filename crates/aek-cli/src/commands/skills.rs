//! Command handler for `aek skills`.

use crate::cli::SkillsSubcommand;
use aek_agents::adapter::AgentAdapter;
use aek_agents::antigravity::AntigravityAdapter;
use aek_platform::paths::PlatformPaths;
use aek_skills::discovery::SkillDiscoverer;
use aek_skills::installer::{install_skill, remove_skill};
use aek_skills::registry::SkillRegistry;
use aek_skills::validator::parse_skill_markdown;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn find_skills_dir(custom_path: Option<&Path>) -> PathBuf {
    if let Some(p) = custom_path {
        return p.to_path_buf();
    }

    let candidates = [
        PathBuf::from("skills"),
        PathBuf::from("../skills"),
        PathBuf::from("../../skills"),
    ];

    for c in &candidates {
        if c.exists() && c.is_dir() {
            return c.clone();
        }
    }

    let platform = PlatformPaths::resolve();
    platform.data_dir.join("skills")
}

pub fn execute_skills(action: &SkillsSubcommand) -> ExitCode {
    match action {
        SkillsSubcommand::List { json } => list_skills(*json),
        SkillsSubcommand::Show { name } => show_skill(name),
        SkillsSubcommand::Validate { path } => validate_skills_cmd(path.as_deref()),
        SkillsSubcommand::Recommend { path } => recommend_skills(path.as_deref()),
        SkillsSubcommand::Install {
            name,
            target,
            dry_run,
        } => install_skill_cmd(name, target.as_deref(), *dry_run),
        SkillsSubcommand::Remove {
            name,
            target,
            dry_run,
        } => remove_skill_cmd(name, target.as_deref(), *dry_run),
    }
}

fn list_skills(as_json: bool) -> ExitCode {
    let skills_dir = find_skills_dir(None);
    let mut registry = SkillRegistry::new();
    let _ = registry.load_directory(&skills_dir);

    let skills = registry.list_skills();
    if as_json {
        let items: Vec<serde_json::Value> = skills
            .iter()
            .map(|s| {
                serde_json::json!({
                    "name": s.metadata.name,
                    "description": s.metadata.description,
                    "version": s.metadata.version,
                    "size_bytes": s.body.len(),
                    "path": s.root_dir.to_string_lossy(),
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
    println!("Available Skills ({} loaded):", skills.len());
    println!("{:-<75}", "");
    println!("{:<32} {:<10} Description", "Skill Name", "Version");
    println!("{:-<75}", "");

    for s in &skills {
        let desc = if s.metadata.description.len() > 30 {
            format!("{}...", &s.metadata.description[..27])
        } else {
            s.metadata.description.clone()
        };
        println!(
            "{:<32} {:<10} {}",
            s.metadata.name, s.metadata.version, desc
        );
    }

    println!("{:-<75}", "");
    ExitCode::SUCCESS
}

fn show_skill(name: &str) -> ExitCode {
    let skills_dir = find_skills_dir(None);
    let mut registry = SkillRegistry::new();
    let _ = registry.load_directory(&skills_dir);

    match registry.get_skill(name) {
        Some(skill) => {
            println!();
            println!("Skill: {}", skill.metadata.name);
            println!("Version: {}", skill.metadata.version);
            println!("Path: {}", skill.instructions_path.display());
            println!("Description:\n  {}", skill.metadata.description);
            println!("{:-<70}", "");
            println!("{}", skill.body);
            println!("{:-<70}", "");
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("Error: Skill '{}' not found in registry.", name);
            ExitCode::FAILURE
        }
    }
}

fn validate_skills_cmd(custom_path: Option<&Path>) -> ExitCode {
    let skills_dir = find_skills_dir(custom_path);
    let mut registry = SkillRegistry::new();
    let _ = registry.load_directory(&skills_dir);

    let skills = registry.list_skills();
    if skills.is_empty() {
        eprintln!("No skills found in {}", skills_dir.display());
        return ExitCode::FAILURE;
    }

    let mut failed = 0;
    println!(
        "Validating {} skills in {}...",
        skills.len(),
        skills_dir.display()
    );
    for s in &skills {
        let content = fs::read_to_string(&s.instructions_path).unwrap_or_default();
        match parse_skill_markdown(&content) {
            Ok(_) => {
                println!("  [OK] {} ({} B)", s.metadata.name, s.body.len());
            }
            Err(e) => {
                println!("  [FAIL] {}: {}", s.metadata.name, e);
                failed += 1;
            }
        }
    }

    if failed == 0 {
        println!("\nAll {} skills passed validation.", skills.len());
        ExitCode::SUCCESS
    } else {
        eprintln!("\n{} skill(s) failed validation.", failed);
        ExitCode::FAILURE
    }
}

fn recommend_skills(path: Option<&Path>) -> ExitCode {
    let project_dir = match path {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    };

    let recs = SkillDiscoverer::discover(&project_dir);

    println!();
    println!(
        "Project Skill Recommendations for '{}':",
        project_dir.display()
    );
    println!("{:-<70}", "");
    if recs.is_empty() {
        println!("No specific language/framework markers detected.");
    } else {
        for r in &recs {
            println!(
                "  • {:<30} (Confidence: {:<6}) -> {}",
                r.skill_name,
                format!("{:.0}%", r.confidence * 100.0),
                r.reason
            );
        }
    }
    println!("{:-<70}", "");
    ExitCode::SUCCESS
}

fn install_skill_cmd(name: &str, target: Option<&Path>, dry_run: bool) -> ExitCode {
    let skills_dir = find_skills_dir(None);
    let mut registry = SkillRegistry::new();
    let _ = registry.load_directory(&skills_dir);

    let skill = match registry.get_skill(name) {
        Some(s) => s,
        None => {
            eprintln!("Error: Skill '{}' not found in registry.", name);
            return ExitCode::FAILURE;
        }
    };

    let target_dir = match target {
        Some(p) => p.to_path_buf(),
        None => {
            let adapter = AntigravityAdapter::new();
            adapter
                .global_skills_dir()
                .unwrap_or_else(|| PathBuf::from(".agents/skills"))
        }
    };

    if dry_run {
        println!(
            "[Dry run] Would install skill '{}' to: {}",
            skill.metadata.name,
            target_dir.join(&skill.metadata.name).display()
        );
        return ExitCode::SUCCESS;
    }

    match install_skill(skill, &target_dir, false) {
        Ok(res) => {
            println!(
                "Successfully installed skill '{}' to {}",
                skill.metadata.name,
                res.target_dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to install skill '{}': {}", skill.metadata.name, e);
            ExitCode::FAILURE
        }
    }
}

fn remove_skill_cmd(name: &str, target: Option<&Path>, dry_run: bool) -> ExitCode {
    let target_dir = match target {
        Some(p) => p.to_path_buf(),
        None => {
            let adapter = AntigravityAdapter::new();
            adapter
                .global_skills_dir()
                .unwrap_or_else(|| PathBuf::from(".agents/skills"))
        }
    };

    if dry_run {
        println!(
            "[Dry run] Would remove skill '{}' from: {}",
            name,
            target_dir.join(name).display()
        );
        return ExitCode::SUCCESS;
    }

    match remove_skill(name, &target_dir, false) {
        Ok(res) => {
            println!(
                "Successfully removed skill '{}' from {}",
                name,
                res.target_dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to remove skill '{}': {}", name, e);
            ExitCode::FAILURE
        }
    }
}
