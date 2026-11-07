"use client";

import { useEffect, useRef, useState } from "react";
import { useReducedMotion } from "motion/react";

type BotType = "spider" | "drone" | "sentinel" | "orb" | "beetle";

interface ArcadeBot {
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
  maxHp: number;
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
  { color: "#4285F4", glow: "rgba(66, 133, 244, 0.85)" },
  { color: "#10B981", glow: "rgba(16, 185, 129, 0.85)" },
  { color: "#A78BFA", glow: "rgba(167, 139, 250, 0.85)" },
];

export default function CyberBotArcade() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const shouldReduceMotion = useReducedMotion();
  const [stats, setStats] = useState({ score: 0, kills: 0, combo: 1 });
  const [showHud, setShowHud] = useState(false);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    let ctx: CanvasRenderingContext2D | null = null;
    try {
      ctx = canvas.getContext("2d");
    } catch {
      return;
    }
    if (!ctx) return;

    let animId: number;
    let width = window.innerWidth;
    let height = window.innerHeight;

    const resize = () => {
      width = window.innerWidth;
      height = window.innerHeight;
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      canvas.width = width * dpr;
      canvas.height = height * dpr;
      canvas.style.width = `${width}px`;
      canvas.style.height = `${height}px`;
      ctx.scale(dpr, dpr);
    };

    resize();
    window.addEventListener("resize", resize);

    // Mouse coordinates in viewport
    const mouse = {
      x: -1000,
      y: -1000,
      active: false,
    };

    // Initialize 16 diverse autonomous cyber bots
    let nextBotId = 1;
    const createBot = (fromEdge = false): ArcadeBot => {
      const type = BOT_TYPES[Math.floor(Math.random() * BOT_TYPES.length)];
      const palette = BOT_PALETTES[Math.floor(Math.random() * BOT_PALETTES.length)];
      let x = Math.random() * width;
      let y = Math.random() * height;

      if (fromEdge) {
        const edge = Math.floor(Math.random() * 4);
        if (edge === 0) { x = -20; y = Math.random() * height; }
        else if (edge === 1) { x = width + 20; y = Math.random() * height; }
        else if (edge === 2) { x = Math.random() * width; y = -20; }
        else { x = Math.random() * width; y = height + 20; }
      }

      return {
        id: nextBotId++,
        x,
        y,
        vx: (Math.random() - 0.5) * 1.2,
        vy: (Math.random() - 0.5) * 1.2,
        angle: Math.random() * Math.PI * 2,
        targetAngle: Math.random() * Math.PI * 2,
        size: type === "sentinel" ? 15 : type === "spider" ? 12 : 10,
        walkCycle: Math.random() * 100,
        speed: type === "beetle" ? 2.2 : type === "drone" ? 1.6 : 1.1,
        type,
        color: palette.color,
        glow: palette.glow,
        hp: type === "sentinel" ? 2 : 1,
        maxHp: type === "sentinel" ? 2 : 1,
        idleTimer: Math.random() * 80,
        targetX: Math.random() * width,
        targetY: Math.random() * height,
        destroyed: false,
        respawnTimer: 0,
      };
    };

    const bots: ArcadeBot[] = Array.from({ length: 16 }).map(() => createBot(false));
    const debris: PixelDebris[] = [];
    const shockwaves: Shockwave[] = [];
    const floatingScores: FloatingScore[] = [];

    let currentScore = 0;
    let currentKills = 0;
    let currentCombo = 1;
    let comboResetTimer = 0;

    // Track mouse movement
    const onMouseMove = (e: MouseEvent) => {
      mouse.x = e.clientX;
      mouse.y = e.clientY;
      mouse.active = true;
    };

    const onMouseLeave = () => {
      mouse.active = false;
      mouse.x = -1000;
      mouse.y = -1000;
    };

    // Click to shoot / eliminate bots ("mrr vi skiye")
    const onPointerDown = (e: MouseEvent | TouchEvent) => {
      // Don't fire if user is clicking an interactive element (button, link, code block, etc.)
      const target = e.target as HTMLElement | null;
      if (target && target.closest("button, a, input, textarea, pre, code, [role='button']")) {
        return;
      }

      let clickX = mouse.x;
      let clickY = mouse.y;

      if ("clientX" in e) {
        clickX = e.clientX;
        clickY = e.clientY;
      } else if (e.touches && e.touches.length > 0) {
        clickX = e.touches[0].clientX;
        clickY = e.touches[0].clientY;
      }

      if (clickX < 0 || clickY < 0) return;

      // Create plasma sonic blast shockwave
      shockwaves.push({
        x: clickX,
        y: clickY,
        radius: 4,
        maxRadius: 65,
        alpha: 1,
        color: "#FFD36B",
      });

      // Blast hit detection (within blast radius of 70px)
      const blastRadius = 70;
      let hitCount = 0;

      bots.forEach((bot) => {
        if (bot.destroyed) return;
        const dx = bot.x - clickX;
        const dy = bot.y - clickY;
        const dist = Math.sqrt(dx * dx + dy * dy);

        if (dist <= blastRadius) {
          bot.hp--;
          hitCount++;

          if (bot.hp <= 0) {
            bot.destroyed = true;
            bot.respawnTimer = 120 + Math.random() * 80;

            // Spawn 16 glowing pixel fragments
            for (let p = 0; p < 16; p++) {
              const pAngle = Math.random() * Math.PI * 2;
              const pSpeed = 1.5 + Math.random() * 4;
              debris.push({
                x: bot.x,
                y: bot.y,
                vx: Math.cos(pAngle) * pSpeed,
                vy: Math.sin(pAngle) * pSpeed,
                size: 2 + Math.random() * 2.5,
                color: Math.random() > 0.4 ? bot.color : "#FFFFFF",
                alpha: 1,
                life: 1,
                maxLife: 25 + Math.random() * 20,
              });
            }

            // Score pop-up
            currentKills++;
            currentCombo++;
            comboResetTimer = 120;
            const points = 100 * currentCombo;
            currentScore += points;

            floatingScores.push({
              id: Math.random(),
              x: bot.x,
              y: bot.y - 12,
              text: `+${points} ${currentCombo > 1 ? `x${currentCombo} COMBO!` : "NEUTRALIZED"}`,
              color: bot.color,
              alpha: 1,
              life: 1,
              maxLife: 40,
            });

            setStats({
              score: currentScore,
              kills: currentKills,
              combo: currentCombo,
            });
            setShowHud(true);
          } else {
            // Flash on damage
            bot.color = "#FFFFFF";
            setTimeout(() => {
              bot.color = "#FFD36B";
            }, 80);
          }
        }
      });
    };

    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("mouseleave", onMouseLeave);
    window.addEventListener("mousedown", onPointerDown);
    window.addEventListener("touchstart", onPointerDown, { passive: true });

    let radarSpin = 0;

    // Simulation Render Loop
    const render = () => {
      ctx.clearRect(0, 0, width, height);
      radarSpin += 0.025;

      // Handle combo decay
      if (comboResetTimer > 0) {
        comboResetTimer--;
        if (comboResetTimer <= 0) {
          currentCombo = 1;
          setStats((prev) => ({ ...prev, combo: 1 }));
        }
      }

      // Draw active shockwaves
      for (let s = shockwaves.length - 1; s >= 0; s--) {
        const sw = shockwaves[s];
        sw.radius += 3.8;
        sw.alpha = 1 - sw.radius / sw.maxRadius;

        if (sw.alpha <= 0) {
          shockwaves.splice(s, 1);
          continue;
        }

        ctx.save();
        ctx.strokeStyle = sw.color;
        ctx.globalAlpha = sw.alpha * 0.75;
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.arc(sw.x, sw.y, sw.radius, 0, Math.PI * 2);
        ctx.stroke();

        // Inner shockwave ripple
        ctx.strokeStyle = "#F4661B";
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.arc(sw.x, sw.y, sw.radius * 0.65, 0, Math.PI * 2);
        ctx.stroke();
        ctx.restore();
      }

      // Draw cursor target reticle & lock lines to nearest bot
      if (mouse.active && mouse.x > 0 && mouse.y > 0) {
        let closestBot: ArcadeBot | null = null;
        let closestDist = 200;

        bots.forEach((b) => {
          if (b.destroyed) return;
          const d = Math.hypot(b.x - mouse.x, b.y - mouse.y);
          if (d < closestDist) {
            closestDist = d;
            closestBot = b;
          }
        });

        if (closestBot) {
          const cb = closestBot as ArcadeBot;
          ctx.save();
          ctx.strokeStyle = "rgba(255, 211, 107, 0.4)";
          ctx.lineWidth = 0.8;
          ctx.setLineDash([3, 4]);
          ctx.beginPath();
          ctx.moveTo(mouse.x, mouse.y);
          ctx.lineTo(cb.x, cb.y);
          ctx.stroke();

          // Target lock marker over closest bot
          ctx.strokeStyle = cb.color;
          ctx.lineWidth = 1.2;
          ctx.setLineDash([]);
          ctx.strokeRect(cb.x - cb.size - 4, cb.y - cb.size - 4, (cb.size + 4) * 2, (cb.size + 4) * 2);
          ctx.restore();
        }
      }

      // Update and render Bots
      bots.forEach((bot, bIndex) => {
        // Handle respawning
        if (bot.destroyed) {
          bot.respawnTimer--;
          if (bot.respawnTimer <= 0) {
            bots[bIndex] = createBot(true);
          }
          return;
        }

        // AI Movement: roam or gravitate towards cursor
        if (mouse.active) {
          const mDist = Math.hypot(mouse.x - bot.x, mouse.y - bot.y);
          if (mDist < 300 && mDist > 35) {
            bot.targetX = mouse.x;
            bot.targetY = mouse.y;
          }
        } else {
          bot.idleTimer--;
          if (bot.idleTimer <= 0) {
            bot.targetX = 40 + Math.random() * (width - 80);
            bot.targetY = 40 + Math.random() * (height - 80);
            bot.idleTimer = 100 + Math.random() * 150;
          }
        }

        const dx = bot.targetX - bot.x;
        const dy = bot.targetY - bot.y;
        const dist = Math.hypot(dx, dy);

        if (!shouldReduceMotion && dist > 5) {
          bot.targetAngle = Math.atan2(dy, dx);
          let diff = bot.targetAngle - bot.angle;
          while (diff < -Math.PI) diff += Math.PI * 2;
          while (diff > Math.PI) diff -= Math.PI * 2;
          bot.angle += diff * 0.1;

          bot.vx = Math.cos(bot.angle) * bot.speed;
          bot.vy = Math.sin(bot.angle) * bot.speed;
          bot.x += bot.vx;
          bot.y += bot.vy;
          bot.walkCycle += bot.speed * 0.3;
        }

        // Screen boundary bounce
        if (bot.x < 15) { bot.x = 15; bot.vx *= -1; }
        if (bot.x > width - 15) { bot.x = width - 15; bot.vx *= -1; }
        if (bot.y < 15) { bot.y = 15; bot.vy *= -1; }
        if (bot.y > height - 15) { bot.y = height - 15; bot.vy *= -1; }

        // RENDER BOT MODELS
        ctx.save();
        ctx.translate(bot.x, bot.y);
        ctx.rotate(bot.angle);

        if (bot.type === "spider") {
          // Model 1: SPIDER CRAWLER (Jointed stepping legs)
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.2;
          const swing = Math.sin(bot.walkCycle) * 3;

          for (let side = -1; side <= 1; side += 2) {
            ctx.beginPath();
            ctx.moveTo(0, side * bot.size * 0.4);
            ctx.lineTo(-bot.size * 0.5, side * (bot.size * 0.8 + swing));
            ctx.lineTo(-bot.size * 0.9, side * (bot.size * 0.6 + swing));
            ctx.stroke();

            ctx.beginPath();
            ctx.moveTo(3, side * bot.size * 0.4);
            ctx.lineTo(bot.size * 0.5, side * (bot.size * 0.8 - swing));
            ctx.lineTo(bot.size * 0.9, side * (bot.size * 0.6 - swing));
            ctx.stroke();
          }

          ctx.fillStyle = "#120D24";
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.5;
          ctx.beginPath();
          ctx.arc(0, 0, bot.size * 0.5, 0, Math.PI * 2);
          ctx.fill();
          ctx.stroke();

          // Optic core
          ctx.fillStyle = bot.color;
          ctx.shadowColor = bot.glow;
          ctx.shadowBlur = 6;
          ctx.beginPath();
          ctx.arc(bot.size * 0.25, 0, 2, 0, Math.PI * 2);
          ctx.fill();
          ctx.shadowBlur = 0;
        } else if (bot.type === "drone") {
          // Model 2: HOVER RECON DRONE (Diamond body + thruster sparks)
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

          // Thruster flame
          ctx.fillStyle = "#FF8A3D";
          ctx.beginPath();
          ctx.arc(-bot.size * 0.5 - (Math.random() * 3), 0, 1.8, 0, Math.PI * 2);
          ctx.fill();
        } else if (bot.type === "sentinel") {
          // Model 3: ARMORED SENTINEL (Heavy hexagon with radar dish)
          ctx.fillStyle = "#19122C";
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 2;
          ctx.beginPath();
          for (let h = 0; h < 6; h++) {
            const hAngle = (h * Math.PI) / 3;
            const hx = Math.cos(hAngle) * bot.size * 0.7;
            const hy = Math.sin(hAngle) * bot.size * 0.7;
            if (h === 0) ctx.moveTo(hx, hy);
            else ctx.lineTo(hx, hy);
          }
          ctx.closePath();
          ctx.fill();
          ctx.stroke();

          // Rotating radar line
          ctx.strokeStyle = "#FFFFFF";
          ctx.lineWidth = 1;
          ctx.beginPath();
          ctx.moveTo(0, 0);
          ctx.lineTo(Math.cos(radarSpin) * bot.size * 0.5, Math.sin(radarSpin) * bot.size * 0.5);
          ctx.stroke();
        } else if (bot.type === "orb") {
          // Model 4: QUANTUM ORB (Spinning rings)
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
          // Model 5: SCOUT BEETLE (Fast oval bug)
          ctx.fillStyle = "#160F2B";
          ctx.strokeStyle = bot.color;
          ctx.lineWidth = 1.4;
          ctx.beginPath();
          ctx.ellipse(0, 0, bot.size * 0.6, bot.size * 0.4, 0, 0, Math.PI * 2);
          ctx.fill();
          ctx.stroke();

          // Antennae
          ctx.beginPath();
          ctx.moveTo(bot.size * 0.5, -2);
          ctx.lineTo(bot.size * 0.8, -4);
          ctx.moveTo(bot.size * 0.5, 2);
          ctx.lineTo(bot.size * 0.8, 4);
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

      // Render floating score indicators
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
      window.removeEventListener("mousemove", onMouseMove);
      window.removeEventListener("mouseleave", onMouseLeave);
      window.removeEventListener("mousedown", onPointerDown);
      window.removeEventListener("touchstart", onPointerDown);
    };
  }, [shouldReduceMotion]);

  return (
    <>
      <div
        className="fixed inset-0 pointer-events-none z-0 overflow-hidden"
        aria-hidden="true"
      >
        <canvas ref={canvasRef} className="w-full h-full block" />
      </div>

      {/* Retro Classified Cyber Defense Arcade HUD Pill (Shown when playing) */}
      {showHud && (
        <div
          className="fixed bottom-4 left-4 z-30 pointer-events-auto flex items-center gap-2.5 px-3 py-1.5 rounded-lg border-2 border-primary-gold/60 bg-space-black/95 backdrop-blur-md text-xs font-mono font-bold text-primary-gold shadow-hard-md select-none animate-in fade-in slide-in-from-bottom-2 duration-300"
          role="status"
          aria-live="polite"
        >
          <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
          <span>CYBER_DEFENSE</span>
          <span className="text-paper-beige">|</span>
          <span className="text-paper-beige">SCORE: {stats.score}</span>
          <span className="text-paper-beige">|</span>
          <span className="text-warm-orange">KILLS: {stats.kills}</span>
          {stats.combo > 1 && (
            <span className="px-1.5 py-0.5 rounded bg-warm-orange/20 border border-warm-orange text-warm-orange text-[10px] animate-pulse">
              {stats.combo}x COMBO
            </span>
          )}
          <button
            onClick={() => setShowHud(false)}
            className="ml-1 text-sand/60 hover:text-paper-beige text-[10px]"
            title="Dismiss HUD"
            aria-label="Dismiss arcade HUD"
          >
            ✕
          </button>
        </div>
      )}
    </>
  );
}
