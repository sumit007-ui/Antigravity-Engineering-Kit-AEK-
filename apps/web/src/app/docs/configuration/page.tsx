export default function ConfigurationDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">CONFIGURATION</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          Configuration & Profiles
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          TOML schema, atomic writes, backup rotations, and resource profile tiers.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Configuration Files</h2>
          <p className="text-xs text-sand/80">
            AEK uses strict TOML serialization with unknown-field rejection to prevent typos and invalid configurations.
          </p>
          <ul className="space-y-2 text-xs font-mono text-custard pl-4 border-l-2 border-brandy-light">
            <li>• Global Config: <code className="text-sand">%APPDATA%\aek\config.toml</code> or <code className="text-sand">~/.config/aek/config.toml</code></li>
            <li>• Project Config: <code className="text-sand">.aek/config.toml</code></li>
            <li>• Managed Agents State: <code className="text-sand">.aek/managed-agents.toml</code></li>
          </ul>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Resource Profiles</h2>
          <p className="text-xs text-sand/80">
            Calibrate AEK resource usage for your target development machine:
          </p>
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 font-mono text-xs pt-2">
            <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-flamingo font-bold block mb-1">LIGHT</span>
              <span className="text-sand/70 text-[11px]">Sub-20 MB footprint for constrained 8 GB laptops. Zero heavy MCP overhead.</span>
            </div>
            <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-custard font-bold block mb-1">ENGINEERING</span>
              <span className="text-sand/70 text-[11px]">Recommended default. Full MCP integrations, compilers, and quality gates.</span>
            </div>
            <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border">
              <span className="text-ivory font-bold block mb-1">FULL</span>
              <span className="text-sand/70 text-[11px]">Uncapped resources. Enables Stanford DSPy AI bridges and security scanning.</span>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}

