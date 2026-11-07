# animation.md — Animation catalog & recipes

> Concrete sequences, one per UI element. Rules/tokens are in `motion.md`.
> Legend: ✅ element exists on page · 🟡 behavior inferred · 🔴 measure exact values.

## A1. Loader → hero reveal ✅ element / 🟡 behavior
**Trigger:** page load. **Skippable by click** ✅.
```
0%   → logo fades in (opacity 0→1, 400ms)
0→100 → counter "Loading - 00%" ticks (linear, ~1.5–2.5s, tied to asset load)
100% → loader exits (translateY(-100%) or fade, 600ms ease-out)
+200ms → hero bg scale 1.08→1, title steps in, countdown + CTAs stagger 80ms
```

## A2. Hero ambience 🟡
- Beach bg / Tactical grid: slow parallax (translateY ≤ 8% of scroll)
- `LIVE RADAR` / `STATUS` caption: blinking dot, `opacity 1↔.3`, 1.2s `steps(2)`.
- `SCROLL` cue: arrow bobs `translateY(0→6px)`, 1.4s ease-in-out infinite.

## A3. Countdown ✅ element
Digit change: 120ms vertical flip/slide (`translateY(-100%)` old, in new). Use `tabular-nums` + fixed width to avoid layout shift. SSR: render placeholder, start on mount.

## A4. Marquee ticker ✅ (list duplicated in DOM)
```css
.marquee { overflow: hidden; }
.marquee__track { display: flex; width: max-content; animation: marquee 30s linear infinite; }
.marquee:hover .marquee__track { animation-play-state: paused; }
@keyframes marquee { to { transform: translateX(-50%); } }
```

## A5. Locked/sealed cards 🟡
- Idle: subtle scanline or flicker on `ENCRYPTION ACTIVE // UNVEILING SOON` (opacity 1→.6, 2.4s steps).
- Hover: card lifts `translate(-2px,-2px)` + hard shadow grows 4px→6px, 120ms.

## A6. Vault decrypt ✅ element
`ATTEMPT VAULT DECRYPT` / Scramble effect with text cycling characters before resolving.

## A7. Timeline expand ✅ element
Click stage card → details expand (grid-rows trick), chevron/label swaps.

## A8. Celestial time dial / Phase selector ✅ element
Phases drive active states, readouts, and status panels.

## A9. Terminal card ✅ element
Lines reveal sequentially (typewriter 25ms/char for commands, instant for output), caret blinks `steps(2)` 1s.

## A10. Evaluation matrix ✅ element
Segmented bar: segments grow `scaleX 0→1` on view, staggered 80ms. Click/hover segment → highlight.

## A11. FAQ terminal ✅ element
Filter chip click → rows filter with layout animation. Row open → answer reveals.

## A12. Partner / Agent logos ✅ element
Hover → logo grayscale(1)→0, tooltip with description fades in 160ms.

## A13. Menu "Console" overlay ✅ element
Open: backdrop fade 200ms → panel clip-path wipe → numbered links stagger 40ms. Close reverse. Trap focus, ESC closes.

## A14. Section entrance (global) 🟡
Eyebrow → H2 → subtitle → content: `opacity 0→1`, `translateY 16→0`, 600ms ease-out, stagger 80ms, once.

