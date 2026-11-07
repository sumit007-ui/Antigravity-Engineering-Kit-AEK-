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

    /// Language toolchain management, project detection, and diagnostics.
    Language(LanguageArgs),

    /// Project-aware validation pipeline execution across detected language projects.
    Validate(ValidateArgs),

    /// Security vulnerability scanning, Semgrep SAST, and dependency auditing.
    Security(SecurityArgs),

    /// Inspect and export structured validation and security reports.
    Report(ReportArgs),

    /// Continuous Integration (CI/CD) workflow generation and security validation.
    Ci(CiArgs),

    /// AI prompt optimization, evaluation, dataset benchmarking, and versioning.
    Prompt(PromptArgs),

    /// Create or list verified configuration backups.
    Backup(BackupArgs),

    /// Restore configuration from a verified backup identifier.
    Restore(RestoreArgs),

    /// Check for and apply version updates to the AEK installation.
    Update(UpdateArgs),

    /// Revert AEK to a previously archived version.
    Rollback(RollbackArgs),

    /// Safe uninstallation of AEK-managed files and binaries.
    Uninstall(UninstallArgs),

    /// Installation status, diagnostics, and integrity verification.
    Install(InstallArgs),

    /// Configure and setup AI coding agent environments (e.g. Antigravity).
    Setup(SetupArgs),
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

    /// Check, preview, and apply versioned configuration schema migrations.
    Migrate {
        /// Check if migrations are pending without applying.
        #[arg(long)]
        check: bool,

        /// Preview migration steps without modifying configuration.
        #[arg(long)]
        dry_run: bool,

        /// Optional path to configuration file.
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
    /// List all supported AI coding agents.
    List,

    /// Detect active AI agent environment and paths.
    Detect,

    /// Display supported capabilities and limits for detected or specified agent.
    Capabilities {
        /// Optional target agent name (e.g. antigravity, claude-code, opencode, cursor).
        agent: Option<String>,
    },

    /// Inspect agent integration health and configuration validity.
    Doctor,

    /// Setup and configure a target agent.
    Setup {
        /// Target agent name (antigravity, claude-code, opencode, cursor).
        agent: String,

        /// Preview installation without writing files to disk.
        #[arg(long)]
        dry_run: bool,

        /// Optional profile name to apply.
        #[arg(long)]
        profile: Option<String>,
    },

    /// Synchronize AEK rules, skills, and MCP configurations across agents.
    Sync {
        /// Synchronize all supported agents.
        #[arg(long)]
        all: bool,

        /// Specific target agent to synchronize.
        #[arg(long)]
        agent: Option<String>,

        /// Preview synchronization actions without modifying disk.
        #[arg(long)]
        dry_run: bool,

        /// Conflict resolution strategy (keep-existing, replace-managed, skip, manual-resolution).
        #[arg(long, default_value = "replace-managed")]
        conflict: String,
    },

    /// Inspect, validate, or export agent native configurations.
    Config {
        #[command(subcommand)]
        action: AgentConfigSubcommand,
    },

    /// Manage and apply multi-agent profiles.
    Profile {
        #[command(subcommand)]
        action: AgentProfileSubcommand,
    },

    /// Safely uninstall AEK-managed files from an agent.
    Uninstall {
        /// Target agent name to remove managed files for.
        agent: String,

        /// Preview removal without deleting files.
        #[arg(long)]
        dry_run: bool,
    },

    /// Display multi-agent compatibility report and matrix.
    Compatibility {
        /// Optional target agent name to filter report.
        agent: Option<String>,
    },

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
}

/// Subcommands for `aek agent config`.
#[derive(Subcommand, Debug)]
pub enum AgentConfigSubcommand {
    /// Show current agent configuration files and locations.
    Show {
        /// Target agent name.
        agent: String,
    },
    /// Validate current agent configuration syntax and presence.
    Validate {
        /// Target agent name.
        agent: String,
    },
    /// Export translated agent native configuration files to standard output.
    Export {
        /// Target agent name.
        agent: String,
    },
}

/// Subcommands for `aek agent profile`.
#[derive(Subcommand, Debug)]
pub enum AgentProfileSubcommand {
    /// List all available agent profiles.
    List,
    /// Show details for a specific profile.
    Show {
        /// Profile name or slug.
        profile: String,
    },
    /// Apply an agent profile to active workspace.
    Apply {
        /// Profile name or slug.
        profile: String,
        /// Preview without modifying filesystem.
        #[arg(long)]
        dry_run: bool,
    },
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

/// Arguments for `aek language`.
#[derive(Args, Debug, Clone)]
pub struct LanguageArgs {
    #[command(subcommand)]
    pub command: LanguageSubcommand,
}

/// Subcommands under `aek language`.
#[derive(Subcommand, Debug, Clone)]
pub enum LanguageSubcommand {
    /// List all supported language ecosystems.
    List,

    /// Detect language projects in current or specified workspace.
    Detect(LanguageDetectArgs),

    /// Audit language toolchains and environment health.
    Doctor(LanguageDoctorArgs),

    /// Inspect standard tools registered for a specific language.
    Tools(LanguageToolsArgs),

    /// Check toolchain and project compatibility for a language.
    Compatibility(LanguageCompatibilityArgs),

    /// Generate or inspect safe installation plan for missing tools.
    Setup(LanguageSetupArgs),

    /// Execute validation commands for a specific language.
    Validate(LanguageValidateArgs),
}

/// Arguments for `aek language detect`.
#[derive(Args, Debug, Clone)]
pub struct LanguageDetectArgs {
    /// Directory path to scan (defaults to current working directory).
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Emit machine-readable JSON output.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `aek language doctor`.
#[derive(Args, Debug, Clone)]
pub struct LanguageDoctorArgs {
    /// Specific language to audit (e.g. rust, go, cpp, zig, python, typescript, flutter, ros2).
    pub language: Option<String>,

    /// Emit machine-readable JSON output.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `aek language tools`.
#[derive(Args, Debug, Clone)]
pub struct LanguageToolsArgs {
    /// Language identifier (e.g. rust, go, cpp, zig, python, typescript, flutter, ros2).
    pub language: String,
}

/// Arguments for `aek language compatibility`.
#[derive(Args, Debug, Clone)]
pub struct LanguageCompatibilityArgs {
    /// Language identifier to evaluate.
    pub language: String,

    /// Path to project root (defaults to current working directory).
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

/// Arguments for `aek language setup`.
#[derive(Args, Debug, Clone)]
pub struct LanguageSetupArgs {
    /// Language identifier (e.g. rust, go, cpp, zig, python, typescript, flutter, ros2).
    pub language: String,

    /// Preview planned installation commands without executing.
    #[arg(long)]
    pub dry_run: bool,
}

/// Arguments for `aek language validate`.
#[derive(Args, Debug, Clone)]
pub struct LanguageValidateArgs {
    /// Language identifier to validate.
    pub language: String,

    /// Path to project root (defaults to current working directory).
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Preview commands without executing.
    #[arg(long)]
    pub dry_run: bool,

    /// Emit machine-readable JSON output.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `aek validate`.
#[derive(Args, Debug, Clone)]
pub struct ValidateArgs {
    #[command(subcommand)]
    pub action: Option<ValidateSubcommand>,

    /// Target project directory (defaults to current working directory).
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Run full validation including extended stages.
    #[arg(long)]
    pub full: bool,

    /// Run incremental validation on git-modified files only.
    #[arg(long)]
    pub changed: bool,

    /// Optional Git base reference for change comparison (e.g. main).
    #[arg(long)]
    pub base: Option<String>,

    /// Validation policy to enforce (default, strict, ci, or path to TOML).
    #[arg(long)]
    pub policy: Option<String>,

    /// Preview validation commands without execution.
    #[arg(long)]
    pub dry_run: bool,

    /// Emit machine-readable JSON output.
    #[arg(long)]
    pub json: bool,
}

/// Subcommands under `aek validate`.
#[derive(Subcommand, Debug, Clone)]
pub enum ValidateSubcommand {
    /// Inspect or manage validation policies.
    Policy {
        #[command(subcommand)]
        action: ValidatePolicySubcommand,
    },
}

/// Subcommands under `aek validate policy`.
#[derive(Subcommand, Debug, Clone)]
pub enum ValidatePolicySubcommand {
    /// List available built-in validation policies.
    List,

    /// Show TOML contents of a specific validation policy.
    Show {
        /// Policy name (default, strict, ci).
        name: String,
    },

    /// Validate a policy TOML file against schema.
    Check {
        /// Path to policy file.
        file: PathBuf,
    },
}

/// Arguments for `aek security`.
#[derive(Args, Debug, Clone)]
pub struct SecurityArgs {
    #[command(subcommand)]
    pub action: SecuritySubcommand,
}

/// Subcommands under `aek security`.
#[derive(Subcommand, Debug, Clone)]
pub enum SecuritySubcommand {
    /// Run security scans across the repository.
    Scan(SecurityScanArgs),

    /// Audit security toolchains and scanner availability on PATH.
    Doctor,
}

/// Arguments for `aek security scan`.
#[derive(Args, Debug, Clone)]
pub struct SecurityScanArgs {
    /// Target directory to scan (defaults to current directory).
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Run Semgrep static analysis rules.
    #[arg(long)]
    pub semgrep: bool,

    /// Run Trivy filesystem vulnerability scan.
    #[arg(long)]
    pub trivy: bool,

    /// Run dependency vulnerability audit across detected lockfiles.
    #[arg(long)]
    pub dependencies: bool,

    /// Output findings as machine-readable JSON.
    #[arg(long)]
    pub json: bool,

    /// Output findings in OASIS SARIF v2.1.0 format.
    #[arg(long)]
    pub sarif: bool,
}

/// Arguments for `aek report`.
#[derive(Args, Debug, Clone)]
pub struct ReportArgs {
    #[command(subcommand)]
    pub action: ReportSubcommand,
}

/// Subcommands under `aek report`.
#[derive(Subcommand, Debug, Clone)]
pub enum ReportSubcommand {
    /// Display a stored validation or security report in terminal.
    Show {
        /// Path to the report file (JSON).
        report: PathBuf,
    },

    /// Export a report to a specified format (json, sarif, markdown).
    Export {
        /// Path to the report file (JSON).
        report: PathBuf,

        /// Format to export (json, sarif, markdown).
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file path (defaults to stdout if omitted).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

/// Arguments for `aek ci`.
#[derive(Args, Debug, Clone)]
pub struct CiArgs {
    #[command(subcommand)]
    pub action: CiSubcommand,
}

/// Subcommands under `aek ci`.
#[derive(Subcommand, Debug, Clone)]
pub enum CiSubcommand {
    /// Generate least-privilege CI workflow files for target platform (github).
    Generate {
        /// Target CI platform (default: github).
        #[arg(default_value = "github")]
        platform: String,

        /// Target directory for generated workflows (defaults to .github/workflows).
        #[arg(short, long)]
        target: Option<PathBuf>,

        /// Preview generated workflows without writing to disk.
        #[arg(long)]
        dry_run: bool,
    },

    /// Validate existing CI workflow files against least-privilege security standards.
    Validate {
        /// Target directory containing workflows (defaults to .github/workflows).
        #[arg(short, long)]
        target: Option<PathBuf>,
    },
}

/// Arguments for `aek prompt`.
#[derive(Args, Debug, Clone)]
pub struct PromptArgs {
    #[command(subcommand)]
    pub command: PromptSubcommand,
}

/// Subcommands under `aek prompt`.
#[derive(Subcommand, Debug, Clone)]
pub enum PromptSubcommand {
    /// List all registered prompt templates and versioned prompts.
    List {
        /// Emit machine-readable JSON output.
        #[arg(long)]
        json: bool,
    },

    /// Show details, metadata, and content of a prompt record.
    Show {
        /// Unique prompt identifier.
        id: String,

        /// Emit machine-readable JSON output.
        #[arg(long)]
        json: bool,
    },

    /// Create and register a new prompt record.
    Create(PromptCreateArgs),

    /// List all historical versions for a prompt.
    Versions {
        /// Unique prompt identifier.
        id: String,

        /// Emit machine-readable JSON output.
        #[arg(long)]
        json: bool,
    },

    /// Compare content diff between prompt versions.
    Diff {
        /// Unique prompt identifier.
        id: String,

        /// First version (defaults to earlier version).
        #[arg(long)]
        ver_a: Option<String>,

        /// Second version (defaults to latest version).
        #[arg(long)]
        ver_b: Option<String>,
    },

    /// Roll back a prompt to a previous version.
    Rollback {
        /// Unique prompt identifier.
        id: String,

        /// Target version to restore.
        version: String,
    },

    /// Optimize a prompt using the configured prompt optimization adapter.
    Optimize(PromptOptimizeArgs),

    /// Evaluate a prompt against an evaluation dataset.
    Evaluate(PromptEvaluateArgs),

    /// Run benchmark comparison for a prompt.
    Benchmark(PromptBenchmarkArgs),

    /// Compare evaluation results between two prompt versions.
    Compare(PromptCompareArgs),

    /// Export a prompt into an AI coding agent instruction directory.
    Export(PromptExportArgs),

    /// Audit prompt optimization and evaluation toolchain environment.
    Doctor,

    /// List available prompt optimization adapters and their operational statuses.
    Adapters,
}

/// Arguments for `aek prompt create`.
#[derive(Args, Debug, Clone)]
pub struct PromptCreateArgs {
    /// Unique prompt identifier.
    pub id: String,

    /// Human-readable title.
    #[arg(short, long)]
    pub name: Option<String>,

    /// Description of task purpose.
    #[arg(short, long)]
    pub description: Option<String>,

    /// Path to file containing prompt text (reads from stdin if omitted).
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

/// Arguments for `aek prompt optimize`.
#[derive(Args, Debug, Clone)]
pub struct PromptOptimizeArgs {
    /// Prompt identifier to optimize.
    pub id: Option<String>,

    /// Optional raw input file to optimize directly.
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Optimization adapter (prompt-optimizer, dspy, or generic).
    #[arg(short, long, default_value = "prompt-optimizer")]
    pub adapter: String,

    /// Preview optimized output without updating version record.
    #[arg(long)]
    pub dry_run: bool,

    /// Bypass memory optimization cache.
    #[arg(long)]
    pub no_cache: bool,

    /// Emit machine-readable JSON output.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `aek prompt evaluate`.
#[derive(Args, Debug, Clone)]
pub struct PromptEvaluateArgs {
    /// Prompt identifier to evaluate.
    pub id: Option<String>,

    /// Path to evaluation dataset (JSON or JSONL).
    #[arg(short, long)]
    pub dataset: Option<PathBuf>,

    /// Emit machine-readable JSON output.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `aek prompt benchmark`.
#[derive(Args, Debug, Clone)]
pub struct PromptBenchmarkArgs {
    /// Prompt identifier to benchmark.
    pub id: String,

    /// Dataset path to benchmark against.
    #[arg(short, long)]
    pub dataset: Option<PathBuf>,
}

/// Arguments for `aek prompt compare`.
#[derive(Args, Debug, Clone)]
pub struct PromptCompareArgs {
    /// Prompt identifier.
    pub id: String,

    /// First version.
    pub version_a: String,

    /// Second version.
    pub version_b: String,
}

/// Arguments for `aek prompt export`.
#[derive(Args, Debug, Clone)]
pub struct PromptExportArgs {
    /// Prompt identifier to export.
    pub id: String,

    /// Target agent identifier (default: antigravity).
    #[arg(short, long, default_value = "antigravity")]
    pub agent: String,

    /// Target output directory.
    #[arg(short, long)]
    pub target: Option<PathBuf>,

    /// Preview export destination without writing to disk.
    #[arg(long)]
    pub dry_run: bool,
}

/// Arguments for `aek update`.
#[derive(Args, Debug, Clone)]
pub struct UpdateArgs {
    #[command(subcommand)]
    pub action: Option<UpdateSubcommand>,

    /// Preview update actions without modifying installation.
    #[arg(long)]
    pub dry_run: bool,

    /// Target specific version to update or downgrade to.
    #[arg(long)]
    pub version: Option<String>,

    /// Force update even if target is a downgrade.
    #[arg(long)]
    pub force: bool,

    /// Path to a release manifest file to verify against.
    #[arg(long, global = true)]
    pub manifest: Option<PathBuf>,
}

/// Subcommands under `aek update`.
#[derive(Subcommand, Debug, Clone)]
pub enum UpdateSubcommand {
    /// Check if a newer version is available from release source.
    Check,
}

/// Arguments for `aek rollback`.
#[derive(Args, Debug, Clone)]
pub struct RollbackArgs {
    /// List all previously archived versions available for rollback.
    #[arg(long)]
    pub list: bool,

    /// Specific version to roll back to.
    #[arg(long)]
    pub version: Option<String>,

    /// Preview rollback actions without modifying executable.
    #[arg(long)]
    pub dry_run: bool,
}

/// Arguments for `aek uninstall`.
#[derive(Args, Debug, Clone)]
pub struct UninstallArgs {
    /// Preview files and directories that would be removed.
    #[arg(long)]
    pub dry_run: bool,

    /// Purge mode: delete user configuration directory in addition to binary.
    #[arg(long)]
    pub purge: bool,

    /// Bypass confirmation prompt (required for automated non-interactive purge).
    #[arg(short, long)]
    pub yes: bool,
}

/// Arguments for `aek install`.
#[derive(Args, Debug, Clone)]
pub struct InstallArgs {
    #[command(subcommand)]
    pub action: InstallSubcommand,
}

/// Subcommands under `aek install`.
#[derive(Subcommand, Debug, Clone)]
pub enum InstallSubcommand {
    /// Display installation metadata, scope, and paths.
    Info,
    /// Inspect installation integrity, permissions, and PATH configuration.
    Doctor,
    /// Verify cryptographic checksums of installed managed files.
    Verify {
        /// Expected SHA-256 hash to verify against.
        #[arg(long)]
        expected_hash: Option<String>,

        /// Optional installation manifest path to verify against.
        #[arg(long)]
        manifest: Option<std::path::PathBuf>,
    },
}

/// Arguments for `aek setup`.
#[derive(Args, Debug, Clone)]
pub struct SetupArgs {
    #[command(subcommand)]
    pub action: SetupSubcommand,
}

/// Subcommands under `aek setup`.
#[derive(Subcommand, Debug, Clone)]
pub enum SetupSubcommand {
    /// Configure Google Antigravity with AEK rules, skills, and MCP tools.
    Antigravity {
        /// Preview setup actions without writing to disk.
        #[arg(long)]
        dry_run: bool,

        /// Resource profile tier to configure (light, engineering, full).
        #[arg(long)]
        profile: Option<String>,
    },
}
