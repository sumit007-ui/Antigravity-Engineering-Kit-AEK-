# AEK Installation Guide

The **Antigravity Engineering Kit (AEK)** provides secure, user-level, cross-platform installation for Windows and Linux systems without requiring administrator or root privileges.

---

## System Requirements

- **Supported Operating Systems**:
  - Windows 10/11 (x86_64, ARM64)
  - Linux (glibc >= 2.31, x86_64, aarch64)
- **Memory Footprint**: Designed to run cleanly on machines with 8 GB RAM (<50 MB idle memory).
- **Core CLI Dependencies**: Self-contained native binary. No Python, Node.js, or cloud model APIs required for core commands.

---

## 1. Windows Installation (PowerShell)

### One-Line Install
Run PowerShell (non-administrator) and execute:

```powershell
# Default user installation to %LOCALAPPDATA%\Programs\aek\bin
irm https://github.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/releases/latest/download/install.ps1 | iex
```

### Local / Offline Install
If you have downloaded the release archive locally:

```powershell
.\installers\windows\install.ps1 -LocalArtifact aek-v0.1.0-windows-x86_64.zip -ExpectedSha256 <SHA256> -AddToPath
```

### Installation Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `-Version` | String | `latest` | Target version to install (e.g. `0.8.0`) |
| `-InstallDir` | String | `%LOCALAPPDATA%\Programs\aek\bin` | Custom installation directory |
| `-LocalArtifact`| String | None | Path to local `.zip` release archive |
| `-ExpectedSha256`| String | None | Expected SHA-256 digest to verify before extraction |
| `-AddToPath` | Switch | `false` | Automatically append installation directory to User PATH |
| `-DryRun` | Switch | `false` | Preview installation actions without modifying files |
| `-Force` | Switch | `false` | Reinstall or overwrite existing installation |

---

## 2. Linux Installation (Bash)

### One-Line Install
Run your standard shell:

```bash
# Default user installation to ~/.local/bin
curl -sSL https://github.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/releases/latest/download/install.sh | bash
```

### Local / Offline Install
```bash
./installers/linux/install.sh --local-artifact aek-v0.1.0-linux-x86_64.tar.gz --expected-sha256 <SHA256> --add-to-path
```

### Installation Parameters

| Parameter | Type | Default | Description |
|---|---|---|---|
| `--version` | String | `latest` | Target version to install |
| `--install-dir` | Path | `~/.local/bin` | Custom target directory |
| `--local-artifact` | Path | None | Path to local `.tar.gz` archive |
| `--expected-sha256` | String | None | Expected SHA-256 digest |
| `--add-to-path` | Flag | `false` | Appends export PATH statement to `~/.bashrc` or `~/.zshrc` |
| `--dry-run` | Flag | `false` | Preview operations without mutations |
| `--force` | Flag | `false` | Overwrite existing binary |

---

## 3. Installation Verification & Diagnostics

After installation, verify that the binary is functional and correctly configured:

```bash
# Print installed version
aek --version

# Display installation metadata, scope, and paths
aek install info

# Run diagnostics on binary location, PATH, and manifest
aek install doctor

# Verify binary integrity against cryptographic checksums
aek install verify
```

