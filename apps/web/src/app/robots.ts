import { MetadataRoute } from "next";

export default function robots(): MetadataRoute.Robots {
  return {
    rules: {
      userAgent: "*",
      allow: "/",
    },
    sitemap: "https://sumit007-ui.github.io/Antigravity-Engineering-Kit-AEK-/sitemap.xml",
  };
}

