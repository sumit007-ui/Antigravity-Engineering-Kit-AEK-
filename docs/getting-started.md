# Getting Started with Antigravity Engineering Kit (AEK)

Welcome to **Antigravity Engineering Kit (AEK)**! This guide walks you through installing AEK, inspecting your environment, configuring engineering rules and skills, and synchronizing settings across your preferred AI coding agents.

---

## 1. Quick Installation

AEK installs into user-space without administrator or root privileges.

### Windows (PowerShell)
```powershell
# From local repository or release archive
.\installers\windows\install.ps1
```

### Linux & macOS (Bash)
```bash
# From local repository or release archive
./installers/linux/install.sh
```

### Verifying Installation
Ensure `aek` is on your `PATH`:
```bash
aek --version
aek --help
```

---

## 2. Inspecting Your Environment

Run the diagnostic doctor to inspect platform tools, compilers, and agents:
```bash
aek doctor
```

Inspect specific subsystems:
```bash
aek agent doctor      # Inspect Antigravity, Claude Code, Cursor, OpenCode
aek language doctor   # Inspect Rust, Go, Python, TypeScript, C/C++, Flutter
aek security doctor   # Inspect Semgrep, Trivy, and package audit tools
aek mcp doctor        # Inspect Model Context Protocol servers
```

---

## 3. Initializing a Project

Initialize AEK configuration in your project root:
```bash
# Preview what would be created
aek init --dry-run

# Initialize configuration
aek init
```

This creates `.aek/config.toml` with safe defaults, strict schema validation, and automatic backup integration.

---

## 4. Working with Global & Project Rules

AEK provides tiered engineering rules (Global -> Workspace -> Project) to enforce quality standards:
```bash
# List available rules
aek rules list

# Inspect a specific rule
aek rules show precision-rule

# Install rules to your active workspace (dry run preview)
aek rules install --workspace . --dry-run
```

---

## 5. Discovering & Activating Specialized Skills

AEK automatically inspects your repository files (e.g. `Cargo.toml`, `package.json`, `go.mod`, `CMakeLists.txt`) to recommend relevant skills:
```bash
# Discover skills relevant to your project
aek skills discover

# List all 15 specialized engineering skills
aek skills list

# Preview installing a skill
aek skills install rust-systems-engineer --workspace . --dry-run
```

---

## 6. Synchronizing Multi-Agent Workspaces

If you use multiple coding assistants (Google Antigravity, Claude Code, Cursor, OpenCode), AEK translates rules, skills, and MCP servers natively without manual duplication:
```bash
# Detect installed coding agents
aek agent detect

# Inspect capability matrix across all agents
aek agent capabilities

# Preview synchronizing rules and skills to all detected agents
aek agent sync --dry-run

# Synchronize with conflict-safe ownership tracking
aek agent sync --strategy replace-managed
```

---

## 7. Running Project Validation

Execute the standardized 8-stage verification pipeline:
```bash
# Preview validation stages
aek validate --dry-run

# Run full project validation
aek validate

# Run incremental validation on changed files only
aek validate --incremental
```

---

## 8. Next Steps

- Explore the [CLI Reference](cli-reference.md) for complete command options.
- Read [Multi-Agent Architecture](multi-agent-architecture.md) for cross-agent synchronization details.
- Read [MCP Architecture](mcp-architecture.md) to configure Model Context Protocol servers.
- Read [Troubleshooting](troubleshooting.md) for common questions and solutions.

