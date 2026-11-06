import { describe, it, expect } from "vitest";
import { GITHUB_REPO_URL, LATEST_VERSION } from "../lib/utils";

describe("AEK Utilities & Brand Tokens", () => {
  it("has valid repository URL and latest version", () => {
    expect(GITHUB_REPO_URL).toContain("sumit007-ui/Antigravity-Engineering-Kit-AEK-");
    expect(LATEST_VERSION).toBe("0.1.0");
  });
});

