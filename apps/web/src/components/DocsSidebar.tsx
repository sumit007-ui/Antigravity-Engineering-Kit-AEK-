"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { BookOpen, Terminal, Shield, Wrench, Layers, Bot, Cpu, CheckCircle } from "lucide-react";

export const DOCS_NAVIGATION = [
  {
    title: "Getting Started",
    links: [
      { name: "Overview", href: "/docs" },
      { name: "Quickstart", href: "/docs/getting-started" },
      { name: "Installation", href: "/docs/installation" },
      { name: "Configuration", href: "/docs/configuration" },
    ],
  },
  {
    title: "Core Engines",
    links: [
      { name: "Engineering Rules", href: "/docs/rules" },
      { name: "Specialized Skills", href: "/docs/skills" },
      { name: "Model Context Protocol (MCP)", href: "/docs/mcp" },
      { name: "Language Toolchains", href: "/docs/languages" },
    ],
  },
  {
    title: "Quality & Assurance",
    links: [
      { name: "Prompt Optimization & DSPy", href: "/docs/prompt-optimization" },
      { name: "8-Stage Validation Gates", href: "/docs/validation" },
      { name: "Security Auditing", href: "/docs/security" },
    ],
  },
  {
    title: "Ecosystem & Reference",
    links: [
      { name: "Multi-Agent Compatibility", href: "/docs/multi-agent" },
      { name: "CLI Command Reference", href: "/docs/cli-reference" },
      { name: "Troubleshooting & Doctor", href: "/docs/troubleshooting" },
    ],
  },
];

export default function DocsSidebar() {
  const pathname = usePathname();

  return (
    <aside className="w-full lg:w-64 shrink-0 py-6 pr-4 space-y-8 lg:sticky lg:top-24 lg:h-[calc(100vh-6rem)] overflow-y-auto">
      {DOCS_NAVIGATION.map((section) => (
        <div key={section.title} className="space-y-2">
          <h4 className="font-mono text-xs font-bold uppercase tracking-wider text-custard/90 px-3">
            {section.title}
          </h4>
          <ul className="space-y-1">
            {section.links.map((link) => {
              const isActive = pathname === link.href;
              return (
                <li key={link.href}>
                  <Link
                    href={link.href}
                    className={`block px-3 py-1.5 rounded-xl text-xs font-medium transition-all ${
                      isActive
                        ? "bg-brandy text-ivory border border-flamingo/40 font-semibold"
                        : "text-sand/70 hover:text-ivory hover:bg-espresso-surface"
                    }`}
                  >
                    {link.name}
                  </Link>
                </li>
              );
            })}
          </ul>
        </div>
      ))}
    </aside>
  );
}

