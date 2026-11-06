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

## 4. Model Context Protocol (`aek mcp`)

```bash
# List all registered and discovered MCP servers
aek mcp list [--json]

# Show detailed configuration and transport settings for a server
aek mcp show <SERVER_ID>

# Register a new MCP server from a TOML configuration file
aek mcp register <CONFIG_PATH>

# Remove an MCP server from the registry
aek mcp remove <SERVER_ID>

# Enable or disable an MCP server
aek mcp enable <SERVER_ID>
aek mcp disable <SERVER_ID>

# Establish connection and perform handshake with a server
aek mcp connect <SERVER_ID>

# Disconnect a running server session
aek mcp disconnect <SERVER_ID>

# Run diagnostics and dependency check on MCP servers
aek mcp doctor [--server <SERVER_ID>] [--json]

# Query declared capabilities for an MCP server
aek mcp capabilities <SERVER_ID>

# Discover and list tools offered by an MCP server
aek mcp tools <SERVER_ID> [--json]

# Install an MCP server into target agent environment
aek mcp install <SERVER_ID> [--dry-run]

# Uninstall an MCP server package
aek mcp uninstall <SERVER_ID> [--dry-run]

# Export MCP server configurations to agent format (e.g. Google Antigravity)
aek mcp export --agent antigravity [--target <PATH>] [--dry-run]

# Invoke an MCP tool with JSON arguments
aek mcp call <SERVER_ID> <TOOL_NAME> [--input <JSON_ARGS>]
```

---

## 5. System & Diagnostics

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

---

## 7. Language Toolchain Management (`aek language`)

```bash
# List all supported programming language ecosystems
aek language list

# Detect language projects in current or specified workspace
aek language detect [--path <DIR>] [--json]

# Audit language toolchains and environment health
aek language doctor [LANGUAGE] [--json]

# Inspect standard tools registered for a specific language
aek language tools <LANGUAGE>

# Check toolchain and project compatibility
aek language compatibility <LANGUAGE> [--path <DIR>]

# Generate safe installation plan for missing tools
aek language setup <LANGUAGE> [--dry-run]

# Execute validation commands for a specific language
aek language validate <LANGUAGE> [--path <DIR>] [--dry-run] [--json]
```

---

## 8. Project-Aware Validation (`aek validate`)

```bash
# Validate all projects detected in workspace
aek validate [--path <DIR>]

# Preview validation pipeline steps without execution
aek validate --dry-run

# Run full validation including extended stages
aek validate --full

# Run incremental validation on git-changed projects only
aek validate --changed [--base <REF>]

# Run validation under a specific policy preset or path
aek validate --policy <NAME_OR_PATH>

# Output structured JSON validation report
aek validate --json

# List available validation policies
aek validate policy list

# Display detailed configuration for a policy preset
aek validate policy show <NAME>

# Validate syntax and rules of a policy file
aek validate policy check <PATH>
```

---

## 9. Prompt Optimization & Evaluation (`aek prompt`)

```bash
# List all registered prompt templates and records
aek prompt list [--json]

# Show details, metadata, and active content of a prompt record
aek prompt show <ID> [--json]

# Create and register a new prompt record
aek prompt create <ID> [--name <TITLE>] [--description <DESC>] [--file <PATH>]

# List historical versions for a prompt
aek prompt versions <ID> [--json]

# Compare content diff between prompt versions
aek prompt diff <ID> [--ver-a <V1>] [--ver-b <V2>]

# Roll back a prompt to a previous version
aek prompt rollback <ID> <VERSION>

# Optimize a prompt using prompt-optimizer, dspy, or generic adapter
aek prompt optimize <ID> [--adapter <NAME>] [--dry-run] [--no-cache] [--json]

# Evaluate a prompt against an evaluation dataset
aek prompt evaluate <ID> [--dataset <PATH>] [--json]

# Run benchmark comparison for a prompt
aek prompt benchmark <ID> [--dataset <PATH>]

# Compare evaluation results between two prompt versions
aek prompt compare <ID> <VERSION_A> <VERSION_B>

# Export a prompt into AI coding agent instruction directory
aek prompt export <ID> [--agent <NAME>] [--target <DIR>] [--dry-run]

# Audit prompt optimization and evaluation toolchain environment
aek prompt doctor

# List available prompt optimization adapters and statuses
aek prompt adapters
```

---

## 10. Security Scanning & Auditing (`aek security`)

```bash
# Run multi-engine security scan (Semgrep, Trivy, dependency auditors)
aek security scan [--path <DIR>] [--policy <NAME_OR_PATH>]

# Output normalized JSON security scan report
aek security scan --json

# Export OASIS SARIF v2.1.0 for GitHub Code Scanning
aek security scan --sarif

# Audit health and availability of security toolchains
aek security doctor
```

---

## 11. Structured Reporting (`aek report`)

```bash
# Display formatted report summary for validation or security
aek report show [--type <validation|security>] [--file <PATH>]

# Export report to specific formats (json, sarif, markdown)
aek report export [--type <validation|security>] --format <json|sarif|markdown> [--out <PATH>]
```

---

## 12. CI/CD Integration (`aek ci`)

```bash
# Generate least-privilege GitHub Actions workflows
aek ci generate github [--dry-run]

# Audit existing CI workflows for least-privilege permissions
aek ci validate [--path <DIR>]
```
