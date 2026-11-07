"use client";

import React, { useEffect, useRef } from "react";
import Link from "next/link";
import { X, ArrowUpRight, Terminal, Github } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";

interface ConsoleMenuProps {
  isOpen: boolean;
  onClose: () => void;
}

const CONSOLE_LINKS = [
  { num: "01", name: "HOME", href: "/", hotkey: "H" },
  { num: "02", name: "FEATURES", href: "/features", hotkey: "F" },
  { num: "03", name: "AGENTS", href: "/agents", hotkey: "A" },
  { num: "04", name: "INTEGRATIONS", href: "/integrations", hotkey: "I" },
  { num: "05", name: "ARCHITECTURE", href: "/architecture", hotkey: "C" },
  { num: "06", name: "DOCUMENTATION", href: "/docs", hotkey: "D" },
  { num: "07", name: "DOWNLOAD CLI", href: "/download", hotkey: "L" },
  { num: "08", name: "CHANGELOG", href: "/changelog", hotkey: "N" },
  { num: "09", name: "ABOUT", href: "/about", hotkey: "B" },
  { num: "10", name: "TRUST CENTER", href: "/privacy", hotkey: "T" },
];

export default function ConsoleMenu({ isOpen, onClose }: ConsoleMenuProps) {
  const overlayRef = useRef<HTMLDivElement>(null);

  // Close on ESC key + handle hotkeys
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape" && isOpen) {
        onClose();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onClose]);

  // Lock scroll when open
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = "hidden";
    } else {
      document.body.style.overflow = "";
    }
    return () => {
      document.body.style.overflow = "";
    };
  }, [isOpen]);

  if (!isOpen) return null;

  return (
    <div
      ref={overlayRef}
      role="dialog"
      aria-modal="true"
      aria-label="Classified Console Navigation"
      className="fixed inset-0 z-50 bg-black/95 backdrop-blur-2xl flex flex-col justify-between p-6 sm:p-10 lg:p-16 overflow-y-auto animate-fade-in"
    >
      {/* Top Bar */}
      <div className="flex items-center justify-between border-b-2 border-border pb-6 select-none">
        <div className="flex items-center gap-3">
          <span className="w-3 h-3 rounded-full bg-accent-primary animate-radar" />
          <span className="font-mono text-xs font-bold tracking-widest uppercase text-accent-gold">
            SYS_CONSOLE // 15°29&apos;N 73°49&apos;E
          </span>
          <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-surface-card border border-border text-accent-secondary">
            v{LATEST_VERSION} RUST
          </span>
        </div>

        <button
          onClick={onClose}
          className="flex items-center gap-2 px-3 py-2 rounded border-2 border-border bg-surface-card text-xs font-mono font-bold text-fg hover:border-accent-primary hover:text-accent-primary transition-all shadow-hard-sm cursor-pointer select-none min-h-[44px]"
          aria-label="Close menu [ESC]"
        >
          <span>[ESC] CLOSE</span>
          <X className="w-4 h-4" />
        </button>
      </div>

      {/* Main Console Columns */}
      <div className="grid grid-cols-1 md:grid-cols-12 gap-8 my-auto py-10">
        {/* Left Column: Numbered Navigation Links */}
        <div className="md:col-span-8 space-y-2">
          <p className="text-[11px] font-mono text-muted tracking-widest uppercase mb-4">
            [INDEX DIRECTORY // TAP OR PRESS HOTKEY]
          </p>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
            {CONSOLE_LINKS.map((link) => (
              <Link
                key={link.num}
                href={link.href}
                onClick={onClose}
                className="group flex items-center justify-between p-3.5 rounded border border-border bg-surface-card hover:bg-surface-elevated hover:border-accent-primary transition-all shadow-hard-sm"
              >
                <div className="flex items-center gap-3">
                  <span className="text-xs font-mono font-bold text-accent-gold">
                    {link.num} /
                  </span>
                  <span className="font-display text-lg font-bold text-fg group-hover:text-accent-primary transition-colors">
                    {link.name}
                  </span>
                </div>
                <span className="text-[10px] font-mono px-2 py-0.5 rounded border border-border text-muted group-hover:text-fg group-hover:border-accent-primary">
                  [{link.hotkey}]
                </span>
              </Link>
            ))}
          </div>
        </div>

        {/* Right Column: Mission Specs & Live Telemetry */}
        <div className="md:col-span-4 space-y-6 flex flex-col justify-between border-t md:border-t-0 md:border-l-2 border-border pt-6 md:pt-0 md:pl-8">
          <div>
            <p className="text-[11px] font-mono text-muted tracking-widest uppercase mb-3">
              [SYSTEM STATUS]
            </p>
            <div className="space-y-3 font-mono text-xs">
              <div className="p-3 rounded border border-border bg-surface-card flex items-center justify-between">
                <span className="text-muted">WORKSPACES:</span>
                <span className="font-bold text-fg">16 RUST CRATES</span>
              </div>
              <div className="p-3 rounded border border-border bg-surface-card flex items-center justify-between">
                <span className="text-muted">CI STATUS:</span>
                <span className="font-bold text-success">100% GREEN</span>
              </div>
              <div className="p-3 rounded border border-border bg-surface-card flex items-center justify-between">
                <span className="text-muted">AGENTS:</span>
                <span className="font-bold text-accent-secondary">4 UNIFIED</span>
              </div>
            </div>
          </div>

          <div className="pt-4 border-t border-border">
            <p className="text-[11px] font-mono text-muted tracking-widest uppercase mb-3">
              [CONNECT & SOURCE]
            </p>
            <a
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center justify-between p-3.5 rounded border-2 border-border bg-surface-card hover:border-accent-gold text-xs font-mono font-bold text-fg shadow-hard-sm"
            >
              <div className="flex items-center gap-2">
                <Github className="w-4 h-4 text-accent-gold" />
                <span>GITHUB REPOSITORY</span>
              </div>
              <ArrowUpRight className="w-4 h-4 text-accent-primary" />
            </a>
          </div>
        </div>
      </div>

      {/* Bottom Bar: Coordinates & Status */}
      <div className="border-t-2 border-border pt-6 flex flex-wrap items-center justify-between text-xs font-mono text-muted select-none">
        <div>
          <span>ANTIGRAVITY ENGINEERING KIT</span> • <span>RUST 2024</span>
        </div>
        <div>
          <span>GOOD VIBES BUILD BETTER SOFTWARE</span>
        </div>
      </div>
    </div>
  );
}
