# AEK Update & Rollback Guide

AEK includes an integrated, secure update and rollback engine that enforces cryptographic verification, prevents accidental downgrades, and archives previous versions for instant recovery.

---

## 1. Checking for Updates

To query the configured release channel and inspect whether a newer version is available:

```bash
# Check if a newer version is available
aek update check
```

Output:
```text
AEK Update Manager
============================================================
Installed Version: 0.1.0

[UPDATE AVAILABLE] A newer version of AEK is available: 0.8.0
  Artifact: aek-v0.8.0-windows-x86_64.zip
  Run `aek update` to install.
```

---

## 2. Previewing Updates (Dry Run)

Before modifying the active binary, preview the update plan:

```bash
aek update --dry-run
```

---

## 3. Applying Updates

```bash
# Update to latest stable release
aek update

# Update to a specific version
aek update --version 0.8.0

# Force downgrade to an earlier version (requires explicit approval)
aek update --version 0.5.0 --force
```

### Safety Features
- **Automatic Archival**: Before replacing the active binary, AEK archives the current version into `<install_root>/versions/<version>/aek(.exe)`.
- **Downgrade Guard**: AEK rejects requests to install an older version unless the `--force` flag is explicitly provided.
- **Atomic Replacement**: On Windows and Linux, new binaries are downloaded to a staging path (`aek.exe.new`), checksummed, and swapped in place to avoid corrupting a running executable.

---

## 4. Rollback Management

If an updated version introduces unexpected behavior, you can instantly roll back to an archived version:

```bash
# List all previously archived versions
aek rollback --list

# Preview rollback to previous version
aek rollback --dry-run

# Roll back to the most recent previous version
aek rollback

# Roll back to an explicit archived version
aek rollback --version 0.7.0
```

---

## 5. Configuration Schema Migrations

When upgrading across major schema revisions, AEK ensures configurations are migrated safely:

```bash
# Check if configuration schema migrations are pending
aek config migrate --check

# Preview migration changes without modifying config file
aek config migrate --dry-run

# Execute configuration migration (creates automatic .bak backup)
aek config migrate
```
