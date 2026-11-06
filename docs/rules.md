# AEK Global Engineering Rules

The **Antigravity Engineering Kit (AEK)** provides a tiered, deterministic rules subsystem designed to enforce production engineering standards across AI coding agents.

## 1. Rule Scopes & Precedence Hierarchy

AEK resolves rules following strict precedence:

1. **Project Scope (`rules/project/`)** [Priority: 300]:
   Language-specific and project-focused guidelines (e.g. `rust.md`, `go.md`, `cpp.md`, `zig.md`, `python.md`, `typescript.md`, `flutter.md`, `robotics.md`). Project rules take precedence over global baselines.
2. **Workspace Scope (`.agents/rules/`)** [Priority: 200]:
   Repository-specific and team guidelines applied across all contributors working within a given repository.
3. **Global Scope (`rules/global/`)** [Priority: 100]:
   Universal engineering baselines applied globally across all agent sessions:
   - `engineering.md`: TDD workflow, relentless precision, zero unverified claims.
   - `security.md`: Least privilege, path containment, secret sanitization, injection defense.
   - `documentation.md`: Markdown conventions, provenance, technical accuracy.
   - `testing.md`: Red-green-refactor loop, test coverage targets, reproducible fixtures.
   - `performance.md`: Low-latency allocations, zero redundant copies, resource efficiency.

## 2. Antigravity & Standard AI Agent Budget Constraints

To ensure compatibility with Google Antigravity and modern LLM coding agents:
- **Maximum Rule Size**: 24,000 bytes (24 KB) per individual rule file.
- **Aggregate Token Budget**: ~20,000 tokens for active system-level rules.
- **Rule Consolidation**: `aek-rules` automatically resolves precedence collisions, demoting over-budget rules to file path references rather than inlining excessive tokens into context.

## 3. CLI Management Commands

```bash
# List all discovered rules and their scopes
aek rules list

# Output rules list as structured JSON
aek rules list --json

# Inspect a specific rule's metadata and markdown content
aek rules show engineering
aek rules show rust

# Validate all rules against schema and size constraints
aek rules validate

# Install a rule into an agent or workspace directory
aek rules install engineering --dry-run
aek rules install engineering

# Remove an installed rule safely
aek rules remove engineering --dry-run
aek rules remove engineering
```

