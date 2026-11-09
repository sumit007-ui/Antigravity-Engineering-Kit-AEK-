# Changelog

All notable changes to the Antigravity Engineering Kit (AEK) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-11-09

### Security & Integrity (Fail-Closed Architecture)
- **Security Scanning Fail-Closed**: `aek security scan` now treats missing scanner binaries (`semgrep`, `trivy`) as incomplete and a policy violation by default, setting non-zero exit codes. Added `allow_missing_scanners` opt-in to `SecurityPolicy`.
- **Component-Based Path Traversal Protection**: Replaced substring-based `../` checks in `DependencyAuditor` with `Path::components` rejecting `Component::ParentDir`.
- **Honest Adapter Results**: Removed fake metric scores (0.94) and iterations (4) from offline DSPy fallback; returns `DspyOptimizationResult::passthrough()` with `simulated: true`, `score: 0.0`, and non-zero exit unless `--allow-simulated` is passed.
- **Pinned Promptfoo**: Pinned exact Promptfoo version `0.125.1` from npm registry; returns explicit error instead of mock 5/5 passed evaluation when tools are absent.
- **Honest Naming**: Renamed `PromptOptimizerAdapter` to `DeterministicRuleFormatter` (preserving deprecated alias for backwards compatibility) and documented as rule-based heuristic formatting, not ML/LLM optimization.
- **Integrity Verification**: `InstallationVerifier` computes real SHA-256 binary hash before returning `Verified`; returns `Unverified` when no baseline exists and `Mismatch` on discrepancies.
- **Semver Rollback Sorting**: Fixed rollback version ordering to sort by semantic versioning (`Version::cmp`), placing newer semver before older versions.
- **CI Supply Chain Hardening**: Pinned GitHub Actions to immutable full commit SHAs, added MSRV 1.85.0 verification, `cargo deny check` with `deny.toml`, and clean doc build verification.

## [0.1.0] - 2026-11-06

### Added
- **Initial Public Release of Antigravity Engineering Kit (AEK)**:
  - **Universal Multi-Agent Compatibility Framework (`aek agent`)**:
    - Universal agent adapter architecture (`aek-agents`) for Google Gemini Antigravity, Anthropic Claude Code, Cursor, and OpenCode.
    - Typed capability matrix covering rules, skills, MCP servers, hooks, instructions, and permissions.
    - Multi-agent synchronization (`aek agent sync`) with conflict detection (`KeepExisting`, `ReplaceManaged`, `Skip`, `ManualResolution`) and ownership tracking via `.aek/managed-agents.toml`.
    - 5 Predefined Engineering Profiles (`aek agent profile`): `antigravity-engineering`, `claude-code-engineering`, `cursor-engineering`, `opencode-engineering`, `multi-agent-engineering`.
    - Non-destructive agent discovery and doctor diagnostics (`aek agent detect`, `aek agent doctor`).
  - **Open-Source Community Health & Governance**:
    - Dual licensing under Apache-2.0 and MIT (`LICENSE`, `LICENSE-APACHE`, `LICENSE-MIT`).
    - Standard Contributor Covenant v2.1 `CODE_OF_CONDUCT.md`.
    - Open-source contribution guidelines in `CONTRIBUTING.md`.
    - Responsible security disclosure policy in `SECURITY.md`.
    - Community support guide in `SUPPORT.md`.
    - Automated GitHub Actions CI/CD matrix across Ubuntu, Windows, and macOS.
    - Automated dependency management via Dependabot (`.github/dependabot.yml`).
    - GitHub issue templates (`bug_report.yml`, `feature_request.yml`, `documentation.yml`, `compatibility.yml`) and pull request template.
  - **Production Hardening & Quality Assurance**:
    - Zero compiler and Clippy warnings with `-D warnings` across all 16 crates.
    - Comprehensive unit, integration, and security regression test suite (>165 passing tests).
    - Cross-platform release verification for Windows (`x86_64`) and Linux (`x86_64`).
    - Benchmarking harness and performance measurements in `docs/benchmarks.md`.
    - Complete technical documentation in `docs/` and polished `README.md`.

## [0.8.0] - 2026-11-06

### Added
- Phase 8 Cross-Platform Installation, Packaging & Distribution:
  - New `aek-distribution` crate:
    - Typed `DistributionError` covering path traversal, checksum mismatches, incompatible platforms, rollback failures, and migration errors.
    - Persistent `InstallationManifest` recording installed version, scope, executable path, timestamp, method, target platform, and managed files inventory.
    - `DistributionPaths` resolving standard user installation paths (`%LOCALAPPDATA%\Programs\aek\bin` on Windows, `~/.local/bin` on Linux) with strict lexical containment and path traversal defenses.
    - `ReleaseManifest` parser and artifact finder supporting target triples (`x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`) and OS/architecture fallbacks.
    - `VersionManager` providing semantic version comparison, downgrade protection, and channel detection (`stable`, `beta`, `nightly`).
    - Cryptographic verification engine (`compute_file_sha256`, `verify_file_checksum`) and `InstallationVerifier` auditing managed files on disk.
    - `UpdateChecker` generating deterministic `UpdatePlan` records with downgrade guards and atomic staging workflows.
    - `RollbackManager` archiving replaced binaries into `<install_root>/versions/<version>/` and restoring archived versions.
    - `ConfigMigrator` executing versioned configuration migrations with automated backup creation and atomic persistence.
    - `Uninstaller` planning and executing safe uninstallation, preserving user configurations by default, with `--purge` confirmation support.
  - Production Installers & Packaging:
    - Windows PowerShell installer (`installers/windows/install.ps1`) with architecture detection, HTTPS download, SHA-256 verification, safe zip extraction, user PATH configuration, and automated test harness (`test-install.ps1`).
    - Linux Bash installer (`installers/linux/install.sh`) with strict mode, curl/wget fallback, sha256sum verification, safe tarball extraction, executable permission configuration, and automated test harness (`test-install.sh`).
    - Windows packaging script (`packaging/package-windows.ps1`) and Linux packaging script (`packaging/package-linux.sh`).
    - JSON release manifest schema (`packaging/release-manifest.schema.json`).
    - Multi-platform release verification scripts (`scripts/verify-release.ps1`, `scripts/verify-release.sh`).
  - GitHub Actions Release Workflow:
    - Automated release workflow (`.github/workflows/release.yml`) triggered by `v*.*.*` tags with least-privilege permissions, multi-platform build matrix, automated packaging, checksum generation, and GitHub Release publication.
  - User-Facing CLI Suite:
    - `aek install` (`info`, `doctor`, `verify`)
    - `aek update` (`check`, `--dry-run`, `--version <VER>`, `--force`)
    - `aek rollback` (`--list`, `--version <VER>`, `--dry-run`)
    - `aek config migrate` (`--check`, `--dry-run`)
    - `aek uninstall` (`--dry-run`, `--purge`, `--yes`)
    - `aek setup antigravity` (`--dry-run`, `--profile <PROFILE>`)


## [0.7.0] - 2026-11-06

### Added
- Phase 7 Automated Validation, Security Auditing & CI/CD Engine:
  - New `aek-security` crate:
    - Typed `SecurityError` hierarchy and normalized `Severity` scoring (`Critical`, `High`, `Medium`, `Low`, `Info`, `Unknown`).
    - Unified `SecurityFinding` data model with scanner source, rule IDs, file locations, line spans, and remediation suggestions.
    - Security policy engine (`SecurityPolicy`) supporting severity thresholds (`fail_on_critical`, `fail_on_high`, etc.) and glob exclusions.
    - `SemgrepAdapter` SAST engine executing rules (`p/security-audit`, `p/secrets`) with JSON parsing and offline tolerance.
    - `TrivyAdapter` vulnerability and misconfiguration scanner with JSON parsing and container/filesystem support.
    - `DependencyAuditor` orchestrating native package auditors (`cargo audit`, `govulncheck`, `pip-audit`, `npm audit`).
    - `SecurityScanner` orchestrator coordinating SAST, vulnerability scans, dependency audits, and policy enforcement.
    - `SecurityDoctor` inspecting toolchain availability, versions, and installation guidance.
  - New `aek-reporting` crate:
    - Multi-format structured reporters:
      - `TerminalReportGenerator`: High-density terminal status badges, step duration metrics, and finding summaries.
      - `JsonReportGenerator`: Versioned, machine-readable validation and security JSON serializer.
      - `SarifReportGenerator`: OASIS SARIF v2.1.0 generator for GitHub Code Scanning and IDE SARIF viewers.
      - `MarkdownSummaryGenerator`: Step summaries formatted for GitHub Actions (`$GITHUB_STEP_SUMMARY`) and AI agent context injection.
  - Enhanced `aek-validation` crate:
    - `ValidationPlanner` constructing DAG execution pipelines across multi-language projects and monorepos.
    - `GitChangeDetector` detecting local changes and base commit differences (`--changed`, `--base <REF>`) with conservative monorepo invalidation.
    - `QualityGateEngine` enforcing policy-based pass/fail gates across compilation, unit tests, linters, and security findings.
    - Configurable `ValidationPolicy` supporting TOML definitions, presets (`default`, `strict`, `ci`), timeouts, and concurrency bounds.
  - Configuration Policies & Schemas:
    - Validation policy presets: `config/validation/default.toml`, `config/validation/strict.toml`, `config/validation/ci.toml`.
    - JSON schemas: `schemas/validation-policy.schema.json` and `schemas/validation-report.schema.json`.
  - Continuous Integration (CI/CD) Engine:
    - Least-privilege GitHub Actions workflow generator (`aek ci generate github`).
    - Automated CI workflow auditor (`aek ci validate`) verifying explicit `permissions:` declarations and least-privilege security.
  - Google Antigravity Integration:
    - Extended `AgentAdapter` with `generate_security_guidance` and `export_ci_workflow`.
  - User-Facing CLI Suite:
    - `aek validate` enhanced with `--changed`, `--base`, `--policy`, and `aek validate policy` (`list`, `show`, `check`).
    - `aek security scan` and `aek security doctor`.
    - `aek report show` and `aek report export`.
    - `aek ci generate` and `aek ci validate`.


## [0.6.0] - 2026-11-06

### Added
- Phase 6 AI Prompt Optimization & Evaluation Engine:
  - New `aek-prompt` crate:
    - Data models for versioned prompts (`PromptRecord`), lifecycle states (`PromptStatus`: Draft, Optimized, Evaluated, Approved, Deprecated), and cryptographic provenance tracking via SHA-256 (`compute_sha256`).
    - Deterministic template engine (`PromptTemplate`) supporting variable extraction and safe template substitution (`{{variable}}`).
    - Version manager (`PromptVersionManager`) with atomic persistence (write-to-temp-and-rename), directory listing, diffing (`PromptComparison`), and instant rollback.
    - Bounded thread-safe in-memory cache (`PromptCache`) with LRU eviction.
    - Dataset manager (`DatasetManager`) supporting JSON and JSONL datasets, duplicate ID validation, and deterministic (train, val, test) splitting.
    - Deterministic evaluation metric calculator (`EvaluationMetricCalculator`) computing constraint satisfaction scores, penalizing negative constraint violations, and verifying output schema compliance (JSON and Markdown).
    - Context engineering pipeline (`ContextAssembler`) extracting intent, negative constraints, and assembling guidelines.
    - Adversarial security scanner (`detect_injection_risk`) catching prompt injections and delimiter breakout attacks with template variable sanitization (`sanitize_template_variable`).
  - New `aek-prompt-adapters` crate:
    - `PromptOptimizerAdapter`: Native Rust structured prompt optimization engine implementing identity, domain profile, ruthless precision engineering invariants, step-by-step workflow, and deterministic output schema based on `linshenkx/prompt-optimizer`.
    - `DspyAdapter`: Subprocess bridge to Stanford DSPy (`BootstrapFewShot` teleprompter) via JSON stdin/file IPC, featuring an automated internal deterministic compiler fallback mode when Python or DSPy is offline.
    - `PromptfooAdapter`: Evaluation and red-teaming bridge generating `promptfooconfig.yaml` and orchestrating `promptfoo eval` / `npx promptfoo eval`.
  - CLI Command Suite under `aek prompt`:
    - `list`, `show`, `create`, `versions`, `diff`, `rollback`, `optimize`, `evaluate`, `benchmark`, `compare`, `export`, `doctor`, and `adapters`.
  - Reference Datasets & Schemas:
    - 10 domain evaluation datasets under `datasets/examples/`: `rust.jsonl`, `go.jsonl`, `cpp.jsonl`, `zig.jsonl`, `python.jsonl`, `flutter.jsonl`, `ros2.jsonl`, `security.jsonl`, `review.jsonl`, `prompt_engineering.jsonl`.
    - JSON schemas: `schemas/prompt.schema.json`, `schemas/dataset.schema.json`, `schemas/evaluation.schema.json`.
    - Example prompt templates in `prompts/examples/rust_lru.json` and `prompts/templates/engineering_default.txt`.
  - Google Antigravity Integration:
    - Extended `AgentAdapter` with `export_prompt_instruction` method to export versioned prompt instructions into Antigravity instruction trees.
  - Comprehensive unit, integration, and CLI tests (119 workspace tests passing).

## [0.5.0] - 2026-11-06

### Added
- Phase 5 Advanced Language Toolchain & Project Validation Engine:
  - New `aek-languages` crate:
    - Provider architecture supporting 8 language ecosystems: Rust, Go, C/C++, Zig, Python, JavaScript/TypeScript, Flutter/Dart, and ROS 2.
    - Automatic recursive, bounded-depth project detection (`ProjectDetector`) with symlink cycle protection, ignored build directories, monorepo detection, and ROS 2 manifest verification.
    - Asynchronous tool discovery engine (`ToolchainDiscovery`) with executable resolution on system PATH, safe version extraction, and thread-safe caching.
    - Centralized `ToolchainRegistry` with default definitions for compilers, SDKs, linters, and language servers, plus declarative TOML loading.
    - Ecosystem `CompatibilityEngine` evaluating compiler version constraints and operating system boundaries.
    - `LanguageDoctor` diagnostic suite auditing toolchain readiness and generating safe installation plans.
  - New `aek-validation` crate:
    - Standardized 8-stage validation pipeline (`ValidationPipeline`): Environment, Formatting, Static Analysis, Type Checking, Compilation, Unit Testing, Integration Testing, Build Verification.
    - Subprocess execution engine (`ValidationExecutor`) with argument arrays, bounded timeouts, exit code capture, and dry-run preview.
    - Structured JSON report generator (`ValidationReport`) classifying outcomes as Pass, Fail, Skip, Warn, and Error.
  - Declarative Toolchain Configurations:
    - `languages/rust.toml`, `languages/go.toml`, `languages/cpp.toml`, `languages/zig.toml`, `languages/python.toml`, `languages/typescript.toml`, `languages/flutter.toml`, `languages/ros2.toml`.
    - `schemas/language-toolchain.schema.json`.
  - CLI commands under `aek language` (list, detect, doctor, tools, compatibility, setup, validate) and `aek validate`.
  - Google Antigravity Adapter enhancements: project language detection, skill recommendations, and validation guidance export.

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
