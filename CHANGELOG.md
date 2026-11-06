# Changelog

All notable changes to the Antigravity Engineering Kit (AEK) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-11-06

### Added
- Phase 4 Rust MCP Integration Framework:
  - New `aek-mcp` crate:
    - Reusable asynchronous `McpClient` implementing JSON-RPC 2.0 handshake (`protocolVersion: 2024-11-05`), capability negotiation (`tools`, `resources`, `prompts`), dynamic tool listing, and tool invocation.
    - `McpTransport` trait supporting `StdioTransport` (Tokio child process, non-blocking stderr capture, process supervision, timeouts) and `HttpTransport` (Reqwest, verified Rustls TLS, timeouts).
    - `ServerLifecycle` state machine (`DISCONNECTED`, `CONNECTING`, `INITIALIZING`, `READY`, `DEGRADED`, `FAILED`, `STOPPING`) with transition validation.
    - Typed `McpServerRegistry` with TOML configuration persistence, dynamic directory loading, validation, and enable/disable states.
    - Security engine: tool risk classification (`READ_ONLY`, `FILESYSTEM_WRITE`, `NETWORK_WRITE`, `SENSITIVE`, `UNKNOWN`), secret credential redaction (bearer tokens, GitHub PATs, passwords), and path containment verification.
    - `McpDoctor` diagnostic engine auditing executable presence on PATH, authentication environment variables, and secure endpoints.
  - New `aek-mcp-integrations` crate:
    - Built-in configuration adapters and presets for 5 standard MCP servers:
      - Context7 (`upstash/context7`)
      - GitHub (`@modelcontextprotocol/server-github`)
      - Playwright (`@playwright/mcp@latest`)
      - Sequential Thinking (`@modelcontextprotocol/server-sequential-thinking`)
      - Restricted Filesystem (`@modelcontextprotocol/server-filesystem`)
  - Integration Configurations & Schemas:
    - `integrations/mcp/registry.toml`, `context7.toml`, `github.toml`, `playwright.toml`, `sequential-thinking.toml`, `filesystem.toml`.
    - `schemas/mcp-server.schema.json`.
  - Google Antigravity MCP Adapter:
    - Extended `AntigravityAdapter` in `aek-agents` with `export_mcp_config`, `merge_mcp_config`, conflict detection, dry-run previews, and automatic safety backups of `mcp_config.json`.
  - CLI Subcommands under `aek mcp`:
    - `aek mcp list [--json]`
    - `aek mcp show <server>`
    - `aek mcp register <config>`
    - `aek mcp remove <server>`
    - `aek mcp enable <server>` / `disable <server>`
    - `aek mcp connect <server>` / `disconnect <server>`
    - `aek mcp doctor [--server <server>] [--json]`
    - `aek mcp capabilities <server>`
    - `aek mcp tools <server> [--json]`
    - `aek mcp install <server> [--dry-run]` / `uninstall <server> [--dry-run]`
    - `aek mcp export --agent antigravity [--target <path>] [--dry-run]`
    - `aek mcp call <server> <tool> [--input <json>]`
  - Documentation Suite:
    - `docs/mcp-architecture.md`, `docs/mcp-security.md`, `docs/mcp-integrations.md`, `docs/mcp-troubleshooting.md`, updated `docs/cli-reference.md` and `docs/compatibility/antigravity.md`.
  - Comprehensive Unit and Integration Test Suites:
    - `mcp_cli_tests.rs`, `mcp_client_tests.rs`, `registry_tests.rs`, `security_tests.rs`, `integrations_tests.rs`, `antigravity_mcp_tests.rs` (72 total tests passing).

## [0.3.0] - 2026-11-06

### Added
- Phase 3 Global Rules & Specialized Agent Skills:
  - New `aek-rules` crate:
    - Rule models (`Rule`, `RuleScope` for Global, Workspace, Project).
    - Rule registry discovering rules from directories.
    - Rule validator enforcing Google Antigravity constraints (<=24 KB per file).
    - Rule resolver implementing deterministic precedence resolution (Project > Workspace > Global).
    - Rule consolidation and merger capping aggregate token budgets (~20k tokens) and demoting overflowing rules to file references.
    - Rule installer supporting atomic installation, dry-run previews, and path containment validation.
  - New `aek-skills` crate:
    - Skill models (`SkillBundle`, `SkillMetadata`) with YAML frontmatter parsing (`parse_skill_markdown`).
    - Skill registry and directory loader.
    - Project-aware `SkillDiscoverer` automatically analyzing workspace roots for markers (`Cargo.toml`, `go.mod`, `CMakeLists.txt`, `build.zig`, `pyproject.toml`, `pubspec.yaml`, `package.json`, ROS 2 packages, SQL files) and recommending skills with confidence ratings.
    - Skill installer supporting safe atomic directory installation, dry-run previews, and removal.
  - New `aek-agents` crate:
    - Extensible `AgentAdapter` trait defining detection, capability discovery, and rule/skill installation.
    - Native `AntigravityAdapter` for Google Antigravity runtime, directories, and capabilities.
  - 13 Precedence-Tiered Engineering Rules:
    - Global: `engineering.md`, `security.md`, `documentation.md`, `testing.md`, `performance.md`.
    - Project: `rust.md`, `go.md`, `cpp.md`, `zig.md`, `python.md`, `typescript.md`, `flutter.md`, `robotics.md`.
  - 15 Specialized Agent Skills:
    - `rust-systems-engineer`, `go-backend-engineer`, `cpp-systems-engineer`, `zig-engineer`, `python-engineer`, `flutter-engineer`, `ai-agent-engineer`, `robotics-ros2-engineer`, `operating-system-engineer`, `devops-engineer`, `security-auditor`, `database-architect`, `distributed-systems-engineer`, `prompt-engineering-specialist`, `production-code-reviewer`.
  - Standard JSON schemas:
    - `schemas/rule.schema.json`
    - `schemas/skill.schema.json`
  - CLI Subcommands:
    - `aek rules list [--json]`, `aek rules show <rule>`, `aek rules validate`, `aek rules install <rule> [--dry-run]`, `aek rules remove <rule> [--dry-run]`, `aek rules enable/disable <rule>`.
    - `aek skills list [--json]`, `aek skills show <skill>`, `aek skills validate`, `aek skills recommend [--path <dir>]`, `aek skills install <skill> [--dry-run]`, `aek skills remove <skill> [--dry-run]`.
    - `aek agent detect`, `aek agent capabilities`, `aek agent doctor`, `aek agent install-rules [--dry-run]`, `aek agent install-skills [--dry-run]`.
  - Comprehensive documentation:
    - `docs/compatibility/antigravity.md`, `docs/rules.md`, `docs/skills.md`, `docs/skill-authoring.md`, `docs/security.md`, and updated `docs/cli-reference.md`.
  - Comprehensive integration test suites (`rules_cli_tests.rs`, `skills_cli_tests.rs`, `agent_cli_tests.rs`, `rules_tests.rs`, `skills_tests.rs`, `agents_tests.rs`).

## [0.2.0] - 2026-11-06

### Added
- Phase 2 Rust Core Engine & CLI Framework:
  - New `aek-platform` crate providing cross-platform OS & architecture detection, standard directory resolution, and path traversal security invariants.
  - New `SafeCommand` subprocess execution engine in `aek-core` with argument-array execution, secret credential redaction, kill-on-drop timeout cancellation, and dry-run simulation.
  - Three resource profiles: `LIGHT`, `ENGINEERING`, and `FULL` with profile descriptors, CLI commands (`aek profile list`, `aek profile show`, `aek profile apply`), and preservation of unrelated user configuration settings.
  - Advanced configuration models with strict schema constraints, unknown-field rejection, and atomic temporary writes.
  - Crash-resilient backup & restore engine (`aek backup`, `aek backup list`, `aek restore <id>`) with alphanumeric identifier validation, path containment verification, and automatic safety backups before destructive operations.
  - Idempotent `aek init` supporting `--dry-run` and `--force` (with automatic pre-init safety backup).
  - Enhanced `aek doctor` reporting standard `PASS`, `WARN`, `FAIL`, and `SKIP` diagnostic outcomes, with reliable Antigravity detection without false positives from empty directories.
  - Standard JSON schema `schemas/aek-config.schema.json` and profile template definitions in `config/`.
  - Comprehensive documentation suite: `docs/architecture.md`, `docs/configuration.md`, and `docs/cli-reference.md`.
  - Comprehensive unit, integration, and security test suites.

## [0.1.0] - 2026-11-06

### Added
- Phase 1 Foundation:
  - Cargo workspace structure targeting Rust 2024 edition.
  - `aek-core` crate providing core error types, system execution abstractions, and `DoctorRunner` diagnostics.
  - `aek-config` crate supporting TOML configuration, strict constraint validation, and atomic filesystem persistence.
  - `aek-cli` crate implementing `aek --version`, `aek doctor` (human and JSON output modes), and `aek config validate`/`aek config init`.
  - Comprehensive unit and integration test suites covering doctor diagnostics, config serialization, and CLI invocations.
  - Cross-platform GitHub Actions CI workflow for Linux, Windows, and macOS.
  - Architecture Decision Record (ADR 0001) documenting crate boundaries and design decisions.
