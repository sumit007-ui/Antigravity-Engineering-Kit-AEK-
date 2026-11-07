import { Terminal, Check, ArrowRight } from "lucide-react";
import Link from "next/link";
import { LATEST_VERSION } from "@/lib/utils";

export default function GettingStartedPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">GETTING STARTED</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          Quickstart Guide
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          Set up AEK in your development environment and synchronize engineering rules in minutes.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        {/* Step 1 */}
        <section className="p-4 sm:p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3 overflow-hidden">
          <h2 className="font-editorial text-xl sm:text-2xl text-ivory">Step 1: Install AEK</h2>
          <p className="text-xs text-sand/80">
            Install the precompiled binary for your operating system without requiring administrator or root privileges:
          </p>
          <div className="p-3.5 sm:p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-[11px] sm:text-xs text-custard space-y-2 overflow-x-auto">
            <div>
              <span className="text-muted-text"># Windows (PowerShell):</span>
              <div className="select-all break-all sm:break-normal">irm https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/windows/install.ps1 | iex</div>
            </div>
            <div className="pt-2">
              <span className="text-muted-text"># Linux & macOS (Bash):</span>
              <div className="select-all break-all sm:break-normal">curl -fsSL https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/linux/install.sh | bash</div>
            </div>
          </div>
        </section>

        {/* Step 2 */}
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Step 2: Initialize Workspace</h2>
          <p className="text-xs text-sand/80">
            Navigate to your repository root and initialize an AEK configuration profile:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard">
            $ aek init --profile engineering
          </div>
          <p className="text-xs text-sand/70">
            This inspects your project toolchains (Rust, Node, Python, Go) and writes a strict schema configuration to <code className="text-custard">.aek/config.toml</code>.
          </p>
        </section>

        {/* Step 3 */}
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Step 3: Synchronize AI Coding Agents</h2>
          <p className="text-xs text-sand/80">
            Translate and export your rules, skills, and verified MCP servers natively to all installed coding agents:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard">
            $ aek agent sync --all
          </div>
          <p className="text-xs text-sand/70">
            AEK generates <code className="text-custard">~/.gemini/rules/</code>, <code className="text-custard">CLAUDE.md</code>, <code className="text-custard">.cursor/rules/</code>, and <code className="text-custard">AGENTS.md</code> with zero conflict risk.
          </p>
        </section>

        {/* Step 4 */}
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Step 4: Run Quality Gates</h2>
          <p className="text-xs text-sand/80">
            Execute the 8-stage verification pipeline to audit code health before committing:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard">
            $ aek validate --strict
          </div>
        </section>
      </div>
    </div>
  );
}

