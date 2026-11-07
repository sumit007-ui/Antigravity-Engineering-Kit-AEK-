#!/usr/bin/env python3
"""Procedural 16-bit sunset beach scene (original artwork, no AI).

Usage:
    python make_pixel_scene.py [--seed 7] [--out scene] [--scales 4 6]
Outputs:  <out>-320x180.png  and  <out>@<scale>x.png (nearest-neighbor upscales)
Requires: Pillow>=10
"""
from __future__ import annotations
import argparse, math, random
from PIL import Image, ImageDraw

W, H, HORIZON = 320, 180, 100
BAYER4 = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]]

def hex2rgb(h: str) -> tuple[int, int, int]:
    h = h.lstrip("#")
    return int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16)

SKY = [hex2rgb(c) for c in ("#050505", "#120d24", "#2b1544", "#5a1f55", "#a02f58", "#e0553f", "#ff8a3d", "#ffc15e")]
SEA = [hex2rgb(c) for c in ("#6a2459", "#4d1f55", "#33194a", "#1f1437", "#120d24")]
SAND, WET, FORE, SIL = hex2rgb("#3b2418"), hex2rgb("#7a4630"), hex2rgb("#1a0f0c"), hex2rgb("#07070a")
SUN_TOP, SUN_BOT, STAR = hex2rgb("#ffd36b"), hex2rgb("#ff8a3d"), hex2rgb("#f5e9c8")

def ramp(colors, t: float, x: int, y: int):
    """Pick a color from `colors` at position t in [0,1], ordered-dithering between neighbors."""
    t = min(max(t, 0.0), 0.9999) * (len(colors) - 1)
    i, frac = int(t), t - int(t)
    return colors[i + 1] if frac > BAYER4[y % 4][x % 4] / 16 else colors[i]

def bezier(p0, p1, p2, n=40):
    for k in range(n + 1):
        t = k / n
        yield ((1 - t) ** 2 * p0[0] + 2 * (1 - t) * t * p1[0] + t * t * p2[0],
               (1 - t) ** 2 * p0[1] + 2 * (1 - t) * t * p1[1] + t * t * p2[1])

def stroke(draw: ImageDraw.ImageDraw, p0, p1, p2, color, w0: float, w1: float, n: int = 300):
    """Tapered quadratic stroke drawn with hard-edged (non-antialiased) discs."""
    for k in range(n + 1):
        t = k / n
        x = (1 - t) ** 2 * p0[0] + 2 * (1 - t) * t * p1[0] + t * t * p2[0]
        y = (1 - t) ** 2 * p0[1] + 2 * (1 - t) * t * p1[1] + t * t * p2[1]
        r = max(0.5, (w0 + (w1 - w0) * t) / 2)
        draw.ellipse([x - r, y - r, x + r, y + r], fill=color)

def palm(draw, base_x, top, lean, scale=1.0):
    tx, ty = top
    stroke(draw, (base_x, H + 4), (base_x + lean, (H + ty) // 2), (tx, ty), SIL, 6 * scale, 3 * scale)
    fronds = [(-168, 40), (-142, 44), (-115, 38), (-90, 26), (-65, 38), (-38, 44), (-12, 40)]
    for ang, ln in fronds:
        r = math.radians(ang)
        dx, dy = math.cos(r), math.sin(r)
        L = ln * scale
        end = (tx + dx * L, ty + dy * L + 0.55 * L * abs(dx) + 4 * scale)
        ctl = (tx + dx * L * 0.55, ty + dy * L * 0.55 - 7 * scale)
        stroke(draw, (tx, ty), ctl, end, SIL, 5 * scale, 0.8)
        for k in range(3, 10):
            t = k / 10
            x = (1 - t) ** 2 * tx + 2 * (1 - t) * t * ctl[0] + t * t * end[0]
            y = (1 - t) ** 2 * ty + 2 * (1 - t) * t * ctl[1] + t * t * end[1]
            stroke(draw, (x, y), (x + dx * 2, y + 4 * scale), (x + dx * 3, y + 8 * scale * (1 - t) + 2), SIL, 2.2, 0.6, n=12)

def build(seed: int = 7) -> Image.Image:
    rnd = random.Random(seed)
    img = Image.new("RGB", (W, H))
    px = img.load()
    for y in range(H):
        for x in range(W):
            if y < HORIZON:
                px[x, y] = ramp(SKY, (y / HORIZON) ** 1.15, x, y)
            elif y < 140:
                px[x, y] = ramp(SEA, (y - HORIZON) / 40, x, y)
            else:
                px[x, y] = FORE if y >= 165 else SAND
    for _ in range(46):
        x, y = rnd.randrange(W), rnd.randrange(0, 52)
        px[x, y] = STAR
        if rnd.random() < 0.2 and x + 1 < W:
            px[x + 1, y] = STAR
    cx, r = 160, 27
    for y in range(HORIZON - r, HORIZON):
        if y > 78 and (y - 78) % 5 in (3, 4):
            continue
        half = int(math.sqrt(max(0, r * r - (y - HORIZON) ** 2)))
        for x in range(cx - half, cx + half + 1):
            px[x, y] = SUN_TOP if y < 88 else SUN_BOT
    for y in range(HORIZON + 1, 140):
        spread = 6 + (y - HORIZON) // 2
        for x in range(cx - spread, cx + spread):
            if rnd.random() < 0.45 and (y - HORIZON) % 2 == 0:
                px[x, y] = SUN_BOT if rnd.random() < 0.7 else SUN_TOP
        if (y - HORIZON) % 4 == 0:
            for _ in range(5):
                x0, ln = rnd.randrange(W - 12), rnd.randrange(4, 12)
                for x in range(x0, x0 + ln):
                    px[x, y] = hex2rgb("#a02f58")
    for x in range(W):
        e = 140 + int(1.5 * math.sin(x / 9) + rnd.random())
        for y in range(e, e + 2):
            px[x, y] = WET
    d = ImageDraw.Draw(img)
    palm(d, 36, (66, 86), 14)
    palm(d, 290, (254, 108), -10, scale=0.8)
    return img

def upscale(img: Image.Image, k: int) -> Image.Image:
    return img.resize((img.width * k, img.height * k), Image.NEAREST)

def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--seed", type=int, default=7)
    ap.add_argument("--out", default="assets/hero-scene")
    ap.add_argument("--scales", type=int, nargs="*", default=[4, 6])
    a = ap.parse_args()
    base = build(a.seed)
    base.save(f"{a.out}-320x180.png")
    for k in a.scales:
        upscale(base, k).save(f"{a.out}@{k}x.png")
    print("saved", a.out)

if __name__ == "__main__":
    main()

