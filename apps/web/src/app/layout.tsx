import type { Metadata } from "next";
import "./globals.css";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import SmoothScroll from "@/components/motion/SmoothScroll";
import CustomCursor from "@/components/motion/CustomCursor";

export const metadata: Metadata = {
  title: "Antigravity Engineering Kit (AEK) — One Engineering Kit. Every AI Coding Agent.",
  description:
    "Rust-powered engineering infrastructure for AI coding agents. Unify engineering rules, specialized agent skills, MCP integrations, language toolchains, and automated validation in one high-performance toolkit.",
  keywords: [
    "Antigravity Engineering Kit",
    "AEK",
    "Google Antigravity",
    "Claude Code",
    "Cursor",
    "OpenCode",
    "Model Context Protocol",
    "MCP",
    "Rust",
    "AI coding agents",
    "engineering rules",
    "developer tools"
  ],
  authors: [{ name: "Antigravity Engineering Kit Contributors" }],
  openGraph: {
    title: "Antigravity Engineering Kit (AEK)",
    description: "One Engineering Kit. Every AI Coding Agent. Rust-powered engineering infrastructure.",
    type: "website",
    locale: "en_US",
    siteName: "Antigravity Engineering Kit (AEK)",
  },
  twitter: {
    card: "summary_large_image",
    title: "Antigravity Engineering Kit (AEK)",
    description: "Rust-powered engineering infrastructure for AI coding agents.",
  },
  robots: {
    index: true,
    follow: true,
  },
  icons: {
    icon: "/favicon.svg",
    shortcut: "/favicon.svg",
    apple: "/icon.svg",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="dark">
      <body className="min-h-screen flex flex-col bg-espresso-deep text-ivory antialiased selection:bg-flamingo/30 selection:text-ivory">
        <SmoothScroll>
          <CustomCursor />
          <Navbar />
          <main className="flex-1 w-full">{children}</main>
          <Footer />
        </SmoothScroll>
      </body>
    </html>
  );
}
