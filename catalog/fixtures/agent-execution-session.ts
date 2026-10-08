import type { AgentSession } from "../../web/src/lib/controller/agent-execution-types.js";
export function fixtureSession(input: {
  repository: string;
  operation: string;
  commit: string;
  status: string;
  permission: boolean;
  workspacePath: string;
  lastActivityAt: number | null;
  preferences: AgentSession["preferences"];
  advertisedModels: boolean;
  modeReady: boolean;
}): AgentSession {
  return {
    conversation_id: null,
    repository_bindings: [],
    id: "44".repeat(16),
    setup_id: "helper",
    recorded_session_ids: ["a1".repeat(16)],
    workspace_name: "agent-" + "44".repeat(16),
    workspace_operation: input.operation,
    connected: !["Offline", "Starting"].includes(input.status),
    repository: input.repository,
    target_id: "fixture",
    runner: "codex",
    title: "Repository helper",
    avatar: "🤖",
    workspace: input.workspacePath,
    base_commit: input.commit,
    review_commit: input.commit,
    operation: input.operation,
    state:
      input.permission || input.status === "Working"
        ? "running"
        : input.status === "Offline"
          ? "ended"
          : input.status === "Starting"
            ? "opening"
            : "idle",
    adapter_session_id: "adapter",
    capabilities: {
      initialize: { loadSession: true },
      session: {
        modes: {
          availableModes: [
            { id: "ask", name: "Ask" },
            { id: "read-only", name: "Read only" },
          ],
        },
        ...(input.advertisedModels
          ? {
              models: {
                availableModels: [
                  { modelId: "legacy-model", name: "Legacy model" },
                ],
              },
              configOptions: [
                {
                  category: "model",
                  options: [{ value: "codex-fast", name: "Codex Fast" }],
                },
              ],
            }
          : {}),
      },
      modeReady: input.modeReady,
    },
    preferences: input.preferences,
    last_sequence: 2,
    published_sequence: 1,
    last_activity_at_millis: input.lastActivityAt,
    last_error: null,
  };
}

export const fixtureRunners = [
  {
    id: "cursor",
    label: "Cursor",
    available: true,
    qualified: false,
    install_hint: null,
    providers: [],
  },
  {
    id: "codex",
    label: "Codex",
    available: true,
    qualified: false,
    install_hint: null,
    providers: [],
  },
];

export function fixtureAgentTargets(
  repository: string,
  operation: string,
  commit: string,
) {
  return [
    {
      id: "fixture",
      repository,
      name: "Fixture",
      folder: "/device/source",
      profile: "local_jj_git",
      operation,
      commit,
    },
    {
      id: "alternate",
      repository,
      name: "Alternate workspace",
      folder: "/device/alternate",
      profile: "local_jj_git",
      operation,
      commit: "66".repeat(20),
    },
  ];
}

export const fixtureJsonResponse = (body: unknown, status = 200) =>
  new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json" },
  });

export function prepareSessionFixture(
  status: import("../../web/src/lib/controller/agent-execution-types.js").AgentStatus,
  empty: boolean,
  associated: boolean,
  retainedError: boolean,
) {
  if (empty) status.sessions = [];
  else if (!associated) status.sessions[0].setup_id = null;
  if (retainedError) status.sessions[0].last_error = "process_outcome_unknown";
}
