use aek_config::model::AekConfig;
use aek_core::ProfileKind;

#[test]
fn test_default_config_schema() {
    let config = AekConfig::default();
    assert_eq!(config.general.config_version, 1);
    assert_eq!(config.general.profile, ProfileKind::Light);
    assert_eq!(config.logging.level, "info");
    assert_eq!(config.logging.format, "text");
    assert_eq!(config.execution.max_concurrent_tasks, 2);
    assert_eq!(config.execution.default_timeout_seconds, 30);
    assert!(!config.integrations.mcp_enabled);

    assert!(config.validate().is_ok());
}

#[test]
fn test_profile_application_preserves_unrelated_settings() {
    let mut config = AekConfig::default();
    config.general.name = "Custom Project Name".to_string();
    config.logging.level = "debug".to_string();
    config.security.allow_destructive_operations = false;

    // Apply Engineering profile
    config.apply_profile(ProfileKind::Engineering);
    assert_eq!(config.general.profile, ProfileKind::Engineering);
    assert!(config.integrations.mcp_enabled);
    assert_eq!(config.execution.max_concurrent_tasks, 4);

    // Unrelated user settings must be preserved!
    assert_eq!(config.general.name, "Custom Project Name");
    assert_eq!(config.logging.level, "debug");
    assert!(!config.security.allow_destructive_operations);

    // Apply Full profile
    config.apply_profile(ProfileKind::Full);
    assert_eq!(config.general.profile, ProfileKind::Full);
    assert!(config.integrations.prompt_optimization_enabled);
    assert!(config.integrations.security_scanning_enabled);
    assert_eq!(config.execution.max_concurrent_tasks, 8);
    assert_eq!(config.general.name, "Custom Project Name");
}
