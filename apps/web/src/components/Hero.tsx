"use client";

import { motion } from "motion/react";
import Link from "next/link";
import { ArrowRight, Terminal, Shield, Download, Sparkles, CheckCircle2 } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import HeroArtifact from "@/components/three/HeroArtifact";
import MagneticButton from "@/components/motion/MagneticButton";

export default function Hero() {
  return (
    <section className="relative min-h-[92vh] flex items-center justify-center pt-8 pb-20 px-4 sm:px-6 lg:px-8 overflow-hidden bg-espresso-deep">
      {/* Background ambient lighting */}
      <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[500px] bg-radial-gradient opacity-40 pointer-events-none" />
      <div className="absolute inset-0 bg-tactile-grid opacity-20 pointer-events-none" />

      <div className="max-w-content w-full mx-auto relative z-10 grid grid-cols-1 lg:grid-cols-12 gap-12 items-center">
        {/* Left Column: Editorial Headline & Actions */}
        <div className="lg:col-span-7 flex flex-col items-center lg:items-start text-center lg:text-left">
          {/* Eyebrow Pill */}
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
            className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-6 shadow-tactile"
          >
            <span className="w-2 h-2 rounded-full bg-flamingo animate-pulse" />
            <span>AEK v{LATEST_VERSION} OPEN-SOURCE LAUNCH</span>
            <span className="text-espresso-border">•</span>
            <span className="text-sand/80">Rust 2024 Edition</span>
          </motion.div>

          {/* Primary Editorial Headline */}
          <motion.h1
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.1 }}
            className="font-editorial text-5xl sm:text-7xl lg:text-8xl text-ivory tracking-tight leading-[1.02] mb-6"
          >
            One Engineering Kit. <br />
            <span className="italic font-normal text-gradient-editorial">
              Infinite Possibilities.
            </span>
          </motion.h1>

          {/* Secondary Subtitle */}
          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.2 }}
            className="text-lg sm:text-xl text-sand/90 font-normal max-w-2xl leading-relaxed mb-4"
          >
            One powerful Rust-powered foundation for modern AI coding agents.
          </motion.p>

          <motion.p
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.25 }}
            className="text-xs sm:text-sm text-secondary-text max-w-xl mb-8"
          >
            Unify global engineering rules, specialized agent skills, MCP integrations, multi-language toolchains, prompt optimization, and automated validation with <strong className="text-custard font-medium">ruthless precision</strong> across Antigravity, Claude Code, Cursor, and OpenCode.
          </motion.p>

          {/* Action CTAs */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.35 }}
            className="flex flex-wrap items-center justify-center lg:justify-start gap-4 mb-10"
          >
            <Link href="/download">
              <MagneticButton variant="primary" className="!px-6 !py-3.5 !text-sm font-semibold flex items-center gap-2">
                <Download className="w-4 h-4 text-custard" />
                <span>Download for Windows & Linux</span>
              </MagneticButton>
            </Link>

            <Link href="/docs/getting-started">
              <MagneticButton variant="secondary" className="!px-6 !py-3.5 !text-sm font-medium flex items-center gap-2">
                <span>Quickstart Guide</span>
                <ArrowRight className="w-4 h-4 text-flamingo" />
              </MagneticButton>
            </Link>

            <a
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="px-4 py-3 text-xs font-mono text-sand/90 hover:text-ivory border border-espresso-border rounded-xl bg-espresso-surface/50 hover:border-flamingo/50 transition-all flex items-center gap-2"
            >
              <span>View Source</span>
              <span className="text-[10px] px-1.5 py-0.5 rounded bg-brandy text-custard font-mono">v{LATEST_VERSION}</span>
            </a>
          </motion.div>

          {/* Verified Quality Badges */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.8, delay: 0.5 }}
            className="flex flex-wrap items-center justify-center lg:justify-start gap-6 text-xs text-sand/80 pt-6 border-t border-espresso-border/60"
          >
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
          </motion.div>
        </div>

        {/* Right Column: 3D AEK Sculptural Artifact */}
        <div className="lg:col-span-5 relative flex items-center justify-center">
          <div className="relative w-full max-w-[500px]">
            {/* Ambient backdrop glow */}
            <div className="absolute inset-0 rounded-full bg-gradient-to-tr from-brandy/30 via-flamingo/20 to-custard/10 blur-3xl" />
            
            {/* 3D Scene */}
            <HeroArtifact />

            {/* Dimensional floating caption */}
            <div className="absolute bottom-2 left-1/2 -translate-x-1/2 px-4 py-1.5 rounded-full bg-espresso-surface/90 border border-espresso-border text-[11px] font-mono text-custard backdrop-blur-md shadow-tactile whitespace-nowrap">
              ✦ Precision-Machined Monogram Artifact
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
