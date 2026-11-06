# AEK Agent Profiles

Agent profiles provide pre-bundled, validated sets of engineering rules, skills, MCP servers, and validation gates targeted for specific agent platforms or team configurations.

## Built-In Profiles

### 1. `antigravity-engineering`
- **Target**: Gemini Antigravity
- **Enabled Rules**: `engineering-universal`, `tdd-workflow`, `graphify`, `prompt-optimizer`
- **Enabled Skills**: `backend-architect`, `rust-systems-engineer`, `security-auditor`, `production-code-reviewer`
- **MCP Servers**: `sequential-thinking`, `playwright`, `filesystem`
- **Policy**: `strict`

### 2. `claude-code-engineering`
- **Target**: Claude Code
- **Enabled Rules**: `engineering-universal`, `tdd-workflow`
- **Enabled Skills**: `backend-architect`, `security-auditor`
- **MCP Servers**: `sequential-thinking`, `filesystem`
- **Permissions**: `allow_hooks: false`, `require_approval_for_hooks: true`
- **Policy**: `standard`

### 3. `opencode-engineering`
- **Target**: OpenCode
- **Enabled Rules**: `engineering-universal`, `tdd-workflow`
- **Enabled Skills**: `backend-architect`, `devops-engineer`
- **MCP Servers**: `sequential-thinking`
- **Policy**: `standard`

### 4. `cursor-engineering`
- **Target**: Cursor
- **Enabled Rules**: `engineering-universal`, `tdd-workflow`
- **Enabled Skills**: `backend-architect`, `production-code-reviewer`
- **MCP Servers**: `sequential-thinking`, `filesystem`
- **Policy**: `standard`

### 5. `multi-agent-engineering`
- **Target**: All supported agents (Antigravity, Claude Code, OpenCode, Cursor)
- **Enabled Rules**: Full universal suite
- **Enabled Skills**: Full systems & review suite
- **MCP Servers**: Full standard toolchain
- **Policy**: `strict`

## CLI Commands

```bash
# List all profiles
aek agent profile list

# Inspect profile specifications
aek agent profile show multi-agent-engineering

# Apply profile to workspace (dry-run)
aek agent profile apply multi-agent-engineering --dry-run

# Apply profile to workspace
aek agent profile apply multi-agent-engineering
```

