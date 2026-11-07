"use client";

import { useState } from "react";
import { Download, Check, Copy, ShieldCheck, Terminal, Cpu, HardDrive } from "lucide-react";
import { LATEST_VERSION, GITHUB_REPO_URL } from "@/lib/utils";
import TiltCard from "@/components/motion/TiltCard";

const WINDOWS_HASH = "666d59ca589ed821022c072ff937591b3ece50332152bbf6354d1959948452f4";
const LINUX_HASH = "Verified via release-manifest.json in GitHub Releases";
const MACOS_HASH = "Verified via release-manifest.json in GitHub Releases";

export default function DownloadPage() {
  const [copiedHash, setCopiedHash] = useState<string | null>(null);
  const [copiedCmd, setCopiedCmd] = useState<string | null>(null);

  const copyToClipboard = (text: string, type: "hash" | "cmd") => {
    navigator.clipboard.writeText(text);
    if (type === "hash") {
      setCopiedHash(text);
      setTimeout(() => setCopiedHash(null), 2000);
    } else {
      setCopiedCmd(text);
      setTimeout(() => setCopiedCmd(null), 2000);
    }
  };

  const winCmd = "irm https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/windows/install.ps1 | iex";
  const macCmd = "curl -fsSL https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/linux/install.sh | bash";
  const linuxCmd = "curl -fsSL https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/linux/install.sh | bash";

  return (
    <div className="py-20 px-4 sm:px-6 lg:px-8 max-w-content mx-auto">
      {/* Header */}
      <div className="text-center max-w-3xl mx-auto mb-16">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-flamingo/40 bg-brandy/20 text-xs font-mono text-custard mb-4">
          ✦ OFFICIAL RELEASE v{LATEST_VERSION}
        </div>
        <h1 className="font-editorial text-4xl sm:text-6xl lg:text-7xl text-ivory tracking-tight mb-6">
          Your engineering environment, <br />
          <span className="italic text-gradient-editorial">elevated.</span>
        </h1>
        <p className="text-secondary-text text-base sm:text-lg">
          Fast, user-level installation without administrator or root permissions. Pre-compiled standalone binaries for Windows, macOS, and Linux.
        </p>
      </div>

      {/* Three Platform Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto mb-16">
        {/* Windows Card */}
        <TiltCard glowColor="custard" className="flex flex-col justify-between">
          <div>
            <div className="flex items-center justify-between mb-4">
              <span className="font-mono text-xs text-custard px-2.5 py-1 rounded bg-brandy/50 border border-brandy-light">
                WINDOWS x86_64
              </span>
              <span className="text-xs font-mono text-emerald-400">v{LATEST_VERSION}</span>
            </div>

            <h3 className="font-editorial text-2xl sm:text-3xl text-ivory mb-2">Windows 10 / 11</h3>
            <p className="text-xs text-sand/80 mb-6">
              Installs binary to <code className="text-custard">%LOCALAPPDATA%\Programs\aek\bin</code> and registers standard user PATH.
            </p>

            <div className="space-y-4 mb-6">
              <div>
                <span className="text-[11px] font-mono text-sand/70 block mb-1">PowerShell Installer:</span>
                <div className="flex items-center justify-between p-3 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs">
                  <span className="text-custard truncate mr-2">{winCmd}</span>
                  <button
                    onClick={() => copyToClipboard(winCmd, "cmd")}
                    className="p-1 hover:text-flamingo text-muted-text"
                    aria-label="Copy command"
                  >
                    {copiedCmd === winCmd ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  </button>
                </div>
              </div>

              <div>
                <span className="text-[11px] font-mono text-sand/70 block mb-1">SHA-256 Checksum:</span>
                <div className="flex items-center justify-between p-2.5 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-[11px]">
                  <span className="text-sand/80 truncate mr-2">{WINDOWS_HASH}</span>
                  <button
                    onClick={() => copyToClipboard(WINDOWS_HASH, "hash")}
                    className="p-1 hover:text-flamingo text-muted-text"
                    aria-label="Copy SHA-256 hash"
                  >
                    {copiedHash === WINDOWS_HASH ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  </button>
                </div>
              </div>
            </div>
          </div>

          <a
            href={`${GITHUB_REPO_URL}/releases/download/v${LATEST_VERSION}/aek-v${LATEST_VERSION}-windows-x86_64.zip`}
            className="w-full py-3.5 rounded-xl bg-brandy hover:bg-brandy-hover border border-flamingo/40 text-ivory text-xs font-semibold font-mono tracking-wide flex items-center justify-center gap-2 shadow-tactile transition-all"
          >
            <Download className="w-4 h-4 text-custard" />
            <span>Download Windows .zip</span>
          </a>
        </TiltCard>

        {/* macOS Card */}
        <TiltCard glowColor="custard" className="flex flex-col justify-between">
          <div>
            <div className="flex items-center justify-between mb-4">
              <span className="font-mono text-xs text-primary-gold px-2.5 py-1 rounded bg-brandy/50 border border-primary-gold/40">
                macOS UNIVERSAL
              </span>
              <span className="text-xs font-mono text-emerald-400">v{LATEST_VERSION}</span>
            </div>

            <h3 className="font-editorial text-2xl sm:text-3xl text-ivory mb-2">Apple Silicon & Intel</h3>
            <p className="text-xs text-sand/80 mb-6">
              Installs binary to <code className="text-custard">~/.local/bin/aek</code> with native arm64 / x86_64 performance.
            </p>

            <div className="space-y-4 mb-6">
              <div>
                <span className="text-[11px] font-mono text-sand/70 block mb-1">Terminal Installer:</span>
                <div className="flex items-center justify-between p-3 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs">
                  <span className="text-custard truncate mr-2">{macCmd}</span>
                  <button
                    onClick={() => copyToClipboard(macCmd, "cmd")}
                    className="p-1 hover:text-flamingo text-muted-text"
                    aria-label="Copy command"
                  >
                    {copiedCmd === macCmd ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  </button>
                </div>
              </div>

              <div>
                <span className="text-[11px] font-mono text-sand/70 block mb-1">SHA-256 Checksum:</span>
                <div className="p-2.5 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-[11px] text-sand/80 truncate">
                  {MACOS_HASH}
                </div>
              </div>
            </div>
          </div>

          <a
            href={`${GITHUB_REPO_URL}/releases/download/v${LATEST_VERSION}/aek-v${LATEST_VERSION}-macos-aarch64.tar.gz`}
            className="w-full py-3.5 rounded-xl bg-espresso-surface hover:bg-espresso-elevated border border-primary-gold/40 text-primary-gold text-xs font-semibold font-mono tracking-wide flex items-center justify-center gap-2 shadow-tactile transition-all"
          >
            <Download className="w-4 h-4 text-primary-gold" />
            <span>Download macOS .tar.gz</span>
          </a>
        </TiltCard>

        {/* Linux Card */}
        <TiltCard glowColor="flamingo" className="flex flex-col justify-between">
          <div>
            <div className="flex items-center justify-between mb-4">
              <span className="font-mono text-xs text-flamingo px-2.5 py-1 rounded bg-brandy/50 border border-brandy-light">
                LINUX x86_64
              </span>
              <span className="text-xs font-mono text-emerald-400">v{LATEST_VERSION}</span>
            </div>

            <h3 className="font-editorial text-2xl sm:text-3xl text-ivory mb-2">Linux (glibc / musl)</h3>
            <p className="text-xs text-sand/80 mb-6">
              Installs binary to <code className="text-custard">~/.local/bin/aek</code> adhering strictly to XDG Base Directory conventions.
            </p>

            <div className="space-y-4 mb-6">
              <div>
                <span className="text-[11px] font-mono text-sand/70 block mb-1">One-line Shell Installer:</span>
                <div className="flex items-center justify-between p-3 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs">
                  <span className="text-custard truncate mr-2">{linuxCmd}</span>
                  <button
                    onClick={() => copyToClipboard(linuxCmd, "cmd")}
                    className="p-1 hover:text-flamingo text-muted-text"
                    aria-label="Copy command"
                  >
                    {copiedCmd === linuxCmd ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  </button>
                </div>
              </div>

              <div>
                <span className="text-[11px] font-mono text-sand/70 block mb-1">SHA-256 Checksum:</span>
                <div className="p-2.5 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-[11px] text-sand/80 truncate">
                  {LINUX_HASH}
                </div>
              </div>
            </div>
          </div>

          <a
            href={`${GITHUB_REPO_URL}/releases/download/v${LATEST_VERSION}/aek-v${LATEST_VERSION}-linux-x86_64.tar.gz`}
            className="w-full py-3.5 rounded-xl bg-espresso-surface hover:bg-espresso-elevated border border-espresso-border text-custard text-xs font-semibold font-mono tracking-wide flex items-center justify-center gap-2 shadow-tactile transition-all"
          >
            <Download className="w-4 h-4 text-flamingo" />
            <span>Download Linux .tar.gz</span>
          </a>
        </TiltCard>
      </div>

      {/* Lifecycle Features Banner */}
      <div className="max-w-4xl mx-auto rounded-2xl bg-espresso-surface/80 border border-espresso-border p-6 flex flex-col sm:flex-row items-center justify-between gap-6 text-xs font-mono text-sand">
        <div className="flex items-center gap-3">
          <ShieldCheck className="w-5 h-5 text-emerald-400" />
          <span>Non-root isolation</span>
        </div>
        <div className="flex items-center gap-3">
          <HardDrive className="w-5 h-5 text-custard" />
          <span>Atomic in-place updates: <code className="text-flamingo">$ aek update</code></span>
        </div>
        <div className="flex items-center gap-3">
          <Terminal className="w-5 h-5 text-flamingo" />
          <span>Instant version rollback: <code className="text-flamingo">$ aek rollback</code></span>
        </div>
      </div>
    </div>
  );
}

