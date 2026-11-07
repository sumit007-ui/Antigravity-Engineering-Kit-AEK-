"""
assets/pixelate.py — Anti-slop 16-color pixel art quantizer & scaler
Usage:
  python pixelate.py raw.png out.png --width 320 --palette "#050505 #120d24 ..." --scale 4
"""

import argparse
import sys
from PIL import Image

DEFAULT_PALETTE = [
    "#050505", "#120D24", "#2B1544", "#5A1F55",
    "#A02F58", "#E0553F", "#FF4B18", "#FF8A3D",
    "#FFC15E", "#FFD36B", "#F5E9C8", "#6A2459",
    "#3B2418", "#7A4630", "#98B4D4", "#F4F4EE"
]

def hex_to_rgb(hex_str):
    hex_str = hex_str.strip().lstrip('#')
    if len(hex_str) == 3:
        hex_str = ''.join([c*2 for c in hex_str])
    return tuple(int(hex_str[i:i+2], 16) for i in (0, 2, 4))

def color_distance_sq(c1, c2):
    return (c1[0]-c2[0])**2 + (c1[1]-c2[1])**2 + (c1[2]-c2[2])**2

def quantize_to_palette(img, palette_rgb):
    # img is RGBA or RGB
    has_alpha = img.mode == 'RGBA'
    pixels = img.load()
    width, height = img.size
    
    for y in range(height):
        for x in range(width):
            p = pixels[x, y]
            if has_alpha and len(p) == 4:
                r, g, b, a = p
                if a < 128:
                    pixels[x, y] = (0, 0, 0, 0)
                    continue
            else:
                r, g, b = p[:3]
                a = 255
            
            # Find closest palette color
            best_color = min(palette_rgb, key=lambda c: color_distance_sq((r, g, b), c))
            if has_alpha:
                pixels[x, y] = (best_color[0], best_color[1], best_color[2], 255)
            else:
                pixels[x, y] = best_color
    return img

def main():
    parser = argparse.ArgumentParser(description="16-bit Pixel Art Quantizer")
    parser.add_argument("input", help="Input image file")
    parser.add_argument("output", help="Output image file")
    parser.add_argument("--width", type=int, default=320, help="Target pixel width")
    parser.add_argument("--palette", type=str, default="", help="Space or comma-separated hex colors")
    parser.add_argument("--scale", type=int, default=4, help="Upscale factor using nearest neighbor")
    args = parser.parse_args()

    # Parse palette
    palette_hex = args.palette.replace(',', ' ').split() if args.palette else DEFAULT_PALETTE
    palette_rgb = [hex_to_rgb(h) for h in palette_hex if h]

    try:
        im = Image.open(args.input)
    except Exception as e:
        print(f"Error opening image: {e}")
        sys.exit(1)

    # Compute target height maintaining aspect ratio
    orig_w, orig_h = im.size
    aspect = orig_h / orig_w
    target_w = args.width
    target_h = max(1, round(target_w * aspect))

    # Resize down with Box or Bilinear filter
    mode = 'RGBA' if im.mode in ('RGBA', 'LA') or ('transparency' in im.info) else 'RGB'
    im = im.convert(mode)
    im_small = im.resize((target_w, target_h), Image.Resampling.BILINEAR)

    # Quantize to strict 16-color palette
    quantized = quantize_to_palette(im_small, palette_rgb)

    # Scale back up with NEAREST neighbor to preserve crisp pixel grid
    if args.scale > 1:
        final_w = target_w * args.scale
        final_h = target_h * args.scale
        quantized = quantized.resize((final_w, final_h), Image.Resampling.NEAREST)

    quantized.save(args.output)
    print(f"Exported pixel art: {args.output} ({quantized.size[0]}x{quantized.size[1]}) with {len(palette_rgb)} colors.")

if __name__ == "__main__":
    main()

