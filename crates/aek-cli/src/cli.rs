//! Command-line argument parser definitions for AEK.

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Antigravity Engineering Kit (AEK) - Rust-first AI engineering toolkit.
#[derive(Parser, Debug)]
#[command(
    name = "aek",
    author = "Antigravity Engineering Kit Contributors",
    version,
    about = "Cross-platform AI engineering toolkit for Google Antigravity and modern coding agents",
    long_about = "Antigravity Engineering Kit (AEK) provides global engineering rules, \
                  agent skills, MCP server orchestration, prompt optimization, \
                  and multi-language toolchains with ruthless precision."
)]
pub struct Cli {
    /// Enable verbose diagnostic tracing output.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-essential output.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Supported AEK CLI subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize AEK directories and generate default configuration.
    Init(InitArgs),

    /// Inspect system environment, toolchains, and Antigravity readiness.
    Doctor(DoctorArgs),

    /// Manage, view, and validate AEK configuration files.
    Config(ConfigArgs),

    /// Manage and apply resource profiles (LIGHT, ENGINEERING, FULL).
    Profile(ProfileArgs),

    /// Create or list verified configuration backups.
    Backup(BackupArgs),

    /// Restore configuration from a verified backup identifier.
    Restore(RestoreArgs),
}

/// Arguments for `aek init`.
#[derive(Args, Debug)]
pub struct InitArgs {
    /// Preview filesystem operations without creating directories or files.
    #[arg(long)]
    pub dry_run: bool,

    /// Replace existing configuration after creating a verified safety backup.
    #[arg(short, long)]
    pub force: bool,

    /// Custom path for configuration file (defaults to platform standard location).
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

/// Arguments for `aek doctor`.
#[derive(Args, Debug)]
pub struct DoctorArgs {
    /// Output diagnostics report in machine-readable JSON format.
    #[arg(long)]
    pub json: bool,

    /// Workspace path to inspect for agent rules and customizations.
    #[arg(short, long)]
    pub workspace: Option<PathBuf>,
}

/// Arguments for `aek config`.
#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigSubcommand,
}

/// Subcommands under `aek config`.
#[derive(Subcommand, Debug)]
pub enum ConfigSubcommand {
    /// Display current active configuration.
    Show {
        /// Optional path to configuration file.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Validate configuration file against schema and constraints.
    Validate {
        /// Optional path to configuration file.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Print resolved configuration file path.
    Path {
        /// Optional explicit path to check.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

/// Arguments for `aek profile`.
#[derive(Args, Debug)]
pub struct ProfileArgs {
    #[command(subcommand)]
    pub action: ProfileSubcommand,
}

/// Subcommands under `aek profile`.
#[derive(Subcommand, Debug)]
pub enum ProfileSubcommand {
    /// List available resource profiles and indicate active tier.
    List {
        /// Optional path to configuration file.
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Show detailed metadata and capabilities for a specific profile.
    Show {
        /// Profile tier name (light, engineering, full).
        name: String,
    },

    /// Apply a profile tier to the configuration while preserving unrelated settings.
    Apply {
        /// Profile tier name (light, engineering, full).
        name: String,

        /// Preview changes without modifying configuration on disk.
        #[arg(long)]
        dry_run: bool,

        /// Path to configuration file to modify.
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
}

/// Arguments for `aek backup`.
#[derive(Args, Debug)]
pub struct BackupArgs {
    #[command(subcommand)]
    pub action: Option<BackupSubcommand>,

    /// Path to configuration file to back up.
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Directory where backups are stored.
    #[arg(short, long, global = true)]
    pub backup_dir: Option<PathBuf>,
}

/// Subcommands under `aek backup`.
#[derive(Subcommand, Debug)]
pub enum BackupSubcommand {
    /// Create a new configuration backup (default if no subcommand given).
    Create {
        /// Optional reason or tag for the backup.
        #[arg(short, long)]
        reason: Option<String>,
    },

    /// List all available backups.
    List,
}

/// Arguments for `aek restore`.
#[derive(Args, Debug)]
pub struct RestoreArgs {
    /// Unique backup identifier to restore.
    pub backup_id: String,

    /// Preview restoration without writing to target file.
    #[arg(long)]
    pub dry_run: bool,

    /// Target path for restoration (defaults to platform standard config file).
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Directory containing backups.
    #[arg(short, long)]
    pub backup_dir: Option<PathBuf>,
}
