# Antigravity Engineering Kit (AEK)

[![CI](https://github.com/google-antigravity/aek/actions/workflows/ci.yml/badge.svg)](https://github.com/google-antigravity/aek/actions/workflows/ci.yml)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/License-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE)
[![Rust 2024](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)

**Antigravity Engineering Kit (AEK)** is a high-performance, modular, cross-platform AI engineering toolkit written in Rust. AEK integrates with Google Antigravity and modern coding agents to provide global engineering rules, reusable agent skills, MCP server integration, prompt optimization, and automated validation with **ruthless precision**.

---

## Key Features

- **Rust-First Architecture**: Built for memory efficiency (<50 MB idle footprint), deterministic behavior, and zero shell-injection risks.
- **Platform Foundation**: Standard cross-platform path resolution, directory traversal prevention, and OS/arch detection.
- **Resource Profiles**: Three calibrated tiers: `LIGHT` (minimal, 8 GB laptops), `ENGINEERING` (MCP & compilers), and `FULL` (DSPy & security).
- **Environment Doctor (`aek doctor`)**: Non-destructive diagnostic inspection of toolchains, runtimes, and Antigravity readiness.
- **Validated Configuration (`aek config`)**: TOML-backed configuration with strict schema validation, unknown-field rejection, and atomic writes.
- **Backup & Restore Engine (`aek backup`, `aek restore`)**: Unique-ID timestamped backups with safety copies prior to destructive modifications.
- **Safe Subprocess Engine**: Direct argument-array process execution with timeout cleanup and secret credential redaction.

---

## Workspace Structure

The workspace is organized into modular Rust crates:

- [`crates/aek-platform`](crates/aek-platform): OS & arch detection, standard directory resolution, path traversal defense.
- [`crates/aek-core`](crates/aek-core): Safe execution engine, diagnostics runner, and resource profile tiers.
- [`crates/aek-config`](crates/aek-config): Schema validation, TOML persistence, atomic writes, and backup/restore engine.
- [`crates/aek-cli`](crates/aek-cli): User-facing command-line interface (`aek`).

---

## Quickstart

### Prerequisites

- Stable Rust toolchain (`rustc >= 1.85.0`, `cargo`)
- `git`

### Build and Install

```bash
# Clone the repository
git clone https://github.com/google-antigravity/aek.git
cd aek

# Build release binary
cargo build --release

# Initialize AEK
cargo run -- init

# Run diagnostic check
cargo run -- doctor
```

### CLI Command Highlights

```bash
# Initialize AEK (idempotent, dry-run, or force with backup)
aek init
aek init --dry-run
aek init --force

# Inspect environment
aek doctor
aek doctor --json
aek doctor --verbose

# Manage configuration
aek config show
aek config validate
aek config path

# Resource profiles
aek profile list
aek profile show engineering
aek profile apply engineering --dry-run
aek profile apply engineering

# Backups and restoration
aek backup
aek backup list
aek restore <backup-id> --dry-run
aek restore <backup-id>
```

---

## Development & Verification

AEK adheres strictly to **Test-Driven Development (TDD)** and automated verification:

```bash
# Code formatting check
cargo fmt --all -- --check

# Strict linting (zero warnings permitted)
cargo clippy --workspace --all-targets -- -D warnings

# Execute test suite
cargo test --workspace

# Build release artifacts
cargo build --workspace --release
```

---

## Documentation

- [Architecture Guide](docs/architecture.md)
- [Configuration Reference](docs/configuration.md)
- [CLI Command Reference](docs/cli-reference.md)
- [Architecture Decision Records (ADRs)](docs/adr/)
- [Contributing Guide](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

---

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
