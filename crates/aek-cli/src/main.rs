//! Antigravity Engineering Kit (AEK) - CLI entrypoint.

mod cli;
mod commands;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};
use std::process::ExitCode;
use tracing_subscriber::{EnvFilter, fmt};

fn init_tracing(verbose: bool, quiet: bool) {
    if quiet {
        return;
    }

    let default_filter = if verbose { "debug" } else { "info" };
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    let _ = fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .without_time()
        .try_init();
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    init_tracing(cli.verbose, cli.quiet);

    match &cli.command {
        Some(Commands::Init(args)) => commands::init::execute_init(args),
        Some(Commands::Doctor(args)) => commands::doctor::execute_doctor(args, cli.verbose),
        Some(Commands::Config(args)) => commands::config::execute_config(&args.action),
        Some(Commands::Profile(args)) => commands::profile::execute_profile(&args.action),
        Some(Commands::Rules(args)) => commands::rules::execute_rules(&args.action),
        Some(Commands::Skills(args)) => commands::skills::execute_skills(&args.action),
        Some(Commands::Agent(args)) => commands::agent::execute_agent(&args.action),
        Some(Commands::Mcp(args)) => commands::mcp::execute_mcp(&args.action),
        Some(Commands::Language(args)) => commands::language::execute_language(args),
        Some(Commands::Validate(args)) => commands::validate::execute_validate(args),
        Some(Commands::Security(args)) => commands::security::execute_security(args),
        Some(Commands::Report(args)) => commands::report::execute_report(args),
        Some(Commands::Ci(args)) => commands::ci::execute_ci(args),
        Some(Commands::Prompt(args)) => commands::prompt::execute_prompt(args),
        Some(Commands::Backup(args)) => commands::backup::execute_backup(args),
        Some(Commands::Restore(args)) => commands::restore::execute_restore(args),
        Some(Commands::Update(args)) => commands::distribution::execute_update(args),
        Some(Commands::Rollback(args)) => commands::distribution::execute_rollback(args),
        Some(Commands::Uninstall(args)) => commands::distribution::execute_uninstall(args),
        Some(Commands::Install(args)) => commands::distribution::execute_install(args),
        Some(Commands::Setup(args)) => commands::distribution::execute_setup(args),
        None => {
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
            println!();
            ExitCode::SUCCESS
        }
    }
}
