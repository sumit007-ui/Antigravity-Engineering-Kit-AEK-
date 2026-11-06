# AEK Release Process & Supply Chain Security

This document outlines the deterministic, multi-platform release lifecycle, cryptographic artifact verification, and GitHub Actions release automation for AEK.

---

## 1. Versioning & Tag Convention

AEK adheres strictly to [Semantic Versioning 2.0.0](https://semver.org/).

Release tags follow the format:
```text
v<MAJOR>.<MINOR>.<PATCH>
```
Examples: `v0.1.0`, `v0.8.0`, `v1.0.0`.

---

## 2. Automated Release Pipeline

The release workflow is defined in `.github/workflows/release.yml` with least-privilege security permissions (`contents: write`).

```
           Push Tag 'v*.*.*'
                  │
                  ▼
         ┌─────────────────┐
         │  Verify Stage   │
         │ (fmt, clippy,   │
         │  tests)         │
         └────────┬────────┘
                  │
         ┌────────┴────────┐
         ▼                 ▼
┌─────────────────┐  ┌─────────────────┐
│  Build Windows  │  │   Build Linux   │
│ (x86_64 msvc)   │  │ (x86_64 glibc)  │
│  test-install   │  │  test-install   │
│  package .zip   │  │  package .tar.gz│
└────────┬────────┘  └────────┬────────┘
         │                    │
         └────────┬───────────┘
                  │
                  ▼
         ┌─────────────────┐
         │ Publish Release │
         │  SHA-256 hashes │
         │  release-manif. │
         │  GitHub Release │
         └─────────────────┘
```

---

## 3. Packaging & Artifacts

Each release generates deterministic packages containing:
1. Native compiled binary (`aek` / `aek.exe`)
2. `README.md`
3. `LICENSE-APACHE` & `LICENSE-MIT`
4. Cryptographic SHA-256 checksum file (`*.sha256`)

| Platform | Target Triple | Artifact File |
|---|---|---|
| Windows | `x86_64-pc-windows-msvc` | `aek-v<VER>-windows-x86_64.zip` |
| Linux | `x86_64-unknown-linux-gnu` | `aek-v<VER>-linux-x86_64.tar.gz` |

---

## 4. Release Manifest Specification

Every release attaches a machine-readable `release-manifest.json` conforming to `packaging/release-manifest.schema.json`:

```json
{
  "schema_version": "1.0.0",
  "version": "0.8.0",
  "release_channel": "stable",
  "timestamp": "2026-11-06T18:00:00Z",
  "artifacts": [
    {
      "filename": "aek-v0.8.0-windows-x86_64.zip",
      "target_triple": "x86_64-pc-windows-msvc",
      "os": "windows",
      "arch": "x86_64",
      "size_bytes": 2544270,
      "sha256": "1cbdeecc34a813b46e600b7ec1e0f614904c1afa940802c274499d616aadb608",
      "download_url": "https://github.com/google-antigravity/aek/releases/download/v0.8.0/aek-v0.8.0-windows-x86_64.zip"
    }
  ]
}
```

---

## 5. Artifact Verification & Provenance

To verify downloaded artifacts against official checksums:

```powershell
# Windows
.\scripts\verify-release.ps1 -DistDir dist -ManifestPath dist/release-manifest.json
```

```bash
# Linux
./scripts/verify-release.sh dist dist/release-manifest.json
```
