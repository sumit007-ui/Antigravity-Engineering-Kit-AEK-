"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Menu, X, ArrowUpRight, Github, Terminal } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import MagneticButton from "@/components/motion/MagneticButton";

const NAV_LINKS = [
  { name: "Features", href: "/features" },
  { name: "Agents", href: "/agents" },
  { name: "Integrations", href: "/integrations" },
  { name: "Architecture", href: "/architecture" },
  { name: "Docs", href: "/docs" },
  { name: "Download", href: "/download" },
];

export default function Navbar() {
  const [scrolled, setScrolled] = useState(false);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const pathname = usePathname();

  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 20);
    };
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  return (
    <header
      className={`sticky top-0 z-50 w-full transition-all duration-300 border-b ${
        scrolled
          ? "bg-espresso-deep/90 backdrop-blur-md border-espresso-border shadow-tactile"
          : "bg-transparent border-transparent"
      }`}
    >
      <div className="max-w-content mx-auto px-4 sm:px-6 lg:px-8 h-18 flex items-center justify-between">
        {/* Brand Monogram & Title */}
        <Link href="/" className="flex items-center gap-3 group">
          <div className="w-10 h-10 rounded-xl bg-plum border-2 border-popcorn/70 flex items-center justify-center shadow-[3px_3px_0px_rgba(248,216,120,0.3)] group-hover:shadow-[3px_3px_0px_#98B4D4] group-hover:border-cerulean transition-all">
            <span className="font-editorial text-lg font-black text-popcorn group-hover:text-cerulean transition-colors">
              AEK
            </span>
          </div>
          <div className="flex flex-col">
            <div className="flex items-center gap-2">
              <span className="font-editorial text-lg font-bold text-ivory tracking-tight group-hover:text-popcorn transition-colors">
                Antigravity Kit
              </span>
              <span className="flex h-2 w-2 relative">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
                <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500" />
              </span>
            </div>
            <span className="text-[10px] font-mono tracking-wider text-cerulean uppercase font-bold">
              v{LATEST_VERSION} RUST • ONLINE
            </span>
          </div>
        </Link>

        {/* Desktop Navigation Links */}
        <nav className="hidden md:flex items-center gap-1 bg-plum-surface/90 px-3 py-1.5 rounded-full border border-plum-border backdrop-blur-md shadow-[2px_2px_0px_rgba(0,0,0,0.3)]">
          {NAV_LINKS.map((link) => {
            const isActive = pathname.startsWith(link.href);
            return (
              <Link
                key={link.name}
                href={link.href}
                className={`relative px-4 py-1.5 text-xs font-semibold rounded-full transition-all duration-200 ${
                  isActive
                    ? "text-plum-deep bg-popcorn font-bold shadow-[2px_2px_0px_rgba(0,0,0,0.2)]"
                    : "text-sand/90 hover:text-ivory hover:bg-plum/60"
                }`}
              >
                {link.name}
              </Link>
            );
          })}
        </nav>

        {/* Right Action Buttons */}
        <div className="hidden lg:flex items-center gap-3">
          <a
            href={GITHUB_REPO_URL}
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-2 px-3.5 py-1.5 rounded-xl text-xs font-mono font-bold text-sand hover:text-ivory border-2 border-plum-border hover:border-cerulean shadow-[2px_2px_0px_rgba(0,0,0,0.3)] active:translate-x-[1px] active:translate-y-[1px] active:shadow-none transition-all bg-plum-surface/70"
          >
            <Github className="w-3.5 h-3.5 text-popcorn" />
            <span>GitHub ★</span>
          </a>

          <Link href="/docs/getting-started">
            <button className="py-1.5 px-4 text-xs font-bold bg-popcorn text-plum-deep rounded-xl border-2 border-popcorn shadow-[3px_3px_0px_#241B1C] hover:brightness-105 active:translate-x-[1px] active:translate-y-[1px] active:shadow-none transition-all cursor-pointer">
              Launch Kit →
            </button>
          </Link>
        </div>

        {/* Mobile Menu Button */}
        <button
          onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
          className="md:hidden p-2 rounded-xl text-sand hover:text-ivory border border-espresso-border"
          aria-label="Toggle menu"
        >
          {mobileMenuOpen ? <X className="w-5 h-5" /> : <Menu className="w-5 h-5" />}
        </button>
      </div>

      {/* Mobile Drawer */}
      {mobileMenuOpen && (
        <div className="md:hidden border-b border-espresso-border bg-espresso-surface/95 backdrop-blur-lg px-6 py-6 space-y-4">
          <div className="flex flex-col space-y-3">
            {NAV_LINKS.map((link) => (
              <Link
                key={link.name}
                href={link.href}
                onClick={() => setMobileMenuOpen(false)}
                className="text-sm font-medium text-sand hover:text-ivory py-1"
              >
                {link.name}
              </Link>
            ))}
          </div>
          <div className="pt-4 border-t border-espresso-border flex flex-col gap-3">
            <a
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center justify-center gap-2 py-2.5 rounded-xl text-xs font-mono border border-espresso-border text-sand"
            >
              <Github className="w-4 h-4" />
              <span>GitHub Repository</span>
            </a>
            <Link
              href="/download"
              onClick={() => setMobileMenuOpen(false)}
              className="flex items-center justify-center py-2.5 rounded-xl text-xs font-semibold bg-brandy text-ivory border border-flamingo/40"
            >
              Download AEK v{LATEST_VERSION}
            </Link>
          </div>
        </div>
      )}
    </header>
  );
}
