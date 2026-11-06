use aek_languages::detection::ProjectDetector;
use aek_languages::model::{LanguageId, ToolRequirement};
use aek_languages::providers::get_all_providers;
use aek_languages::registry::ToolchainRegistry;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_detect_rust_project() {
    let dir = tempdir().unwrap();
    let cargo_toml = dir.path().join("Cargo.toml");
    fs::write(
        &cargo_toml,
        r#"[package]
name = "demo-rust"
version = "0.1.0"
edition = "2024"
"#,
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let projects = detector.detect(dir.path()).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].language, LanguageId::Rust);
    assert_eq!(projects[0].name, "demo-rust");
}

#[test]
fn test_detect_go_project() {
    let dir = tempdir().unwrap();
    let go_mod = dir.path().join("go.mod");
    fs::write(
        &go_mod,
        r#"module example.com/my/project

go 1.22
"#,
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let projects = detector.detect(dir.path()).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].language, LanguageId::Go);
    assert_eq!(projects[0].name, "example.com/my/project");
}

#[test]
fn test_detect_cpp_project() {
    let dir = tempdir().unwrap();
    let cmake_file = dir.path().join("CMakeLists.txt");
    fs::write(
        &cmake_file,
        r#"cmake_minimum_required(VERSION 3.20)
project(EngineCore LANGUAGES CXX)
add_executable(engine src/main.cpp)
"#,
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let projects = detector.detect(dir.path()).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].language, LanguageId::Cpp);
    assert_eq!(projects[0].name, "EngineCore");
}

#[test]
fn test_detect_zig_project() {
    let dir = tempdir().unwrap();
    let zig_build = dir.path().join("build.zig");
    fs::write(
        &zig_build,
        r#"const std = @import("std");
pub fn build(b: *std.Build) void {}
"#,
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let projects = detector.detect(dir.path()).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].language, LanguageId::Zig);
}

#[test]
fn test_detect_python_project() {
    let dir = tempdir().unwrap();
    let pyproject = dir.path().join("pyproject.toml");
    fs::write(
        &pyproject,
        r#"[project]
name = "ml-service"
version = "0.1.0"
dependencies = ["numpy", "torch"]
"#,
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let projects = detector.detect(dir.path()).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].language, LanguageId::Python);
    assert_eq!(projects[0].name, "ml-service");
}

#[test]
fn test_detect_javascript_typescript_project() {
    let dir = tempdir().unwrap();
    let pkg_json = dir.path().join("package.json");
    fs::write(
        &pkg_json,
        r#"{
  "name": "web-frontend",
  "version": "1.0.0",
  "scripts": { "build": "vite build" }
}
"#,
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let projects = detector.detect(dir.path()).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].language, LanguageId::Javascript);
    assert_eq!(projects[0].name, "web-frontend");
}

#[test]
fn test_detect_flutter_project() {
    let dir = tempdir().unwrap();
    let pubspec = dir.path().join("pubspec.yaml");
    fs::write(
        &pubspec,
        r#"name: mobile_client
description: Mobile application
environment:
  sdk: '>=3.0.0 <4.0.0'
dependencies:
  flutter:
    sdk: flutter
"#,
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let projects = detector.detect(dir.path()).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].language, LanguageId::Flutter);
    assert_eq!(projects[0].name, "mobile_client");
}

#[test]
fn test_detect_ros2_package_vs_generic_xml() {
    let dir = tempdir().unwrap();

    // Generic XML should NOT be recognized as ROS 2
    let generic_dir = dir.path().join("generic");
    fs::create_dir_all(&generic_dir).unwrap();
    fs::write(
        generic_dir.join("package.xml"),
        "<random><tag>not a ros package</tag></random>",
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let generic_projects = detector.detect(&generic_dir).unwrap();
    assert_eq!(generic_projects.len(), 0);

    // Legitimate ROS 2 package
    let ros_dir = dir.path().join("ros_pkg");
    fs::create_dir_all(&ros_dir).unwrap();
    fs::write(
        ros_dir.join("package.xml"),
        r#"<?xml version="1.0"?>
<?xml-model href="http://download.ros.org/schema/package_format3.xsd" schematypens="http://www.w3.org/2001/XMLSchema"?>
<package format="3">
  <name>navigation_node</name>
  <version>1.0.0</version>
  <description>Robot navigation</description>
  <buildtool_depend>ament_cmake</buildtool_depend>
</package>
"#,
    )
    .unwrap();

    let ros_projects = detector.detect(&ros_dir).unwrap();
    assert_eq!(ros_projects.len(), 1);
    assert_eq!(ros_projects[0].language, LanguageId::Ros2);
    assert_eq!(ros_projects[0].name, "navigation_node");
}

#[test]
fn test_monorepo_and_nested_detection() {
    let dir = tempdir().unwrap();

    // Root Cargo workspace
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"[workspace]
members = ["crates/core", "crates/cli"]
"#,
    )
    .unwrap();

    let core_dir = dir.path().join("crates").join("core");
    fs::create_dir_all(&core_dir).unwrap();
    fs::write(
        core_dir.join("Cargo.toml"),
        r#"[package]
name = "core"
version = "0.1.0"
"#,
    )
    .unwrap();

    let py_dir = dir.path().join("services").join("analytics");
    fs::create_dir_all(&py_dir).unwrap();
    fs::write(
        py_dir.join("requirements.txt"),
        "pandas==2.0.0\nfastapi==0.110.0\n",
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let projects = detector.detect(dir.path()).unwrap();

    assert!(projects.len() >= 2);
    let langs: Vec<LanguageId> = projects.iter().map(|p| p.language.clone()).collect();
    assert!(langs.contains(&LanguageId::Rust));
    assert!(langs.contains(&LanguageId::Python));
}

#[test]
fn test_symlink_and_ignore_safety() {
    let dir = tempdir().unwrap();

    // Ignored directories should not be scanned
    let node_modules = dir.path().join("node_modules").join("subpkg");
    fs::create_dir_all(&node_modules).unwrap();
    fs::write(node_modules.join("package.json"), r#"{"name": "subpkg"}"#).unwrap();

    let target_dir = dir.path().join("target").join("dummy");
    fs::create_dir_all(&target_dir).unwrap();
    fs::write(
        target_dir.join("Cargo.toml"),
        r#"[package]
name = "dummy"
version = "0.1.0""#,
    )
    .unwrap();

    let detector = ProjectDetector::new();
    let projects = detector.detect(dir.path()).unwrap();
    assert_eq!(projects.len(), 0);
}

#[test]
fn test_toolchain_registry_default_tools() {
    let registry = ToolchainRegistry::default();
    let rust_tools = registry.get_tools_for_language(&LanguageId::Rust);
    assert!(!rust_tools.is_empty());
    assert!(rust_tools.iter().any(|t| t.binary == "cargo"));
    assert!(rust_tools.iter().any(|t| t.binary == "rustc"));

    let go_tools = registry.get_tools_for_language(&LanguageId::Go);
    assert!(go_tools.iter().any(|t| t.binary == "go"));

    let py_tools = registry.get_tools_for_language(&LanguageId::Python);
    assert!(
        py_tools
            .iter()
            .any(|t| t.binary == "python" || t.binary == "python3")
    );

    let required = rust_tools.iter().find(|t| t.binary == "cargo").unwrap();
    assert_eq!(required.requirement, ToolRequirement::Required);
}

#[test]
fn test_skill_recommendations_for_detected_languages() {
    let providers = get_all_providers();
    let rust_provider = providers
        .iter()
        .find(|p| p.id() == LanguageId::Rust)
        .unwrap();
    assert_eq!(
        rust_provider.recommended_skills(),
        vec!["rust-systems-engineer"]
    );

    let go_provider = providers.iter().find(|p| p.id() == LanguageId::Go).unwrap();
    assert_eq!(
        go_provider.recommended_skills(),
        vec!["go-backend-engineer"]
    );

    let ros2_provider = providers
        .iter()
        .find(|p| p.id() == LanguageId::Ros2)
        .unwrap();
    assert_eq!(
        ros2_provider.recommended_skills(),
        vec!["robotics-ros2-engineer"]
    );
}
