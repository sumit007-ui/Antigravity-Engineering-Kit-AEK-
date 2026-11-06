# AEK Quality Gates & Policy Enforcement

AEK enforces rigorous, deterministic quality gates across language builds, tests, static analyzers, and security scans.

---

## What is an AEK Quality Gate?

A **Quality Gate** is a policy-driven decision engine (`QualityGateEngine`) that evaluates the complete results of a validation pipeline and security scan against explicit organizational standards.

If any standard is violated, the Quality Gate reports a failed status (`Passed: false`) and enumerates the exact reasons why the gate failed.

---

## Policy Presets

AEK provides three pre-configured policy presets in `config/validation/`:

### 1. `default` (`config/validation/default.toml`)
- Balanced policy for local development workflows.
- `fail_fast = false`: Runs through all projects to report all diagnostics.
- `fail_on_critical = true`, `fail_on_high = true`: Gated on critical security vulnerabilities.
- `fail_on_test_failure = true`.
- Medium/low severity warnings and non-blocking lint warnings do not fail the gate.

### 2. `strict` (`config/validation/strict.toml`)
- Zero-tolerance policy for pre-commit hooks and release tags.
- `fail_fast = true`: Aborts immediately upon first error.
- `fail_on_critical = true`, `fail_on_high = true`, `fail_on_medium = true`.
- `fail_on_lint_warning = true`.
- `require_all_stages = true`: Requires formatting, linting, tests, build, and security scans to run and succeed.

### 3. `ci` (`config/validation/ci.toml`)
- Optimized for continuous integration servers (GitHub Actions, GitLab CI).
- Structured logging, full SARIF/JSON generation.
- Concurrency bounded to available runner CPU cores.
- `fail_on_critical = true`, `fail_on_high = true`.

---

## Policy Configuration Schema

```toml
name = "custom-team-policy"
description = "Strict gate for core platform repositories"
default_timeout_seconds = 180
fail_fast = true
full_validation = true
max_concurrency = 4

[stages]
run_env_check = true
run_format = true
run_lint = true
run_typecheck = true
run_check = true
run_test = true
run_integration = true
run_build = true
run_security = true

[security]
fail_on_critical = true
fail_on_high = true
fail_on_medium = false
fail_on_low = false
exclude_rules = ["generic.secrets.test-key"]
exclude_paths = ["tests/**"]

[quality_gate]
fail_on_compile_error = true
fail_on_test_failure = true
fail_on_lint_error = true
fail_on_lint_warning = false
```

---

## CLI Management

```bash
# List available policies
aek validate policy list

# Inspect policy configuration details
aek validate policy show default
aek validate policy show strict

# Validate custom policy file syntax and rules
aek validate policy check ./my-policy.toml

# Execute validation under a specific policy
aek validate --policy strict
```

