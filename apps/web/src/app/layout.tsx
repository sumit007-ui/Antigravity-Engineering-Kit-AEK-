import type { Metadata } from "next";
import "./globals.css";
import { fontDisplay, fontMono, fontPixel, fontBody } from "./fonts";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import SmoothScroll from "@/components/motion/SmoothScroll";
import CustomCursor from "@/components/motion/CustomCursor";
import BootLoader from "@/components/motion/BootLoader";
import CyberBotArcade from "@/components/motion/CyberBotArcade";

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
    <html
      lang="en-IN"
      className={`dark ${fontDisplay.variable} ${fontMono.variable} ${fontPixel.variable} ${fontBody.variable}`}
    >
      <body className="min-h-screen flex flex-col bg-bg text-fg font-body antialiased selection:bg-accent-primary/30 selection:text-fg">
        <BootLoader />
        <CyberBotArcade />
        <SmoothScroll>
          <CustomCursor />
          <Navbar />
          <main className="flex-1 w-full relative z-10">{children}</main>
          <Footer />
        </SmoothScroll>
      </body>
    </html>
  );
}
