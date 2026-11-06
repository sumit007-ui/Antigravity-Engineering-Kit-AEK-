export default function RulesDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">ENGINEERING RULES</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          Rules Engine & Precedence
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          How AEK resolves, audits, and enforces global and project-level engineering standards.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">3-Tier Precedence Hierarchy</h2>
          <p className="text-xs text-sand/80">
            Rules are merged in strict ascending order, with project-specific instructions overriding workspace and global settings:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard space-y-1">
            <div>1. Global Tier (~/.gemini/antigravity/rules/ or global config)</div>
            <div>2. Workspace Tier (Parent multi-repository workspace root)</div>
            <div>3. Project Tier (.gemini/rules/, .cursor/rules/, or project root) [HIGHEST]</div>
          </div>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Size & Token Budgeting Constraints</h2>
          <p className="text-xs text-sand/80">
            To prevent context window bloat and instruction dilution, AEK enforces a strict <strong>24 KB maximum size</strong> per rule file.
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-sand">
            $ aek rules list --audit
          </div>
        </section>
      </div>
    </div>
  );
}

