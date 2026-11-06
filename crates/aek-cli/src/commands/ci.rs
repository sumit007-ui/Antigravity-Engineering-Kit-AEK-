//! CLI command handler for `aek ci`.

use crate::cli::{CiArgs, CiSubcommand};
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const GITHUB_CI_WORKFLOW: &str = r#"name: AEK Validation CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

permissions:
  contents: read

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"

jobs:
  validate:
    name: AEK Multi-Stage Validation (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, windows-latest]
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Set up Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Verify Formatting
        run: cargo fmt --all -- --check

      - name: Run Clippy Linter
        run: cargo clippy --workspace --all-targets -- -D warnings

      - name: Execute Test Suite
        run: cargo test --workspace --verbose

      - name: Build Release Binary
        run: cargo build --workspace --release
"#;

const GITHUB_SECURITY_WORKFLOW: &str = r#"name: AEK Security Scan

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 4 * * 1'

permissions:
  contents: read
  security-events: write

jobs:
  security-audit:
    name: Security & Vulnerability Audit
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Set up Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-audit
        run: cargo install cargo-audit --locked

      - name: Run Cargo Audit
        run: cargo audit
"#;

/// Entry point from `main` returning `ExitCode`.
pub fn execute_ci(args: &CiArgs) -> ExitCode {
    match handle_ci(args.clone()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}

/// Executes CI subcommands.
pub fn handle_ci(args: CiArgs) -> Result<()> {
    match args.action {
        CiSubcommand::Generate {
            platform,
            target,
            dry_run,
        } => {
            if platform.to_lowercase() != "github" {
                bail!("Unsupported CI platform: '{platform}'. Currently supported: 'github'.");
            }

            let workflows_dir = target.unwrap_or_else(|| PathBuf::from(".github/workflows"));

            if dry_run {
                println!(
                    "\n[Dry Run] Would generate GitHub Actions workflows in '{}':",
                    workflows_dir.display()
                );
                println!("  - aek-ci.yml");
                println!("  - aek-security.yml\n");
                println!("=== aek-ci.yml Preview ===\n{GITHUB_CI_WORKFLOW}");
                println!("=== aek-security.yml Preview ===\n{GITHUB_SECURITY_WORKFLOW}\n");
            } else {
                fs::create_dir_all(&workflows_dir).with_context(|| {
                    format!("Failed to create directory '{}'", workflows_dir.display())
                })?;

                let ci_path = workflows_dir.join("aek-ci.yml");
                let sec_path = workflows_dir.join("aek-security.yml");

                fs::write(&ci_path, GITHUB_CI_WORKFLOW)?;
                fs::write(&sec_path, GITHUB_SECURITY_WORKFLOW)?;

                println!("\nSuccessfully generated GitHub Actions workflows:");
                println!("  - {}", ci_path.display());
                println!("  - {}\n", sec_path.display());
            }
        }
        CiSubcommand::Validate { target } => {
            let workflows_dir = target.unwrap_or_else(|| PathBuf::from(".github/workflows"));

            if !workflows_dir.is_dir() {
                bail!(
                    "Workflows directory '{}' does not exist.",
                    workflows_dir.display()
                );
            }

            println!("\nAuditing CI Workflows in '{}':", workflows_dir.display());
            println!("{:-<60}", "");

            let mut all_valid = true;
            for entry in fs::read_dir(&workflows_dir)?.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("yml")
                    || path.extension().and_then(|e| e.to_str()) == Some("yaml")
                {
                    let content = fs::read_to_string(&path)?;
                    let has_permissions = content.contains("permissions:");
                    let has_pull_request_write = content.contains("pull_request:")
                        && content.contains("pull-requests: write");

                    let status = if !has_permissions {
                        all_valid = false;
                        "[WARN] Missing explicit `permissions:` block (violates least-privilege)"
                    } else if has_pull_request_write {
                        all_valid = false;
                        "[WARN] Dangerous `pull-requests: write` in pull_request trigger"
                    } else {
                        "[PASS] Valid least-privilege permissions"
                    };

                    println!(
                        "Workflow {:<24} : {}",
                        path.file_name().unwrap().to_string_lossy(),
                        status
                    );
                }
            }

            println!("{:-<60}\n", "");
            if !all_valid {
                bail!("One or more CI workflows violated security standards.");
            }
            println!("All CI workflows passed security validation.\n");
        }
    }
    Ok(())
}
