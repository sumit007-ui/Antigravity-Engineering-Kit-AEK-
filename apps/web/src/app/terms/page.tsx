export default function TermsPage() {
  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-3xl mx-auto">
      <div className="mb-12">
        <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ DUAL-LICENSED OPEN SOURCE
        </div>
        <h1 className="font-editorial text-4xl sm:text-5xl text-ivory tracking-tight mb-4">
          Terms of Use & License
        </h1>
        <p className="text-secondary-text text-sm">Last updated: November 2026</p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed p-8 rounded-3xl ceramic-card border border-espresso-border">
        <section className="space-y-3">
          <h2 className="font-editorial text-xl text-ivory">1. Open Source Dual Licensing</h2>
          <p>
            Antigravity Engineering Kit (AEK) is licensed under either of:
          </p>
          <ul className="list-disc pl-5 space-y-1 text-custard font-mono text-xs">
            <li>Apache License, Version 2.0 (LICENSE-APACHE)</li>
            <li>MIT License (LICENSE-MIT)</li>
          </ul>
          <p>
            You may use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the software subject to the terms of these licenses.
          </p>
        </section>

        <section className="space-y-3">
          <h2 className="font-editorial text-xl text-ivory">2. Disclaimer of Warranty</h2>
          <p>
            The software is provided &quot;as is&quot;, without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement.
          </p>
        </section>
      </div>
    </div>
  );
}

