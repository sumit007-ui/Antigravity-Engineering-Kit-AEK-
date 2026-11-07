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



