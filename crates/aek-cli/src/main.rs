//! Antigravity Engineering Kit (AEK) - CLI entrypoint.

mod args;
mod config_cmd;
mod doctor_cmd;

use args::{Cli, Commands};
use clap::{CommandFactory, Parser};
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
        Some(Commands::Doctor(args)) => doctor_cmd::execute_doctor(args),
        Some(Commands::Config(args)) => config_cmd::execute_config(&args.action),
        None => {
            // When run without arguments, print helpful overview
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
            println!();
            ExitCode::SUCCESS
        }
    }
}
