# Global Engineering Principle: Documentation & Provenance

## Objective
Ensure code, architecture, and developer interfaces remain thoroughly documented and transparent.

## Documentation Invariants
1. **Public API Documentation**: Document all public types, structs, enums, functions, and modules with standard doc comments (`///` and `//!`).
2. **Architecture Decision Records (ADRs)**: Document significant design choices, trade-offs, and crate boundaries in `docs/adr/`.
3. **Verify Against Official Sources**: Base implementation guides on official platform documentation. Avoid speculation or outdated tutorials.
4. **Synchronized Documentation**: Whenever CLI flags, schemas, or behaviors change, synchronously update `README.md`, `cli-reference.md`, and `CHANGELOG.md`.
