"use client";

import { useEffect, useState } from "react";
import Image from "next/image";
import Link from "next/link";
import { ArrowRight, Terminal, Download, CheckCircle2, Clock, ChevronDown, Radio, Activity, ShieldCheck } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import TactileButton from "@/components/primitives/TactileButton";
import DecryptText from "@/components/motion/DecryptText";

export default function Hero() {
  const [timeStr, setTimeStr] = useState<string>("00:00:00 UTC");
  const [scrollY, setScrollY] = useState(0);

  useEffect(() => {
    // Clock update
    const updateTime = () => {
      const now = new Date();
      setTimeStr(now.toTimeString().split(" ")[0] + " UTC");
    };
    updateTime();
    const interval = setInterval(updateTime, 1000);

    // Parallax update (Recipe A2)
    const handleScroll = () => {
      // Limit to 8% of scroll
      setScrollY(window.scrollY * 0.08);
    };

    const prefersReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    if (!prefersReducedMotion) {
      window.addEventListener("scroll", handleScroll, { passive: true });
    }

    return () => {
      clearInterval(interval);
      window.removeEventListener("scroll", handleScroll);
    };
  }, []);

  return (
    <section className="relative min-h-[95vh] flex flex-col justify-between pt-8 pb-12 px-4 sm:px-6 lg:px-8 overflow-hidden bg-space-black">
      {/* 16-bit Coastal Pixel Art Background with Ambient Parallax (A2) */}
      <div
        className="absolute inset-0 z-0 overflow-hidden pointer-events-none select-none transition-transform duration-75 ease-out"
        style={{ transform: `translate3d(0, ${scrollY}px, 0)` }}
      >
        <Image
          src="/images/hero-scene@4x.png"
          alt="16-bit Coastal Sunset Pixel Art"
          fill
          priority
          sizes="100vw"
          className="object-cover object-bottom opacity-25 mix-blend-screen pixelated"
        />
        {/* Coastal sunset gradient overlay */}
        <div className="absolute inset-0 bg-gradient-to-t from-space-black via-space-black/80 to-transparent" />
        <div className="absolute inset-0 bg-tactile-grid opacity-15" />
      </div>

      <div className="max-w-content w-full mx-auto relative z-10 grid grid-cols-1 lg:grid-cols-12 gap-12 items-center my-auto">
        {/* Left Column: Editorial Headline & Actions */}
        <div className="lg:col-span-7 flex flex-col items-center lg:items-start text-center lg:text-left">
          
          {/* Classified Radar & Time Awareness Beacon (A2 & A3) */}
          <div className="inline-flex flex-wrap items-center gap-2 px-3.5 py-1.5 rounded-full border-2 border-primary-gold/40 bg-space-violet/90 text-xs font-mono text-primary-gold mb-6 shadow-hard-sm">
            <span className="relative flex h-2.5 w-2.5">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
              <span className="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-500" />
            </span>
            <span className="font-bold tracking-wider">
              <DecryptText text="15°29'N 73°49'E" />
            </span>
            <span className="text-secondary-mauve">•</span>
            <span className="text-paper-beige font-semibold">SEC_00: RADAR ACTIVE</span>
            <span className="text-secondary-mauve">•</span>
            <span className="text-primary-gold font-mono font-bold flex items-center gap-1">
              <Clock className="w-3 h-3 text-primary-gold" />
              <span className="tabular-nums">{timeStr}</span>
            </span>
          </div>

          {/* Primary Editorial Headline */}
          <h1 className="font-editorial text-5xl sm:text-7xl lg:text-8xl text-paper-beige tracking-tight leading-[1.02] mb-6">
            One Engineering Kit. <br />
            <span className="italic font-normal text-gradient-editorial">
              Autonomous Precision.
            </span>
          </h1>

          {/* Secondary Subtitle */}
          <p className="text-lg sm:text-xl text-sand/90 font-normal max-w-2xl leading-relaxed mb-4">
            The high-performance Rust foundation built for developers who ship software with autonomous AI agents.
          </p>

          <p className="text-xs sm:text-sm text-secondary-text max-w-xl mb-8">
            Unify global engineering rules, 15 specialized skills, Model Context Protocol (MCP), and automated test-driven validation with <strong className="text-primary-gold font-bold">ruthless precision</strong> across Antigravity, Claude Code, Cursor, and OpenCode.
          </p>

          {/* Action CTAs - Neo-Brutalist Tactile Buttons with Hotkeys */}
          <div className="flex flex-wrap items-center justify-center lg:justify-start gap-4 mb-10">
            <Link href="/download">
              <TactileButton variant="gold" size="lg" hotkey="D" className="flex items-center gap-2">
                <Download className="w-4 h-4 text-space-black" />
                <span>Get AEK v{LATEST_VERSION}</span>
              </TactileButton>
            </Link>

            <Link href="/docs/getting-started">
              <TactileButton variant="secondary" size="lg" hotkey="G" className="flex items-center gap-2">
                <span>Quickstart Guide</span>
                <ArrowRight className="w-4 h-4 text-warm-orange" />
              </TactileButton>
            </Link>

            <a
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-block"
            >
              <TactileButton variant="terminal" size="lg" hotkey="C" className="flex items-center gap-2">
                <Terminal className="w-4 h-4 text-primary-gold" />
                <span>cargo install aek-cli</span>
              </TactileButton>
            </a>
          </div>

          {/* Verified Quality Badges */}
          <div className="flex flex-wrap items-center justify-center lg:justify-start gap-6 text-xs text-sand/80 pt-6 border-t border-space-border/60">
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400" />
              <span>Zero Shell-Injection Risk</span>
            </div>
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400" />
              <span>Sub-50 MB Memory Footprint</span>
            </div>
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-emerald-400" />
              <span>Cryptographic Rollbacks</span>
            </div>
          </div>
        </div>

        {/* Right Column: 16-Bit Coastal Pixel Radar Station & HUD Telemetry */}
        <div className="lg:col-span-5 relative flex items-center justify-center">
          <div className="relative w-full max-w-[480px] rounded-3xl bg-space-black/95 border-2 border-primary-gold/40 p-6 shadow-hard-lg overflow-hidden group">
            {/* Ambient sunset glow backdrop */}
            <div className="absolute inset-0 bg-gradient-to-tr from-warm-orange/15 via-burnt-orange/10 to-transparent pointer-events-none" />
            <div className="absolute inset-0 bg-tactile-grid opacity-20 pointer-events-none" />

            {/* Tactical Classified Header Bar */}
            <div className="flex items-center justify-between border-b-2 border-space-border/80 pb-3 mb-4 text-xs font-mono">
              <div className="flex items-center gap-2">
                <span className="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse" />
                <span className="font-bold text-paper-beige">COASTAL_RADAR::STATION_00</span>
              </div>
              <span className="text-[11px] text-primary-gold font-bold">1420.405 MHz</span>
            </div>

            {/* Pixel Art Visual Showcase: Beach Shack & Horizon */}
            <div className="relative w-full h-56 rounded-2xl border-2 border-space-border overflow-hidden bg-space-deep flex items-center justify-center">
              <Image
                src="/images/beach-shack@4x.png"
                alt="16-bit Beach Shack Terminal at Dusk"
                fill
                sizes="(max-width: 768px) 100vw, 480px"
                className="object-cover pixelated"
                priority
              />
              <div className="absolute inset-0 bg-gradient-to-t from-space-black/80 via-transparent to-transparent pointer-events-none" />

              {/* Radar Crosshair Sweep Overlay */}
              <div className="absolute inset-0 pointer-events-none border border-primary-gold/20 flex items-center justify-center">
                <div className="w-24 h-24 rounded-full border border-primary-gold/30 animate-radar" />
                <div className="absolute w-full h-[1px] bg-primary-gold/20" />
                <div className="absolute h-full w-[1px] bg-primary-gold/20" />
              </div>

              {/* Inset Pixel Crest Badge */}
              <div className="absolute top-3 right-3 w-12 h-12 rounded-xl bg-space-black/80 border border-primary-gold/60 p-1 shadow-hard-sm">
                <Image
                  src="/images/radar-crest@4x.png"
                  alt="Coastal Radar Crest"
                  width={40}
                  height={40}
                  className="pixelated object-contain"
                />
              </div>

              {/* Live Tactical Overlay Readout */}
              <div className="absolute bottom-3 left-3 px-2.5 py-1 rounded bg-space-black/90 border border-warm-orange/50 text-[10px] font-mono text-warm-orange flex items-center gap-1.5 shadow-hard-sm">
                <Radio className="w-3 h-3 text-warm-orange animate-pulse" />
                <span>RUST_SUBPROCESS::ACTIVE</span>
              </div>
            </div>

            {/* Telemetry Metric Matrix */}
            <div className="grid grid-cols-3 gap-2.5 mt-4 font-mono text-center">
              <div className="p-2.5 rounded-xl bg-space-surface border border-space-border/80">
                <span className="block text-[10px] text-sand/60">KERNEL</span>
                <span className="text-xs font-bold text-primary-gold">AEK_v{LATEST_VERSION}</span>
              </div>
              <div className="p-2.5 rounded-xl bg-space-surface border border-space-border/80">
                <span className="block text-[10px] text-sand/60">MEMORY</span>
                <span className="text-xs font-bold text-emerald-400">&lt; 32MB</span>
              </div>
              <div className="p-2.5 rounded-xl bg-space-surface border border-space-border/80">
                <span className="block text-[10px] text-sand/60">GATES</span>
                <span className="text-xs font-bold text-paper-beige">8/8 PASS</span>
              </div>
            </div>

            {/* Dimensional Classified Caption */}
            <div className="mt-4 pt-3 border-t border-space-border/60 flex items-center justify-between text-[11px] font-mono text-sand/70">
              <span className="flex items-center gap-1.5">
                <ShieldCheck className="w-3.5 h-3.5 text-emerald-400" />
                <span>NO TELEMETRY • OFFLINE-FIRST</span>
              </span>
              <span className="text-primary-gold font-bold">15°29&apos;N 73°49&apos;E</span>
            </div>
          </div>
        </div>
      </div>

      {/* A2: SCROLL Cue Indicator */}
      <div className="w-full flex flex-col items-center justify-center pt-8 text-[11px] font-mono text-primary-gold/70 select-none">
        <span className="tracking-widest uppercase mb-1">▼ SCROLL TO DECRYPT SYSTEM</span>
        <ChevronDown className="w-4 h-4 animate-bounce text-warm-orange" />
      </div>
    </section>
  );
}
