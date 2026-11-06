export default function PrivacyPage() {
  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-3xl mx-auto">
      <div className="mb-12">
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ LOCAL-FIRST COMMITMENT
        </div>
        <h1 className="font-editorial text-4xl sm:text-5xl text-ivory tracking-tight mb-4">
          Privacy Policy
        </h1>
        <p className="text-secondary-text text-sm">Last updated: November 2026</p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed p-8 rounded-3xl ceramic-card border border-espresso-border">
        <section className="space-y-3">
          <h2 className="font-editorial text-xl text-ivory">1. Zero Telemetry Architecture</h2>
          <p>
            Antigravity Engineering Kit (AEK) collects <strong>zero telemetry</strong>, zero usage analytics, and zero biometric or identifying data. We do not track commands executed, projects inspected, or AI models queried.
          </p>
        </section>

        <section className="space-y-3">
          <h2 className="font-editorial text-xl text-ivory">2. Local Execution Invariant</h2>
          <p>
            All verification pipelines, prompt evaluations, AST parsing, and agent configuration synchronizations execute strictly on your local computer. AEK does not transmit your source code, configuration files, or proprietary logic to any external remote servers.
          </p>
        </section>

        <section className="space-y-3">
          <h2 className="font-editorial text-xl text-ivory">3. Third-Party Integrations</h2>
          <p>
            When utilizing third-party Model Context Protocol (MCP) servers (e.g. GitHub or Context7), communications occur directly between the local client and the configured endpoint under your personal authentication credentials. AEK does not proxy or inspect these requests.
          </p>
        </section>
      </div>
    </div>
  );
}

