//! Platform directory resolution and filesystem path safety.

use directories::{BaseDirs, ProjectDirs};
use std::path::{Component, Path, PathBuf};

/// Resolved standard directories for AEK on the host platform.
#[derive(Debug, Clone)]
pub struct PlatformPaths {
    /// Platform configuration directory (e.g. `~/.config/aek` or `%APPDATA%\aek`).
    pub config_dir: PathBuf,
    /// Standard location for primary user configuration file (`<config_dir>/config.toml`).
    pub config_file: PathBuf,
    /// Platform data directory (e.g. `~/.local/share/aek` or `%LOCALAPPDATA%\aek\data`).
    pub data_dir: PathBuf,
    /// Cache directory (e.g. `~/.cache/aek` or `%LOCALAPPDATA%\aek\cache`).
    pub cache_dir: PathBuf,
    /// Directory dedicated to configuration backups.
    pub backup_dir: PathBuf,
    /// Detected Google Antigravity global directory (`~/.gemini`).
    pub antigravity_dir: Option<PathBuf>,
}

impl PlatformPaths {
    /// Resolves standard directories using the host platform conventions without hardcoding.
    #[must_use]
    pub fn resolve() -> Self {
        let (config_dir, data_dir, cache_dir) = if let Ok(custom) = std::env::var("AEK_CONFIG_DIR") {
            let custom_p = PathBuf::from(custom);
            (
                custom_p.clone(),
                custom_p.join("data"),
                custom_p.join("cache"),
            )
        } else {
            let proj_dirs = ProjectDirs::from("com", "antigravity", "aek");
            match proj_dirs {
                Some(dirs) => (
                    dirs.config_dir().to_path_buf(),
                    dirs.data_dir().to_path_buf(),
                    dirs.cache_dir().to_path_buf(),
                ),
                None => {
                    let fallback = PathBuf::from(".aek");
                    (
                        fallback.join("config"),
                        fallback.join("data"),
                        fallback.join("cache"),
                    )
                }
            }
        };

        let base_dirs = BaseDirs::new();
        let config_file = config_dir.join("config.toml");
        let backup_dir = data_dir.join("backups");

        let antigravity_dir = base_dirs.map(|b| b.home_dir().join(".gemini"));

        Self {
            config_dir,
            config_file,
            data_dir,
            cache_dir,
            backup_dir,
            antigravity_dir,
        }
    }

    /// Normalizes path components and verifies that `child` is strictly located within `parent`.
    /// Protects against directory traversal attacks (e.g. `..` escapes or symlink manipulation).
    #[must_use]
    pub fn is_safe_subpath(parent: &Path, child: &Path) -> bool {
        // Try canonicalize if both paths exist on disk
        if parent.exists() && child.exists() {
            if let (Ok(canon_parent), Ok(canon_child)) =
                (parent.canonicalize(), child.canonicalize())
            {
                return canon_child.starts_with(canon_parent);
            }
        }

        // Logical normalization fallback for prospective/non-existent paths
        let normalize = |p: &Path| -> PathBuf {
            let mut normalized = PathBuf::new();
            for comp in p.components() {
                match comp {
                    Component::ParentDir => {
                        normalized.pop();
                    }
                    Component::CurDir => {}
                    c => normalized.push(c.as_os_str()),
                }
            }
            normalized
        };

        let norm_parent = normalize(parent);
        let norm_child = normalize(child);

        norm_child.starts_with(&norm_parent)
    }
}
