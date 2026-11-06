use aek_config::backup::BackupEngine;
use aek_config::error::ConfigError;
use aek_config::loader::load_config_from_str;
use aek_config::model::AekConfig;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_path_traversal_backup_id_rejection() {
    let dir = tempdir().expect("tempdir");
    let config_path = dir.path().join("config.toml");
    let backup_dir = dir.path().join("backups");

    // Malicious IDs attempting path traversal
    let evil_ids = [
        "../../etc/passwd",
        r"..\..\Windows\System32",
        "backup/sub/dir",
        "backup\0nullbyte",
        "../backup-1234",
    ];

    for bad_id in evil_ids {
        let err =
            BackupEngine::restore_backup(bad_id, &backup_dir, &config_path, false).unwrap_err();
        assert!(matches!(
            err,
            ConfigError::SecurityViolation(_) | ConfigError::InvalidBackupId(_)
        ));
    }
}

#[test]
fn test_corrupted_backup_detection() {
    let dir = tempdir().expect("tempdir");
    let config_path = dir.path().join("config.toml");
    let backup_dir = dir.path().join("backups");
    fs::create_dir_all(&backup_dir).expect("create dir");

    let bad_id = "backup-corrupted-test";
    let bad_backup_file = backup_dir.join(format!("{bad_id}.toml"));
    fs::write(&bad_backup_file, "malformed toml = [[[{").expect("write bad file");

    let err = BackupEngine::restore_backup(bad_id, &backup_dir, &config_path, false).unwrap_err();
    assert!(matches!(
        err,
        ConfigError::CorruptedBackup { .. } | ConfigError::TomlDeserialization(_)
    ));
}

#[test]
fn test_unknown_fields_rejection() {
    let malicious_toml = r#"
[general]
name = "Test"
config_version = 1
profile = "light"
unauthorized_backdoor = true

[logging]
level = "info"
format = "text"

[execution]
max_concurrent_tasks = 2
default_timeout_seconds = 30
"#;

    let res: Result<AekConfig, _> = load_config_from_str(malicious_toml);
    assert!(res.is_err());
}
