"use client";

import { useEffect, useState, useCallback } from "react";
import Image from "next/image";

export default function BootLoader() {
  const [visible, setVisible] = useState(false);
  const [progress, setProgress] = useState(0);
  const [exiting, setExiting] = useState(false);

  const dismissLoader = useCallback(() => {
    setExiting(true);
    try {
      sessionStorage.setItem("aek_boot_completed", "true");
    } catch {
      // Ignore storage errors in private browsing
    }
    setTimeout(() => {
      setVisible(false);
    }, 450);
  }, []);

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
    const duration = 1200; // Fast responsive boot sequence (~1.2s)

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
  }, [dismissLoader]);

  // Handle ESC key to dismiss loader
  useEffect(() => {
    if (!visible || exiting) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        dismissLoader();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [visible, exiting, dismissLoader]);

  // Body scroll lock during boot presentation
  useEffect(() => {
    if (visible && !exiting) {
      const originalOverflow = document.body.style.overflow;
      document.body.style.overflow = "hidden";
      return () => {
        document.body.style.overflow = originalOverflow;
      };
    }
  }, [visible, exiting]);

  if (!visible) return null;

  const getStageTelemetry = (pct: number) => {
    if (pct < 28) return "[01/04] VERIFYING CRYPTOGRAPHIC KERNEL & INTEGRITY...";
    if (pct < 58) return "[02/04] LINKING MULTI-AGENT PROTOCOL & MCP TRANSPORT...";
    if (pct < 88) return "[03/04] OPTIMIZING PROMPT COMPILER & AST DIAGNOSTICS...";
    return "[04/04] AEK SYSTEM READY • LAUNCHING INTERFACE...";
  };

  return (
    <div
      onClick={dismissLoader}
      role="dialog"
      aria-modal="true"
      aria-label="System Initializing"
      tabIndex={-1}
      className={`fixed inset-0 z-50 flex flex-col items-center justify-center bg-[#050505] transition-all duration-500 ease-out cursor-pointer select-none ${
        exiting
          ? "opacity-0 scale-95 -translate-y-4 pointer-events-none"
          : "opacity-100 scale-100 translate-y-0"
      }`}
    >
      {/* Subtle Ambient Radial Lighting */}
      <div className="absolute inset-0 bg-[radial-gradient(circle_at_50%_45%,rgba(255,75,24,0.12),rgba(43,21,68,0.25)_45%,transparent_75%)] pointer-events-none" />

      {/* High-Precision Tactical Grid Overlay */}
      <div className="absolute inset-0 bg-tactile-grid opacity-20 pointer-events-none" />

      {/* Floating Classified Command Card */}
      <div
        onClick={(e) => e.stopPropagation()}
        className="max-w-md w-full mx-4 px-7 py-8 text-center relative z-10 space-y-6 rounded-2xl bg-[#0E0B14]/90 border-2 border-[#2E243A] shadow-[0_20px_50px_rgba(0,0,0,0.8),0_0_40px_rgba(255,75,24,0.15)] backdrop-blur-xl"
      >
        {/* Glowing Geometric Emblem */}
        <div className="relative w-20 h-20 mx-auto flex items-center justify-center">
          <div className="absolute inset-0 rounded-2xl bg-gradient-to-tr from-[#FF4B18]/30 via-[#FFC15E]/20 to-transparent blur-md animate-pulse" />
          <div className="relative w-full h-full rounded-2xl bg-[#150F1F] border-2 border-[#FFC15E] flex items-center justify-center shadow-hard-md overflow-hidden">
            <Image
              src="/icons/symbol.svg"
              alt="AEK Emblem"
              width={48}
              height={48}
              className="w-12 h-12 object-contain drop-shadow-[0_0_10px_rgba(255,193,94,0.5)]"
              priority
            />
            <div className="absolute top-1 left-1.5 text-[8px] font-pixel text-[#FFC15E]/60">01</div>
            <div className="absolute bottom-1 right-1.5 text-[8px] font-mono text-[#FF4B18] font-bold">RUST</div>
          </div>
        </div>

        {/* Header & Kernel Information */}
        <div className="space-y-1.5">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-[#1F162C] border border-[#FFC15E]/40 text-[10px] font-mono text-[#FFC15E] tracking-widest uppercase font-bold shadow-sm">
            <span className="w-1.5 h-1.5 rounded-full bg-[#10B981] animate-ping" />
            <span>✦ SYSTEM_KERNEL • INITIALIZING</span>
          </div>

          <h2 className="font-display text-2xl text-[#F4F4EE] font-bold tracking-tight">
            Antigravity Engineering Kit
          </h2>

          <p className="text-xs font-mono text-[#F5E9C8]/90 flex items-center justify-center gap-1.5">
            <span className="text-[#FF4B18] font-bold">⚡</span>
            <span>BOOTING RUST ENGINE v0.2.0 ...</span>
          </p>
        </div>

        {/* 16-Bit Precision Progress Bar */}
        <div className="space-y-2.5">
          <div
            role="progressbar"
            aria-valuenow={progress}
            aria-valuemin={0}
            aria-valuemax={100}
            aria-label="System Initializing Progress"
            className="w-full bg-[#120D24] rounded-lg border-2 border-[#2E243A] h-4 overflow-hidden p-0.5 shadow-hard-sm relative"
          >
            <div
              className="h-full bg-gradient-to-r from-[#FF4B18] via-[#FF8A3D] to-[#FFC15E] transition-all duration-75 ease-out rounded-sm relative overflow-hidden"
              style={{ width: `${progress}%` }}
            >
              <div className="absolute inset-0 bg-[linear-gradient(90deg,transparent_0%,rgba(255,255,255,0.35)_50%,transparent_100%)] animate-[shimmer_1.5s_infinite]" />
            </div>
          </div>

          <div className="flex items-center justify-between text-[11px] font-mono text-[#FFC15E]">
            <span className="truncate max-w-[270px] text-left text-[10px] text-[#F5E9C8]/70">
              {getStageTelemetry(progress)}
            </span>
            <span className="tabular-nums font-bold tracking-wider shrink-0 ml-2">
              [{progress.toString().padStart(3, " ")}%]
            </span>
          </div>
        </div>

        {/* Interactive Tactical Skip Action */}
        <div className="pt-2">
          <button
            type="button"
            onClick={dismissLoader}
            className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-lg bg-[#1F162C]/80 hover:bg-[#1F162C] border border-[#2E243A] hover:border-[#FFC15E]/60 text-[10px] font-mono text-[#F5E9C8]/80 hover:text-[#F4F4EE] transition-all cursor-pointer shadow-sm group"
          >
            <span>[ CLICK ANYWHERE OR PRESS ESC TO SKIP ]</span>
          </button>
        </div>
      </div>
    </div>
  );
}
