import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const GITHUB_REPO_URL = "https://github.com/sumit007-ui/Antigravity-Engineering-Kit-AEK-";
export const LATEST_VERSION = "0.2.0";
export const RELEASE_DATE = "2026-11-07";
export const WINDOWS_SHA256 = "666d59ca589ed821022c072ff937591b3ece50332152bbf6354d1959948452f4";

