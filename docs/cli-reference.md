# AEK CLI Reference Manual

Comprehensive command-line reference for the **Antigravity Engineering Kit (`aek`)** CLI tool.

## Global Flags

- `-v, --verbose`: Enable detailed debug tracing.
- `-q, --quiet`: Suppress non-essential output.
- `-h, --help`: Display help information.
- `-V, --version`: Print version information (`aek 0.1.0`).

---

## 1. Rules Management (`aek rules`)

```bash
# List available rules (global and project scopes)
aek rules list [--json]

# Show details and full Markdown content of a rule
aek rules show <RULE_NAME>

# Validate all rules against size limits and schema
aek rules validate [--path <DIR>]

# Install a rule into the agent's rules directory
aek rules install <RULE_NAME> [--target <DIR>] [--dry-run]

# Enable or disable a rule
aek rules enable <RULE_NAME>
aek rules disable <RULE_NAME>

# Remove an installed rule
aek rules remove <RULE_NAME> [--target <DIR>] [--dry-run]
```

---

## 2. Skills Management (`aek skills`)

```bash
# List all discovered and bundled skills
aek skills list [--json]

# Show metadata and full instructions for a skill
aek skills show <SKILL_NAME>

# Validate skills against YAML frontmatter and formatting standards
aek skills validate [--path <DIR>]

# Recommend relevant skills based on project files in a directory
aek skills recommend [--path <PROJECT_DIR>]

# Install a skill into the agent's skills directory
aek skills install <SKILL_NAME> [--target <DIR>] [--dry-run]

# Remove an installed skill
aek skills remove <SKILL_NAME> [--target <DIR>] [--dry-run]
```

---

## 3. Agent Integration (`aek agent`)

```bash
# Detect active AI agent runtime and filesystem paths
aek agent detect

# Display agent capabilities, limits, and scopes
aek agent capabilities

# Install all bundled rules into the active agent
aek agent install-rules [--dry-run]

# Install all bundled skills into the active agent
aek agent install-skills [--dry-run]

# Perform a comprehensive agent integration diagnostics check
aek agent doctor
```

---

## 4. System & Diagnostics

```bash
# Initialize AEK environment and configuration
aek init [--path <PATH>] [--force] [--dry-run]

# Run system doctor diagnostics
aek doctor [--workspace <PATH>] [--json]
```

---

## 5. Configuration & Profiles

```bash
# View active configuration
aek config show [--path <PATH>]

# Validate configuration against schema and safety constraints
aek config validate [--path <PATH>]

# Print path to configuration file
aek config path

# List resource profiles (LIGHT, ENGINEERING, FULL)
aek profile list

# Show metadata for a profile tier
aek profile show <light|engineering|full>

# Apply a resource profile
aek profile apply <light|engineering|full> [--dry-run]
```

---

## 6. Backup & Restoration

```bash
# Create a configuration backup
aek backup create [--reason <NOTE>]

# List all configuration backups
aek backup list

# Restore configuration from a verified backup
aek restore <BACKUP_ID> [--dry-run]
```
