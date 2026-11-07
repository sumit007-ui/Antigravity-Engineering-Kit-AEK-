import React from "react";

export type AgentId = "antigravity" | "claude" | "cursor" | "opencode";

interface AgentLogoProps {
  agent: AgentId;
  className?: string;
  size?: number;
}

export default function AgentLogo({ agent, className = "w-6 h-6", size = 24 }: AgentLogoProps) {
  switch (agent) {
    case "antigravity":
      // Official Google Gemini / Antigravity 4-pointed radiant sparkle star
      return (
        <svg
          viewBox="0 0 24 24"
          width={size}
          height={size}
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          className={className}
          aria-label="Google Gemini Antigravity"
        >
          <defs>
            <linearGradient id="geminiGrad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stopColor="#4285F4" />
              <stop offset="35%" stopColor="#9B72CB" />
              <stop offset="70%" stopColor="#D96570" />
              <stop offset="100%" stopColor="#1BA1E2" />
            </linearGradient>
          </defs>
          <path
            d="M12 0C12 6.62742 6.62742 12 0 12C6.62742 12 12 17.3726 12 24C12 17.3726 17.3726 12 24 12C17.3726 12 12 6.62742 12 0Z"
            fill="url(#geminiGrad)"
          />
        </svg>
      );

    case "claude":
      // Official Anthropic Claude terracotta starburst / sunburst icon
      return (
        <svg
          viewBox="0 0 24 24"
          width={size}
          height={size}
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          className={className}
          aria-label="Anthropic Claude Code"
        >
          <path
            d="M12 2L13.1 7.2L17.5 4.1L15.3 8.9L20.5 8.5L16.7 12L20.5 15.5L15.3 15.1L17.5 19.9L13.1 16.8L12 22L10.9 16.8L6.5 19.9L8.7 15.1L3.5 15.5L7.3 12L3.5 8.5L8.7 8.9L6.5 4.1L10.9 7.2L12 2Z"
            fill="#D97757"
          />
          <circle cx="12" cy="12" r="2.5" fill="#FAF5EF" />
        </svg>
      );

    case "cursor":
      // Official Cursor AI 3D Isometric cube / diamond
      return (
        <svg
          viewBox="0 0 24 24"
          width={size}
          height={size}
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          className={className}
          aria-label="Cursor AI IDE"
        >
          <path
            d="M12 1.5L21.5 7V17L12 22.5L2.5 17V7L12 1.5Z"
            fill="#1E1E2E"
            stroke="#A78BFA"
            strokeWidth="1.5"
            strokeLinejoin="round"
          />
          <path
            d="M12 1.5V12L21.5 7"
            stroke="#C4B5FD"
            strokeWidth="1.5"
            strokeLinejoin="round"
          />
          <path
            d="M12 12L2.5 7"
            stroke="#8B5CF6"
            strokeWidth="1.5"
            strokeLinejoin="round"
          />
          <path
            d="M12 12V22.5"
            stroke="#7C3AED"
            strokeWidth="1.5"
            strokeLinejoin="round"
          />
          <circle cx="12" cy="12" r="2.5" fill="#DDD6FE" />
        </svg>
      );

    case "opencode":
      // Official OpenCode terminal bracket and cyber prompt
      return (
        <svg
          viewBox="0 0 24 24"
          width={size}
          height={size}
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          className={className}
          aria-label="OpenCode"
        >
          <rect
            x="2"
            y="3"
            width="20"
            height="18"
            rx="4"
            fill="#062E22"
            stroke="#10B981"
            strokeWidth="1.5"
          />
          <path
            d="M7 8L11 12L7 16"
            stroke="#34D399"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
          <line
            x1="13"
            y1="16"
            x2="17"
            y2="16"
            stroke="#10B981"
            strokeWidth="2"
            strokeLinecap="round"
          />
        </svg>
      );
  }
}

