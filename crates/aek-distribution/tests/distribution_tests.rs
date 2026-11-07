use aek_distribution::errors::DistributionError;
use aek_distribution::manifest::{ReleaseArtifact, ReleaseManifest};
use aek_distribution::migration::{ConfigMigrationStep, ConfigMigrator};
use aek_distribution::model::{
    InstallationManifest, InstallationMethod, InstallationScope, PlatformSpec, VerificationStatus,
};
use aek_distribution::paths::DistributionPaths;
use aek_distribution::rollback::RollbackManager;
use aek_distribution::uninstall::Uninstaller;
use aek_distribution::updates::UpdateChecker;
use aek_distribution::verification::{compute_file_sha256, verify_file_checksum};
use aek_distribution::versions::{ReleaseChannel, VersionManager};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_paths_and_traversal_guards() {
    let temp = TempDir::new().unwrap();
    let base = temp.path().join("install_base");
    fs::create_dir_all(&base).unwrap();

    let paths = DistributionPaths::with_base(base.clone());
    assert!(paths.bin_dir().starts_with(&base));
    assert!(paths.versions_dir().starts_with(&base));

    // Valid path inside base
    let valid_child = base.join("bin").join("aek.exe");
    assert!(DistributionPaths::validate_containment(&valid_child, &base).is_ok());

    // Path traversal outside base must fail
    let malicious = base.join("..").join("outside.exe");
    assert!(matches!(
        DistributionPaths::validate_containment(&malicious, &base),
        Err(DistributionError::PathTraversal { .. })
    ));
}

#[test]
fn test_paths_with_spaces() {
    let temp = TempDir::new().unwrap();
    let space_dir = temp.path().join("My Program Files with Spaces");
    fs::create_dir_all(&space_dir).unwrap();

    let paths = DistributionPaths::with_base(space_dir.clone());
    let bin_path = paths.binary_path();
    assert!(
        bin_path
            .to_str()
            .unwrap()
            .contains("My Program Files with Spaces")
    );
    assert!(DistributionPaths::validate_containment(&bin_path, &space_dir).is_ok());
}

#[test]
fn test_version_comparisons_and_downgrade_guard() {
    let vm = VersionManager::new("0.1.0");
    assert_eq!(vm.current_version(), "0.1.0");

    assert!(vm.is_upgrade("0.2.0").unwrap());
    assert!(!vm.is_upgrade("0.1.0").unwrap());
    assert!(!vm.is_upgrade("0.0.9").unwrap());

    assert!(vm.is_downgrade("0.0.9").unwrap());
    assert!(!vm.is_downgrade("0.2.0").unwrap());

    assert_eq!(vm.channel("0.1.0"), ReleaseChannel::Stable);
    assert_eq!(vm.channel("0.2.0-beta.1"), ReleaseChannel::Beta);
    assert_eq!(
        vm.channel("0.3.0-nightly.20261106"),
        ReleaseChannel::Nightly
    );
}

#[test]
fn test_release_manifest_parsing_and_lookup() {
    let json_data = r#"{
        "schema_version": "1.0.0",
        "version": "0.2.0",
        "release_channel": "stable",
        "timestamp": "2026-11-06T18:00:00Z",
        "artifacts": [
            {
                "filename": "aek-v0.2.0-windows-x86_64.zip",
                "target_triple": "x86_64-pc-windows-msvc",
                "os": "windows",
                "arch": "x86_64",
                "size_bytes": 10485760,
                "sha256": "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                "download_url": "https://github.com/google-antigravity/aek/releases/download/v0.2.0/aek-v0.2.0-windows-x86_64.zip"
            },
            {
                "filename": "aek-v0.2.0-linux-x86_64.tar.gz",
                "target_triple": "x86_64-unknown-linux-gnu",
                "os": "linux",
                "arch": "x86_64",
                "size_bytes": 9437184,
                "sha256": "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                "download_url": "https://github.com/google-antigravity/aek/releases/download/v0.2.0/aek-v0.2.0-linux-x86_64.tar.gz"
            }
        ]
    }"#;

    let manifest = ReleaseManifest::from_json(json_data).expect("parse release manifest");
    assert_eq!(manifest.version, "0.2.0");
    assert_eq!(manifest.artifacts.len(), 2);

    let win_platform = PlatformSpec {
        target_triple: "x86_64-pc-windows-msvc".to_string(),
        os: "windows".to_string(),
        arch: "x86_64".to_string(),
    };
    let win_artifact = manifest
        .find_artifact(&win_platform)
        .expect("find win artifact");
    assert_eq!(win_artifact.filename, "aek-v0.2.0-windows-x86_64.zip");

    let linux_platform = PlatformSpec {
        target_triple: "x86_64-unknown-linux-gnu".to_string(),
        os: "linux".to_string(),
        arch: "x86_64".to_string(),
    };
    let linux_artifact = manifest
        .find_artifact(&linux_platform)
        .expect("find linux artifact");
    assert_eq!(linux_artifact.filename, "aek-v0.2.0-linux-x86_64.tar.gz");
}

#[test]
fn test_checksum_computation_and_verification() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join("test_file.bin");
    fs::write(&file_path, b"hello antigravity engineering kit").unwrap();

    let hash = compute_file_sha256(&file_path).unwrap();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 64);

    // Matching checksum should pass
    assert!(verify_file_checksum(&file_path, &hash).is_ok());

    // Mismatched checksum should fail
    let bad_hash = "0000000000000000000000000000000000000000000000000000000000000000";
    let res = verify_file_checksum(&file_path, bad_hash);
    assert!(matches!(
        res,
        Err(DistributionError::ChecksumMismatch { .. })
    ));
}

#[test]
fn test_installation_manifest_serialization() {
    let manifest = InstallationManifest {
        version: "0.1.0".to_string(),
        scope: InstallationScope::User,
        install_dir: PathBuf::from("/home/user/.local/bin"),
        binary_path: PathBuf::from("/home/user/.local/bin/aek"),
        binary_checksum: None,
        installed_at: "2026-11-06T18:00:00Z".to_string(),
        method: InstallationMethod::InstallerScript,
        platform: PlatformSpec {
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            os: "linux".to_string(),
            arch: "x86_64".to_string(),
        },
        managed_files: vec![PathBuf::from("/home/user/.local/bin/aek")],
        previous_version: None,
        verification_status: VerificationStatus::Verified,
    };

    let json = serde_json::to_string_pretty(&manifest).unwrap();
    let deserialized: InstallationManifest = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.version, "0.1.0");
    assert_eq!(
        deserialized.verification_status,
        VerificationStatus::Verified
    );
}

#[test]
fn test_update_checker_and_plan() {
    let current_version = "0.1.0";
    let manifest = ReleaseManifest {
        schema_version: "1.0.0".to_string(),
        version: "0.2.0".to_string(),
        release_channel: "stable".to_string(),
        timestamp: "2026-11-06T18:00:00Z".to_string(),
        artifacts: vec![ReleaseArtifact {
            filename: "aek-v0.2.0-windows-x86_64.zip".to_string(),
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            os: "windows".to_string(),
            arch: "x86_64".to_string(),
            size_bytes: 1024,
            sha256: "abc".to_string(),
            download_url: "https://example.com/aek.zip".to_string(),
        }],
    };

    let platform = PlatformSpec {
        target_triple: "x86_64-pc-windows-msvc".to_string(),
        os: "windows".to_string(),
        arch: "x86_64".to_string(),
    };

    let checker = UpdateChecker::new(current_version, platform);
    let plan = checker.create_plan(&manifest, false).unwrap();

    assert!(plan.is_update_available);
    assert_eq!(plan.target_version, "0.2.0");
    assert_eq!(plan.artifact.filename, "aek-v0.2.0-windows-x86_64.zip");
}

#[test]
fn test_rollback_manager() {
    let temp = TempDir::new().unwrap();
    let base = temp.path().join("aek_app");
    fs::create_dir_all(&base).unwrap();

    let paths = DistributionPaths::with_base(base);
    let manager = RollbackManager::new(paths.clone());

    // Initially no rollbacks
    assert!(manager.list_rollbacks().unwrap().is_empty());

    // Archive version 0.1.0
    let bin_path = paths.binary_path();
    if let Some(parent) = bin_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&bin_path, b"binary v0.1.0").unwrap();

    manager.archive_version("0.1.0", &bin_path).unwrap();

    let rollbacks = manager.list_rollbacks().unwrap();
    assert_eq!(rollbacks.len(), 1);
    assert_eq!(rollbacks[0].version, "0.1.0");

    // Replace current binary with v0.2.0
    fs::write(&bin_path, b"binary v0.2.0").unwrap();

    // Roll back to 0.1.0 in dry-run mode (should not change binary)
    let plan = manager.plan_rollback(Some("0.1.0")).unwrap();
    assert_eq!(plan.target_version, "0.1.0");
    assert_eq!(fs::read(&bin_path).unwrap(), b"binary v0.2.0");

    // Execute rollback
    manager.execute_rollback(&plan).unwrap();
    assert_eq!(fs::read(&bin_path).unwrap(), b"binary v0.1.0");
}

#[test]
fn test_config_migration_engine() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join("config.toml");
    fs::write(
        &config_path,
        r#"
schema_version = 1
app_name = "AEK"
profile = "light"
"#,
    )
    .unwrap();

    let mut migrator = ConfigMigrator::new();
    migrator.register_step(ConfigMigrationStep {
        from_version: 1,
        to_version: 2,
        description: "Add update_channel field to configuration".to_string(),
        migrate_fn: Box::new(|mut toml_val| {
            if let toml::Value::Table(ref mut map) = toml_val {
                map.insert(
                    "update_channel".to_string(),
                    toml::Value::String("stable".to_string()),
                );
                map.insert("schema_version".to_string(), toml::Value::Integer(2));
            }
            Ok(toml_val)
        }),
    });

    let plan = migrator.plan_migration(&config_path, 2).unwrap();
    assert_eq!(plan.steps.len(), 1);
    assert_eq!(plan.from_version, 1);
    assert_eq!(plan.to_version, 2);

    migrator.execute_migration(&config_path, &plan).unwrap();

    let updated_content = fs::read_to_string(&config_path).unwrap();
    assert!(updated_content.contains("update_channel = \"stable\""));
    assert!(updated_content.contains("schema_version = 2"));
}

#[test]
fn test_uninstaller_safe_and_purge() {
    let temp = TempDir::new().unwrap();
    let install_dir = temp.path().join("install");
    let config_dir = temp.path().join("config");
    fs::create_dir_all(&install_dir).unwrap();
    fs::create_dir_all(&config_dir).unwrap();

    let bin_path = install_dir.join("aek.exe");
    let manifest_path = install_dir.join("manifest.json");
    let config_file = config_dir.join("config.toml");

    fs::write(&bin_path, b"aek executable").unwrap();
    fs::write(&manifest_path, b"{}").unwrap();
    fs::write(&config_file, b"user custom rules and config").unwrap();

    let manifest = InstallationManifest {
        version: "0.1.0".to_string(),
        scope: InstallationScope::User,
        install_dir: install_dir.clone(),
        binary_path: bin_path.clone(),
        binary_checksum: None,
        installed_at: "2026-11-06T18:00:00Z".to_string(),
        method: InstallationMethod::InstallerScript,
        platform: PlatformSpec {
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            os: "windows".to_string(),
            arch: "x86_64".to_string(),
        },
        managed_files: vec![bin_path.clone(), manifest_path.clone()],
        previous_version: None,
        verification_status: VerificationStatus::Verified,
    };

    let uninstaller = Uninstaller::new(manifest, config_dir.clone());

    // 1. Dry run
    let plan = uninstaller.plan_uninstall(false);
    assert!(plan.files_to_remove.contains(&bin_path));
    assert!(plan.files_to_remove.contains(&manifest_path));
    assert!(!plan.dirs_to_remove.contains(&config_dir));

    // 2. Safe uninstall (preserves config)
    uninstaller.execute(&plan).unwrap();
    assert!(!bin_path.exists());
    assert!(!manifest_path.exists());
    assert!(
        config_file.exists(),
        "Safe uninstall must preserve user config!"
    );

    // 3. Purge uninstall (removes config)
    let purge_plan = uninstaller.plan_uninstall(true);
    assert!(purge_plan.dirs_to_remove.contains(&config_dir));
    uninstaller.execute(&purge_plan).unwrap();
    assert!(!config_file.exists());
}
