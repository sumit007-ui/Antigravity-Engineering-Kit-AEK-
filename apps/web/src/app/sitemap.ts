import { MetadataRoute } from "next";

export default function sitemap(): MetadataRoute.Sitemap {
  const baseUrl = "https://sumit007-ui.github.io/Antigravity-Engineering-Kit-AEK-";

  const routes = [
    "",
    "/features",
    "/agents",
    "/integrations",
    "/architecture",
    "/download",
    "/roadmap",
    "/changelog",
    "/about",
    "/privacy",
    "/terms",
    "/docs",
    "/docs/getting-started",
    "/docs/installation",
    "/docs/configuration",
    "/docs/rules",
    "/docs/skills",
    "/docs/mcp",
    "/docs/languages",
    "/docs/prompt-optimization",
    "/docs/validation",
    "/docs/security",
    "/docs/multi-agent",
    "/docs/cli-reference",
    "/docs/troubleshooting",
  ];

  return routes.map((route) => ({
    url: `${baseUrl}${route}`,
    lastModified: new Date(),
    changeFrequency: route.startsWith("/docs") ? "weekly" : "monthly",
    priority: route === "" ? 1.0 : route.startsWith("/docs") ? 0.8 : 0.6,
  }));
}

