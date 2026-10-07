<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import type { AgentStatus } from "../../web/src/lib/controller/agent-execution-types.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import {
    fixtureSession,
    fixtureJsonResponse,
  } from "./agent-execution-session.js";
  import { agentSessions } from "./collaboration.js";
  import { IDS, workingCopies } from "./repository.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    page = "agents",
    unmatched = false,
  }: { page?: "agents" | "files"; unmatched?: boolean } = $props();
  let posts = $state(0);
  const base = untrack(() =>
    createInMemoryStoreController(page, {
      workingCopies: [
        ...workingCopies,
        { ...workingCopies[0]!, workspace: "exact-agent-workspace" },
      ],
    }),
  );
  const captured = untrack(() => ({
    ...fixtureSession({
      repository: base.state.repository.id,
      operation: base.state.route.snapshotOperationId!,
      commit: IDS.current,
      status: "Offline",
      permission: false,
      workspacePath: "/fixture/agent",
      lastActivityAt: null,
      preferences: { model: null, effort: null, mode: null },
      advertisedModels: false,
      modeReady: false,
    }),
    title: "Nyra",
    avatar: "🐝",
    workspace_name: "exact-agent-workspace",
    recorded_session_ids: unmatched ? [] : [agentSessions[0]!.session_id],
  }));
  const status: AgentStatus = {
    version: 1,
    live_agents: [],
    conversations: [],
    runtime: "ready",
    revision: 1,
    setups: [],
    sessions: [captured],
    runners: [],
  };
  const manager = untrack(
    () =>
      new AgentExecution(
        () => true,
        async (input, init) => {
          if (init?.method === "POST") posts++;
          if (String(input).includes("/targets"))
            return fixtureJsonResponse([]);
          return fixtureJsonResponse(status);
        },
      ),
  );
  manager.value = status;
  onDestroy(() => manager.destroy());
  const controller: StoreController = {
    ...base,
    execution: manager,
    get state() {
      const view = base.state;
      return {
        ...view,
        agentSessions: view.agentSessions.map((session, index) =>
          index === 0
            ? {
                ...session,
                integration: "owned_controller" as const,
                first_activity_at_millis: null,
              }
            : session,
        ),
        agentSession: view.agentSession
          ? {
              ...view.agentSession,
              session: {
                ...view.agentSession.session,
                integration: "owned_controller" as const,
              },
            }
          : undefined,
      };
    },
  };
  const storage = untrack(createMemoryStoryStorage);
</script>

<StoreRepositoryApplication {controller} layoutStorage={storage} />
<output aria-label="Execution commands">{posts}</output>
