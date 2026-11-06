## Description

Please include a summary of the change, the problem solved, and relevant issue links.

Fixes #(issue)

---

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that causes existing functionality to change)
- [ ] Documentation update or fix
- [ ] Security hardening or dependency update

---

## Security Considerations

- [ ] Does this PR introduce or modify subprocess execution? (Must use direct argument vectors; NO shell string interpolation)
- [ ] Does this PR introduce path manipulation? (Must use `sanitize_relative_path` and verify against path traversal)
- [ ] Does this PR handle secrets or credentials? (Must ensure redacting in logs and avoiding hardcoded secrets)
- [ ] Does this PR modify configuration writing? (Must use atomic writes via temporary files)

---

## Verification & TDD Checklist

- [ ] My code follows the code style and guidelines of this project
- [ ] I have followed the TDD workflow (RED -> GREEN -> REFACTOR)
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] `cargo fmt --all -- --check` passes with exit code 0
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes with exit code 0
- [ ] `cargo test --workspace` passes with exit code 0
- [ ] Any required documentation updates (`docs/`, `README.md`) have been included
