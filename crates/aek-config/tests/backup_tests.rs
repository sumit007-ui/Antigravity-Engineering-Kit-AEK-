use aek_config::backup::BackupEngine;
use aek_config::model::AekConfig;
use aek_config::writer::save_config_atomic;
use tempfile::tempdir;

#[test]
fn test_create_and_list_backups() {
    let dir = tempdir().expect("tempdir");
    let config_path = dir.path().join("config.toml");
    let backup_dir = dir.path().join("backups");

    let config = AekConfig::default();
    save_config_atomic(&config_path, &config).expect("save config");

    let meta = BackupEngine::create_backup(&config_path, &backup_dir, Some("manual-backup"))
        .expect("create backup");

    assert!(meta.path.exists());
    assert!(meta.id.starts_with("backup-"));

    let backups = BackupEngine::list_backups(&backup_dir).expect("list backups");
    assert_eq!(backups.len(), 1);
    assert_eq!(backups[0].id, meta.id);
}

#[test]
fn test_restore_backup_with_safety_backup() {
    let dir = tempdir().expect("tempdir");
    let config_path = dir.path().join("config.toml");
    let backup_dir = dir.path().join("backups");

    // 1. Initial config (Light)
    let initial_config = AekConfig::default();
    save_config_atomic(&config_path, &initial_config).expect("save initial");

    let meta = BackupEngine::create_backup(&config_path, &backup_dir, None).expect("create backup");

    // 2. Modify active config to Full
    let mut modified = initial_config.clone();
    modified.apply_profile(aek_core::ProfileKind::Full);
    save_config_atomic(&config_path, &modified).expect("save modified");

    // 3. Dry-run restore
    let dry_run_res = BackupEngine::restore_backup(&meta.id, &backup_dir, &config_path, true)
        .expect("dry run restore");
    assert!(dry_run_res.dry_run);

    // Active file should still be Full
    let active = aek_config::loader::load_config_from_file(&config_path).expect("load active");
    assert_eq!(active.general.profile, aek_core::ProfileKind::Full);

    // 4. Actual restore
    let restore_res = BackupEngine::restore_backup(&meta.id, &backup_dir, &config_path, false)
        .expect("actual restore");
    assert!(!restore_res.dry_run);
    assert!(restore_res.safety_backup_id.is_some());

    // Active file should now be restored to Light!
    let restored = aek_config::loader::load_config_from_file(&config_path).expect("load restored");
    assert_eq!(restored.general.profile, aek_core::ProfileKind::Light);
}
