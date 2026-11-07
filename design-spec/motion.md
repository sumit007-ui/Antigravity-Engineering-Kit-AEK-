# motion.md — Motion system (principles & tokens)

> Legend: ✅ verified · 🟡 inferred · 🔴 measure.

## 1. Tokens
```css
:root {
  /* durations */
  --dur-instant: 80ms;   /* hover color/press */
  --dur-fast:   160ms;   /* chips, toggles */
  --dur-base:   280ms;   /* expand/collapse, tab swap */
  --dur-slow:   600ms;   /* section reveal */
  --dur-scene: 1200ms;   /* sky/phase transitions */
  --dur-ambient: 30s;    /* marquee full loop */

  /* easings */
  --ease-out:    cubic-bezier(.22,1,.36,1);
  --ease-in-out: cubic-bezier(.65,0,.35,1);
  --ease-step:   steps(8, end);          /* pixel/sprite feel */
  --ease-linear: linear;                 /* marquee, counters */

  /* distances */
  --move-sm: 8px;
  --move-md: 24px;
  --move-lg: 48px;
}
```

## 2. Rules
1. **Animate only `transform` and `opacity`** (plus `clip-path`/`filter` sparingly) → keeps 60fps.
2. **Reveal on scroll once**, not every time (IntersectionObserver / `viewport: { once: true }`).
3. **Stagger** children 40–80ms; cap total stagger at ~500ms.
4. **Stepped easing for retro/pixel assets; ease-out for UI.** Never mix on one element.
5. **Ambient loops are slow and pausable** (marquee pauses on hover/focus).
6. **Interactive state change ≤ 300ms**, scene change ≤ 1.2s.
7. **`prefers-reduced-motion`**: stop marquee, skip loader animation, replace reveals with instant fade, disable parallax.

