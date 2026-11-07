"use client";

import Link from "next/link";
import { ArrowUpRight } from "lucide-react";
import TactileButton from "@/components/primitives/TactileButton";
import AutonomousAgentSwarm from "@/components/motion/AutonomousAgentSwarm";
import { GITHUB_REPO_URL } from "@/lib/utils";

export default function TypographicInterlude() {
  return (
    <section className="relative py-20 sm:py-28 px-4 sm:px-6 lg:px-8 bg-space-violet border-y-2 border-primary-gold/40 overflow-hidden text-center group/manifesto">
      {/* Interactive Autonomous Cyber Robot Swarm (Follows Cursor) */}
      <AutonomousAgentSwarm />

      {/* Decorative ambient tactile backdrop */}
      <div className="absolute inset-0 bg-tactile-grid opacity-15 pointer-events-none z-1" />

      <div className="max-w-4xl mx-auto relative z-10 pointer-events-auto px-4 py-8 sm:py-10 rounded-3xl bg-space-black/40 backdrop-blur-[2px] border border-primary-gold/20 shadow-hard-md">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-primary-gold/50 bg-space-black/90 text-xs font-mono text-primary-gold mb-6 font-bold shadow-hard-sm">
          <span>§07 — A MANIFESTO FOR DEVELOPER INFRASTRUCTURE</span>
        </div>

        <h2 className="font-editorial text-4xl sm:text-6xl md:text-7xl lg:text-8xl text-paper-beige tracking-tight leading-[1.05] mb-8 font-bold drop-shadow-[0_4px_24px_rgba(5,5,5,0.9)]">
          Engineering deserves <br />
          <span className="italic font-normal text-gradient-editorial">better tools.</span>
        </h2>

        <p className="text-paper-beige text-base sm:text-lg lg:text-xl font-normal max-w-2xl mx-auto leading-relaxed mb-10 drop-shadow-[0_2px_12px_rgba(5,5,5,0.9)]">
          AI coding agents shouldn&apos;t run on fragile string templates, unverified shell commands, or unversioned configurations. AEK brings Rust-grade determinism, cryptographic integrity, and architectural discipline to modern software development.
        </p>

        <div className="inline-flex flex-wrap items-center justify-center gap-4">
          <Link href="/docs/getting-started">
            <TactileButton variant="gold" size="lg" className="flex items-center gap-2">
              <span>Explore the Architecture</span>
              <ArrowUpRight className="w-4 h-4 text-space-black" />
            </TactileButton>
          </Link>
          <a
            href={GITHUB_REPO_URL}
            target="_blank"
            rel="noopener noreferrer"
          >
            <TactileButton variant="secondary" size="lg" className="flex items-center gap-2">
              <span>GitHub Repository</span>
            </TactileButton>
          </a>
        </div>
      </div>
    </section>
  );
}
