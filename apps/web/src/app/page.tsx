import Hero from "@/components/Hero";
import LiveTelemetry from "@/components/LiveTelemetry";
import MarqueeTicker from "@/components/MarqueeTicker";
import ScrollStorytelling from "@/components/ScrollStorytelling";
import EditorialBentoGrid from "@/components/EditorialBentoGrid";
import InteractiveTerminal from "@/components/InteractiveTerminal";
import AgentShowcase from "@/components/AgentShowcase";
import ArchitectureVisualization from "@/components/ArchitectureVisualization";
import TypographicInterlude from "@/components/TypographicInterlude";
import OpenSourceSection from "@/components/OpenSourceSection";

export default function HomePage() {
  return (
    <div className="flex flex-col w-full">
      <Hero />
      <LiveTelemetry />
      <MarqueeTicker />
      <ScrollStorytelling />
      <EditorialBentoGrid />
      <InteractiveTerminal />
      <AgentShowcase />
      <ArchitectureVisualization />
      <TypographicInterlude />
      <OpenSourceSection />
    </div>
  );
}
