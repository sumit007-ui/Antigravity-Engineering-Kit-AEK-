# Antigravity Engineering Kit (AEK)

[![CI](https://github.com/google-antigravity/aek/actions/workflows/ci.yml/badge.svg)](https://github.com/google-antigravity/aek/actions/workflows/ci.yml)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/License-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE)
[![Rust 2024](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)

**Antigravity Engineering Kit (AEK)** is a high-performance, modular, cross-platform AI engineering toolkit written in Rust. AEK integrates with Google Antigravity and modern coding agents to provide global engineering rules, reusable agent skills, MCP server integration, prompt optimization, and automated validation with **ruthless precision**.

---

## Key Features

- **Rust-First Architecture**: Built for memory efficiency (<50 MB idle footprint), deterministic behavior, and zero shell-injection risks.
- **Environment Doctor (`aek doctor`)**: Inspects host toolchains (Rust, Git, Node.js, Python), platform specs, and Antigravity paths.
- **Validated Configuration (`aek config`)**: TOML-backed configuration with strict schema validation and atomic writes.
- **Agent Agnostic**: Seamless integration with Google Antigravity while maintaining architectural independence.

---

## Workspace Structure

The workspace is organized into modular Rust crates:

- [`crates/aek-core`](crates/aek-core): Shared abstractions, system diagnostics, and execution policies.
- [`crates/aek-config`](crates/aek-config): Schema validation, TOML persistence, and atomic writes.
- [`crates/aek-cli`](crates/aek-cli): Command-line interface (`aek`).

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

# Run diagnostic check
cargo run -- doctor
```

### CLI Usage

```bash
# Check version
aek --version

# Run diagnostic environment check
aek doctor

# Output diagnostic report in JSON
aek doctor --json

# Initialize a new project configuration
aek config init

# Validate configuration
aek config validate
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
```

---

## Documentation

- [Architecture Decision Records (ADRs)](docs/adr/)
- [Contributing Guide](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

---

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
