use aek_config::{AekConfig, ConfigError};
use aek_core::ProfileKind;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_default_config_is_valid() {
    let config = AekConfig::default();
    assert_eq!(config.general.config_version, 1);
    assert_eq!(config.general.profile, ProfileKind::Light);
    assert_eq!(config.logging.level, "info");
    assert_eq!(config.execution.default_timeout_seconds, 30);
    assert!(!config.integrations.mcp_enabled);

    // Validation should succeed
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_rules() {
    let config_bad_timeout = AekConfig {
        execution: aek_config::ExecutionConfig {
            default_timeout_seconds: 0,
            ..Default::default()
        },
        ..Default::default()
    };
    let err = config_bad_timeout.validate().unwrap_err();
    assert!(matches!(err, ConfigError::Validation(_)));

    let config_bad_log = AekConfig {
        logging: aek_config::LoggingConfig {
            level: "super_verbose_unknown".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    let err = config_bad_log.validate().unwrap_err();
    assert!(matches!(err, ConfigError::Validation(_)));

    let config_bad_name = AekConfig {
        general: aek_config::GeneralConfig {
            name: String::new(),
            ..Default::default()
        },
        ..Default::default()
    };
    let err = config_bad_name.validate().unwrap_err();
    assert!(matches!(err, ConfigError::Validation(_)));
}

#[test]
fn test_config_serialization_and_deserialization() {
    let config = AekConfig::default();
    let toml_str = toml::to_string_pretty(&config).expect("failed to serialize config to TOML");

    let loaded: AekConfig =
        toml::from_str(&toml_str).expect("failed to deserialize config from TOML");
    assert_eq!(loaded.general.config_version, config.general.config_version);
    assert_eq!(loaded.general.profile, config.general.profile);
    assert_eq!(
        loaded.execution.default_timeout_seconds,
        config.execution.default_timeout_seconds
    );
}

#[test]
fn test_atomic_save_and_load() {
    let dir = tempdir().expect("failed to create tempdir");
    let file_path = dir.path().join("aek.toml");

    let config = AekConfig {
        execution: aek_config::ExecutionConfig {
            default_timeout_seconds: 45,
            ..Default::default()
        },
        logging: aek_config::LoggingConfig {
            level: "debug".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    // Save atomically
    config
        .save_to_file(&file_path)
        .expect("failed to save config atomically");
    assert!(file_path.exists());

    // Load back
    let loaded = AekConfig::load_from_file(&file_path).expect("failed to load saved config");
    assert_eq!(loaded.execution.default_timeout_seconds, 45);
    assert_eq!(loaded.logging.level, "debug");

    // Overwrite atomically with new content
    let mut updated = loaded;
    updated.execution.default_timeout_seconds = 60;
    updated
        .save_to_file(&file_path)
        .expect("failed to overwrite config atomically");

    let reloaded = AekConfig::load_from_file(&file_path).expect("failed to reload updated config");
    assert_eq!(reloaded.execution.default_timeout_seconds, 60);
}

#[test]
fn test_load_nonexistent_file() {
    let dir = tempdir().expect("failed to create tempdir");
    let nonexistent = dir.path().join("does_not_exist.toml");

    let err = AekConfig::load_from_file(&nonexistent).unwrap_err();
    assert!(matches!(err, ConfigError::NotFound(_)));
}

#[test]
fn test_load_malformed_toml() {
    let dir = tempdir().expect("failed to create tempdir");
    let malformed_file = dir.path().join("malformed.toml");
    fs::write(&malformed_file, "this is not valid toml = [[[").expect("write failed");

    let err = AekConfig::load_from_file(&malformed_file).unwrap_err();
    assert!(matches!(err, ConfigError::TomlDeserialization(_)));
}
