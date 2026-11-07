# font.md — Typography

## 1. Candidate Stack
| Role | Family | Fallback | Use |
|---|---|---|---|
| Pixel accents | **Press Start 2P** / **Silkscreen** | monospace | Badges, coordinates, retro micro-tags |
| Mono | **JetBrains Mono** / **Space Mono** | ui-monospace, monospace | Terminal UI, code, telemetry |
| Display/headings | **Space Grotesk** / **Anton** / **Archivo Black** | sans-serif | Section titles, H2, stat numbers |
| Body | **Inter** / **Plus Jakarta Sans** / **Geist** | sans-serif | Paragraphs, documentation |

## 2. Type scale
| Token | Size (clamp) | Weight | Tracking | Case | Use |
|---|---|---|---|---|---|
| `--fs-hero` | `clamp(2.5rem, 8vw, 6rem)` | 800 | -0.02em | upper | Hero titles |
| `--fs-h2` | `clamp(2rem, 4.5vw, 3.5rem)` | 700 | -0.01em | title | Section titles |
| `--fs-h3` | `clamp(1.25rem, 2vw, 1.75rem)` | 700 | 0 | upper | Card titles |
| `--fs-body` | `1rem / 1.6` | 400 | 0 | sentence | Paragraphs |
| `--fs-small` | `.875rem` | 400 | 0 | sentence | Captions |
| `--fs-label` | `.75rem` | 500 | 0.14em | upper | Eyebrows, tags |
| `--fs-micro` | `.6875rem` | 500 | 0.18em | upper | SEC_02, coords |
| `--fs-stat` | `clamp(3rem, 7vw, 5rem)` | 800 | -0.02em | n/a | Metrics, countdown |

