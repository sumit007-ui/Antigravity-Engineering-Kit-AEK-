"use client";

import Link from "next/link";
import { Github, Download, Star, ShieldCheck, Terminal, GitBranch } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import MagneticButton from "@/components/motion/MagneticButton";

export default function OpenSourceSection() {
  return (
    <section className="py-24 px-4 sm:px-6 lg:px-8 bg-espresso-deep border-t border-espresso-border relative overflow-hidden">
      <div className="max-w-4xl mx-auto rounded-3xl ceramic-card p-8 sm:p-12 border border-espresso-border shadow-tactile-hover text-center relative z-10">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-6">
          <Star className="w-3.5 h-3.5 text-flamingo fill-flamingo" />
          <span>FREE & OPEN SOURCE SOFTWARE</span>
        </div>

        <h2 className="font-editorial text-3xl sm:text-5xl lg:text-6xl text-ivory tracking-tight mb-4">
          Ready to elevate your <br />
          <span className="italic text-gradient-editorial">engineering environment?</span>
        </h2>

        <p className="text-secondary-text text-sm sm:text-base max-w-xl mx-auto mb-8 leading-relaxed">
          AEK is free, open source, and dual-licensed under Apache 2.0 or MIT. Designed with ruthless precision for professional software teams and AI agent developers.
        </p>

        <div className="flex flex-wrap items-center justify-center gap-4 mb-8">
          <Link href="/download">
            <MagneticButton variant="primary" className="!px-7 !py-3.5 !text-sm font-semibold flex items-center gap-2">
              <Download className="w-4 h-4 text-custard" />
              <span>Download AEK v{LATEST_VERSION}</span>
            </MagneticButton>
          </Link>

          <a
            href={GITHUB_REPO_URL}
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-2 px-6 py-3.5 rounded-xl bg-espresso-surface hover:bg-espresso-elevated border border-espresso-border text-custard hover:text-ivory text-sm font-mono transition-all"
          >
            <Github className="w-4 h-4" />
            <span>Star on GitHub</span>
          </a>
        </div>

        {/* Quick Install Snippets */}
        <div className="max-w-2xl mx-auto p-4 rounded-2xl bg-espresso-deep border border-espresso-border text-left font-mono text-xs">
          <div className="flex items-center justify-between text-muted-text text-[11px] mb-2 pb-2 border-b border-espresso-border/60">
            <span>ONE-LINE USER INSTALLATION</span>
            <span>NO ADMIN/ROOT REQUIRED</span>
          </div>
          <div className="space-y-2">
            <div>
              <span className="text-sand/60 select-none mr-2">Windows:</span>
              <code className="text-custard select-all">
                irm https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/windows/install.ps1 | iex
              </code>
            </div>
            <div>
              <span className="text-sand/60 select-none mr-2">Linux:</span>
              <code className="text-custard select-all">
                curl -fsSL https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/linux/install.sh | bash
              </code>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
