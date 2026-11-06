"use client";

import { motion } from "motion/react";
import Link from "next/link";
import { ArrowUpRight } from "lucide-react";

export default function TypographicInterlude() {
  return (
    <section className="relative py-28 px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-brandy-dark via-brandy to-espresso-surface border-y border-brandy-light/40 overflow-hidden text-center">
      {/* Decorative ambient backdrop */}
      <div className="absolute inset-0 bg-tactile-grid opacity-20 pointer-events-none" />

      <div className="max-w-4xl mx-auto relative z-10">
        <motion.p
          initial={{ opacity: 0, y: 15 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.6 }}
          className="text-xs sm:text-sm font-mono tracking-widest uppercase text-flamingo font-bold mb-6"
        >
          A MANIFESTO FOR DEVELOPER INFRASTRUCTURE
        </motion.p>

        <motion.h2
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7, delay: 0.1 }}
          className="font-editorial text-4xl sm:text-6xl md:text-7xl lg:text-8xl text-custard tracking-tight leading-[1.05] mb-8 font-normal"
        >
          Engineering deserves <br />
          <span className="italic font-normal text-gradient-editorial">better tools.</span>
        </motion.h2>

        <motion.p
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7, delay: 0.2 }}
          className="text-ivory/80 text-base sm:text-lg lg:text-xl font-normal max-w-2xl mx-auto leading-relaxed mb-10"
        >
          AI coding agents shouldn&apos;t run on fragile string templates, unverified shell commands, or unversioned configurations. AEK brings Rust-grade determinism, cryptographic integrity, and architectural discipline to modern software development.
        </motion.p>

        <motion.div
          initial={{ opacity: 0, scale: 0.95 }}
          whileInView={{ opacity: 1, scale: 1 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5, delay: 0.3 }}
          className="inline-flex items-center gap-4"
        >
          <Link
            href="/docs/getting-started"
            className="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-custard text-brandy-dark font-semibold text-sm hover:bg-ivory transition-all shadow-tactile"
          >
            Explore the Architecture
            <ArrowUpRight className="w-4 h-4 text-brandy" />
          </Link>
          <Link
            href="https://github.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 px-6 py-3 rounded-xl border border-custard/40 text-ivory text-sm hover:border-flamingo hover:text-flamingo transition-all"
          >
            GitHub Repository
          </Link>
        </motion.div>
      </div>
    </section>
  );
}

