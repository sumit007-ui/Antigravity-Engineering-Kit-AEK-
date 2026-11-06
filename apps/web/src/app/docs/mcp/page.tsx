export default function McpDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">MODEL CONTEXT PROTOCOL</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          MCP Client Engine & Presets
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          High-performance JSON-RPC 2.0 client architecture with Stdio and HTTP transports.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">5 Standard Integration Presets</h2>
          <div className="space-y-3 font-mono text-xs">
            <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold">context7</span>: Documentation search & API lookups
            </div>
            <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold">github</span>: Repositories, PRs, issues, commits
            </div>
            <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold">playwright</span>: Browser automation & DOM snapshots
            </div>
            <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold">sequential-thinking</span>: Multi-step algorithmic reasoning
            </div>
            <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold">filesystem</span>: Sandboxed workspace file operations
            </div>
          </div>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Export to Any Agent</h2>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard space-y-1">
            <div>$ aek mcp export --agent antigravity</div>
            <div>$ aek mcp export --agent claude</div>
            <div>$ aek mcp export --agent cursor</div>
            <div>$ aek mcp export --agent opencode</div>
          </div>
        </section>
      </div>
    </div>
  );
}

