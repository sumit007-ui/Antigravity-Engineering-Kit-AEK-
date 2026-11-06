# Google Antigravity & Agent Skills Compatibility Specification

## 1. Verified Official Capabilities

Based on inspection of the official Antigravity customization guides, built-in skill configurations, and runtime discovery roots, the following mechanisms are verified:

### 1.1 Customization Discovery Roots
- **Global Customizations**: `~/.gemini/config/` (or `%USERPROFILE%\.gemini\config\` on Windows).
  - Global rules: loaded from `~/.gemini/config/rules/*.md` and directory rules.
  - Global skills: mounted from `~/.gemini/config/skills/<name>/SKILL.md`.
  - Global MCP config: `~/.gemini/config/mcp_config.json`.
- **Workspace Customizations**:
  - Project root directory `.agents/` (or `.agent/`, `_agents/`, `_agent/`).
  - Workspace rules: `.agents/rules/*.md`, `AGENTS.md`, and `GEMINI.md`.
  - Workspace skills: `.agents/skills/<name>/SKILL.md` or `skills/<name>/SKILL.md`.
  - Workspace MCP config: `mcp_config.json` or `.agents/mcp_config.json`.

### 1.2 Rule Loading & Budgets
- **Rule Format**: Standard GitHub-flavored Markdown.
- **Rule Scope**: Directory-based hierarchy (walking up from CWD to repository root) + Global discovery.
- **Per-File Size Limit**: Capped at `24,000 bytes` (24 KB). Larger files are truncated on line boundaries.
- **Aggregate Rules Budget**: Capped at `20,000 tokens` (`defaultRulesBudget`). Over-budget rules are demoted to file-pointer references.
- **Deduplication**: Files are deduplicated by resolved canonical path. A single rule is never injected multiple times in a turn.

### 1.3 Agent Skill Specification
- **Directory Structure**:
  ```text
  skills/<skill_name>/
  ├── SKILL.md          # Required: Instructions with YAML frontmatter
  ├── scripts/          # Optional: Helper scripts and executable utilities
  ├── examples/         # Optional: Reference code patterns
  ├── resources/        # Optional: Assets or templates
  └── references/       # Optional: In-depth documentation
  ```
- **Frontmatter Fields**:
  - `name` (string, required): Lowercase, hyphenated identifier (e.g. `rust-systems-engineer`).
  - `description` (string, required): Third-person description specifying **what** the skill does and **when** it activates.
- **Progressive Disclosure**: Only skill names and descriptions are injected into the agent system prompt. The full `SKILL.md` body is loaded on-demand when the skill is activated.

---

## 2. Unsupported Features & AEK-Managed Equivalents

| Requested Feature | Native Antigravity Status | AEK-Managed Solution |
| :--- | :--- | :--- |
| Dynamic skill activation API | Not exposed programmatically | AEK installs skills into `.agents/skills/` or `~/.gemini/config/skills/`, relying on Antigravity's progressive disclosure |
| Automated prompt interception | No external prompt proxy hook | AEK provides explicit CLI commands (`aek skills recommend`, `aek agent install-rules`) |
| Arbitrary plugin code execution | Security sandbox restricted | AEK treats all skill scripts as static text; never executes downloaded scripts automatically |
| Project-aware skill recommendation | Not present natively in AGY | AEK inspects project manifests (`Cargo.toml`, `go.mod`, etc.) and recommends appropriate skills |
| Conflict resolution & rollbacks | Overwrites silently if unmanaged | AEK creates atomic backups and validates content containment prior to rule/skill installation |

