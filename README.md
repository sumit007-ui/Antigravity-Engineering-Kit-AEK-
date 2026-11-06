# Antigravity Engineering Kit (AEK)

[![CI](https://github.com/google-antigravity/aek/actions/workflows/ci.yml/badge.svg)](https://github.com/google-antigravity/aek/actions/workflows/ci.yml)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/License-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE)
[![Rust 2024](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)

**Antigravity Engineering Kit (AEK)** is a high-performance, modular, cross-platform AI engineering toolkit written in Rust. AEK integrates with Google Antigravity and modern coding agents to provide global engineering rules, reusable agent skills, MCP server integration, prompt optimization, and automated validation with **ruthless precision**.

---

## Key Features

- **Rust-First Architecture**: Built for memory efficiency (<50 MB idle footprint), deterministic behavior, and zero shell-injection risks.
- **Language Toolchain Engine (`aek language`)**: Compilers, SDKs, linters, and language servers across 8 major ecosystems (Rust, Go, C/C++, Zig, Python, TypeScript, Flutter, ROS 2).
- **Project-Aware Validation Engine (`aek validate`)**: Standardized 8-stage verification pipeline (Formatting, Linting, Type Checking, Compilation, Testing) with dry-run support and structured JSON reporting.
- **Model Context Protocol Engine (`aek mcp`)**: Complete JSON-RPC 2.0 client engine with Stdio and Streamable HTTP transports, dynamic tool discovery, permission classification, and lifecycle supervision.
- **5 Verified MCP Integrations**: Presets and verified configurations for Context7, GitHub, Playwright, Sequential Thinking, and Filesystem.
- **Global & Project Engineering Rules (`aek rules`)**: Tiered precedence system (Global -> Workspace -> Project) with size validation (<=24 KB) and token budgeting.
- **15 Specialized Agent Skills (`aek skills`)**: Progressive-disclosure runbooks covering Rust, Go, C++, Zig, Python, Flutter, Robotics (ROS 2), OS Internals, DevOps, Security Auditing, Databases, Distributed Systems, and Prompt Engineering.
- **Project-Aware Skill Discovery**: Automatic codebase inspection recommending targeted skills with confidence scoring based on repository manifests.
- **Extensible Agent Adapters (`aek agent`)**: Native integration with Google Antigravity runtime, directories, and capabilities.
- **Resource Profiles (`aek profile`)**: Three calibrated tiers: `LIGHT` (minimal, 8 GB laptops), `ENGINEERING` (MCP & compilers), and `FULL` (DSPy & security).
- **Environment Doctor (`aek doctor`)**: Non-destructive diagnostic inspection of toolchains, runtimes, and Antigravity readiness.
- **Validated Configuration (`aek config`)**: TOML-backed configuration with strict schema validation, unknown-field rejection, and atomic writes.
- **Backup & Restore Engine (`aek backup`, `aek restore`)**: Unique-ID timestamped backups with safety copies prior to destructive modifications.

---

## Workspace Structure

The workspace is organized into modular Rust crates:

- [`crates/aek-platform`](crates/aek-platform): OS & arch detection, standard directory resolution, path traversal defense.
- [`crates/aek-core`](crates/aek-core): Safe execution engine, diagnostics runner, and resource profile tiers.
- [`crates/aek-config`](crates/aek-config): Schema validation, TOML persistence, atomic writes, and backup/restore engine.
- [`crates/aek-rules`](crates/aek-rules): Global and project rule models, registry, precedence resolution, validation, and safe installation.
- [`crates/aek-skills`](crates/aek-skills): Skill models, frontmatter parsing, project-aware discovery, validation, and installation.
- [`crates/aek-agents`](crates/aek-agents): Extensible agent adapter trait, Antigravity integration, and native MCP config exporter.
- [`crates/aek-mcp`](crates/aek-mcp): Reusable MCP client engine, Stdio/HTTP transports, lifecycle state machine, and security policy.
- [`crates/aek-mcp-integrations`](crates/aek-mcp-integrations): Official MCP server configuration adapters (Context7, GitHub, Playwright, Sequential Thinking, Filesystem).
- [`crates/aek-languages`](crates/aek-languages): Language provider interface, toolchain registry, project detection, and diagnostics.
- [`crates/aek-validation`](crates/aek-validation): 8-stage verification pipeline, execution sandbox, and structured reporting.
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
# Engineering Rules Management
aek rules list
aek rules show engineering
aek rules validate
aek rules install engineering --dry-run
aek rules install engineering

# Specialized Agent Skills
aek skills list
aek skills show rust-systems-engineer
aek skills validate
aek skills recommend
aek skills install rust-systems-engineer --dry-run
aek skills install rust-systems-engineer

# Model Context Protocol (MCP) Management
aek mcp list
aek mcp show context7
aek mcp doctor
aek mcp export --agent antigravity --dry-run
aek mcp export --agent antigravity
aek mcp enable context7

# Agent Integration & Capabilities
aek agent detect
aek agent capabilities
aek agent doctor
aek agent install-rules --dry-run
aek agent install-skills --dry-run

# Environment & Resource Profiles
aek doctor
aek profile list
aek profile apply engineering

# Configuration & Backups
aek config show
aek config validate
aek backup create
aek backup list
```

---

## Documentation

- [Google Antigravity Compatibility Guide](docs/compatibility/antigravity.md)
- [MCP Architecture & Engine](docs/mcp-architecture.md)
- [MCP Security & Permission Policies](docs/mcp-security.md)
- [MCP Standard Integrations](docs/mcp-integrations.md)
- [MCP Troubleshooting Guide](docs/mcp-troubleshooting.md)
- [Engineering Rules Overview](docs/rules.md)
- [Specialized Agent Skills](docs/skills.md)
- [Skill Authoring Guide](docs/skill-authoring.md)
- [CLI Reference Manual](docs/cli-reference.md)
- [Security Architecture & Invariants](docs/security.md)

---

## Development & Verification

AEK adheres strictly to **Test-Driven Development (TDD)** and automated verification:

```bash
# Code formatting check
cargo fmt --all -- --check

# Strict linting (zero warnings permitted)
cargo clippy --workspace --all-targets -- -D warnings

# Execute entire test suite
cargo test --workspace

# Build optimized release binary
cargo build --workspace --release
```

---

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
