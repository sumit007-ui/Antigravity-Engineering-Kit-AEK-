"use client";

import { useEffect, useState } from "react";
import Image from "next/image";

export default function BootLoader() {
  const [visible, setVisible] = useState(false);
  const [progress, setProgress] = useState(0);
  const [exiting, setExiting] = useState(false);

  useEffect(() => {
    // Only run in browser
    if (typeof window === "undefined") return;

    // Honor prefers-reduced-motion: skip loader immediately
    const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    if (prefersReducedMotion) return;

    // Check if previously booted in this session
    const hasBooted = sessionStorage.getItem("aek_boot_completed");
    if (hasBooted) return;

    setVisible(true);

    const startTime = Date.now();
    const duration = 1200; // Fast retro boot sequence (~1.2s)

    const timer = setInterval(() => {
      const elapsed = Date.now() - startTime;
      const pct = Math.min(100, Math.floor((elapsed / duration) * 100));
      setProgress(pct);

      if (pct >= 100) {
        clearInterval(timer);
        dismissLoader();
      }
    }, 30);

    return () => clearInterval(timer);
  }, []);

  const dismissLoader = () => {
    setExiting(true);
    try {
      sessionStorage.setItem("aek_boot_completed", "true");
    } catch {
      // Ignore storage errors in private browsing
    }
    setTimeout(() => {
      setVisible(false);
    }, 400);
  };

  if (!visible) return null;

  return (
    <div
      onClick={dismissLoader}
      role="dialog"
      aria-label="System Initializing"
      className={`fixed inset-0 z-50 flex flex-col items-center justify-center bg-space-black transition-transform duration-400 cursor-pointer select-none ${
        exiting ? "-translate-y-full opacity-0" : "translate-y-0 opacity-100"
      }`}
    >
      <div className="absolute inset-0 bg-tactile-grid opacity-15 pointer-events-none" />

      <div className="max-w-md w-full px-6 text-center relative z-10 space-y-5">
        <div className="w-16 h-16 mx-auto rounded-2xl bg-space-surface border-2 border-primary-gold flex items-center justify-center shadow-hard-md">
          <span className="font-editorial text-2xl font-bold text-paper-beige">AEK</span>
        </div>

        <div>
          <div className="text-[11px] font-mono text-primary-gold tracking-widest uppercase mb-1 font-bold">
            ✦ SYSTEM_KERNEL • INITIALIZING
          </div>
          <h2 className="font-editorial text-2xl text-paper-beige font-bold">
            Antigravity Engineering Kit
          </h2>
          <p className="text-xs font-mono text-sand/80 mt-1">
            BOOTING RUST ENGINE v0.1.0 ...
          </p>
        </div>

        {/* 16-bit Segmented Progress Bar */}
        <div className="w-full bg-space-surface rounded border-2 border-space-border h-4 overflow-hidden p-0.5 shadow-hard-sm">
          <div
            className="h-full bg-warm-orange transition-all duration-75 ease-linear rounded-sm"
            style={{ width: `${progress}%` }}
          />
        </div>

        <div className="flex items-center justify-between text-[11px] font-mono text-primary-gold">
          <span>PROGRESS</span>
          <span className="tabular-nums font-bold">[{progress.toString().padStart(3, " ")}%]</span>
        </div>

        <div className="pt-4 text-[10px] font-mono text-sand/60">
          [ CLICK ANYWHERE OR PRESS ESC TO SKIP ]
        </div>
      </div>
    </div>
  );
}
