# AEK CLI Command Reference

## Global Options

- `--verbose`, `-v`: Enable debug tracing output.
- `--quiet`, `-q`: Suppress non-essential output.
- `--version`, `-V`: Display binary version.
- `--help`, `-h`: Print help information.

---

## 1. `aek init`

Initializes AEK standard directories and generates a default configuration file with the LIGHT profile.

```bash
# Standard initialization
aek init

# Dry-run preview
aek init --dry-run

# Force reinitialization with automatic verified safety backup
aek init --force

# Custom destination path
aek init --path ./my-config.toml
```

---

## 2. `aek doctor`

Performs non-destructive environment diagnostics. Evaluates host platform, core toolchains (Rust, Cargo, Git), optional runtimes (Node.js, Python), and Antigravity readiness.

```bash
# Human-readable diagnostics
aek doctor

# Machine-readable JSON output
aek doctor --json

# Detailed diagnostics output
aek doctor --verbose
```

---

## 3. `aek config`

Manages and validates configuration files.

```bash
# View active configuration
aek config show

# Validate configuration against schema and security constraints
aek config validate

# Print path to resolved active configuration file
aek config path
```

---

## 4. `aek profile`

Inspects and applies resource profiles (`light`, `engineering`, `full`).

```bash
# List available profiles with active profile indicator
aek profile list

# Show metadata for a specific profile
aek profile show engineering
aek profile show full

# Preview profile application without modifying files
aek profile apply engineering --dry-run

# Apply profile to active configuration (creates safety backup)
aek profile apply engineering
```

---

## 5. `aek backup` & `aek restore`

Creates and restores crash-resilient configuration backups.

```bash
# Create an on-demand verified backup
aek backup

# List all stored backups
aek backup list

# Preview backup restoration
aek restore backup-20261106-123000-a1b2c3 --dry-run

# Atomically restore backup (creates safety backup of existing file)
aek restore backup-20261106-123000-a1b2c3
```

