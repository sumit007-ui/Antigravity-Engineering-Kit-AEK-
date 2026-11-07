"use client";

import { useEffect, useRef, useState } from "react";
import { useReducedMotion } from "motion/react";

type BotType = "spider" | "drone" | "sentinel" | "orb" | "beetle";

interface Bot {
  id: number;
  x: number;
  y: number;
  vx: number;
  vy: number;
  angle: number;
  targetAngle: number;
  size: number;
  walkCycle: number;
  speed: number;
  type: BotType;
  color: string;
  glow: string;
  hp: number;
  idleTimer: number;
  targetX: number;
  targetY: number;
  destroyed: boolean;
  respawnTimer: number;
}

interface PixelDebris {
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  color: string;
  alpha: number;
  life: number;
  maxLife: number;
}

interface Shockwave {
  x: number;
  y: number;
  radius: number;
  maxRadius: number;
  alpha: number;
  color: string;
}

interface FloatingScore {
  id: number;
  x: number;
  y: number;
  text: string;
  color: string;
  alpha: number;
  life: number;
  maxLife: number;
}

const BOT_TYPES: BotType[] = ["spider", "drone", "sentinel", "orb", "beetle"];
const BOT_PALETTES = [
  { color: "#FFD36B", glow: "rgba(255, 211, 107, 0.85)" },
  { color: "#F4661B", glow: "rgba(244, 102, 27, 0.85)" },
  { color: "#FF8A3D", glow: "rgba(255, 138, 61, 0.85)" },
  { color: "#4285F4", glow: "rgba(66, 133, 244, 0.85)" },
  { color: "#10B981", glow: "rgba(16, 185, 129, 0.85)" },
];

export default function AutonomousAgentSwarm() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const containerRef = useRef<HTMLDivElement | null>(null);
  const shouldReduceMotion = useReducedMotion();
  const [stats, setStats] = useState({ score: 0, kills: 0, combo: 1 });
  const [showHud, setShowHud] = useState(false);

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;

    let ctx: CanvasRenderingContext2D | null = null;
    try {
      ctx = canvas.getContext("2d");
    } catch {
      return;
    }
    if (!ctx) return;

    let animId: number;
    let width = 0;
    let height = 0;

    const resize = () => {
      const rect = container.getBoundingClientRect();
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      width = rect.width;
      height = rect.height;
      canvas.width = width * dpr;
      canvas.height = height * dpr;
      canvas.style.width = `${width}px`;
      canvas.style.height = `${height}px`;
      ctx.scale(dpr, dpr);
    };

    resize();
    window.addEventListener("resize", resize);

    const mouse = {
      x: -1000,
      y: -1000,
      active: false,
    };

    let nextBotId = 1;
    const createBot = (fromEdge = false): Bot => {
      const type = BOT_TYPES[Math.floor(Math.random() * BOT_TYPES.length)];
      const palette = BOT_PALETTES[Math.floor(Math.random() * BOT_PALETTES.length)];
      let x = Math.random() * (width || 800);
      let y = Math.random() * (height || 500);

      if (fromEdge && width > 0 && height > 0) {
        const edge = Math.floor(Math.random() * 4);
        if (edge === 0) { x = -15; y = Math.random() * height; }
        else if (edge === 1) { x = width + 15; y = Math.random() * height; }
        else if (edge === 2) { x = Math.random() * width; y = -15; }
        else { x = Math.random() * width; y = height + 15; }
      }

      return {
        id: nextBotId++,
        x,
        y,
        vx: (Math.random() - 0.5) * 1.1,
        vy: (Math.random() - 0.5) * 1.1,
        angle: Math.random() * Math.PI * 2,
        targetAngle: Math.random() * Math.PI * 2,
        size: type === "sentinel" ? 14 : type === "spider" ? 12 : 10,
        walkCycle: Math.random() * 100,
        speed: type === "beetle" ? 2.1 : type === "drone" ? 1.6 : 1.1,
        type,
        color: palette.color,
        glow: palette.glow,
        hp: 1,
        idleTimer: Math.random() * 100,
        targetX: Math.random() * (width || 800),
        targetY: Math.random() * (height || 500),
        destroyed: false,
        respawnTimer: 0,
      };
    };

    const isMobileScreen = typeof window !== "undefined" && window.innerWidth < 640;
    const botCount = isMobileScreen ? 5 : 12;
    const bots: Bot[] = Array.from({ length: botCount }).map(() => createBot(false));
    const debris: PixelDebris[] = [];
    const shockwaves: Shockwave[] = [];
    const floatingScores: FloatingScore[] = [];

    let currentScore = 0;
    let currentKills = 0;
    let currentCombo = 1;
    let comboTimer = 0;

    // Mouse Listeners restricted to this section container
    const handleMouseMove = (e: MouseEvent) => {
      const rect = container.getBoundingClientRect();
      mouse.x = e.clientX - rect.left;
      mouse.y = e.clientY - rect.top;
      mouse.active = true;
    };

    const handleMouseEnter = (e: MouseEvent) => {
      const rect = container.getBoundingClientRect();
      mouse.x = e.clientX - rect.left;
      mouse.y = e.clientY - rect.top;
      mouse.active = true;
    };

    const handleMouseLeave = () => {
      mouse.active = false;
      mouse.x = -1000;
      mouse.y = -1000;
    };

    // Click to blast / eliminate bots inside section ("mrr vi skiye")
    const handlePointerDown = (e: MouseEvent | TouchEvent) => {
      const target = e.target as HTMLElement | null;
      if (target && target.closest("button, a, input, textarea, pre, code, [role='button']")) {
        return;
      }

      const rect = container.getBoundingClientRect();
      let clickX = mouse.x;
      let clickY = mouse.y;

      if ("clientX" in e) {
        clickX = e.clientX - rect.left;
        clickY = e.clientY - rect.top;
      } else if (e.touches && e.touches.length > 0) {
        clickX = e.touches[0].clientX - rect.left;
        clickY = e.touches[0].clientY - rect.top;
      }

      if (clickX < 0 || clickY < 0 || clickX > width || clickY > height) return;

      // Expand plasma shockwave
      shockwaves.push({
        x: clickX,
        y: clickY,
        radius: 3,
        maxRadius: 60,
        alpha: 1,
        color: "#FFD36B",
      });

      // Blast hit detection
      const blastRadius = 65;
      bots.forEach((bot) => {
        if (bot.destroyed) return;
        const dx = bot.x - clickX;
        const dy = bot.y - clickY;
        const dist = Math.hypot(dx, dy);

        if (dist <= blastRadius) {
          bot.destroyed = true;
          bot.respawnTimer = 110 + Math.random() * 70;

          // Explosion pixel debris
          for (let p = 0; p < 16; p++) {
            const pAngle = Math.random() * Math.PI * 2;
            const pSpeed = 1.4 + Math.random() * 3.8;
            debris.push({
              x: bot.x,
              y: bot.y,
              vx: Math.cos(pAngle) * pSpeed,
              vy: Math.sin(pAngle) * pSpeed,
              size: 2 + Math.random() * 2,
              color: Math.random() > 0.4 ? bot.color : "#FFFFFF",
              alpha: 1,
              life: 1,
              maxLife: 26 + Math.random() * 18,
            });
          }

          // Points
          currentKills++;
          currentCombo++;
          comboTimer = 110;
          const points = 100 * currentCombo;
          currentScore += points;

          floatingScores.push({
            id: Math.random(),
            x: bot.x,
            y: bot.y - 10,
            text: `+${points} ${currentCombo > 1 ? `x${currentCombo} COMBO!` : "DEFUSED"}`,
            color: bot.color,
            alpha: 1,
            life: 1,
            maxLife: 35,
          });

          setStats({
            score: currentScore,
            kills: currentKills,
            combo: currentCombo,
          });
          setShowHud(true);
        }
      });
    };

    const targetElement = container.closest("section") || container;

    targetElement.addEventListener("mousemove", handleMouseMove);
    targetElement.addEventListener("mouseenter", handleMouseEnter);
    targetElement.addEventListener("mouseleave", handleMouseLeave);
    targetElement.addEventListener("mousedown", handlePointerDown);
    targetElement.addEventListener("touchstart", handlePointerDown, { passive: true });

    let radarSpin = 0;

    // Simulation render loop
    const render = () => {
      ctx.clearRect(0, 0, width, height);
      radarSpin += 0.025;

      if (comboTimer > 0) {
        comboTimer--;
        if (comboTimer <= 0) {
          currentCombo = 1;
          setStats((prev) => ({ ...prev, combo: 1 }));
        }
      }

      // Render shockwaves
      for (let s = shockwaves.length - 1; s >= 0; s--) {
        const sw = shockwaves[s];
        sw.radius += 3.5;
        sw.alpha = 1 - sw.radius / sw.maxRadius;

        if (sw.alpha <= 0) {
          shockwaves.splice(s, 1);
          continue;
        }

        ctx.save();
        ctx.strokeStyle = sw.color;
        ctx.globalAlpha = sw.alpha * 0.7;
        ctx.lineWidth = 1.8;
        ctx.beginPath();
        ctx.arc(sw.x, sw.y, sw.radius, 0, Math.PI * 2);
        ctx.stroke();
        ctx.restore();
      }

      // Draw cursor reticle
      if (mouse.active && mouse.x > 0 && mouse.y > 0) {
        ctx.save();
        ctx.translate(mouse.x, mouse.y);
        ctx.strokeStyle = "rgba(255, 211, 107, 0.4)";
        ctx.lineWidth = 1;
        ctx.setLineDash([3, 4]);
        ctx.beginPath();
        ctx.arc(0, 0, 24, 0, Math.PI * 2);
        ctx.stroke();

        ctx.strokeStyle = "rgba(244, 102, 27, 0.7)";
        ctx.setLineDash([]);
        ctx.beginPath();
        ctx.arc(0, 0, 2.5, 0, Math.PI * 2);
        ctx.fill();
        ctx.restore();
      }

      // Constellation lines
      for (let i = 0; i < bots.length; i++) {
        if (bots[i].destroyed) continue;
        for (let j = i + 1; j < bots.length; j++) {
          if (bots[j].destroyed) continue;
          const d = Math.hypot(bots[i].x - bots[j].x, bots[i].y - bots[j].y);
          if (d < 100) {
            ctx.save();
            ctx.strokeStyle = `rgba(255, 211, 107, ${(1 - d / 100) * 0.18})`;
            ctx.lineWidth = 0.8;
            ctx.setLineDash([2, 4]);
            ctx.beginPath();
            ctx.moveTo(bots[i].x, bots[i].y);
            ctx.lineTo(bots[j].x, bots[j].y);
            ctx.stroke();
            ctx.restore();
          }
        }
      }

      // Update and render bots
      bots.forEach((bot, idx) => {
        if (bot.destroyed) {
          bot.respawnTimer--;
          if (bot.respawnTimer <= 0) {
            bots[idx] = createBot(true);
          }
          return;
        }

        if (mouse.active) {
          const mDist = Math.hypot(mouse.x - bot.x, mouse.y - bot.y);
          if (mDist < 260 && mDist > 30) {
            bot.targetX = mouse.x;
            bot.targetY = mouse.y;
          }
        } else {
          bot.idleTimer--;
          if (bot.idleTimer <= 0) {
            if (isMobileScreen) {
              // On mobile, keep bots patrolling perimeter edges to keep central text pristine
              const side = Math.floor(Math.random() * 4);
              if (side === 0) {
                bot.targetX = 15 + Math.random() * (width - 30);
                bot.targetY = 20 + Math.random() * 60;
              } else if (side === 1) {
                bot.targetX = 15 + Math.random() * (width - 30);
                bot.targetY = height - (20 + Math.random() * 70);
              } else if (side === 2) {
                bot.targetX = 15 + Math.random() * 50;
                bot.targetY = 20 + Math.random() * (height - 40);
              } else {
                bot.targetX = width - (15 + Math.random() * 50);
                bot.targetY = 20 + Math.random() * (height - 40);
              }
            } else {
              bot.targetX = 30 + Math.random() * (width - 60);
              bot.targetY = 30 + Math.random() * (height - 60);
            }
            bot.idleTimer = 80 + Math.random() * 140;
          }
        }

        const dx = bot.targetX - bot.x;
        const dy = bot.targetY - bot.y;
        const dist = Math.hypot(dx, dy);

        if (!shouldReduceMotion && dist > 4) {
          bot.targetAngle = Math.atan2(dy, dx);
          let diff = bot.targetAngle - bot.angle;
          while (diff < -Math.PI) diff += Math.PI * 2;
          while (diff > Math.PI) diff -= Math.PI * 2;
          bot.angle += diff * 0.12;

          bot.vx = Math.cos(bot.angle) * bot.speed;
          bot.vy = Math.sin(bot.angle) * bot.speed;
          bot.x += bot.vx;
          bot.y += bot.vy;
          bot.walkCycle += bot.speed * 0.35;
        }

        bot.x = Math.max(15, Math.min(width - 15, bot.x));
        bot.y = Math.max(15, Math.min(height - 15, bot.y));

        // RENDER BOT MODELS
        ctx.save();
        ctx.translate(bot.x, bot.y);
        ctx.rotate(bot.angle);

        if (bot.type === "spider") {
          // Model 1: SPIDER CRAWLER
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.2;
          const legSwing = Math.sin(bot.walkCycle) * 3.5;

          for (let side = -1; side <= 1; side += 2) {
            ctx.beginPath();
            ctx.moveTo(0, side * bot.size * 0.4);
            ctx.lineTo(-bot.size * 0.5, side * (bot.size * 0.8 + legSwing));
            ctx.lineTo(-bot.size * 0.9, side * (bot.size * 0.6 + legSwing));
            ctx.stroke();

            ctx.beginPath();
            ctx.moveTo(3, side * bot.size * 0.4);
            ctx.lineTo(bot.size * 0.5, side * (bot.size * 0.8 - legSwing));
            ctx.lineTo(bot.size * 0.9, side * (bot.size * 0.6 - legSwing));
            ctx.stroke();
          }

          ctx.fillStyle = "#120D24";
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.5;
          ctx.beginPath();
          ctx.arc(0, 0, bot.size * 0.5, 0, Math.PI * 2);
          ctx.fill();
          ctx.stroke();

          ctx.fillStyle = bot.color;
          ctx.shadowColor = bot.glow;
          ctx.shadowBlur = 6;
          ctx.beginPath();
          ctx.arc(bot.size * 0.25, 0, 2, 0, Math.PI * 2);
          ctx.fill();
          ctx.shadowBlur = 0;
        } else if (bot.type === "drone") {
          // Model 2: HOVER RECON DRONE
          ctx.fillStyle = "#0D0A1C";
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.5;
          ctx.beginPath();
          ctx.moveTo(bot.size * 0.8, 0);
          ctx.lineTo(-bot.size * 0.5, -bot.size * 0.5);
          ctx.lineTo(-bot.size * 0.2, 0);
          ctx.lineTo(-bot.size * 0.5, bot.size * 0.5);
          ctx.closePath();
          ctx.fill();
          ctx.stroke();

          ctx.fillStyle = "#FF8A3D";
          ctx.beginPath();
          ctx.arc(-bot.size * 0.5 - (Math.random() * 3), 0, 1.8, 0, Math.PI * 2);
          ctx.fill();
        } else if (bot.type === "sentinel") {
          // Model 3: ARMORED SENTINEL
          ctx.fillStyle = "#19122C";
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 2;
          ctx.beginPath();
          for (let h = 0; h < 6; h++) {
            const hAngle = (h * Math.PI) / 3;
            const hx = Math.cos(hAngle) * bot.size * 0.65;
            const hy = Math.sin(hAngle) * bot.size * 0.65;
            if (h === 0) ctx.moveTo(hx, hy);
            else ctx.lineTo(hx, hy);
          }
          ctx.closePath();
          ctx.fill();
          ctx.stroke();

          ctx.strokeStyle = "#FFFFFF";
          ctx.lineWidth = 1;
          ctx.beginPath();
          ctx.moveTo(0, 0);
          ctx.lineTo(Math.cos(radarSpin) * bot.size * 0.5, Math.sin(radarSpin) * bot.size * 0.5);
          ctx.stroke();
        } else if (bot.type === "orb") {
          // Model 4: QUANTUM ORB
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.2;
          ctx.beginPath();
          ctx.arc(0, 0, bot.size * 0.6, 0, Math.PI * 2);
          ctx.stroke();

          ctx.fillStyle = bot.color;
          ctx.beginPath();
          ctx.arc(0, 0, 2.5, 0, Math.PI * 2);
          ctx.fill();
        } else {
          // Model 5: SCOUT BEETLE
          ctx.fillStyle = "#160F2B";
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.4;
          ctx.beginPath();
          ctx.ellipse(0, 0, bot.size * 0.6, bot.size * 0.4, 0, 0, Math.PI * 2);
          ctx.fill();
          ctx.stroke();
        }

        ctx.restore();
      });

      // Render explosion pixel debris
      for (let p = debris.length - 1; p >= 0; p--) {
        const d = debris[p];
        d.x += d.vx;
        d.y += d.vy;
        d.vx *= 0.94;
        d.vy *= 0.94;
        d.life++;
        d.alpha = 1 - d.life / d.maxLife;

        if (d.alpha <= 0) {
          debris.splice(p, 1);
          continue;
        }

        ctx.save();
        ctx.fillStyle = d.color;
        ctx.globalAlpha = d.alpha;
        ctx.fillRect(d.x - d.size / 2, d.y - d.size / 2, d.size, d.size);
        ctx.restore();
      }

      // Render floating scores
      for (let f = floatingScores.length - 1; f >= 0; f--) {
        const fs = floatingScores[f];
        fs.y -= 0.8;
        fs.life++;
        fs.alpha = 1 - fs.life / fs.maxLife;

        if (fs.alpha <= 0) {
          floatingScores.splice(f, 1);
          continue;
        }

        ctx.save();
        ctx.font = "bold 11px monospace";
        ctx.fillStyle = fs.color;
        ctx.globalAlpha = fs.alpha;
        ctx.shadowColor = fs.color;
        ctx.shadowBlur = 8;
        ctx.fillText(fs.text, fs.x - 20, fs.y);
        ctx.restore();
      }

      animId = requestAnimationFrame(render);
    };

    animId = requestAnimationFrame(render);

    return () => {
      cancelAnimationFrame(animId);
      window.removeEventListener("resize", resize);
      targetElement.removeEventListener("mousemove", handleMouseMove);
      targetElement.removeEventListener("mouseenter", handleMouseEnter);
      targetElement.removeEventListener("mouseleave", handleMouseLeave);
      targetElement.removeEventListener("mousedown", handlePointerDown);
      targetElement.removeEventListener("touchstart", handlePointerDown);
    };
  }, [shouldReduceMotion]);

  return (
    <div
      ref={containerRef}
      className="absolute inset-0 pointer-events-auto overflow-hidden z-0 select-none"
      aria-hidden="true"
    >
      <canvas ref={canvasRef} className="w-full h-full block" />

      {/* Retro Classified Mini Arcade HUD Pill (Shown when eliminating bots) */}
      {showHud && (
        <div
          className="absolute bottom-3 left-4 z-20 flex items-center gap-2 px-2.5 py-1 rounded border border-primary-gold/50 bg-space-black/90 backdrop-blur-md text-[11px] font-mono font-bold text-primary-gold shadow-hard-sm"
          role="status"
        >
          <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
          <span>SCORE: {stats.score}</span>
          <span className="text-sand/50">|</span>
          <span className="text-warm-orange">DEFUSED: {stats.kills}</span>
          {stats.combo > 1 && (
            <span className="text-[10px] text-warm-orange font-black">
              {stats.combo}x COMBO!
            </span>
          )}
        </div>
      )}
    </div>
  );
}
