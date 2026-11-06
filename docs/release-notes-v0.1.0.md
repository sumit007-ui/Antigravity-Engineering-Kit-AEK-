# Release Notes: Antigravity Engineering Kit (AEK) v0.1.0

We are proud to announce the initial public release of **Antigravity Engineering Kit (AEK) v0.1.0** — a high-performance, modular, cross-platform engineering toolkit written in Rust for developers working with Google Antigravity and modern AI coding assistants.

---

## Highlights

### 1. Universal Multi-Agent Compatibility Framework
- Native translation and synchronization across 4 primary coding agents:
  - **Google Gemini Antigravity**: Rules, skills, profiles, and MCP configurations.
  - **Anthropic Claude Code**: Consolidated `CLAUDE.md`, `.claude/mcp.json`, and `.claude/settings.json`.
  - **Cursor AI IDE**: Modular `.cursor/rules/*.mdc`, `.cursorrules`, and `.cursor/mcp.json`.
  - **OpenCode**: `AGENTS.md`, `.opencode/config.json`, and `.opencode/skills/`.
- Conflict-safe multi-agent synchronization (`aek agent sync`) with SHA-256 ownership tracking via `.aek/managed-agents.toml` and rollback safeguards.
- 5 Predefined Engineering Profiles (`aek agent profile`):
  - `antigravity-engineering`
  - `claude-code-engineering`
  - `cursor-engineering`
  - `opencode-engineering`
  - `multi-agent-engineering`

### 2. Global Engineering Rules & Specialized Skills
- Tiered precedence resolution (Global -> Workspace -> Project) with 24 KB size bounds and token budget estimation.
- 15 specialized production engineering skills covering Rust, Go, C/C++, Zig, Python, TypeScript, Flutter, ROS 2, DevOps, Security, and Prompt Engineering.
- Automated project-aware discovery recommending skills based on codebase manifests (`Cargo.toml`, `package.json`, `go.mod`, etc.).

### 3. Model Context Protocol (MCP) Framework
- High-throughput JSON-RPC 2.0 client engine supporting Stdio and Streamable HTTP transports.
- Out-of-the-box verified integration presets for 5 major MCP servers:
  - `context7` (Library documentation)
  - `github` (Repositories, issues, PRs)
  - `playwright` (Browser automation)
  - `sequential-thinking` (Dynamic reasoning)
  - `filesystem` (Contained scratchpad & workspace files)

### 4. Language Toolchain & Verification Engine
- Toolchain discovery and diagnostics across 8 major language ecosystems.
- 8-stage verification DAG pipeline (`aek validate`) with incremental Git change detection and configurable quality gates.
- Multi-scanner security orchestration (`aek security`) supporting Semgrep, Trivy, and native package dependency auditors.
- Structured reporting (`aek report`) with OASIS SARIF v2.1.0, JSON, and GitHub Markdown job summaries.

### 5. Cross-Platform Lifecycle Engine
- User-space installation without administrator/root privileges (`installers/windows/install.ps1`, `installers/linux/install.sh`).
- In-place atomic binary updates (`aek update`) with SHA-256 integrity verification.
- Instant version rollback archives (`aek rollback`).
- Safe uninstallation with optional `--purge` (`aek uninstall`).

---

## Supported Platforms

| Platform | Architecture | Status | Binary / Archive |
| :--- | :--- | :---: | :--- |
| **Windows** | x86_64 | Verified | `aek-v0.1.0-windows-x86_64.zip` |
| **Linux** | x86_64 | Verified | `aek-v0.1.0-linux-x86_64.tar.gz` |
| **macOS** | aarch64 / x86_64 | Verified (CI) | Source build / GitHub Actions |

---

## Installation

### Windows
```powershell
irm https://raw.githubusercontent.com/google-antigravity/aek/main/installers/windows/install.ps1 | iex
```

### Linux & macOS
```bash
curl -fsSL https://raw.githubusercontent.com/google-antigravity/aek/main/installers/linux/install.sh | bash
```

---

## Known Limitations

- **Stanford DSPy Bridge**: Requires a local Python 3.10+ installation with `dspy-ai` installed; classified as EXPERIMENTAL.
- **Third-Party Security Scanners**: `aek security scan` relies on external binaries (`semgrep`, `trivy`) if present; gracefully skips with diagnostic recommendations if absent.
- **Claude Code Hooks**: Claude Code shell execution hooks are labeled PARTIAL and require explicit user consent in `.claude/settings.json`.

---

## Security

AEK enforces strict security invariants:
- Direct argument-array subprocess execution; zero shell string evaluation.
- Path traversal defense (`sanitize_relative_path` and `is_safe_subpath`).
- Atomic configuration mutations using same-filesystem temporary files.
- Zero telemetry and zero remote transmission of user source code.

