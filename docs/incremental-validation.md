# AEK Incremental Validation & Git Change Detection

Large multi-project repositories and monorepos often contain dozens of services. Running complete test suites across every project on every minor edit wastes developer time and CI compute.

The AEK Incremental Validation Engine (`GitChangeDetector`) calculates the minimum sufficient set of projects to test based on Git diffs.

---

## How It Works

```
                     ┌───────────────────────┐
                     │   Git Repository      │
                     └──────────┬────────────┘
                                │
               git diff --name-only / untracked files
                                │
                                ▼
                     ┌───────────────────────┐
                     │   GitChangeDetector   │
                     └──────────┬────────────┘
                                │
         ┌──────────────────────┴──────────────────────┐
         ▼                                             ▼
┌────────────────────────┐                   ┌────────────────────────┐
│ Workspace Root Changed?│                   │ File Inside Subproject?│
│ (e.g. Cargo.lock, CI)  │                   │ (e.g. crates/aek-mcp/) │
└────────┬───────────────┘                   └─────────┬──────────────┘
         │ YES                                         │ YES
         ▼                                             ▼
Validate ALL Projects                         Validate ONLY Affected
(Conservative Fallback)                        Projects & Dependents
```

---

## Detection Mechanics

1. **Working Tree Diffs**:
   - Compares uncommitted modifications (`git diff --name-only HEAD`).
   - Discovers untracked files (`git ls-files --others --exclude-standard`).
2. **Base Branch Diffs**:
   - Supports comparing the current HEAD against a target branch: `aek validate --changed --base origin/main`.
   - Uses `git diff --name-only <BASE>...HEAD`.
3. **Monorepo Mapping**:
   - Each changed file path is mapped to its containing project directory.
4. **Conservative Invalidation**:
   - If a root build file (such as root `Cargo.toml`, root `package.json`, `.github/workflows/`, or top-level configuration) changes, AEK conservatively triggers validation across all detected projects in the repository.

---

## Safe Degradation

If:
- The target directory is not a Git repository, or
- The `git` executable is not available on system `PATH`,

AEK logs a warning and transparently falls back to validating all detected projects, guaranteeing that validation never silently skips code under test.

---

## CLI Usage

```bash
# Validate only projects containing uncommitted local changes
aek validate --changed

# Preview what would be validated without running tests
aek validate --changed --dry-run

# Validate projects changed relative to main branch
aek validate --changed --base origin/main
```

