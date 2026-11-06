import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import Domain3DIcon from "../components/three/Domain3DIcon";
import DocsSidebar, { DOCS_NAVIGATION } from "../components/DocsSidebar";

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

