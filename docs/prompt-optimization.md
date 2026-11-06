# Prompt Optimization Framework & Adapters

AEK provides a pluggable prompt optimization pipeline designed to transform ambiguous, unstructured developer instructions into structured, production-grade agent prompts.

---

## 1. Supported Adapters

### 1.1. Native Prompt Optimizer (`prompt-optimizer`)
Based on the principles of `linshenkx/prompt-optimizer`, this adapter structures instructions into high-precision, unambiguous engineering specifications without requiring external API dependencies:

- **Role Assignment**: Infers specific systems authority (e.g., `Principal Rust Systems Engineer`, `Principal Go Backend Architect`, `Principal Robotics & ROS 2 Systems Engineer`).
- **Profile Specification**: Sets strict engineering postures (ruthless precision, zero placeholder stubs, production-grade release standards).
- **Core Engineering Invariants**:
  1. *Accuracy*: Zero hallucinated endpoints or speculative APIs.
  2. *Security*: Strict input sanitization, least-privilege containment, no shell injection.
  3. *Completeness*: Complete code implementations only (`// TODO` forbidden).
  4. *Quality*: Mandatory Test-Driven Development (TDD) cycle.
- **Structured Workflow**: Decomposes requests into Requirements -> Test Implementation -> Production Code -> Verification.
- **Deterministic Output Schema**: Fenced code blocks with language identifiers and GitHub-flavored markdown.

#### Usage:
```bash
aek prompt optimize rust-lru --adapter prompt-optimizer
```

---

### 1.2. Stanford DSPy Bridge (`dspy`)
Stanford DSPy replaces brittle manual string prompts with declarative modules and self-optimizing teleprompters.

AEK interfaces with DSPy via an isolated Python bridge (`integrations/prompt/dspy/bridge.py`):
1. Serializes `PromptRecord`, evaluation dataset, and metric targets to a temporary JSON contract.
2. Invokes Python subprocess via `SafeCommand` with a 60-second timeout.
3. The bridge imports DSPy, defines a `dspy.Signature` with `dspy.InputField` and `dspy.OutputField`, and runs `dspy.teleprompt.BootstrapFewShot`.
4. Returns optimized instructions, few-shot demonstrations, and achieved metric score.
5. **Deterministic Fallback**: If Python or `dspy` is not installed on the system, AEK automatically falls back to an internal deterministic compiler mode, guaranteeing zero offline build breakages.

#### Usage:
```bash
aek prompt optimize rust-lru --adapter dspy
```

---

### 1.3. Generic Rule Assembler (`generic`)
Enriches prompts by combining user text with repository context, language-specific global rules, and specialized agent skill references using `ContextAssembler`.

#### Usage:
```bash
aek prompt optimize rust-lru --adapter generic
```
