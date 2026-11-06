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

    /// Manage and inspect global and project-specific engineering rules.
    Rules(RulesArgs),

    /// Discover, inspect, validate, and install specialized agent skills.
    Skills(SkillsArgs),

    /// Antigravity and AI agent integration, capability discovery, and installation.
    Agent(AgentArgs),

    /// Model Context Protocol (MCP) server lifecycle, discovery, and tool management.
    Mcp(McpArgs),

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

/// Arguments for `aek rules`.
#[derive(Args, Debug)]
pub struct RulesArgs {
    #[command(subcommand)]
    pub action: RulesSubcommand,
}

/// Subcommands under `aek rules`.
#[derive(Subcommand, Debug)]
pub enum RulesSubcommand {
    /// List available engineering rules (global and project scopes).
    List {
        /// Format output as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Show contents and metadata for a specific rule.
    Show {
        /// Name of the rule to inspect.
        name: String,
    },

    /// Validate rule files against schema and Antigravity constraints.
    Validate {
        /// Optional directory containing rules to validate.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Install a rule into an agent or workspace rules directory.
    Install {
        /// Name of the rule to install.
        name: String,

        /// Target directory for installation (defaults to active agent rules directory).
        #[arg(short, long)]
        target: Option<PathBuf>,

        /// Preview installation without modifying the filesystem.
        #[arg(long)]
        dry_run: bool,
    },

    /// Enable a rule in configuration.
    Enable {
        /// Name of the rule to enable.
        name: String,
    },

    /// Disable a rule in configuration.
    Disable {
        /// Name of the rule to disable.
        name: String,
    },

    /// Remove an installed rule.
    Remove {
        /// Name of the rule to remove.
        name: String,

        /// Target directory from which to remove rule.
        #[arg(short, long)]
        target: Option<PathBuf>,

        /// Preview removal without modifying the filesystem.
        #[arg(long)]
        dry_run: bool,
    },
}

/// Arguments for `aek skills`.
#[derive(Args, Debug)]
pub struct SkillsArgs {
    #[command(subcommand)]
    pub action: SkillsSubcommand,
}

/// Subcommands under `aek skills`.
#[derive(Subcommand, Debug)]
pub enum SkillsSubcommand {
    /// List all discovered or bundled agent skills.
    List {
        /// Format output as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Show detailed metadata and instructions for a specific skill.
    Show {
        /// Name of the skill to inspect.
        name: String,
    },

    /// Validate skill definitions against schema and Antigravity constraints.
    Validate {
        /// Optional directory containing skills to validate.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Recommend relevant skills based on project files in a directory.
    Recommend {
        /// Directory of project to analyze (defaults to current working directory).
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Install an agent skill into target directory or agent environment.
    Install {
        /// Name of the skill to install.
        name: String,

        /// Target directory for installation (defaults to active agent skills directory).
        #[arg(short, long)]
        target: Option<PathBuf>,

        /// Preview installation without modifying the filesystem.
        #[arg(long)]
        dry_run: bool,
    },

    /// Remove an installed agent skill.
    Remove {
        /// Name of the skill to remove.
        name: String,

        /// Target directory from which to remove skill.
        #[arg(short, long)]
        target: Option<PathBuf>,

        /// Preview removal without modifying the filesystem.
        #[arg(long)]
        dry_run: bool,
    },
}

/// Arguments for `aek agent`.
#[derive(Args, Debug)]
pub struct AgentArgs {
    #[command(subcommand)]
    pub action: AgentSubcommand,
}

/// Subcommands under `aek agent`.
#[derive(Subcommand, Debug)]
pub enum AgentSubcommand {
    /// Detect active AI agent environment and paths.
    Detect,

    /// Display supported capabilities and limits for detected agent.
    Capabilities,

    /// Install global engineering rules into agent configuration.
    InstallRules {
        /// Preview installation without modifying filesystem.
        #[arg(long)]
        dry_run: bool,
    },

    /// Install bundled engineering skills into agent configuration.
    InstallSkills {
        /// Preview installation without modifying filesystem.
        #[arg(long)]
        dry_run: bool,
    },

    /// Inspect agent integration health and configuration validity.
    Doctor,
}

/// Arguments for `aek mcp`.
#[derive(Args, Debug)]
pub struct McpArgs {
    #[command(subcommand)]
    pub action: McpSubcommand,
}

/// Subcommands under `aek mcp`.
#[derive(Subcommand, Debug)]
pub enum McpSubcommand {
    /// List all registered and discovered MCP servers.
    List {
        /// Format output as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Show detailed configuration and transport settings for a server.
    Show {
        /// Unique server identifier.
        name: String,
    },

    /// Register a new MCP server from a configuration TOML file.
    Register {
        /// Path to the server TOML configuration file.
        config: PathBuf,
    },

    /// Remove an MCP server from the registry.
    Remove {
        /// Server identifier to remove.
        name: String,
    },

    /// Enable a registered MCP server.
    Enable {
        /// Server identifier to enable.
        name: String,
    },

    /// Disable an MCP server without removing its configuration.
    Disable {
        /// Server identifier to disable.
        name: String,
    },

    /// Establish connection and perform handshake with an MCP server.
    Connect {
        /// Server identifier to connect.
        name: String,
    },

    /// Disconnect and terminate a running MCP server session.
    Disconnect {
        /// Server identifier to disconnect.
        name: String,
    },

    /// Inspect toolchain dependencies, credentials, and health for MCP servers.
    Doctor {
        /// Optional specific server identifier to audit.
        #[arg(short, long)]
        server: Option<String>,

        /// Format diagnostic output as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Discover and display declared protocol capabilities for a server.
    Capabilities {
        /// Server identifier to query.
        name: String,
    },

    /// List all discovered tools offered by an MCP server.
    Tools {
        /// Server identifier to query.
        name: String,

        /// Format output as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Install an MCP server package into target agent or platform environment.
    Install {
        /// Server identifier to install.
        name: String,

        /// Preview installation without modifying filesystem.
        #[arg(long)]
        dry_run: bool,
    },

    /// Uninstall or unregister an MCP server package.
    Uninstall {
        /// Server identifier to uninstall.
        name: String,

        /// Preview removal without modifying filesystem.
        #[arg(long)]
        dry_run: bool,
    },

    /// Export MCP server definitions to agent configuration format (e.g. Google Antigravity).
    Export {
        /// Target agent platform identifier (e.g. `antigravity`).
        #[arg(short, long)]
        agent: String,

        /// Destination path for exported configuration (defaults to agent's native path).
        #[arg(short, long)]
        target: Option<PathBuf>,

        /// Preview exported configuration without writing to disk.
        #[arg(long)]
        dry_run: bool,
    },

    /// Invoke a specific MCP tool with JSON arguments.
    Call {
        /// Server providing the tool.
        server: String,

        /// Name of tool to execute.
        tool: String,

        /// JSON argument string (e.g. '{"query": "rust"}').
        #[arg(short, long)]
        input: Option<String>,
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
