#!/usr/bin/env python3
"""Generate additional 16-bit coastal pixel art assets following the style bible.
Assets generated:
1. beach-shack: Thatched beach shack at dusk with warm lantern glow on sand.
2. laptop-sand: Open laptop on wooden beach table at dusk with starfield and ocean horizon.
3. radar-crest: Tactical classified coastal radar crest badge.
Outputs:
  apps/web/public/images/<asset>.png
  apps/web/public/images/<asset>@4x.png
"""
from __future__ import annotations
import os, math, random
from PIL import Image, ImageDraw

def hex2rgb(h: str) -> tuple[int, int, int]:
    h = h.lstrip("#")
    return int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16)

# 16-color style bible palette
PALETTE = {
    "BLACK": hex2rgb("#050505"),
    "DEEP_VIOLET": hex2rgb("#120d24"),
    "NIGHT_PURPLE": hex2rgb("#2b1544"),
    "MAUVE_PURPLE": hex2rgb("#5a1f55"),
    "CRIMSON_BERRY": hex2rgb("#a02f58"),
    "BURNT_ORANGE": hex2rgb("#e0553f"),
    "WARM_ORANGE": hex2rgb("#ff8a3d"),
    "PRIMARY_GOLD": hex2rgb("#ffc15e"),
    "PALE_GOLD": hex2rgb("#ffd36b"),
    "PAPER_BEIGE": hex2rgb("#f5e9c8"),
    "PLUM": hex2rgb("#6a2459"),
    "DARK_SAND": hex2rgb("#3b2418"),
    "WARM_SAND": hex2rgb("#7a4630"),
    "DEEP_NIGHT": hex2rgb("#07070a"),
    "EMERALD_GLOW": hex2rgb("#10b981"),
    "SKY_CYAN": hex2rgb("#38bdf8"),
}

BAYER4 = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]]

def save_variants(img: Image.Image, name: str, out_dir: str):
    os.makedirs(out_dir, exist_ok=True)
    img_1x = os.path.join(out_dir, f"{name}.png")
    img_4x = os.path.join(out_dir, f"{name}@4x.png")
    img.save(img_1x)
    img.resize((img.width * 4, img.height * 4), Image.Resampling.NEAREST).save(img_4x)
    print(f"Generated {img_1x} ({img.width}x{img.height}) and {img_4x}")

def generate_beach_shack(w=160, h=160) -> Image.Image:
    img = Image.new("RGB", (w, h), PALETTE["BLACK"])
    draw = ImageDraw.Draw(img)
    # Sky gradient
    for y in range(h // 2):
        t = y / (h // 2)
        color = PALETTE["NIGHT_PURPLE"] if t < 0.5 else PALETTE["CRIMSON_BERRY"]
        draw.line([(0, y), (w, y)], fill=color)
    # Stars
    rnd = random.Random(42)
    for _ in range(20):
        draw.point((rnd.randint(0, w - 1), rnd.randint(0, 50)), fill=PALETTE["PAPER_BEIGE"])
    # Sand ground
    for y in range(h // 2, h):
        c = PALETTE["DARK_SAND"] if y > h // 2 + 30 else PALETTE["WARM_SAND"]
        draw.line([(0, y), (w, y)], fill=c)

    # Wooden Shack Body (symmetrical centered)
    shack_w, shack_h = 70, 48
    shack_x = (w - shack_w) // 2
    shack_y = h // 2 + 10
    draw.rectangle([shack_x, shack_y, shack_x + shack_w, shack_y + shack_h], fill=PALETTE["DARK_SAND"], outline=PALETTE["BLACK"])
    # Wood planks horizontal lines
    for py in range(shack_y + 8, shack_y + shack_h, 8):
        draw.line([(shack_x, py), (shack_x + shack_w, py)], fill=PALETTE["DEEP_NIGHT"])

    # Thatched Roof (trapezoid / stepped pyramid)
    roof_bot_w = 90
    roof_h = 24
    roof_bot_x = (w - roof_bot_w) // 2
    roof_bot_y = shack_y
    for i in range(roof_h):
        cur_w = roof_bot_w - (i * 2)
        cur_x = (w - cur_w) // 2
        cur_y = roof_bot_y - i
        color = PALETTE["PRIMARY_GOLD"] if (i % 4 == 0) else PALETTE["WARM_ORANGE"]
        draw.line([(cur_x, cur_y), (cur_x + cur_w, cur_y)], fill=color)

    # Door
    door_w, door_h = 16, 28
    door_x = (w - door_w) // 2
    door_y = shack_y + shack_h - door_h
    draw.rectangle([door_x, door_y, door_x + door_w, door_y + door_h], fill=PALETTE["DEEP_VIOLET"], outline=PALETTE["BLACK"])

    # Glowing Window (warm light)
    win_w, win_h = 14, 14
    win_x = shack_x + 8
    win_y = shack_y + 12
    draw.rectangle([win_x, win_y, win_x + win_w, win_y + win_h], fill=PALETTE["PALE_GOLD"], outline=PALETTE["BURNT_ORANGE"])
    # Window cross
    draw.line([(win_x + win_w // 2, win_y), (win_x + win_w // 2, win_y + win_h)], fill=PALETTE["DARK_SAND"])
    draw.line([(win_x, win_y + win_h // 2), (win_x + win_w, win_y + win_h // 2)], fill=PALETTE["DARK_SAND"])

    # Warm Lantern on right side
    lan_x = shack_x + shack_w - 14
    lan_y = shack_y + 14
    draw.rectangle([lan_x, lan_y, lan_x + 6, lan_y + 8], fill=PALETTE["PALE_GOLD"], outline=PALETTE["BURNT_ORANGE"])
    draw.point((lan_x + 3, lan_y + 4), fill=PALETTE["PAPER_BEIGE"])

    # Lantern string / wire
    draw.line([(shack_x, shack_y - 2), (lan_x + 3, lan_y)], fill=PALETTE["WARM_ORANGE"])

    return img

def generate_laptop_sand(w=96, h=96) -> Image.Image:
    img = Image.new("RGB", (w, h), PALETTE["BLACK"])
    draw = ImageDraw.Draw(img)
    # Sky
    for y in range(48):
        t = y / 48
        c = PALETTE["NIGHT_PURPLE"] if t < 0.6 else PALETTE["CRIMSON_BERRY"]
        draw.line([(0, y), (w, y)], fill=c)
    # Sunset Horizon line
    draw.line([(0, 48), (w, 48)], fill=PALETTE["PRIMARY_GOLD"])
    # Ocean reflection
    for y in range(49, 64):
        draw.line([(0, y), (w, y)], fill=PALETTE["PLUM"])
    # Beach sand
    for y in range(64, h):
        draw.line([(0, y), (w, y)], fill=PALETTE["DARK_SAND"])

    # Stars
    for (sx, sy) in [(12, 10), (45, 8), (78, 14), (88, 22), (28, 26)]:
        draw.point((sx, sy), fill=PALETTE["PAPER_BEIGE"])

    # Wooden table top
    tbl_x, tbl_y, tbl_w, tbl_h = 16, 68, 64, 8
    draw.rectangle([tbl_x, tbl_y, tbl_x + tbl_w, tbl_y + tbl_h], fill=PALETTE["WARM_SAND"], outline=PALETTE["DEEP_NIGHT"])
    # Table legs
    draw.line([(tbl_x + 6, tbl_y + tbl_h), (tbl_x + 6, h - 2)], fill=PALETTE["DARK_SAND"], width=3)
    draw.line([(tbl_x + tbl_w - 6, tbl_y + tbl_h), (tbl_x + tbl_w - 6, h - 2)], fill=PALETTE["DARK_SAND"], width=3)

    # Laptop Base (on table)
    lap_base_x, lap_base_y = 30, tbl_y - 3
    draw.rectangle([lap_base_x, lap_base_y, lap_base_x + 32, lap_base_y + 3], fill=PALETTE["DEEP_VIOLET"], outline=PALETTE["BLACK"])

    # Laptop Screen (open at angle)
    scr_x, scr_y = 33, lap_base_y - 20
    draw.rectangle([scr_x, scr_y, scr_x + 26, scr_y + 19], fill=PALETTE["DEEP_NIGHT"], outline=PALETTE["PRIMARY_GOLD"])
    # Screen display glowing gold (terminal code lines)
    draw.rectangle([scr_x + 2, scr_y + 2, scr_x + 24, scr_y + 17], fill=PALETTE["BLACK"])
    # Code syntax lines on screen
    draw.line([(scr_x + 4, scr_y + 5), (scr_x + 14, scr_y + 5)], fill=PALETTE["PALE_GOLD"])
    draw.line([(scr_x + 4, scr_y + 8), (scr_x + 20, scr_y + 8)], fill=PALETTE["WARM_ORANGE"])
    draw.line([(scr_x + 4, scr_y + 11), (scr_x + 16, scr_y + 11)], fill=PALETTE["EMERALD_GLOW"])
    draw.line([(scr_x + 4, scr_y + 14), (scr_x + 10, scr_y + 14)], fill=PALETTE["PRIMARY_GOLD"])

    # Coffee mug beside laptop
    mug_x, mug_y = 66, tbl_y - 8
    draw.rectangle([mug_x, mug_y, mug_x + 6, mug_y + 7], fill=PALETTE["PAPER_BEIGE"], outline=PALETTE["BLACK"])
    # Handle
    draw.point((mug_x + 7, mug_y + 3), fill=PALETTE["PAPER_BEIGE"])

    return img

def generate_radar_crest(w=128, h=128) -> Image.Image:
    img = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    cx, cy, r = w // 2, h // 2, 54

    # Outer classified octagonal / circular ring
    draw.ellipse([cx - r, cy - r, cx + r, cy + r], fill=PALETTE["BLACK"], outline=PALETTE["PRIMARY_GOLD"], width=2)
    # Inner border
    draw.ellipse([cx - r + 4, cy - r + 4, cx + r - 4, cy + r - 4], outline=PALETTE["WARM_ORANGE"], width=1)

    # Sunset inside crest
    for y in range(cy - r + 6, cy + 10):
        t = (y - (cy - r + 6)) / (r + 4)
        c = PALETTE["NIGHT_PURPLE"] if t < 0.4 else (PALETTE["CRIMSON_BERRY"] if t < 0.7 else PALETTE["BURNT_ORANGE"])
        # Only fill inside circle
        half = int(math.sqrt(max(0, (r - 6)**2 - (y - cy)**2)))
        draw.line([(cx - half, y), (cx + half, y)], fill=c)

    # Ocean inside crest
    for y in range(cy + 10, cy + r - 6):
        half = int(math.sqrt(max(0, (r - 6)**2 - (y - cy)**2)))
        draw.line([(cx - half, y), (cx + half, y)], fill=PALETTE["PLUM"])

    # Sun with horizontal retro cut lines
    sun_r = 16
    sun_cy = cy + 4
    for y in range(sun_cy - sun_r, sun_cy + sun_r):
        if (y - (sun_cy - sun_r)) % 4 in (2, 3) and y > sun_cy:
            continue
        half = int(math.sqrt(max(0, sun_r**2 - (y - sun_cy)**2)))
        draw.line([(cx - half, y), (cx + half, y)], fill=PALETTE["PALE_GOLD"])

    # Radar crosshair grid
    draw.line([(cx - r + 8, cy), (cx + r - 8, cy)], fill=(*PALETTE["PRIMARY_GOLD"], 120))
    draw.line([(cx, cy - r + 8), (cx, cy + r - 8)], fill=(*PALETTE["PRIMARY_GOLD"], 120))

    # Corner classified crosshairs
    for (px, py) in [(6, 6), (w - 7, 6), (6, h - 7), (w - 7, h - 7)]:
        draw.point((px, py), fill=PALETTE["PRIMARY_GOLD"])

    return img

if __name__ == "__main__":
    out_dir = os.path.join(os.path.dirname(__file__), "..", "apps", "web", "public", "images")
    save_variants(generate_beach_shack(), "beach-shack", out_dir)
    save_variants(generate_laptop_sand(), "laptop-sand", out_dir)
    save_variants(generate_radar_crest(), "radar-crest", out_dir)
    print("All pixel assets successfully generated!")

