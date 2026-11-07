"use client";

import { useState } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  Terminal,
  Cpu,
  ShieldCheck,
  Layers,
  Search,
  ChevronDown,
  BookOpen,
} from "lucide-react";

export const DOCS_NAVIGATION = [
  {
    title: "Getting Started",
    code: "§ 01",
    icon: Terminal,
    links: [
      { name: "Overview", href: "/docs" },
      { name: "Quickstart", href: "/docs/getting-started" },
      { name: "Installation", href: "/docs/installation" },
      { name: "Configuration", href: "/docs/configuration" },
    ],
  },
  {
    title: "Core Engines",
    code: "§ 02",
    icon: Cpu,
    links: [
      { name: "Engineering Rules", href: "/docs/rules" },
      { name: "Specialized Skills", href: "/docs/skills" },
      { name: "Model Context Protocol (MCP)", href: "/docs/mcp" },
      { name: "Language Toolchains", href: "/docs/languages" },
    ],
  },
  {
    title: "Quality & Assurance",
    code: "§ 03",
    icon: ShieldCheck,
    links: [
      { name: "Prompt Optimization & DSPy", href: "/docs/prompt-optimization" },
      { name: "8-Stage Validation Gates", href: "/docs/validation" },
      { name: "Security Auditing", href: "/docs/security" },
    ],
  },
  {
    title: "Ecosystem & Reference",
    code: "§ 04",
    icon: Layers,
    links: [
      { name: "Multi-Agent Compatibility", href: "/docs/multi-agent" },
      { name: "CLI Command Reference", href: "/docs/cli-reference" },
      { name: "Troubleshooting & Doctor", href: "/docs/troubleshooting" },
    ],
  },
];

export default function DocsSidebar() {
  const pathname = usePathname();
  const [filterQuery, setFilterQuery] = useState("");
  const [mobileExpanded, setMobileExpanded] = useState(false);

  return (
    <aside className="w-full lg:w-72 shrink-0 space-y-6 lg:sticky lg:top-24 lg:h-[calc(100vh-8rem)] overflow-y-auto pr-2">
      {/* Mobile Accordion Toggle */}
      <button
        onClick={() => setMobileExpanded(!mobileExpanded)}
        className="lg:hidden w-full flex items-center justify-between p-3.5 rounded-xl bg-space-surface border-2 border-space-border text-xs font-mono font-bold text-paper-beige shadow-hard-sm"
      >
        <span className="flex items-center gap-2">
          <BookOpen className="w-4 h-4 text-primary-gold" />
          <span>DOCS NAVIGATION ({DOCS_NAVIGATION.length} SECTIONS)</span>
        </span>
        <ChevronDown
          className={`w-4 h-4 text-primary-gold transition-transform duration-200 ${
            mobileExpanded ? "rotate-180" : ""
          }`}
        />
      </button>

      <div className={`space-y-6 ${mobileExpanded ? "block" : "hidden lg:block"}`}>
        {/* Quick Filter Input */}
        <div className="relative">
          <Search className="w-3.5 h-3.5 absolute left-3 top-1/2 -translate-y-1/2 text-sand/60" />
          <input
            type="text"
            value={filterQuery}
            onChange={(e) => setFilterQuery(e.target.value)}
            placeholder="Filter topics... [ / ]"
            className="w-full pl-8 pr-3 py-2 text-xs font-mono bg-space-surface/90 border-2 border-space-border focus:border-primary-gold rounded-xl text-paper-beige placeholder-sand/50 outline-none transition-all shadow-hard-sm"
          />
          {filterQuery && (
            <button
              onClick={() => setFilterQuery("")}
              className="absolute right-2.5 top-1/2 -translate-y-1/2 text-[10px] font-mono text-sand/60 hover:text-paper-beige"
            >
              ESC
            </button>
          )}
        </div>

        {/* Categorized Navigation Groups */}
        {DOCS_NAVIGATION.map((section) => {
          const Icon = section.icon;
          const filteredLinks = section.links.filter(
            (l) =>
              l.name.toLowerCase().includes(filterQuery.toLowerCase()) ||
              section.title.toLowerCase().includes(filterQuery.toLowerCase())
          );

          if (filteredLinks.length === 0) return null;

          return (
            <div key={section.title} className="space-y-2">
              <div className="flex items-center gap-2 px-2 text-[11px] font-mono font-bold tracking-wider text-primary-gold uppercase">
                <span className="text-[10px] text-warm-orange">{section.code}</span>
                <Icon className="w-3.5 h-3.5 text-primary-gold" />
                <span>{section.title}</span>
              </div>

              <ul className="space-y-1 border-l-2 border-space-border/60 ml-3 pl-2">
                {filteredLinks.map((link) => {
                  const isActive = pathname === link.href;
                  return (
                    <li key={link.href}>
                      <Link
                        href={link.href}
                        className={`flex items-center justify-between px-3 py-2 rounded-lg text-xs font-mono transition-all border ${
                          isActive
                            ? "bg-space-surface border-primary-gold text-paper-beige font-bold shadow-hard-sm"
                            : "border-transparent text-sand/80 hover:text-paper-beige hover:bg-space-surface/60 hover:border-space-border"
                        }`}
                      >
                        <span>{link.name}</span>
                        {isActive && (
                          <span className="w-1.5 h-1.5 rounded-full bg-warm-orange animate-pulse" />
                        )}
                      </Link>
                    </li>
                  );
                })}
              </ul>
            </div>
          );
        })}
      </div>
    </aside>
  );
}
