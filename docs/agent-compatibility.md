# Agent Compatibility Matrix & Specification

## Supported Agents Summary

| Feature Category | Gemini Antigravity | Claude Code | OpenCode | Cursor |
| :--- | :---: | :---: | :---: | :---: |
| **Rules Configuration** | Supported (`.gemini/rules/`) | Supported (`CLAUDE.md`) | Supported (`AGENTS.md`) | Supported (`.cursor/rules/*.mdc`) |
| **Legacy Rules Fallback** | N/A | Supported | N/A | Supported (`.cursorrules`) |
| **Modular Skills** | Supported (`SKILL.md`) | Supported (`.claude/skills/`) | Supported (`.opencode/skills/`) | Supported (`.cursor/skills/`) |
| **MCP Integration** | Supported (Stdio & HTTP) | Supported (`.claude/mcp.json`) | Supported (`.opencode/config.json`)| Supported (`.cursor/mcp.json`) |
| **Lifecycle Hooks** | Partial (Extension sandboxed)| Partial (Requires explicit approval)| Unsupported | Unsupported |
| **Project Instructions** | Supported (`.gemini/`) | Supported (`CLAUDE.md`) | Supported (`AGENTS.md`) | Supported (`.cursorrules`) |
| **Permissions Boundary** | Supported | Supported | Supported | Partial |

## Agent Details

### 1. Gemini Antigravity
- **Primary Rules**: Markdown rules with YAML frontmatter in `~/.gemini/antigravity/rules` and `.gemini/rules/`.
- **Skills**: Modular progressive disclosure runbooks in `~/.gemini/antigravity/skills` and `.gemini/skills/`.
- **MCP Configuration**: JSON servers configuration at `.gemini/mcp_config.json`.

### 2. Claude Code
- **Primary Instructions**: Consolidated Markdown document in `CLAUDE.md`.
- **Skills**: Skill runbooks placed in `.claude/skills/<name>/SKILL.md`.
- **MCP Configuration**: `.claude/mcp.json` with `{"mcpServers": { ... }}` schema.
- **Security Boundary**: Shell hooks are marked `Partial` and never installed automatically without user consent.

### 3. OpenCode
- **Primary Instructions**: Markdown sections within `AGENTS.md`.
- **Skills**: Runbooks located in `.opencode/skills/<name>/SKILL.md`.
- **MCP Configuration**: `.opencode/config.json` containing `mcp_servers` dictionary.

### 4. Cursor
- **Primary Rules**: MDC Markdown documents located in `.cursor/rules/<name>.mdc` with frontmatter (`description`, `globs`, `alwaysApply`).
- **Legacy Fallback**: `.cursorrules` in workspace root for compatibility with older Cursor versions.
- **MCP Configuration**: `.cursor/mcp.json` with `{"mcpServers": { ... }}` schema.

