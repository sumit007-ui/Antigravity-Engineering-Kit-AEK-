# AEK Uninstallation Guide

AEK provides safe, reversible uninstallation that removes binary executables and installation metadata while preserving user configuration, custom prompt templates, and agent rules by default.

---

## 1. Safe Uninstallation (Default)

Safe uninstallation deletes only AEK-managed binaries and version archives:

```bash
# Preview uninstallation steps without deleting files
aek uninstall --dry-run

# Run safe uninstallation
aek uninstall
```

### What is Removed:
- Active executable (`aek` / `aek.exe`)
- Installation manifest (`installation.json`)
- Version rollback archives (`versions/`)

### What is Preserved:
- User configuration file (`config.toml`)
- Custom engineering rules (`~/.gemini/rules` / `.aek/rules`)
- Custom agent skills (`~/.gemini/skills`)
- Custom prompt templates and datasets
- Backups and rollback archives in user data

---

## 2. Purge Uninstallation (Full Removal)

If you wish to completely purge AEK, including all local configurations, rule symlinks, and caches:

```bash
# Interactive purge (prompts for confirmation)
aek uninstall --purge

# Non-interactive automated purge
aek uninstall --purge --yes
```

> [!WARNING]
> Purge mode permanently removes your configuration directory and any local prompt records. Always back up important prompts and configuration prior to running `--purge`.

---

## 3. Platform Uninstall Scripts

You can also execute the uninstaller scripts directly if AEK is not on system `PATH`:

### Windows
```powershell
# Safe removal
.\installers\windows\uninstall.ps1

# Full purge
.\installers\windows\uninstall.ps1 -Purge -Force
```

### Linux
```bash
# Safe removal
./installers/linux/uninstall.sh

# Full purge
./installers/linux/uninstall.sh --purge --force
```

