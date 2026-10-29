import type { AgentStatus } from "../../web/src/lib/controller/agent-execution-types.js";
import { fixtureSession, fixtureRunners } from "./agent-execution-session.js";
export function fleetStatus(
  catalogSession: "live" | "remembered" | undefined,
): AgentStatus {
  return {
    version: 1,
    runtime: "ready",
    revision: 0,
    sessions: catalogSession
      ? [
          fixtureSession({
            repository: "aa".repeat(16),
            operation: "11".repeat(64),
            commit: "22".repeat(20),
            status: catalogSession === "live" ? "Idle" : "Offline",
            permission: false,
            workspacePath: "/device/chat",
            lastActivityAt: 1,
            preferences: { model: null, effort: null, mode: null },
            advertisedModels: true,
            modeReady: true,
          }),
        ]
      : [],
    runners: fixtureRunners,
    conversations: [],
    live_agents: [],
    setups: ["Helper", "Planner"].map((name, index) => ({
      id: `agent-${index}`,
      name,
      avatar: index ? "🦉" : "🤖",
      description: "",
      instructions: "",
      options: {
        inherit_defaults: false,
        provider: null,
        endpoint: null,
        environment: {},
      },
      runner: "codex",
      preferences: { model: null, effort: null, mode: "ask" },
    })),
  };
}
