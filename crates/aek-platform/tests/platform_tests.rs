use aek_platform::detection::{OsKind, detect_arch, detect_os, find_executable_in_path};
use aek_platform::paths::PlatformPaths;
use std::path::Path;

#[test]
fn test_os_and_arch_detection() {
    let os = detect_os();
    let arch = detect_arch();

    assert!(!arch.is_empty());
    #[cfg(target_os = "windows")]
    assert_eq!(os, OsKind::Windows);

    #[cfg(target_os = "linux")]
    assert_eq!(os, OsKind::Linux);

    #[cfg(target_os = "macos")]
    assert_eq!(os, OsKind::MacOS);
}

#[test]
fn test_find_executable_in_path() {
    // cargo and rustc should always be in PATH in our dev environment
    let cargo_path = find_executable_in_path("cargo");
    assert!(cargo_path.is_some());
    let path = cargo_path.unwrap();
    assert!(path.exists());

    // Nonexistent binary
    assert!(find_executable_in_path("nonexistent_binary_xyz_987654321").is_none());
}

#[test]
fn test_platform_paths_resolution() {
    let paths = PlatformPaths::resolve();
    assert!(!paths.config_dir.as_os_str().is_empty());
    assert!(!paths.data_dir.as_os_str().is_empty());
    assert!(!paths.cache_dir.as_os_str().is_empty());
    assert!(!paths.backup_dir.as_os_str().is_empty());
}

#[test]
fn test_path_traversal_prevention() {
    let parent = Path::new("/var/aek/data");
    let safe_child = Path::new("/var/aek/data/backups/file.toml");
    let evil_child = Path::new("/var/aek/data/../../etc/passwd");

    assert!(PlatformPaths::is_safe_subpath(parent, safe_child));
    assert!(!PlatformPaths::is_safe_subpath(parent, evil_child));
}
