import { LATEST_VERSION, GITHUB_REPO_URL } from "@/lib/utils";
import { Download, ShieldCheck, HardDrive } from "lucide-react";

export default function InstallationDocPage() {
  return (
    <div className="space-y-8">
      <div>
        <span className="font-mono text-xs text-flamingo font-semibold">INSTALLATION & UPDATES</span>
        <h1 className="font-editorial text-4xl text-ivory tracking-tight mt-1 mb-3">
          Cross-Platform Installation
        </h1>
        <p className="text-secondary-text text-sm sm:text-base leading-relaxed">
          How to install, verify, update, and uninstall AEK on Windows and Linux systems.
        </p>
      </div>

      <div className="space-y-8 text-sand/90 text-sm leading-relaxed">
        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Windows Installation</h2>
          <p className="text-xs text-sand/80">
            Open PowerShell (no administrator privileges needed) and run:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard select-all">
            irm https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/windows/install.ps1 | iex
          </div>
          <p className="text-xs text-sand/70">
            Destination: <code className="text-custard">%LOCALAPPDATA%\Programs\aek\bin\aek.exe</code>
          </p>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Linux & macOS Installation</h2>
          <p className="text-xs text-sand/80">
            Run the official POSIX-compliant Bash installer:
          </p>
          <div className="p-4 rounded-xl bg-espresso-deep border border-espresso-border font-mono text-xs text-custard select-all">
            curl -fsSL https://raw.githubusercontent.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-/main/installers/linux/install.sh | bash
          </div>
          <p className="text-xs text-sand/70">
            Destination: <code className="text-custard">~/.local/bin/aek</code>
          </p>
        </section>

        <section className="p-6 rounded-2xl ceramic-card border border-espresso-border space-y-3">
          <h2 className="font-editorial text-2xl text-ivory">Lifecycle Management (Updates & Rollbacks)</h2>
          <div className="space-y-3 font-mono text-xs">
            <div>
              <span className="text-sand/70 block mb-1">Check for and apply latest updates:</span>
              <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border text-custard">
                $ aek update
              </div>
            </div>
            <div>
              <span className="text-sand/70 block mb-1">Instantly rollback to previous binary:</span>
              <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border text-custard">
                $ aek rollback
              </div>
            </div>
            <div>
              <span className="text-sand/70 block mb-1">Clean uninstallation:</span>
              <div className="p-3 rounded-xl bg-espresso-deep border border-espresso-border text-custard">
                $ aek uninstall --purge
              </div>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}

