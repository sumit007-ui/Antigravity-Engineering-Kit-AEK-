import Link from "next/link";
import { ArrowUpRight, ShieldCheck, Cpu, Code2 } from "lucide-react";
import { GITHUB_REPO_URL, LATEST_VERSION } from "@/lib/utils";

export default function AboutPage() {
  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-4xl mx-auto">
      <div className="text-center mb-16">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ MANIFESTO & PHILOSOPHY
        </div>
        <h1 className="font-editorial text-4xl sm:text-6xl text-ivory tracking-tight mb-4">
          The Art of <span className="italic text-gradient-editorial">Engineering Intelligence.</span>
        </h1>
        <p className="text-secondary-text text-base sm:text-lg max-w-2xl mx-auto">
          Why we built Antigravity Engineering Kit: moving beyond fragile prompts into durable, memory-safe developer infrastructure.
        </p>
      </div>

      <div className="space-y-12 text-sand/90 text-sm sm:text-base leading-relaxed">
        <div className="p-8 rounded-3xl ceramic-card border border-espresso-border space-y-4">
          <h2 className="font-editorial text-2xl sm:text-3xl text-ivory">The Fragility Problem</h2>
          <p>
            AI coding agents represent an unprecedented leap in developer leverage. Yet in practice, development teams configure them using brittle prompt templates, unversioned markdown snippets, and loose shell execution scripts. A single missing environment variable or ambiguous instruction can lead an agent to hallucinate dependencies, introduce security vulnerabilities, or overwrite working code.
          </p>
          <p>
            We realized that AI coding agents do not merely need better conversational prompts; they need <strong>rigorous systems engineering infrastructure</strong>.
          </p>
        </div>

        <div className="p-8 rounded-3xl ceramic-card border border-espresso-border space-y-4">
          <h2 className="font-editorial text-2xl sm:text-3xl text-ivory">Why Rust First?</h2>
          <p>
            AEK is written from the ground up in modern Rust (2024 edition). Rust was chosen intentionally for:
          </p>
          <ul className="space-y-3 pl-4 border-l-2 border-brandy-light font-mono text-xs text-custard">
            <li>• Memory footprint under 50 MB, allowing continuous background operation on any developer laptop.</li>
            <li>• Direct argument array subprocess execution, eliminating string concatenation and shell injection risks.</li>
            <li>• Compile-time guarantees of thread safety and memory correctness across multi-agent sync workflows.</li>
            <li>• Instant startup latency (&lt;10ms), enabling quality gates to run on every commit or agent action.</li>
          </ul>
        </div>

        <div className="p-8 rounded-3xl ceramic-card border border-espresso-border space-y-4">
          <h2 className="font-editorial text-2xl sm:text-3xl text-ivory">Open Source & Local Execution</h2>
          <p>
            AEK is 100% free software dual-licensed under Apache 2.0 or MIT. It executes entirely on your local machine with zero telemetry, zero analytics tracking, and zero code transmission to third-party cloud servers.
          </p>
          <div className="pt-4 flex items-center gap-4">
            <a
              href={GITHUB_REPO_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-brandy text-ivory text-xs font-semibold hover:bg-brandy-hover transition-colors"
            >
              <span>GitHub Repository</span>
              <ArrowUpRight className="w-3.5 h-3.5" />
            </a>
            <Link
              href="/download"
              className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-espresso-surface border border-espresso-border text-custard text-xs font-mono hover:text-ivory transition-colors"
            >
              Download AEK v{LATEST_VERSION}
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}

