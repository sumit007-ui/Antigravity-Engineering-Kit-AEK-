# Prompt Evaluation, Benchmarking & Red-Teaming

AEK provides automated evaluation, red-teaming, and benchmarking suites to guarantee quality, safety, and regression prevention across prompt versions.

---

## 1. Domain Evaluation Datasets

AEK ships with 10 reference JSONL evaluation datasets in `datasets/examples/`:

| Dataset | Domain | Evaluated Properties | Negative Constraints |
|---|---|---|---|
| `rust.jsonl` | Rust Systems | `Arc`, `RwLock`, `Send`, `Sync`, unit tests | `unsafe`, panic in production code |
| `go.jsonl` | Go Backend | `context.Context`, `sync.RWMutex`, error wrapping | global mutable state, goroutine leaks |
| `cpp.jsonl` | C++ Systems | `std::unique_ptr`, `constexpr`, RAII, GoogleTest | raw pointers, `malloc`, `free`, C-casts |
| `zig.jsonl` | Zig Programming | `std.mem.Allocator`, `defer deinit()`, errors | unhandled allocations, silent leaks |
| `python.jsonl` | Python Infrastructure | `dataclass`, type annotations, `pytest` | bare `except:`, global variables |
| `flutter.jsonl` | Mobile Architecture | `ChangeNotifier`, `Bloc`, `StatelessWidget` | mutable state in widgets, blocking UI |
| `ros2.jsonl` | Robotics Architecture | `rclcpp::Node`, publishers, QoS profiles | hardcoded topics, blocking spin loops |
| `security.jsonl` | Security & Cryptography | `ConstantTimeEq`, `zeroize`, secure random | raw `rand()`, timing attacks, leaks |
| `review.jsonl` | Code Review Quality | complexity, invariants, test coverage | bikeshedding, vague suggestions |
| `prompt_engineering.jsonl` | Meta-Prompting | role clarity, step reasoning, negative limits | speculative guidelines, broken schemas |

---

## 2. Evaluation Metrics

`EvaluationMetricCalculator` computes deterministic scores for any candidate response against test case criteria:

1. **Constraint Satisfaction**: Ratio of expected keywords/properties present in the output.
2. **Negative Constraint Penalization**: Reduces score by 50% for any violated negative constraint.
3. **Schema Validity**: Verifies that the output cleanly parses as JSON or well-formed GitHub Markdown.

### Running Evaluations
```bash
# Evaluate a prompt against a domain dataset
aek prompt evaluate rust-lru --dataset datasets/examples/rust.jsonl

# Emit machine-readable JSON
aek prompt evaluate rust-lru --dataset datasets/examples/rust.jsonl --json
```

---

## 3. Benchmarking & Version Comparison

Compare candidate prompts against baselines or previous versions:

```bash
# Benchmark a prompt record
aek prompt benchmark rust-lru --dataset datasets/examples/rust.jsonl

# Compare version 1.0.0 and 1.0.0.opt
aek prompt compare rust-lru 1.0.0 1.0.0.opt
```

---

## 4. Promptfoo Integration

AEK can orchestrate Promptfoo test suites via `PromptfooAdapter`:
- Generates `promptfooconfig.yaml` with providers, prompts, and test assertions (`is-json`, `contains`, `similar`, `llm-rubric`).
- Executes evaluations via `promptfoo eval` or `npx promptfoo eval`.
- Parses test results and failure logs.

