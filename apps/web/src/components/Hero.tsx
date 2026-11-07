"use client";

import { useEffect, useState } from "react";
import Image from "next/image";
import Link from "next/link";
import {
  ArrowRight,
  Terminal,
  Download,
  CheckCircle2,
  Clock,
  ChevronDown,
  Copy,
  Check,
  ShieldCheck,
  Sparkles,
} from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import TactileButton from "@/components/primitives/TactileButton";
import DecryptText from "@/components/motion/DecryptText";

export default function Hero() {
  const [timeStr, setTimeStr] = useState<string>("00:00:00 UTC");
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    const updateTime = () => {
      const now = new Date();
      setTimeStr(now.toTimeString().split(" ")[0] + " UTC");
    };
    updateTime();
    const interval = setInterval(updateTime, 1000);
    return () => clearInterval(interval);
  }, []);

  const handleCopyCmd = () => {
    navigator.clipboard.writeText("cargo install aek-cli");
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <section className="relative min-h-[calc(100vh-4.5rem)] flex flex-col justify-between pt-6 pb-12 px-4 sm:px-6 lg:px-8 overflow-hidden bg-space-black">
      {/* 16-bit Coastal Pixel Art Sunset Background */}
      <div className="absolute inset-0 z-0 overflow-hidden pointer-events-none select-none">
        <Image
          src="/images/hero-scene@4x.png"
          alt="16-bit Coastal Sunset Pixel Art"
          fill
          priority
          sizes="100vw"
          className="object-cover object-bottom opacity-35 mix-blend-screen pixelated"
        />
        {/* Coastal sunset gradient overlay */}
        <div className="absolute inset-0 bg-gradient-to-t from-space-black via-space-black/75 to-transparent" />
        <div className="absolute inset-0 bg-tactile-grid opacity-15" />
      </div>

      {/* Main One-Page Fit Grid */}
      <div className="max-w-content w-full mx-auto relative z-10 grid grid-cols-1 lg:grid-cols-12 gap-8 lg:gap-12 items-center my-auto">
        {/* Left Column: Text & Editorial Hierarchy (Left te text show kr) */}
        <div className="lg:col-span-7 flex flex-col items-center lg:items-start text-center lg:text-left">
          {/* System Status & Time Awareness Beacon */}
          <div className="inline-flex flex-wrap items-center gap-2 px-3.5 py-1.5 rounded-full border-2 border-primary-gold/40 bg-space-surface/90 text-xs font-mono text-primary-gold mb-4 sm:mb-6 shadow-hard-sm">
            <span className="relative flex h-2.5 w-2.5">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
              <span className="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500" />
            </span>
            <span className="font-bold tracking-wider">
              <DecryptText text="AEK_CORE::ONLINE" />
            </span>
            <span className="text-secondary-mauve">•</span>
            <span className="text-paper-beige font-semibold">AUTONOMOUS ENGINE</span>
            <span className="text-secondary-mauve">•</span>
            <span className="text-primary-gold font-mono font-bold flex items-center gap-1">
              <Clock className="w-3 h-3 text-primary-gold" />
              <span className="tabular-nums">{timeStr}</span>
            </span>
          </div>

          {/* Primary Editorial Headline */}
          <h1 className="font-editorial text-4xl sm:text-6xl lg:text-7xl xl:text-8xl text-paper-beige tracking-tight leading-[1.04] mb-4">
            One Engineering Kit. <br />
            <span className="italic font-normal text-gradient-editorial">
              Autonomous Precision.
            </span>
          </h1>

          {/* Secondary Subtitle */}
          <p className="text-base sm:text-lg lg:text-xl text-sand/90 font-normal max-w-2xl leading-relaxed mb-3">
            The high-performance Rust foundation built for developers who ship software with autonomous AI agents.
          </p>

          <p className="text-xs sm:text-sm text-secondary-text max-w-xl mb-6">
            Unify global engineering rules, 15 specialized skills, Model Context Protocol (MCP), and automated test-driven validation with{" "}
            <strong className="text-primary-gold font-bold">ruthless precision</strong> across Antigravity, Claude Code, Cursor, and OpenCode.
          </p>

          {/* Verified Quality Invariants */}
          <div className="flex flex-wrap items-center justify-center lg:justify-start gap-4 sm:gap-6 text-xs text-sand/80 pt-4 border-t border-space-border/60">
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
              <span>Zero Shell-Injection Risk</span>
            </div>
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
              <span>Sub-50 MB Memory Footprint</span>
            </div>
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
              <span>Cryptographic Rollbacks</span>
            </div>
          </div>
        </div>

        {/* Right Column: Tactical Action Deck & Buttons (Right button) */}
        <div className="lg:col-span-5 flex flex-col items-center justify-center w-full max-w-md mx-auto lg:mx-0">
          <div className="w-full rounded-2xl bg-space-surface/95 border-2 border-primary-gold/40 p-6 shadow-hard-lg relative backdrop-blur-md">
            {/* Terminal Card Header */}
            <div className="flex items-center justify-between pb-3 mb-5 border-b-2 border-space-border text-xs font-mono">
              <div className="flex items-center gap-2">
                <span className="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse" />
                <span className="font-bold text-paper-beige">AEK_LAUNCH_DECK</span>
              </div>
              <span className="text-[11px] text-primary-gold font-bold">v{LATEST_VERSION} RUST</span>
            </div>

            {/* Action Buttons Stack */}
            <div className="space-y-3.5">
              {/* Primary Download Button */}
              <Link href="/download" className="block w-full">
                <TactileButton variant="gold" size="lg" hotkey="D" className="w-full flex items-center justify-center gap-2.5 text-space-black font-bold text-sm">
                  <Download className="w-4 h-4 text-space-black" />
                  <span>Get AEK v{LATEST_VERSION}</span>
                </TactileButton>
              </Link>

              {/* Quickstart Guide Button */}
              <Link href="/docs/getting-started" className="block w-full">
                <TactileButton variant="secondary" size="lg" hotkey="G" className="w-full flex items-center justify-center gap-2 text-paper-beige font-semibold text-sm">
                  <span>Quickstart Guide</span>
                  <ArrowRight className="w-4 h-4 text-warm-orange" />
                </TactileButton>
              </Link>

              {/* Cargo CLI Install Command Box */}
              <div className="pt-2">
                <div className="flex items-center justify-between text-[11px] font-mono text-sand/70 mb-1.5 px-1">
                  <span>RUST CRATE INSTALLATION</span>
                  <span className="text-primary-gold font-bold">[C]</span>
                </div>
                <div className="flex items-center justify-between gap-2 p-3 rounded-xl bg-space-black border-2 border-space-border text-xs font-mono shadow-hard-sm">
                  <div className="flex items-center gap-2 text-paper-beige overflow-hidden">
                    <Terminal className="w-4 h-4 text-primary-gold shrink-0" />
                    <code className="text-primary-gold select-all truncate">cargo install aek-cli</code>
                  </div>
                  <button
                    onClick={handleCopyCmd}
                    className="p-1.5 rounded-lg bg-space-surface border border-primary-gold/40 text-primary-gold hover:text-paper-beige hover:border-primary-gold transition-colors shrink-0"
                    title="Copy command"
                  >
                    {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                  </button>
                </div>
              </div>
            </div>

            {/* Platform Badges */}
            <div className="mt-5 pt-3.5 border-t border-space-border/60 flex items-center justify-between text-[10px] font-mono text-sand/60">
              <span className="flex items-center gap-1.5">
                <ShieldCheck className="w-3.5 h-3.5 text-emerald-400" />
                <span>OFFLINE-FIRST</span>
              </span>
              <span>WINDOWS • MACOS • LINUX</span>
            </div>
          </div>
        </div>
      </div>

      {/* Subtle Scroll Cue */}
      <div className="w-full flex flex-col items-center justify-center pt-2 pb-1 text-[10px] font-mono text-primary-gold/70 select-none">
        <span className="tracking-widest uppercase mb-0.5">▼ SCROLL TO DECRYPT SYSTEM</span>
        <ChevronDown className="w-3.5 h-3.5 animate-bounce text-warm-orange" />
      </div>
    </section>
  );
}
