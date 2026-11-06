# Global Engineering Principle: Test-Driven Development (TDD)

## Objective
Guarantee correctness, regression resilience, and high test coverage across all features.

## Testing Invariants
1. **RED -> GREEN -> REFACTOR**:
   - Write tests specifying behavior before writing production code.
   - Run tests and verify the expected failure reason (RED).
   - Write minimal clean code to make tests pass (GREEN).
   - Refactor for performance, readability, and modularity while keeping tests green.
2. **Comprehensive Test Tiers**:
   - Unit tests for internal algorithms and models.
   - Integration tests for public CLI commands and workflows.
   - Security tests for path traversal, malformed input, and corrupted state handling.
3. **Automated Verification**:
   - Every change must pass `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`.

