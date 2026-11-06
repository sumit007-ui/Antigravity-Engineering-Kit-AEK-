import type { Config } from "tailwindcss";
import typography from "@tailwindcss/typography";

const config: Config = {
  darkMode: ["class"],
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        // Pantone Curated Palette
        // 1. PANTONE 19-1110 TCX Chocolate Plum
        plum: {
          DEFAULT: "#382C2D",
          deep: "#241B1C",     // Obsidian Plum Background
          surface: "#302324",  // Card / Container Surface
          elevated: "#3D2D2E", // Elevated Modals & Flyouts
          border: "#4E3A3C",   // Precision Border
          subtle: "rgba(56, 44, 45, 0.6)",
        },
        // 2. PANTONE 12-0825 TCX Popcorn
        popcorn: {
          DEFAULT: "#F8D878",  // Popcorn Yellow
          50: "#FFFAEE",
          100: "#FFF3D1",
          200: "#FCE6A4",
          300: "#F8D878",
          400: "#E5BE52",
          500: "#CCA135",
        },
        // 3. PANTONE 15-4020 TCX Cerulean
        cerulean: {
          DEFAULT: "#98B4D4",  // Cerulean Blue
          hover: "#7FA2C8",
          light: "#C8D9EC",
          glow: "rgba(152, 180, 212, 0.25)",
        },
        ivory: {
          DEFAULT: "#FFF9F0",
          pure: "#FFFFFA",
          muted: "#F5EFE6",
        },

        // Backward-compatible Theme Aliases
        custard: {
          DEFAULT: "#F8D878", // Popcorn
          50: "#FFFAEE",
          100: "#FFF3D1",
          200: "#FCE6A4",
          300: "#F8D878",
          400: "#E5BE52",
        },
        flamingo: {
          DEFAULT: "#98B4D4", // Cerulean
          hover: "#7FA2C8",
          light: "#C8D9EC",
          muted: "#98B4D4",
          glow: "rgba(152, 180, 212, 0.25)",
        },
        brandy: {
          DEFAULT: "#382C2D", // Chocolate Plum
          hover: "#4A393A",
          dark: "#241B1C",
          light: "#5A4547",
          subtle: "rgba(56, 44, 45, 0.25)",
        },
        espresso: {
          DEFAULT: "#302324", // Chocolate Plum Surface
          deep: "#241B1C",    // Obsidian Plum
          surface: "#302324", // Surface
          elevated: "#3D2D2E",// Elevated
          border: "#4E3A3C",  // Border
          subtle: "rgba(36, 27, 28, 0.7)",
        },
        dustyRose: "#C8D9EC",
        cocoa: "#A89495",
        sand: "#F8D878",

        // Semantic System Mappings
        background: "#241B1C",
        surface: "#302324",
        "surface-elevated": "#3D2D2E",
        "surface-border": "#4E3A3C",
        "primary-text": "#FFF9F0",
        "secondary-text": "#F8D878",
        "muted-text": "#A89495",
        border: "#4E3A3C",
        accent: {
          DEFAULT: "#98B4D4",
          hover: "#7FA2C8",
          plum: "#382C2D",
          popcorn: "#F8D878",
          cerulean: "#98B4D4",
        },
        status: {
          success: "#34D399",
          error: "#F87171",
          warning: "#FBBF24",
          info: "#98B4D4",
        },
      },
      fontFamily: {
        serif: [
          "var(--font-editorial)",
          "Playfair Display",
          "Newsreader",
          "Instrument Serif",
          "Georgia",
          "serif",
        ],
        sans: [
          "var(--font-sans)",
          "Plus Jakarta Sans",
          "Geist Sans",
          "-apple-system",
          "BlinkMacSystemFont",
          "sans-serif",
        ],
        mono: [
          "var(--font-mono)",
          "Geist Mono",
          "JetBrains Mono",
          "monospace",
        ],
      },
      maxWidth: {
        content: "1320px",
      },
      boxShadow: {
        tactile: "0 4px 20px -2px rgba(24, 17, 18, 0.8), 0 2px 6px -1px rgba(56, 44, 45, 0.3)",
        "tactile-hover": "0 12px 32px -4px rgba(24, 17, 18, 0.9), 0 4px 12px -2px rgba(152, 180, 212, 0.3)",
        ceramic: "inset 0 1px 1px 0 rgba(255, 249, 240, 0.15), 0 8px 24px -4px rgba(24, 17, 18, 0.7)",
        glow: "0 0 50px -10px rgba(152, 180, 212, 0.3)",
        "glow-plum": "0 0 50px -10px rgba(56, 44, 45, 0.4)",
        "glow-popcorn": "0 0 50px -10px rgba(248, 216, 120, 0.25)",
      },
    },
  },
  plugins: [typography],
};

export default config;
