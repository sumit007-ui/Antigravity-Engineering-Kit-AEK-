import DocsSidebar from "@/components/DocsSidebar";

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="w-full bg-space-black border-b border-space-border min-h-screen text-paper-beige">
      {/* Classified Docs Header Bar */}
      <div className="w-full border-b border-space-border/80 bg-space-deep/90 py-2.5 px-4 sm:px-8 text-xs font-mono text-sand/70 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
          <span className="font-bold text-primary-gold">AEK_MANUAL::CLASSIFIED_DOCS</span>
          <span className="text-secondary-mauve">•</span>
          <span>SYSTEM_VERSION: 0.1.0</span>
        </div>
        <div className="hidden sm:flex items-center gap-2 text-[11px] text-primary-gold/80">
          <span>ENGINE: RUST 2024</span>
        </div>
      </div>

      <div className="max-w-content mx-auto px-4 sm:px-6 lg:px-8 py-8 flex flex-col lg:flex-row gap-8">
        <DocsSidebar />
        <article className="flex-1 min-w-0 max-w-4xl py-2 prose prose-invert">{children}</article>
      </div>
    </div>
  );
}
