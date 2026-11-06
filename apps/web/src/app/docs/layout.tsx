import DocsSidebar from "@/components/DocsSidebar";

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="w-full bg-espresso-deep border-b border-espresso-border min-h-screen">
      <div className="max-w-content mx-auto px-4 sm:px-6 lg:px-8 py-8 flex flex-col lg:flex-row gap-8">
        <DocsSidebar />
        <article className="flex-1 min-w-0 max-w-4xl py-4">{children}</article>
      </div>
    </div>
  );
}

