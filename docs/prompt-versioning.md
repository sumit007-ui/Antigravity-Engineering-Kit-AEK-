# Prompt Versioning, Diffing & Rollback

Prompt engineering is an iterative software discipline that requires strict version control, cryptographic provenance, diffing, and fast rollback capabilities.

---

## 1. Cryptographic Provenance & Content Hashing

Every prompt version computes a SHA-256 digest of its active content:
```rust
compute_sha256(record.active_content())
```
- If an optimized prompt is generated, `content_hash` reflects the optimized content.
- If no optimized content exists, `content_hash` reflects the original draft.
- Any modification results in a distinct SHA-256 fingerprint, ensuring immutability.

---

## 2. Version Storage & Atomic Persistence

`PromptVersionManager` persists version records under `<prompts_dir>/<id>/<version>.json`:
- All writes are performed using atomic write-to-temp-and-rename semantics (`<version>.tmp` -> `<version>.json`).
- `latest.json` is updated atomically to track the currently active version.

---

## 3. CLI Operations

### Listing Versions
```bash
aek prompt versions rust-lru
```
Output:
```
Version History for 'rust-lru' (2 versions):
VERSION      STATUS         CREATED            HASH                            
--------------------------------------------------------------------------------
1.0.0        draft          2026-11-06T18:00   e3b0c44298fc                    
1.0.0.opt    optimized      2026-11-06T18:15   9a1b2c3d4e5f                    
```

### Computing Diff Between Versions
```bash
aek prompt diff rust-lru
```
Outputs added (`+`) and removed (`-`) lines between consecutive versions.

### Rolling Back to an Earlier Version
```bash
aek prompt rollback rust-lru 1.0.0
```
Restores version `1.0.0` as the active pointer in `latest.json`.

---

## 4. Exporting to AI Coding Agents

Export approved prompt instructions directly into the target AI coding agent directory (e.g. Google Antigravity instructions folder):

```bash
# Preview export destination
aek prompt export rust-lru --dry-run

# Export into Antigravity instruction hierarchy
aek prompt export rust-lru
```
Generates `<antigravity_dir>/instructions/rust-lru.md` with headers and active prompt instructions.

