export default function LanguagesDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">LANGUAGE TOOLCHAINS</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          Language Diagnostics & Detection
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          Verifying compilers, SDKs, linters, and language servers across eight ecosystems.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Supported Ecosystems</h2>
          <p className="text-xs text-sand/80">
            AEK provides native toolchain inspection for:
          </p>
          <ul className="grid grid-cols-2 sm:grid-cols-4 gap-2 font-mono text-xs text-custard pt-2">
            <li className="p-2 rounded bg-espresso-deep border border-espresso-border">Rust (cargo, clippy)</li>
            <li className="p-2 rounded bg-espresso-deep border border-espresso-border">Go (go, golangci-lint)</li>
            <li className="p-2 rounded bg-espresso-deep border border-espresso-border">C/C++ (clang, cmake)</li>
            <li className="p-2 rounded bg-espresso-deep border border-espresso-border">Zig (zig)</li>
            <li className="p-2 rounded bg-espresso-deep border border-espresso-border">Python (uv, ruff, mypy)</li>
            <li className="p-2 rounded bg-espresso-deep border border-espresso-border">TypeScript (tsc, eslint)</li>
            <li className="p-2 rounded bg-espresso-deep border border-espresso-border">Flutter (flutter, dart)</li>
            <li className="p-2 rounded bg-espresso-deep border border-espresso-border">ROS 2 (colcon, ament)</li>
          </ul>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Language Doctor Diagnostics</h2>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard">
            $ aek language doctor rust
          </div>
        </section>
      </div>
    </div>
  );
}

