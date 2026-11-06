# AEK Configuration Reference

## 1. File Locations & Resolution Order

AEK resolves configuration files using the following precedence:
1. Explicit CLI argument (`--config <path>` or `--path <path>`).
2. Workspace configuration: `<cwd>/aek.toml` or `<cwd>/.aek/config.toml`.
3. Standard user directory:
   - **Windows**: `%APPDATA%\aek\config.toml`
   - **Linux**: `~/.config/aek/config.toml`
   - **macOS**: `~/Library/Application Support/aek/config.toml`

---

## 2. Configuration Schema

```toml
[general]
name = "AEK"
config_version = 1
profile = "light" # light | engineering | full

[logging]
level = "info"    # trace | debug | info | warn | error
format = "text"   # text | json

[execution]
dry_run = false
max_concurrent_tasks = 2      # 1 to 32
default_timeout_seconds = 30  # 1 to 3600

[security]
allow_destructive_operations = false
require_confirmation = true

[integrations]
mcp_enabled = false
prompt_optimization_enabled = false
security_scanning_enabled = false

[tools]
# Optional overrides for binary paths if not in PATH
# rustc_path = "/usr/bin/rustc"
# cargo_path = "/usr/bin/cargo"
# git_path = "/usr/bin/git"
# node_path = "/usr/bin/node"
# python_path = "/usr/bin/python3"
```

---

## 3. Atomic Writes & Safety Guarantees

All updates to configuration files use **atomic temporary writes**:
1. Content is written to a temporary file (`.aek-config-tmp-*.toml`) inside the destination directory.
2. The operating system flushes all dirty pages to permanent storage (`sync_all()`).
3. The temporary file is atomically renamed over the target destination.

If a crash, power outage, or OS fault occurs at any point during writing, the previous configuration remains 100% intact and uncorrupted.

