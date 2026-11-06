//! Command-line argument definitions for AEK CLI.

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

/// Available CLI subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Diagnose development environment, toolchains, and Antigravity paths.
    Doctor(DoctorArgs),

    /// Manage and validate AEK configuration files.
    Config(ConfigArgs),
}

/// Arguments for `aek doctor`.
#[derive(Args, Debug)]
pub struct DoctorArgs {
    /// Emit diagnostic report in machine-readable JSON format.
    #[arg(long)]
    pub json: bool,

    /// Target workspace path to inspect for customizations.
    #[arg(short, long)]
    pub workspace: Option<PathBuf>,
}

/// Subcommands and arguments for `aek config`.
#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigSubcommand,
}

/// Configuration operations.
#[derive(Subcommand, Debug)]
pub enum ConfigSubcommand {
    /// Validate an existing configuration file against schema and constraints.
    Validate {
        /// Explicit path to configuration file (defaults to discovering aek.toml).
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Initialize a new default `aek.toml` configuration file.
    Init {
        /// Destination path for configuration file (defaults to ./aek.toml).
        #[arg(short, long, default_value = "aek.toml")]
        path: PathBuf,

        /// Overwrite file if it already exists.
        #[arg(short, long)]
        force: bool,
    },
}
