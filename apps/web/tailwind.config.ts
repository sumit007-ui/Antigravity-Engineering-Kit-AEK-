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

        // Retro 16-Bit / Classified HUD Palette Tokens
        bg: "#050505",
        fg: "#F4F4EE",
        "surface-base": "#0E0B14",
        "surface-card": "#150F1F",
        "surface-elevated": "#1F162C",
        "accent-primary": "#FF4B18",
        "accent-secondary": "#98B4D4",
        "accent-gold": "#FFC15E",
        "accent-sand": "#F5E9C8",

        // Semantic System Mappings
        background: "#050505",
        surface: "#0E0B14",
        "surface-border": "#2E243A",
        "primary-text": "#F4F4EE",
        "secondary-text": "#F5E9C8",
        "muted-text": "#8A8199",
        border: "#2E243A",
        accent: {
          DEFAULT: "#FF4B18",
          hover: "#FF8A3D",
          plum: "#2B1544",
          popcorn: "#FFC15E",
          cerulean: "#98B4D4",
        },
        status: {
          success: "#10B981",
          error: "#EF4444",
          warning: "#FFC15E",
          info: "#98B4D4",
        },
      },
      fontFamily: {
        display: [
          "var(--font-display)",
          "Space Grotesk",
          "-apple-system",
          "sans-serif",
        ],
        body: [
          "var(--font-body)",
          "Inter",
          "system-ui",
          "sans-serif",
        ],
        serif: [
          "var(--font-editorial)",
          "Space Grotesk",
          "sans-serif",
        ],
        sans: [
          "var(--font-body)",
          "Inter",
          "-apple-system",
          "sans-serif",
        ],
        mono: [
          "var(--font-mono)",
          "JetBrains Mono",
          "ui-monospace",
          "monospace",
        ],
        pixel: [
          "var(--font-pixel)",
          "Press Start 2P",
          "monospace",
        ],
      },
      maxWidth: {
        content: "1320px",
      },
      boxShadow: {
        "hard-sm": "2px 2px 0px #050505",
        "hard-md": "4px 4px 0px #050505",
        "hard-primary": "4px 4px 0px #FF4B18",
        "hard-gold": "4px 4px 0px #FFC15E",
        tactile: "4px 4px 0px #050505",
        "tactile-hover": "6px 6px 0px #FF4B18",
        ceramic: "inset 0 1px 1px 0 rgba(255, 249, 240, 0.1), 4px 4px 0px #050505",
        glow: "0 0 30px -5px rgba(255, 75, 24, 0.3)",
        "glow-plum": "0 0 30px -5px rgba(43, 21, 68, 0.4)",
        "glow-popcorn": "0 0 30px -5px rgba(255, 193, 94, 0.3)",
      },
    },
  },
  plugins: [typography],
};

export default config;
