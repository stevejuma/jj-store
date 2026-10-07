import { fixtureSession, fixtureRunners } from "./agent-execution-session.js";
import type {
  AgentStatus,
  AgentSetup,
  AgentSession,
} from "../../web/src/lib/controller/agent-execution-types.js";
export function fixtureExecutionStatus(input: {
  repository: string;
  operation: string;
  commit: string;
  setup: AgentSetup;
  status: string;
  preparing: boolean;
  alternativeAgent: boolean;
  permission: boolean;
  workspacePath: string;
  lastActivityAt: number | null;
  sessionPreferences?: AgentSession["preferences"];
  agentModeReady: boolean;
  advertisedModels: boolean;
}): AgentStatus {
  const {
    repository,
    operation,
    commit,
    setup,
    status,
    preparing,
    alternativeAgent,
    permission,
    workspacePath,
    lastActivityAt,
    sessionPreferences,
    agentModeReady,
    advertisedModels,
  } = input;
  return {
    version: 1,
    live_agents: [],
    conversations: [],
    runtime: preparing ? "preparing" : "ready",
    revision: 0,
    setups:
      status === "Empty"
        ? []
        : [
            setup,
            ...(alternativeAgent
              ? [
                  {
                    ...setup,
                    id: "other-agent",
                    name: "Planning helper",
                    runner: "cursor",
                  },
                ]
              : []),
          ],
    sessions: [
      fixtureSession({
        repository,
        operation,
        commit,
        status,
        permission,
        workspacePath,
        lastActivityAt,
        preferences: sessionPreferences ?? setup.preferences,
        modeReady: agentModeReady,
        advertisedModels,
      }),
    ],
    runners: fixtureRunners,
  };
}
