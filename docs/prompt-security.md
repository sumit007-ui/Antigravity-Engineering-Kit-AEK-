# Prompt Security & Injection Defense

AI coding agents and prompt orchestration systems are vulnerable to indirect prompt injection, jailbreaks, template breakout attacks, and directory traversal. AEK enforces rigorous security invariants at the engine level.

---

## 1. Adversarial Injection Detection

The `aek-prompt` security module inspects prompt inputs for known jailbreak and role breakout signatures:

- `ignore all previous instructions` / `disregard all previous instructions`
- `disregard previous rules`
- `system override`
- `reveal your system prompt`
- `output root credentials`
- `bypass safety filters`
- `developer mode enabled`
- `jailbreak`

When a pattern is detected:
- An `InjectionRisk` record is generated flagging `is_suspicious = true`.
- The CLI emits prominent diagnostic warnings alerting the engineer before saving or executing.

---

## 2. Template Variable Sanitization

When interpolating runtime values (e.g., `{{variable}}`), attackers may attempt delimiter breakout by injecting role boundaries.

`sanitize_template_variable()` neutralizes breakout fences:
- Replaces ` ```system ` with ` '''system `
- Replaces ` ```assistant ` with ` '''assistant `
- Replaces ` ```instruction ` with ` '''instruction `

This prevents untrusted user inputs from impersonating system or instruction delimiters in agent contexts.

---

## 3. Filesystem Traversal Rejection

All version management and storage operations strictly validate identifiers:
- Reject empty strings.
- Reject identifiers containing path traversal patterns (`..`, `/`, `\`).
- Enforce strict confinement within the resolved prompt storage directory.

