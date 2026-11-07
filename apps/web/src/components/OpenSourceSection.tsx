"use client";

import { useState } from "react";
import Link from "next/link";
import { Github, Download, Star, Check, Copy } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";
import TactileButton from "@/components/primitives/TactileButton";

export default function OpenSourceSection() {
  const [copiedWin, setCopiedWin] = useState(false);
  const [copiedLinux, setCopiedLinux] = useState(false);

  const winCmd = "irm https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/windows/install.ps1 | iex";
  const linuxCmd = "curl -fsSL https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/linux/install.sh | bash";

  const handleCopy = (type: "win" | "linux") => {
    const cmd = type === "win" ? winCmd : linuxCmd;
    navigator.clipboard.writeText(cmd);
    if (type === "win") {
      setCopiedWin(true);
      setTimeout(() => setCopiedWin(false), 2000);
    } else {
      setCopiedLinux(true);
      setTimeout(() => setCopiedLinux(false), 2000);
    }
  };

  return (
    <section className="py-24 px-4 sm:px-6 lg:px-8 bg-space-black border-t border-space-border relative overflow-hidden">
      <div className="max-w-4xl mx-auto rounded-3xl p-8 sm:p-12 border-2 border-primary-gold/40 bg-space-deep shadow-hard-md text-center relative z-10">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-primary-gold/40 bg-space-surface text-xs font-mono text-primary-gold mb-6 font-bold shadow-hard-sm">
          <Star className="w-3.5 h-3.5 text-primary-gold fill-primary-gold" />
          <span>§08 — FREE & OPEN SOURCE SOFTWARE • 15°29&apos;N 73°49&apos;E</span>
        </div>

        <h2 className="font-editorial text-3xl sm:text-5xl lg:text-6xl text-paper-beige tracking-tight mb-4 font-bold">
          Ready to elevate your <br />
          <span className="italic text-gradient-editorial">engineering environment?</span>
        </h2>

        <p className="text-sand/90 text-sm sm:text-base max-w-xl mx-auto mb-8 leading-relaxed">
          AEK is free, open source, and dual-licensed under Apache 2.0 or MIT. Designed with ruthless precision for professional software teams and AI agent developers.
        </p>

        <div className="flex flex-wrap items-center justify-center gap-4 mb-8">
          <Link href="/download">
            <TactileButton variant="gold" size="lg" className="flex items-center gap-2">
              <Download className="w-4 h-4 text-space-black" />
              <span>Download AEK v{LATEST_VERSION}</span>
            </TactileButton>
          </Link>

          <a
            href={GITHUB_REPO_URL}
            target="_blank"
            rel="noopener noreferrer"
          >
            <TactileButton variant="secondary" size="lg" className="flex items-center gap-2 font-mono">
              <Github className="w-4 h-4" />
              <span>Star on GitHub</span>
            </TactileButton>
          </a>
        </div>

        {/* Quick Install Snippets */}
        <div className="max-w-2xl mx-auto p-5 rounded-2xl bg-space-black border-2 border-space-border text-left font-mono text-xs shadow-hard-sm">
          <div className="flex items-center justify-between text-primary-gold text-[11px] font-bold mb-3 pb-2 border-b border-space-border/60">
            <span>ONE-LINE USER INSTALLATION</span>
            <span className="text-sand/70">NO ADMIN/ROOT REQUIRED</span>
          </div>
          
          <div className="space-y-3">
            <div className="flex items-center justify-between gap-2 bg-space-surface p-2.5 rounded-xl border border-space-border">
              <div className="overflow-x-auto whitespace-nowrap">
                <span className="text-warm-orange font-bold mr-2">Windows:</span>
                <code className="text-paper-beige select-all">{winCmd}</code>
              </div>
              <button
                onClick={() => handleCopy("win")}
                className="px-2.5 py-1 rounded bg-space-black border border-primary-gold/40 text-[10px] text-primary-gold hover:text-paper-beige flex items-center gap-1 shrink-0"
              >
                {copiedWin ? <Check className="w-3 h-3 text-emerald-400" /> : <Copy className="w-3 h-3" />}
                <span>{copiedWin ? "COPIED" : "COPY"}</span>
              </button>
            </div>

            <div className="flex items-center justify-between gap-2 bg-space-surface p-2.5 rounded-xl border border-space-border">
              <div className="overflow-x-auto whitespace-nowrap">
                <span className="text-warm-orange font-bold mr-2">Linux:</span>
                <code className="text-paper-beige select-all">{linuxCmd}</code>
              </div>
              <button
                onClick={() => handleCopy("linux")}
                className="px-2.5 py-1 rounded bg-space-black border border-primary-gold/40 text-[10px] text-primary-gold hover:text-paper-beige flex items-center gap-1 shrink-0"
              >
                {copiedLinux ? <Check className="w-3 h-3 text-emerald-400" /> : <Copy className="w-3 h-3" />}
                <span>{copiedLinux ? "COPIED" : "COPY"}</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
