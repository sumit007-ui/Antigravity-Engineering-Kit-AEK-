# Multi-Agent Configuration Synchronization

## Overview

The `aek agent sync` subsystem provides atomic, conflict-safe synchronization of AEK engineering configurations into all detected or specified coding agents.

## Ownership Tracking & Defense

To prevent silent overwrites or destruction of user-authored custom instructions:
1. AEK maintains an ownership manifest at `.aek/managed-agents.toml`.
2. Every file written records its canonical path, SHA256 content digest, version, and installation timestamp.
3. Prior to writing any file, AEK inspects the existing disk content and compares digests.
4. If a file exists that was **not** placed by AEK, or whose hash changed outside AEK, it is classified as **externally modified**.

## Conflict Resolution Strategies

The `--conflict` parameter controls how collisions are handled:

| Strategy | Behavior |
| :--- | :--- |
| `keep-existing` | Retains user-modified file untouched; logs notice. |
| `replace-managed` | Creates an atomic backup in `.aek/backups/` and installs AEK version. |
| `skip` | Bypasses conflicting files and proceeds with remaining tasks. |
| `manual-resolution` | Halts execution and requires explicit manual operator action. |

## Usage Examples

```bash
# Preview synchronization across all agents without modifying disk
aek agent sync --all --dry-run

# Synchronize with safe conflict backup
aek agent sync --all --conflict replace-managed

# Synchronize only Claude Code and Cursor
aek agent sync --agent claude-code
aek agent sync --agent cursor
```

