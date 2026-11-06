# AEK Prompt Engine Architecture & Reference

The **Antigravity Engineering Kit (AEK) Prompt Engine** provides an end-to-end, deterministic, cross-platform architecture for prompt authoring, role decomposition, automated optimization, dataset evaluation, red-teaming, versioning, and AI agent instruction export.

---

## 1. Architecture Overview

The prompt engine is organized into two primary crates:

```
┌────────────────────────────────────────────────────────────────────────┐
│                              aek-cli                                   │
│            `aek prompt <list|show|create|optimize|evaluate|...>`       │
└───────────────────▲────────────────────────────────▲───────────────────┘
                    │                                │
┌───────────────────┴──────────────┐   ┌─────────────┴──────────────────┐
│          aek-prompt              │   │      aek-prompt-adapters       │
│  - PromptRecord & Status         │   │  - Prompt Optimizer Adapter    │
│  - PromptTemplate ({{var}})      │   │  - Stanford DSPy Python Bridge │
│  - PromptVersionManager (Atomic) │   │  - Promptfoo Evaluation Bridge │
│  - PromptCache (LRU In-Memory)   │   │  - Generic Rule Assembler      │
│  - DatasetManager (JSON/JSONL)   │   └────────────────────────────────┘
│  - EvaluationMetricCalculator    │
│  - InjectionRisk Detection       │
└──────────────────────────────────┘
```

### Core Crates

1. **`aek-prompt`**: Core data structures, deterministic template engine, atomic version control, metric calculators, and security boundaries. Zero dependency on external runtime environments.
2. **`aek-prompt-adapters`**: Pluggable optimization and red-teaming adapters integrating with external AI frameworks:
   - **Prompt Optimizer**: Structured role, profile, constraints, and deterministic schema enforcement based on `linshenkx/prompt-optimizer`.
   - **DSPy Bridge**: Subprocess integration with Stanford NLP's DSPy library (`BootstrapFewShot`, signatures, few-shot compilation) with a deterministic fallback mode.
   - **Promptfoo**: Evaluation and security red-teaming configuration generation and execution via `promptfoo` CLI.

---

## 2. Prompt Records & Lifecycle

Each prompt is tracked as an immutable versioned `PromptRecord`:

| Field | Type | Description |
|---|---|---|
| `id` | `String` | Unique alphanumeric identifier (e.g. `rust-lru`, `go-grpc`). |
| `name` | `String` | Human-readable title of the prompt task. |
| `description` | `String` | Scope and purpose of the engineering task. |
| `version` | `String` | Semantic version string (e.g. `1.0.0`, `1.0.0.opt`). |
| `original_content` | `String` | Raw or template content as initially drafted. |
| `optimized_content`| `Option<String>` | Compiled/optimized content produced by an adapter. |
| `status` | `PromptStatus` | `draft`, `optimized`, `evaluated`, `approved`, or `deprecated`. |
| `content_hash` | `String` | SHA-256 hex digest of the active content. |
| `provenance` | `String` | Provenance metadata (`manual`, `prompt-optimizer`, `dspy`). |

### Status State Machine

```
[Draft] ──(aek prompt optimize)──> [Optimized] ──(aek prompt evaluate)──> [Evaluated]
                                                                                │
                                                            (Manual Approval)   │
                                                                                ▼
[Deprecated] <───────────────────────────────────────────────────────────── [Approved]
```

---

## 3. Storage Layout

Prompts are stored atomically under the standard AEK data directory (`~/.local/share/aek/prompts` or `%LOCALAPPDATA%\aek\data\prompts`):

```
prompts/
├── <prompt-id>/
│   ├── 1.0.0.json
│   ├── 1.0.0.opt.json
│   └── latest.json         # Symlink/copy pointing to active version
└── examples/               # Bundled repository reference prompts
```

All disk writes utilize atomic write-then-rename semantics to prevent corruption during unexpected shutdowns.

