# AEK Troubleshooting Guide

Common issues, diagnostics, and recovery runbooks for AEK installation, toolchains, and distribution.

---

## 1. Running General Diagnostics

Run `aek doctor` and `aek install doctor` to diagnose environment readiness:

```bash
# Global environment check
aek doctor

# Installation integrity and PATH verification
aek install doctor
```

---

## 2. Common Issues & Solutions

### Issue: `aek: command not found` after installation
**Cause**: The installation directory is not in your current shell's `PATH` variable.
**Solution**:
- **Windows**: Add `%LOCALAPPDATA%\Programs\aek\bin` to your User Environment PATH, or re-run the installer with the `-AddToPath` flag:
  ```powershell
  .\installers\windows\install.ps1 -AddToPath
  ```
  Restart your terminal or PowerShell session.
- **Linux**: Add `export PATH="$HOME/.local/bin:$PATH"` to your `~/.bashrc` or `~/.zshrc`:
  ```bash
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
  source ~/.bashrc
  ```

### Issue: `SHA-256 verification failed!` during install
**Cause**: The downloaded archive was corrupted or modified in transit.
**Solution**:
- Re-download the artifact over secure HTTPS.
- Check the official SHA-256 hash in the release manifest.
- Ensure no proxy or antivirus software is inspecting and altering archive bytes.

### Issue: Rollback failed: `No previous versions found in archive`
**Cause**: AEK has not yet performed an automated upgrade on this machine, so no historical versions have been archived.
**Solution**:
- Download the desired older release from [GitHub Releases](https://github.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/releases) and use `aek update --version <VER> --force`.

### Issue: Configuration migration error
**Cause**: The configuration file syntax was manually corrupted or has an invalid schema version.
**Solution**:
- Check the backup configuration copy generated before migration: `<config_path>.bak`.
- Run:
  ```bash
  aek config validate
  ```
- Or restore from an automated backup:
  ```bash
  aek restore <BACKUP_ID>
  ```

