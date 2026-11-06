import ArchitectureVisualization from "@/components/ArchitectureVisualization";

const CRATES_LIST = [
  { name: "aek-platform", desc: "Platform directory resolution, OS/architecture detection, path traversal defense." },
  { name: "aek-core", desc: "Safe execution engine, subprocess supervisor, diagnostics runner, and resource tiers." },
  { name: "aek-config", desc: "Schema validation, TOML persistence, atomic file writes, backup and restore engine." },
  { name: "aek-rules", desc: "Global and project rule models, registry, 3-tier precedence resolution, and size validation." },
  { name: "aek-skills", desc: "Skill models, frontmatter parsing, project-aware discovery, validation, and installation." },
  { name: "aek-agents", desc: "Universal agent adapter trait, capability registry, native config translators, and sync." },
  { name: "aek-mcp", desc: "Reusable JSON-RPC 2.0 MCP client, Stdio/HTTP transports, and security policy." },
  { name: "aek-mcp-integrations", desc: "Standard configuration adapters for Context7, GitHub, Playwright, Sequential Thinking, Filesystem." },
  { name: "aek-languages", desc: "Language toolchain registry, compiler/linter diagnostics, and project detection across 8 ecosystems." },
  { name: "aek-prompt", desc: "Structured prompt templates, token budgeting, variables, and DSPy declarative signatures." },
  { name: "aek-prompt-adapters", desc: "Promptfoo test suites, red-teaming evaluations, regression assertions, and automated scoring." },
  { name: "aek-validation", desc: "8-stage verification pipeline, DAG execution planner, incremental change detector, quality gates." },
  { name: "aek-security", desc: "Security scanner orchestrator, Semgrep & Trivy adapters, dependency auditing, security doctor." },
  { name: "aek-reporting", desc: "Multi-format report generation (Terminal, Markdown, JSON, OASIS SARIF v2.1.0)." },
  { name: "aek-distribution", desc: "Cross-platform packaging, version checking, atomic in-place updates, rollback archives." },
  { name: "aek-cli", desc: "Command-line interface binary, argument parsing via Clap, and user diagnostics." },
];

export default function ArchitecturePage() {
  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-content mx-auto">
      <div className="text-center max-w-3xl mx-auto mb-16">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ RUST CRATE ECOSYSTEM
        </div>
        <h1 className="font-editorial text-4xl sm:text-6xl text-ivory tracking-tight mb-4">
          Crate <span className="italic text-gradient-editorial">Topology.</span>
        </h1>
        <p className="text-secondary-text text-base sm:text-lg">
          The complete architectural blueprint of the sixteen modular Rust crates comprising AEK.
        </p>
      </div>

      <ArchitectureVisualization />

      {/* Complete Crate Catalog */}
      <div className="mt-20 max-w-5xl mx-auto">
        <h2 className="font-editorial text-3xl text-ivory mb-8 text-center sm:text-left">
          Complete Workspace Manifest (16 Crates)
        </h2>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {CRATES_LIST.map((crate) => (
            <div
              key={crate.name}
              className="p-5 rounded-2xl ceramic-card border border-espresso-border flex flex-col justify-between"
            >
              <div>
                <span className="font-mono text-xs font-bold text-flamingo block mb-1">
                  crates/{crate.name}
                </span>
                <p className="text-xs text-sand/80 leading-relaxed">{crate.desc}</p>
              </div>
              <div className="pt-4 mt-4 border-t border-espresso-border/60 flex items-center justify-between text-[10px] font-mono text-muted-text">
                <span>RUST 2024</span>
                <span className="text-emerald-400">PASSED</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
