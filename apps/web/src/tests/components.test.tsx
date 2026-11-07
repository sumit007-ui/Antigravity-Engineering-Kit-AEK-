import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import Domain3DIcon from "../components/three/Domain3DIcon";
import DocsSidebar, { DOCS_NAVIGATION } from "../components/DocsSidebar";
import MarqueeTicker from "../components/MarqueeTicker";
import LiveTelemetry from "../components/LiveTelemetry";
import AgentLogo from "../components/AgentLogo";

describe("Domain3DIcon", () => {
  it("renders without crashing for all 8 domain types", () => {
    const domains = [
      "rules",
      "skills",
      "mcp",
      "languages",
      "prompt",
      "validation",
      "security",
      "multi-agent",
    ] as const;

    domains.forEach((type) => {
      const { container } = render(<Domain3DIcon type={type} />);
      expect(container.firstChild).toBeTruthy();
    });
  });
});

describe("DocsNavigation", () => {
  it("contains complete 4-tier categories and links", () => {
    expect(DOCS_NAVIGATION.length).toBe(4);
    const totalLinks = DOCS_NAVIGATION.reduce((acc, cat) => acc + cat.links.length, 0);
    expect(totalLinks).toBe(14);
  });
});

describe("MarqueeTicker Component (Goa-style UX)", () => {
  it("renders the animated marquee ticker with key high-energy badges", () => {
    render(<MarqueeTicker />);
    expect(screen.getAllByText(/RUST WORKSPACE/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/MODEL CONTEXT PROTOCOL/i).length).toBeGreaterThan(0);
  });
});

describe("LiveTelemetry Component (Goa-style Infra Metrics)", () => {
  it("renders live builder infrastructure metrics", () => {
    render(<LiveTelemetry />);
    expect(screen.getByText(/16 CRATES/i)).toBeDefined();
    expect(screen.getByText(/100% PASSING/i)).toBeDefined();
    expect(screen.getByText(/COLD BOOT/i)).toBeDefined();
  });
});

describe("AgentLogo Component (Official Icons)", () => {
  it("renders official SVG icons for all 4 supported agents", () => {
    const agents = ["antigravity", "claude", "cursor", "opencode"] as const;
    agents.forEach((agentId) => {
      const { container } = render(<AgentLogo agent={agentId} className="w-6 h-6" />);
      expect(container.querySelector("svg")).toBeTruthy();
    });
  });
});

describe("Phase 2 Primitives: TactileButton", () => {
  it("renders with tap target >= 44px and variant classes", async () => {
    const { default: TactileButton } = await import("../components/primitives/TactileButton");
    render(<TactileButton variant="primary">Launch Protocol</TactileButton>);
    const btn = screen.getByRole("button", { name: /Launch Protocol/i });
    expect(btn).toBeDefined();
    expect(btn.className).toContain("min-h-[44px]");
  });
});

describe("Phase 2 Primitives: SectionHeader", () => {
  it("renders classified section header pattern", async () => {
    const { default: SectionHeader } = await import("../components/primitives/SectionHeader");
    render(
      <SectionHeader
        sectionCode="SEC_02"
        sectionNumber="02"
        sectionTag="MISSION BRIEFING"
        title="Engineering Infrastructure"
        subtitle="Rust-grade determinism across all AI coding agents."
      />
    );
    expect(screen.getByText(/SEC_02/i)).toBeDefined();
    expect(screen.getByText(/MISSION BRIEFING/i)).toBeDefined();
    expect(screen.getByText(/Engineering Infrastructure/i)).toBeDefined();
  });
});

describe("Phase 2 Primitives: PixelCard (Card, Stat, Locked)", () => {
  it("renders StatCard and LockedCard with appropriate tags and actions", async () => {
    const { StatCard, LockedCard } = await import("../components/primitives/PixelCard");
    render(
      <div>
        <StatCard tag="01/CLOCK" value="36" caption="HOURS OF HACKING" subCaption="NON-STOP" />
        <LockedCard id="track-03" title="TRACK 03" status="ENCRYPTED" />
      </div>
    );
    expect(screen.getByText(/01\/CLOCK/i)).toBeDefined();
    expect(screen.getByText("36")).toBeDefined();
    expect(screen.getByText(/ENCRYPTED/i)).toBeDefined();
    expect(screen.getByText(/NOTIFY ME/i)).toBeDefined();
  });
});

describe("Phase 2 Primitives: FilterChip", () => {
  it("renders active and inactive filter chips", async () => {
    const { default: FilterChip } = await import("../components/primitives/FilterChip");
    render(
      <div>
        <FilterChip label="ALL" active={true} onClick={() => {}} />
        <FilterChip label="RULES" active={false} onClick={() => {}} />
      </div>
    );
    expect(screen.getByText("[ALL]")).toBeDefined();
    expect(screen.getByText("[RULES]")).toBeDefined();
  });
});




