import { MetadataRoute } from "next";

export default function robots(): MetadataRoute.Robots {
  return {
    rules: {
      userAgent: "*",
      allow: "/",
    },
    sitemap: "https://antigravity-engineering-kit-aek-taupe.vercel.app/sitemap.xml",
  };
}

