"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import Image from "next/image";
import { usePathname, useRouter } from "next/navigation";
import { Terminal, Github, LayoutGrid, Star } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import ConsoleMenu from "@/components/primitives/ConsoleMenu";
import TactileButton from "@/components/primitives/TactileButton";
import GitHubStarButton from "@/components/motion/GitHubStarButton";

const NAV_ITEMS = [
  { name: "Features", href: "/features" },
  { name: "Agents", href: "/agents" },
  { name: "Integrations", href: "/integrations" },
  { name: "Architecture", href: "/architecture" },
  { name: "Docs", href: "/docs" },
  { name: "Download", href: "/download" },
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
        className={`sticky top-0 z-40 w-full transition-all duration-160 border-b ${
          scrolled
            ? "bg-bg/95 backdrop-blur-xl border-border shadow-hard-md"
            : "bg-bg/85 backdrop-blur-md border-border/40"
        }`}
      >
        <div className="max-w-content mx-auto px-3 sm:px-6 lg:px-8 h-16 sm:h-18 flex items-center justify-between gap-2 sm:gap-4">
          {/* Pure Transparent Brand Symbol (No Box / No Background) */}
          <Link href="/" className="flex items-center gap-2.5 sm:gap-3 group shrink-0">
            <div className="relative flex items-center justify-center w-8 h-8 sm:w-9 sm:h-9">
              <Image
                src="/icons/symbol.png"
                alt="AEK Emblem"
                width={36}
                height={36}
                className="w-full h-full object-contain filter drop-shadow-[0_0_8px_rgba(244,102,27,0.6)] group-hover:drop-shadow-[0_0_14px_rgba(255,211,107,0.9)] group-hover:scale-110 transition-all duration-200"
                priority
              />
            </div>
            <div className="flex flex-col">
              <span className="font-display text-sm sm:text-base font-black text-fg tracking-tight group-hover:text-[#FFD36B] transition-colors leading-tight">
                Antigravity Kit
              </span>
              <span className="text-[10px] font-mono tracking-wider text-sand/80 uppercase font-semibold">
                ONE KIT • EVERY AGENT • V{LATEST_VERSION}
              </span>
            </div>
          </Link>

          {/* Desktop Navigation Links without Hotkey Alphabet Badges (Clean Golden Style) */}
          <nav
            aria-label="Main Navigation"
            className="hidden lg:flex items-center gap-1 xl:gap-2 bg-[#0D0B18]/70 backdrop-blur-md px-3 py-1.5 rounded-xl border border-[#FFD36B]/20 shadow-hard-sm"
          >
            {NAV_ITEMS.map((item) => {
              const isActive = pathname.startsWith(item.href);
              return (
                <Link
                  key={item.name}
                  href={item.href}
                  className={`px-3 py-1 text-sm font-semibold tracking-wide transition-all select-none rounded-lg ${
                    isActive
                      ? "text-[#FFD36B] bg-[#FFD36B]/15 drop-shadow-[0_0_8px_rgba(255,211,107,0.4)]"
                      : "text-[#FFD36B]/85 hover:text-fg hover:drop-shadow-[0_0_6px_rgba(255,211,107,0.3)] hover:bg-[#FFD36B]/10"
                  }`}
                >
                  {item.name}
                </Link>
              );
            })}
          </nav>

          {/* Right Action Bar (Spacious & Balanced) */}
          <div className="flex items-center gap-1.5 sm:gap-2.5 shrink-0">
            {/* Live Direct GitHub Star Button with Golden Glow Hover */}
            <GitHubStarButton />

            {/* Console Menu Trigger Button */}
            <button
              onClick={() => setConsoleOpen(true)}
              className="flex items-center gap-1.5 px-2.5 sm:px-3 py-1.5 sm:py-2 rounded-lg border-2 border-border bg-surface-card hover:border-[#FFD36B] text-xs font-mono font-bold text-fg shadow-hard-sm cursor-pointer select-none min-h-[40px] sm:min-h-[44px]"
              aria-label="Open console menu [M]"
            >
              <LayoutGrid className="w-3.5 h-3.5 text-[#FFD36B]" />
              <span className="hidden sm:inline">[M] MENU</span>
              <span className="sm:hidden font-mono text-[11px]">[M]</span>
            </button>

            {/* Launch / Download Primary CTA */}
            <Link href="/download" className="hidden md:inline-flex">
              <TactileButton variant="primary" className="py-2 px-3 sm:px-4 text-xs font-mono">
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
