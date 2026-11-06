export default function SkillsDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">AGENT SKILLS</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          15 Specialized Skills & Discovery
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          Progressive-disclosure operational runbooks automatically recommended for your codebase.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Skill Catalog</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 font-mono text-xs text-custard pt-2">
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• rust-systems-engineer</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• backend-architect (Go/Node)</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• advanced-systems-programmer (C/C++/Zig)</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• flutter-engineer (Mobile/Desktop)</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• robotics-engineer (ROS 2/Gazebo)</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• operating-system-engineer (Linux kernel)</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• security-auditor (SAST/Trivy)</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• devops-engineer (Docker/CI/CD)</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• prompt-engineering-specialist</div>
            <div className="p-3 rounded-lg bg-espresso-deep border border-espresso-border">• production-code-reviewer</div>
          </div>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Automatic Codebase Discovery</h2>
          <p className="text-xs text-sand/80">
            AEK scans project files to suggest targeted skills with confidence scoring:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard">
            $ aek skills discover
          </div>
        </section>
      </div>
    </div>
  );
}

