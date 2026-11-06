use aek_skills::discovery::SkillDiscoverer;
use aek_skills::installer::{install_skill, remove_skill};
use aek_skills::model::SkillBundle;
use aek_skills::registry::SkillRegistry;
use aek_skills::validator::parse_skill_markdown;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_all_15_repository_skills_are_valid() {
    let skills_dir = Path::new("../../skills");
    if skills_dir.exists() {
        let mut registry = SkillRegistry::new();
        let loaded = registry
            .load_directory(skills_dir)
            .expect("load repo skills");
        assert_eq!(loaded, 15, "Expected all 15 skills to be valid and loaded");
        assert_eq!(registry.list_skills().len(), 15);
    }
}

#[test]
fn test_parse_valid_and_invalid_skill_markdown() {
    let valid_md = r#"---
name: rust-systems-engineer
description: Expert Rust engineer for memory safety, concurrency, and performance.
---

# Rust Systems Engineer
Instructions here.
"#;

    let meta = parse_skill_markdown(valid_md).expect("should parse valid skill");
    assert_eq!(meta.name, "rust-systems-engineer");
    assert!(meta.description.contains("Expert Rust engineer"));

    // Invalid: uppercase in name
    let invalid_md = r#"---
name: Rust-Engineer
description: Invalid uppercase in name
---
"#;
    assert!(parse_skill_markdown(invalid_md).is_err());

    // Invalid: missing description
    let invalid_desc = r#"---
name: rust-engineer
---
"#;
    assert!(parse_skill_markdown(invalid_desc).is_err());
}

#[test]
fn test_project_aware_discovery() {
    let dir = tempdir().expect("tempdir");
    let project_root = dir.path();

    // Create Cargo.toml and pyproject.toml (polyglot project)
    fs::write(
        project_root.join("Cargo.toml"),
        "[package]\nname = \"test\"",
    )
    .expect("write");
    fs::write(
        project_root.join("pyproject.toml"),
        "[project]\nname = \"py\"",
    )
    .expect("write");

    let recommendations = SkillDiscoverer::discover(project_root);
    assert!(recommendations.len() >= 2);

    let rust_rec = recommendations
        .iter()
        .find(|r| r.skill_name == "rust-systems-engineer")
        .unwrap();
    assert_eq!(rust_rec.confidence, 1.0);
    assert!(rust_rec.reason.contains("Cargo.toml"));

    let py_rec = recommendations
        .iter()
        .find(|r| r.skill_name == "python-engineer")
        .unwrap();
    assert_eq!(py_rec.confidence, 1.0);
    assert!(py_rec.reason.contains("pyproject.toml"));
}

#[test]
fn test_skill_installer_and_removal() {
    let dir = tempdir().expect("tempdir");
    let target_skills_dir = dir.path().join(".agents").join("skills");

    let bundle = SkillBundle::new_inline(
        "rust-systems-engineer",
        "Expert Rust systems engineer",
        "# Rust Instructions",
    );

    // Dry-run install
    let dry_res = install_skill(&bundle, &target_skills_dir, true).expect("dry run install");
    assert!(dry_res.dry_run);
    assert!(!target_skills_dir.join("rust-systems-engineer").exists());

    // Actual install
    let install_res = install_skill(&bundle, &target_skills_dir, false).expect("actual install");
    assert!(!install_res.dry_run);
    let installed_file = target_skills_dir
        .join("rust-systems-engineer")
        .join("SKILL.md");
    assert!(installed_file.exists());

    // Remove skill
    remove_skill("rust-systems-engineer", &target_skills_dir, false).expect("remove skill");
    assert!(!target_skills_dir.join("rust-systems-engineer").exists());
}
