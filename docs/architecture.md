# AEK Architecture Specification

## 1. Overview

**Antigravity Engineering Kit (AEK)** is a high-performance, modular AI engineering toolkit written in Rust. AEK provides a foundational platform layer, an execution engine, a configuration manager, and resource profiles that bridge local engineering workflows with Google Antigravity and modern coding agents.

```
+-------------------------------------------------------------+
|                           aek-cli                           |
|        (Commands: init, doctor, config, profile, backup)    |
+------------------------------+------------------------------+
                               |
       +-----------------------+-----------------------+
       |                                               |
       v                                               v
+---------------+                               +---------------+
|   aek-core    |                               |  aek-config   |
| - Diagnostics |                               | - Typed Model |
| - Execution   |<------------------------------|- Atomic Write |
| - Profiles    |                               | - Backup/Rest.|
+-------+-------+                               +-------+-------+
        |                                               |
        +-----------------------+-----------------------+
                                |
                                v
                        +---------------+
                        | aek-platform  |
                        | - Detection   |
                        | - Safe Paths  |
                        +---------------+
```

---

## 2. Crate Responsibilities

### `aek-platform`
- Operating system detection (`windows`, `linux`, `macos`) and Linux distro parsing.
- Architecture detection (`x86_64`, `aarch64`).
- Standard platform directory resolution (XDG Base Directory on Linux, Known Folders on Windows, Library on macOS) without hardcoded paths or elevation requirements.
- Path containment and traversal verification (`is_safe_subpath`) to prevent directory traversal vulnerabilities.

### `aek-core`
- Safe subprocess execution engine (`SafeCommand`):
  - Argument array passing without shell wrappers (`cmd.exe` or `sh -c`) to prevent command injection.
  - Asynchronous execution via Tokio.
  - Automatic child process cleanup and kill-on-drop on timeout.
  - Secret redaction for environment variables (`redact_secrets`).
  - Dry-run simulation mode.
- System diagnostics (`DiagnosticsRunner`):
  - Read-only environment inspection reporting `PASS`, `WARN`, `FAIL`, `SKIP`.
  - Reliable Antigravity detection without false positives from empty directories.
- Resource Profile Engine (`ProfileKind`, `ProfileDescriptor`):
  - `LIGHT`, `ENGINEERING`, and `FULL` tiers.
  - Profile metadata, feature matrices, and target concurrency/timeout limits.

### `aek-config`
- Strongly-typed TOML configuration schema (`AekConfig`).
- Unknown-field denial to prevent silent acceptance of misspelled or rogue security parameters.
- Profile tier application with preservation of unrelated user configurations.
- Crash-resilient atomic file writes using same-volume temporary files and atomic replace.
- Backup & restore engine (`BackupEngine`):
  - Alphanumeric-validated backup identifiers.
  - Path traversal rejection.
  - Automatic safety backups prior to destructive replacement or reinitialization.
  - Post-restoration verification.

### `aek-cli`
- Clap-powered CLI interface with structured argument parsing.
- Structured diagnostic logging with `tracing`.
- Machine-readable `--json` modes alongside human-readable terminal output.

---

## 3. Resource Profiles

| Profile | Concurrency | Timeout | MCP Orchestration | Prompt Optimizer | Security Scan | Target Environment |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **LIGHT** | 2 workers | 30s | Disabled | Disabled | Disabled | Low-spec laptops (8 GB RAM), containers, CI |
| **ENGINEERING** | 4 workers | 60s | Enabled | Disabled | Disabled | Workstations, multi-language coding |
| **FULL** | 8 workers | 120s | Enabled | Enabled (DSPy) | Enabled | Advanced workstations (16 GB+ RAM) |

