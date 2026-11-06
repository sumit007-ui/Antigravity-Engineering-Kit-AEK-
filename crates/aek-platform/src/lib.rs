//! # aek-platform
//!
//! Platform-specific discovery, operating system and architecture detection,
//! standard directory resolution, and path safety invariants for AEK.

pub mod detection;
pub mod paths;

pub use detection::{OsKind, detect_arch, detect_linux_distro, detect_os, find_executable_in_path};
pub use paths::PlatformPaths;
