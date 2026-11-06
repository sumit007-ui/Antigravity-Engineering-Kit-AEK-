# AEK Security Scanning Engine

The `aek-security` engine integrates static analysis security tools, vulnerability scanners, and ecosystem dependency auditors into a single unified security workflow.

---

## Architecture & Adapters

```
                        ┌─────────────────────────┐
                        │     SecurityScanner     │
                        └────────────┬────────────┘
                                     │
         ┌───────────────────────────┼───────────────────────────┐
         ▼                           ▼                           ▼
┌──────────────────┐        ┌──────────────────┐        ┌──────────────────┐
│  SemgrepAdapter  │        │   TrivyAdapter   │        │DependencyAuditor │
│ (SAST & CWEs)    │        │(Vulns & Misconf) │        │ (cargo, npm, etc)│
└────────┬─────────┘        └────────┬─────────┘        └────────┬─────────┘
         │                           │                           │
         └───────────────────────────┼───────────────────────────┘
                                     ▼
                        ┌─────────────────────────┐
                        │ SecurityFinding (Schema)│
                        │ (Normalized Severities) │
                        └────────────┬────────────┘
                                     ▼
                        ┌─────────────────────────┐
                        │   Policy Enforcement    │
                        │ (Fail-on-sev Threshold) │
                        └─────────────────────────┘
```

---

## Supported Security Scanners

1. **Semgrep**:
   - Static Application Security Testing (SAST).
   - Ruleset: `--config auto` / `p/security-audit` / `p/secrets`.
   - Normalizes rule IDs, file paths, line ranges, and message descriptions.
2. **Trivy**:
   - Container, filesystem, and dependency misconfiguration scanning.
   - Command: `trivy fs --format json --security-checks vuln,config`.
   - Normalizes CVE/GHSA IDs, severity levels, and remediation advisories.
3. **Ecosystem Dependency Auditors**:
   - **Rust**: `cargo audit --json` for RustSec advisories.
   - **Go**: `govulncheck ./...` for Go vulnerability database.
   - **Python**: `pip-audit --format json` for PyPI vulnerabilities.
   - **Node.js**: `npm audit --json` for npm advisories.

---

## Normalized Security Finding Model

Every detected issue is normalized into an unambiguous `SecurityFinding` structure:

| Field | Type | Description |
|---|---|---|
| `scanner` | `ScannerKind` | `Semgrep`, `Trivy`, `CargoAudit`, `Govulncheck`, `PipAudit`, `NpmAudit` |
| `id` | `String` | Unique rule or advisory ID (e.g., `RUSTSEC-2023-0001`, `CWE-89`) |
| `title` | `String` | Brief summary of the vulnerability |
| `description` | `String` | Full description of impact and trigger condition |
| `severity` | `Severity` | `Critical`, `High`, `Medium`, `Low`, `Info`, `Unknown` |
| `file_path` | `Option<PathBuf>` | Path to affected source or lockfile |
| `line_start` | `Option<u32>` | Beginning line number |
| `line_end` | `Option<u32>` | Ending line number |
| `recommendation` | `Option<String>` | Upgrade suggestion or fix guidance |

---

## Security Policies & Thresholds

Policies allow granular gating based on severity and path exclusions:

```toml
[security]
fail_on_critical = true
fail_on_high = true
fail_on_medium = false
fail_on_low = false
exclude_rules = ["generic.secrets.test-dummy"]
exclude_paths = ["tests/**", "fixtures/**"]
```

---

## Toolchain Health: `aek security doctor`

The security doctor inspects the host system to report whether scanner binaries are present, functional, and their versions:

```bash
aek security doctor
```

Output:
```text
AEK Security Toolchain Doctor:
------------------------------------------------------------
Scanner              Status               Version / Notes
------------------------------------------------------------
semgrep              [NOT INSTALLED]      Optional SAST engine
trivy                [NOT INSTALLED]      Optional container & filesystem scanner
cargo-audit          [NOT INSTALLED]      Run 'cargo install cargo-audit'
govulncheck          [NOT INSTALLED]      Run 'go install golang.org/x/vuln/cmd/govulncheck@latest'
pip-audit            [NOT INSTALLED]      Run 'pip install pip-audit'
npm                  [INSTALLED]          10.8.2
------------------------------------------------------------
```

---

## CLI Usage

```bash
# Run security scan across the repository
aek security scan

# Output machine-readable JSON
aek security scan --json

# Export OASIS SARIF v2.1.0 for GitHub Code Scanning
aek security scan --sarif > results.sarif

# Enforce strict policy (fails if High or Critical found)
aek security scan --policy strict
```

