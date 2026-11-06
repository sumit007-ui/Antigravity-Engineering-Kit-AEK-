use aek_core::redact_secrets;
use aek_languages::detection::ProjectDetector;
use aek_languages::discovery::extract_clean_version;
use aek_languages::model::LanguageId;
use aek_languages::registry::ToolchainRegistry;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_security_malformed_manifest_does_not_panic() {
    let dir = tempdir().unwrap();

    // Malformed Cargo.toml
    fs::write(dir.path().join("Cargo.toml"), "[[[package corrupt {{{").unwrap();

    // Malformed package.json
    fs::write(dir.path().join("package.json"), "{\"name\": incomplete").unwrap();

    // Malformed package.xml
    fs::write(
        dir.path().join("package.xml"),
        "<package format=\"3\"><broken",
    )
    .unwrap();

    // Malformed build.zig.zon
    fs::write(dir.path().join("build.zig.zon"), ".name = invalid").unwrap();

    let detector = ProjectDetector::new();
    let res = detector.detect(dir.path());
    assert!(
        res.is_ok(),
        "Detector must handle corrupt manifests gracefully"
    );
}

#[test]
fn test_security_directory_traversal_boundaries() {
    let dir = tempdir().unwrap();
    let nested = dir
        .path()
        .join("deep")
        .join("nested")
        .join("sub")
        .join("folder");
    fs::create_dir_all(&nested).unwrap();

    fs::write(
        nested.join("Cargo.toml"),
        r#"[package]
name = "deep-nested"
version = "0.1.0"
"#,
    )
    .unwrap();

    // With depth limit of 2, the deeply nested folder at depth 4 should be ignored
    let detector = ProjectDetector::new().with_max_depth(2);
    let projects = detector.detect(dir.path()).unwrap();
    assert_eq!(projects.len(), 0);

    // With depth limit of 5, it should be detected
    let detector_deep = ProjectDetector::new().with_max_depth(5);
    let projects_deep = detector_deep.detect(dir.path()).unwrap();
    assert_eq!(projects_deep.len(), 1);
    assert_eq!(projects_deep[0].name, "deep-nested");
}

#[test]
fn test_security_secret_redaction_in_toolchain_outputs() {
    let envs = vec![
        (
            "GITHUB_TOKEN".to_string(),
            "ghp_abcdef1234567890XYZ".to_string(),
        ),
        (
            "AWS_SECRET_ACCESS_KEY".to_string(),
            "secret_live_key_99999".to_string(),
        ),
        ("SAFE_VAR".to_string(), "safe_value".to_string()),
    ];
    let redacted = redact_secrets(&envs);
    assert_eq!(redacted.get("GITHUB_TOKEN").unwrap(), "********");
    assert_eq!(redacted.get("AWS_SECRET_ACCESS_KEY").unwrap(), "********");
    assert_eq!(redacted.get("SAFE_VAR").unwrap(), "safe_value");
}

#[test]
fn test_security_version_extraction_sanitization() {
    // Malicious or unexpected outputs should be sanitized without crashing
    let adversarial = "rustc 1.85.0\nrm -rf /\ncat /etc/shadow\n";
    let cleaned = extract_clean_version(adversarial);
    assert_eq!(cleaned, "1.85.0");

    let giant = "x".repeat(1000);
    let cleaned_giant = extract_clean_version(&giant);
    assert!(cleaned_giant.len() <= 40);
}

#[test]
fn test_security_toolchain_registry_rejects_duplicate_ids() {
    let mut reg = ToolchainRegistry::new();
    let tool = aek_languages::model::ToolDefinition {
        id: "duplicate-tool".to_string(),
        name: "Test Tool".to_string(),
        language: LanguageId::Rust,
        binary: "test".to_string(),
        version_args: vec![],
        supported_os: vec![],
        requirement: aek_languages::model::ToolRequirement::Optional,
        category: aek_languages::model::ToolCategory::Other("test".to_string()),
        doc_url: "https://example.com".to_string(),
        install_guidance: "none".to_string(),
    };

    assert!(reg.register(tool.clone()).is_ok());
    assert!(
        reg.register(tool).is_err(),
        "Duplicate IDs must be rejected"
    );
}
