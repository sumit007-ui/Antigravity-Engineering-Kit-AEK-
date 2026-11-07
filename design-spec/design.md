# design.md — Goa Buildathon 2027 (goabuilds.com) Design Spec

> Reverse-engineered design spec.
> Legend: ✅ verified from page content/meta · 🟡 inferred from structure/wording · 🔴 measure with `extract-styles.js`

## 1. Concept
**"Retro 16-bit coastal expedition × classified terminal HUD."**
A dark, pixel-art beach world wrapped in mission-briefing / hacker-console language.

| Aspect | Finding | Status |
|---|---|---|
| Base background | `#050505` (`theme-color`, `msapplication-TileColor`) | ✅ |
| Imagery style | 16-bit pixel art / tactical HUD / retro terminal | ✅ |
| Framework | Next.js (`/_next/image`, `next-size-adjust`) | ✅ |
| Voice | Mission briefing, "CLASSIFIED / SEALED / ENCRYPTED", terminal commands | ✅ |
| Motif | Geo-coordinates / Mission codes / Classified tags | ✅ |
| Mood | Precision, code; sunset warmth against near-black | 🟡 |

## 2. Color tokens
| Token | Value | Status |
|---|---|---|
| `--bg` | `#050505` | ✅ |
| `--fg` | off-white (~`#f5f5f0`) | 🔴 |
| `--accent-primary` | warm sunset orange/amber (`#FF4B18` or Pantone Popcorn `#F8D878`) | 🔴 |
| `--accent-secondary` | sea teal/cyan / Cerulean (`#98B4D4`) | 🔴 |
| `--success` | green (judging "GREEN FLAGS" `#10b981`) | 🟡 |
| `--danger` | red (judging "RED FLAGS" `#ef4444`) | 🟡 |
| `--muted` | grey for labels/eyebrows | 🔴 |

## 3. Reusable patterns
**Section header**
```
✦ MISSION BRIEFING • SEC_02                       ← eyebrow (mono, uppercase, tracked)
§02 — SPECIFICATION                             ← sub-label + badge pill
## Section Headline                              ← H2
one-line subtitle
```
**Stat card:** `[NN/LABEL]` tag → big value → caption → sub-caption (uppercase).
**Locked state:** padlock/`SEALED` badge + `[ COMING SOON ]` + dashed/"redacted" feel.
**Terminal block:** window chrome label (`SYS_TERMINAL: READY`), `$` prompt lines, blinking-cursor feel.
**Ticker:** uppercase phrases + `★` separators, duplicated for seamless loop.
**Buttons:** primary solid, secondary outline with `↗`. Hard offset shadows (2px–4px, pixel-sharp).

