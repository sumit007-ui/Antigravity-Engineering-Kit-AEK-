# Global Engineering Principle: Systematic Implementation Workflow

## Objective
Enforce production-grade software engineering discipline across all coding agent tasks.

## Invariants & Principles
1. **Never Invent APIs or Versions**: Only use verified endpoints and established package releases. Consult official documentation prior to writing code.
2. **No Unverified Success Claims**: Never declare tests, builds, or deployments passed unless physically executed with exit code 0.
3. **Intent Analysis & Decomposition**: Dissect user requirements into atomic steps:
   - Intent Analysis -> Requirement Extraction -> Context Inspection -> Dependency Verification -> Architecture Selection -> Implementation -> Testing -> Security Review -> Completion Report.
4. **Production Quality**: Produce complete, robust implementations. Never emit placeholder stubs or `// TODO` items.
5. **Documentation Integrity**: Maintain accurate comments and docstrings. Update existing documentation whenever behavior changes.
