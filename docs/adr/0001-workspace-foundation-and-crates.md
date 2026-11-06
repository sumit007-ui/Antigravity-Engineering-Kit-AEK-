# ADR 0001: Workspace Foundation and Modular Crate Architecture

## Status
Accepted

## Context
The Antigravity Engineering Kit (AEK) is a cross-platform, modular AI engineering toolkit designed to operate across diverse environments while integrating seamlessly with Google Antigravity and modern coding agents. The project requires:
- High performance, low memory footprint (target ≤ 8 GB RAM machines).
- Robust type safety and deterministic execution policies (`execution_mode: ruthless_precision`).
- Safe subprocess execution with zero command injection vulnerability.
- Atomic configuration writes to prevent data corruption.
- Separation of concerns across modular crates (`aek-core`, `aek-config`, `aek-cli`, and future crates for MCP, rules, skills, and validation).

## Decision
1. **Toolchain & Edition**: Standardize on stable Rust (Rust 2024 edition).
2. **Cargo Workspace**: Split functionality into discrete, loosely-coupled crates:
   - `crates/aek-core`: Shared abstractions, system diagnostics (`doctor`), and execution policies.
   - `crates/aek-config`: Configuration parsing, TOML serialization, validation constraints, and atomic persistence.
   - `crates/aek-cli`: User-facing binary (`aek`), command parsing with `clap`, and output formatting.
3. **Subprocess Safety**: Enforce direct process invocation via `std::process::Command` without shell wrappers (`sh -c` or `cmd.exe`) to prevent shell injection vulnerabilities.
4. **Configuration Safety**: Use atomic file replacement (`tempfile::NamedTempFile` persisted within the same directory volume) and explicit schema validation before writing.
5. **Diagnostics**: Provide both human-readable and machine-readable (`--json`) diagnostic reports via `aek doctor` to verify host prerequisites.

## Consequences
- High maintainability and clean boundary enforcement.
- Strict compile-time safety and zero unsafe code in foundation crates.
- Seamless extension for MCP and agent tooling in subsequent phases.

