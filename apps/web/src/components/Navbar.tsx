"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import { Terminal, Github, LayoutGrid, Star } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import ConsoleMenu from "@/components/primitives/ConsoleMenu";
import TactileButton from "@/components/primitives/TactileButton";
import GitHubStarButton from "@/components/motion/GitHubStarButton";

const NAV_ITEMS = [
  { name: "About", href: "/about", hotkey: "A" },
  { name: "Features", href: "/features", hotkey: "F" },
  { name: "Agents", href: "/agents", hotkey: "G" },
  { name: "Integrations", href: "/integrations", hotkey: "I" },
  { name: "Docs", href: "/docs", hotkey: "D" },
];

export default function Navbar() {
  const [scrolled, setScrolled] = useState(false);
  const [consoleOpen, setConsoleOpen] = useState(false);
  const pathname = usePathname();
  const router = useRouter();

  // Scroll detection
  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 20);
    };
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  // Keyboard Navigation Hotkeys
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const tag = (e.target as HTMLElement)?.tagName?.toLowerCase();
      if (tag === "input" || tag === "textarea" || (e.target as HTMLElement)?.isContentEditable) {
        return;
      }

      if (e.key === "m" || e.key === "M") {
        e.preventDefault();
        setConsoleOpen((prev) => !prev);
      } else if (!consoleOpen) {
        if (e.key === "a" || e.key === "A") {
          router.push("/about");
        } else if (e.key === "f" || e.key === "F") {
          router.push("/features");
        } else if (e.key === "d" || e.key === "D") {
          router.push("/docs");
        } else if (e.key === "i" || e.key === "I") {
          router.push("/integrations");
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [consoleOpen, router]);

  return (
    <>
      <header
        className={`sticky top-0 z-40 w-full transition-all duration-160 border-b-2 ${
          scrolled
            ? "bg-bg/95 backdrop-blur-xl border-border shadow-hard-md"
            : "bg-transparent border-transparent"
        }`}
      >
        <div className="max-w-content mx-auto px-4 sm:px-6 lg:px-8 h-18 flex items-center justify-between">
          {/* Brand Monogram & Coordinates Badge */}
          <Link href="/" className="flex items-center gap-3 group">
            <div className="w-10 h-10 rounded-lg bg-surface-card border-2 border-accent-gold flex items-center justify-center shadow-hard-sm group-hover:border-accent-primary transition-all">
              <span className="font-display text-base font-black text-accent-gold group-hover:text-accent-primary transition-colors">
                AEK
              </span>
            </div>
            <div className="flex flex-col">
              <div className="flex items-center gap-2">
                <span className="font-display text-base font-black text-fg tracking-tight group-hover:text-accent-primary transition-colors">
                  Antigravity Kit
                </span>
                <span className="w-2 h-2 rounded-full bg-success animate-radar" />
              </div>
              <span className="text-[10px] font-mono tracking-wider text-muted uppercase">
                ONE KIT • EVERY AGENT • v{LATEST_VERSION}
              </span>
            </div>
          </Link>

          {/* Desktop Navigation with Hotkey Hints */}
          <nav
            aria-label="Main Navigation"
            className="hidden md:flex items-center gap-1 bg-surface-card px-2.5 py-1.5 rounded-lg border-2 border-border shadow-hard-sm"
          >
            {NAV_ITEMS.map((item) => {
              const isActive = pathname.startsWith(item.href);
              return (
                <Link
                  key={item.name}
                  href={item.href}
                  className={`px-3 py-1.5 text-xs font-mono font-bold tracking-wider uppercase rounded transition-all select-none flex items-center gap-1.5 ${
                    isActive
                      ? "bg-accent-primary text-black border border-black shadow-hard-sm"
                      : "text-muted hover:text-fg hover:bg-surface-elevated"
                  }`}
                >
                  <span className="text-[10px] text-accent-gold opacity-80">[{item.hotkey}]</span>
                  <span>{item.name}</span>
                </Link>
              );
            })}
          </nav>

          {/* Right Action Bar */}
          <div className="flex items-center gap-2 sm:gap-3">
            {/* Aesthetic Motion-Powered GitHub Star Button */}
            <GitHubStarButton stars="1.2k" />

            {/* Console Menu Trigger Button */}
            <button
              onClick={() => setConsoleOpen(true)}
              className="flex items-center gap-1.5 px-3 py-2 rounded-lg border-2 border-border bg-surface-card hover:border-accent-primary text-xs font-mono font-bold text-fg shadow-hard-sm cursor-pointer select-none min-h-[44px]"
              aria-label="Open console menu [M]"
            >
              <LayoutGrid className="w-3.5 h-3.5 text-accent-gold" />
              <span className="hidden sm:inline">[M] MENU</span>
              <span className="sm:hidden">[M]</span>
            </button>

            {/* Launch / Download Primary CTA */}
            <Link href="/download" className="hidden sm:inline-flex">
              <TactileButton variant="primary">
                Launch Kit ↗
              </TactileButton>
            </Link>
          </div>
        </div>
      </header>

      {/* Fullscreen Console Menu */}
      <ConsoleMenu isOpen={consoleOpen} onClose={() => setConsoleOpen(false)} />
    </>
  );
}
