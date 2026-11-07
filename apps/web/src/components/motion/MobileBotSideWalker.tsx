"use client";

import { useEffect, useRef, useState } from "react";
import gsap from "gsap";
import { useReducedMotion } from "motion/react";

interface FloatingScore {
  id: number;
  text: string;
  topPx: number;
}

export default function MobileBotSideWalker() {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const positionWrapperRef = useRef<HTMLDivElement | null>(null);
  const botGraphicRef = useRef<HTMLDivElement | null>(null);
  const leg1Ref = useRef<SVGLineElement | null>(null);
  const leg2Ref = useRef<SVGLineElement | null>(null);
  const leg3Ref = useRef<SVGLineElement | null>(null);
  const leg4Ref = useRef<SVGLineElement | null>(null);
  const radarRef = useRef<SVGCircleElement | null>(null);
  const shouldReduceMotion = useReducedMotion();

  const [scores, setScores] = useState<FloatingScore[]>([]);
  const [comboCount, setComboCount] = useState(0);

  useEffect(() => {
    if (shouldReduceMotion) return;

    const posWrapper = positionWrapperRef.current;
    const botGraphic = botGraphicRef.current;
    if (!posWrapper) return;

    let posY = 140;
    let targetY = 140;
    let lastScrollY = window.scrollY;
    let walkPhase = 0;
    let isFlipping = false;
    let currentDir: "down" | "up" = "down";

    // Set initial position safely visible on screen
    gsap.set(posWrapper, { y: posY, force3D: true });

    const handleScroll = () => {
      const scrollY = window.scrollY;
      const docHeight = Math.max(1, document.documentElement.scrollHeight - window.innerHeight);
      const scrollFraction = Math.min(1, Math.max(0, scrollY / docHeight));

      // Calculate vertical screen position between 15% and 82% of screen height
      const minScreenY = 90;
      const maxScreenY = Math.max(minScreenY + 120, window.innerHeight - 110);
      targetY = minScreenY + scrollFraction * (maxScreenY - minScreenY);

      if (scrollY > lastScrollY + 2 && currentDir !== "down") {
        currentDir = "down";
        if (botGraphic && !isFlipping) {
          gsap.to(botGraphic, { scaleY: 1, duration: 0.2, ease: "power2.out" });
        }
      } else if (scrollY < lastScrollY - 2 && currentDir !== "up") {
        currentDir = "up";
        if (botGraphic && !isFlipping) {
          gsap.to(botGraphic, { scaleY: -1, duration: 0.2, ease: "power2.out" });
        }
      }
      lastScrollY = scrollY;
    };

    window.addEventListener("scroll", handleScroll, { passive: true });

    // GSAP Ticker for smooth 60fps mechanical walking simulation
    const tickerCallback = (_time: number, deltaTime: number) => {
      const dt = Math.min(deltaTime / 1000, 0.1);

      // Smooth lerp towards scroll position with subtle idle wander
      const idleSway = Math.sin(Date.now() * 0.002) * 10;
      const effectiveTargetY = targetY + idleSway;
      const diff = effectiveTargetY - posY;
      const speed = Math.abs(diff);

      posY += diff * Math.min(1, dt * 5);

      if (posWrapper) {
        gsap.set(posWrapper, { y: posY, force3D: true });
      }

      // Leg articulation kinematics
      walkPhase += speed > 1 ? 0.24 : 0.09;
      const legAngle1 = Math.sin(walkPhase) * 16;
      const legAngle2 = Math.cos(walkPhase) * 16;

      if (leg1Ref.current && leg2Ref.current && leg3Ref.current && leg4Ref.current) {
        leg1Ref.current.setAttribute("y2", (10 + legAngle1 * 0.35).toFixed(1));
        leg2Ref.current.setAttribute("y2", (26 - legAngle1 * 0.35).toFixed(1));
        leg3Ref.current.setAttribute("y2", (10 - legAngle2 * 0.35).toFixed(1));
        leg4Ref.current.setAttribute("y2", (26 + legAngle2 * 0.35).toFixed(1));
      }

      // Radar blink
      if (radarRef.current) {
        const radarAlpha = 0.5 + Math.sin(Date.now() * 0.007) * 0.45;
        radarRef.current.setAttribute("opacity", radarAlpha.toFixed(2));
      }
    };

    gsap.ticker.add(tickerCallback);

    return () => {
      window.removeEventListener("scroll", handleScroll);
      gsap.ticker.remove(tickerCallback);
    };
  }, [shouldReduceMotion]);

  // Gaming tap reaction: flip, sparkle, floating score pill, and vibration
  const handleBotTap = () => {
    const botGraphic = botGraphicRef.current;
    const posWrapper = positionWrapperRef.current;
    if (!botGraphic) return;

    // Optional haptic pulse on mobile devices
    if (typeof window !== "undefined" && "vibrate" in navigator) {
      try {
        navigator.vibrate([15, 25]);
      } catch {
        // Safe fallback
      }
    }

    const newCombo = comboCount + 1;
    setComboCount(newCombo);

    // GSAP 360-degree combat hop flip on the inner graphic only
    gsap.timeline()
      .to(botGraphic, {
        scale: 1.35,
        rotation: "+=360",
        duration: 0.35,
        ease: "back.out(2)",
      })
      .to(botGraphic, {
        scale: 1,
        duration: 0.2,
        ease: "power2.inOut",
      });

    // Calculate vertical coordinate for score tag
    let currentTop = 150;
    if (posWrapper) {
      const rect = posWrapper.getBoundingClientRect();
      currentTop = Math.max(80, rect.top);
    }

    const id = Date.now() + Math.random();
    const scoreText = newCombo > 1 ? `+${newCombo * 50} DEFUSED [${newCombo}x]` : "+50 DEFUSED";
    setScores((prev) => [...prev.slice(-3), { id, text: scoreText, topPx: currentTop }]);

    setTimeout(() => {
      setScores((prev) => prev.filter((s) => s.id !== id));
    }, 1200);
  };

  return (
    <aside
      ref={containerRef}
      data-testid="mobile-bot-walker"
      className="md:hidden fixed inset-y-0 right-0 w-11 pointer-events-none z-45 select-none overflow-visible"
      aria-label="Mobile Cyber-Bot Defense Companion"
    >
      {/* Vertical Tactical Energy Guide Rail */}
      <div className="absolute top-16 bottom-16 right-5 w-px border-r border-dashed border-[#FFD36B]/20 pointer-events-none" />

      {/* Floating score notices popping off the robot */}
      {scores.map((score) => (
        <div
          key={score.id}
          data-testid="mobile-bot-score"
          className="absolute right-12 whitespace-nowrap px-2 py-0.5 rounded bg-space-black/95 border border-[#FFD36B] text-[10px] font-mono font-black text-[#FFD36B] shadow-hard-sm pointer-events-none animate-bounce"
          style={{ top: score.topPx }}
        >
          {score.text}
        </div>
      ))}

      {/* Position Wrapper (Strictly driven by GSAP, no React inline style conflict) */}
      <div
        ref={positionWrapperRef}
        className="absolute right-1 w-10 h-10 pointer-events-none flex items-center justify-center"
      >
        {/* Interactive Cyber-Bot Unit */}
        <div
          ref={botGraphicRef}
          data-testid="mobile-bot-unit"
          onClick={handleBotTap}
          className="w-10 h-10 pointer-events-auto cursor-pointer flex items-center justify-center -webkit-tap-highlight-transparent active:scale-90 transition-transform"
          style={{ touchAction: "manipulation" }}
          title="Tap to defuse patrol bot!"
          role="button"
          tabIndex={0}
        >
          {/* Glow backdrop aura */}
          <div className="absolute inset-0 rounded-full bg-[#FFD36B]/25 blur-sm pointer-events-none" />

          {/* Scaled Mechanical Vector Cyber-Bot */}
          <svg
            viewBox="0 0 36 36"
            className="w-9 h-9 drop-shadow-[0_0_8px_rgba(244,102,27,0.85)]"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            {/* Articulated mechanical spider legs */}
            {/* Left legs */}
            <line
              ref={leg1Ref}
              x1="12"
              y1="14"
              x2="2"
              y2="10"
              stroke="#FFD36B"
              strokeWidth="2.2"
              strokeLinecap="round"
            />
            <line
              ref={leg2Ref}
              x1="12"
              y1="22"
              x2="2"
              y2="26"
              stroke="#F4661B"
              strokeWidth="2.2"
              strokeLinecap="round"
            />

            {/* Right legs */}
            <line
              ref={leg3Ref}
              x1="24"
              y1="14"
              x2="34"
              y2="10"
              stroke="#FFD36B"
              strokeWidth="2.2"
              strokeLinecap="round"
            />
            <line
              ref={leg4Ref}
              x1="24"
              y1="22"
              x2="34"
              y2="26"
              stroke="#F4661B"
              strokeWidth="2.2"
              strokeLinecap="round"
            />

            {/* Armor chassis */}
            <rect
              x="10"
              y="9"
              width="16"
              height="18"
              rx="4"
              fill="#120D24"
              stroke="#FFD36B"
              strokeWidth="1.8"
            />

            {/* Central Reactor Power Core */}
            <circle cx="18" cy="18" r="4.5" fill="#F4661B" />
            <circle cx="18" cy="18" r="2.5" fill="#FFD36B" />

            {/* Optical Scanner Radar Eye */}
            <circle
              ref={radarRef}
              cx="18"
              cy="12"
              r="2"
              fill="#4285F4"
              opacity="0.9"
            />

            {/* Micro antenna */}
            <line
              x1="18"
              y1="9"
              x2="18"
              y2="3"
              stroke="#FFD36B"
              strokeWidth="1.4"
              strokeLinecap="round"
            />
            <circle cx="18" cy="2.5" r="1.4" fill="#10B981" />
          </svg>
        </div>
      </div>
    </aside>
  );
}
