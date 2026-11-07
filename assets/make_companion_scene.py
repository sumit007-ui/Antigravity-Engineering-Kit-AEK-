from PIL import Image, ImageDraw
import os

user_img_path = r"C:\Users\hp\.gemini\antigravity\brain\d24cd519-9bc8-4c41-a8de-16e606c0fa57\.user_uploaded\media_1794043612018_67b2663c.png"
out_dir = r"apps\web\public\images"
os.makedirs(out_dir, exist_ok=True)

# 1. Load user's master image for TOP (Hero)
hero_img = Image.open(user_img_path).convert("RGBA")
# Save 1x and nearest-neighbor 4x (1280x720)
hero_img.save(os.path.join(out_dir, "hero-scene.png"))
hero_4x = hero_img.resize((hero_img.width * 4, hero_img.height * 4), Image.Resampling.NEAREST)
hero_4x.save(os.path.join(out_dir, "hero-scene@4x.png"))
print("Saved hero-scene.png and hero-scene@4x.png")

# 2. Create bottom image based on the exact same master image
# We crop and frame the ocean/beach area and add a clean, authentic, non-cartoon pixel developer workstation
bot_img = hero_img.copy()
draw = ImageDraw.Draw(bot_img)

# Exact colors from the master image
C_BLACK = (5, 5, 5, 255)
C_DARK_SAND = (59, 36, 24, 255)
C_WARM_SAND = (122, 70, 48, 255)
C_GOLD = (255, 193, 94, 255)
C_PALE_GOLD = (255, 211, 107, 255)
C_ORANGE = (255, 138, 61, 255)
C_BEIGE = (245, 233, 200, 255)
C_CYAN = (56, 189, 248, 255)
C_VIOLET = (18, 13, 36, 255)

# Place a clean, crisp, minimalist pixel developer workstation on the sand (centered at x=160, y=145)
# Wooden table surface
tbl_w, tbl_h = 72, 6
tbl_x = (320 - tbl_w) // 2
tbl_y = 146

# Table legs (anchored in sand)
draw.rectangle([tbl_x + 8, tbl_y + tbl_h, tbl_x + 11, 172], fill=C_DARK_SAND)
draw.rectangle([tbl_x + tbl_w - 12, tbl_y + tbl_h, tbl_x + tbl_w - 9, 172], fill=C_DARK_SAND)
# Table top plank
draw.rectangle([tbl_x, tbl_y, tbl_x + tbl_w, tbl_y + tbl_h], fill=C_WARM_SAND, outline=C_BLACK)
draw.line([(tbl_x + 1, tbl_y + 2), (tbl_x + tbl_w - 1, tbl_y + 2)], fill=C_GOLD)

# Open laptop on table
lap_w = 34
lap_x = (320 - lap_w) // 2
lap_base_y = tbl_y - 2
draw.rectangle([lap_x, lap_base_y, lap_x + lap_w, lap_base_y + 2], fill=C_VIOLET, outline=C_BLACK)

# Laptop display screen (standing up)
scr_w, scr_h = 28, 20
scr_x = (320 - scr_w) // 2
scr_y = lap_base_y - scr_h
draw.rectangle([scr_x, scr_y, scr_x + scr_w, scr_y + scr_h], fill=C_BLACK, outline=C_GOLD)

# Screen terminal glowing content
draw.rectangle([scr_x + 2, scr_y + 2, scr_x + scr_w - 2, scr_y + scr_h - 2], fill=C_VIOLET)
# Terminal syntax lines (crisp, authentic developer code)
draw.line([(scr_x + 4, scr_y + 5), (scr_x + 14, scr_y + 5)], fill=C_PALE_GOLD)
draw.line([(scr_x + 4, scr_y + 8), (scr_x + 22, scr_y + 8)], fill=C_CYAN)
draw.line([(scr_x + 4, scr_y + 11), (scr_x + 18, scr_y + 11)], fill=C_ORANGE)
draw.line([(scr_x + 4, scr_y + 14), (scr_x + 12, scr_y + 14)], fill=C_GOLD)

# Ceramic coffee mug beside laptop
mug_x, mug_y = lap_x + lap_w + 6, tbl_y - 8
draw.rectangle([mug_x, mug_y, mug_x + 7, mug_y + 7], fill=C_BEIGE, outline=C_BLACK)
draw.point((mug_x + 8, mug_y + 3), fill=C_BEIGE)

# Subtle steam pixels above mug
draw.point((mug_x + 2, mug_y - 2), fill=C_BEIGE)
draw.point((mug_x + 4, mug_y - 4), fill=C_BEIGE)

# Save bottom image 1x and nearest-neighbor 4x (1280x720)
bot_img.save(os.path.join(out_dir, "laptop-sand.png"))
bot_4x = bot_img.resize((bot_img.width * 4, bot_img.height * 4), Image.Resampling.NEAREST)
bot_4x.save(os.path.join(out_dir, "laptop-sand@4x.png"))
print("Saved laptop-sand.png and laptop-sand@4x.png")

