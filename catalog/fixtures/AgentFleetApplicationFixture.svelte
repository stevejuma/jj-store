<script lang="ts">
  import { AgentSettingsFixture } from "./agent-settings-fixture.svelte.js";
  import { onDestroy, untrack } from "svelte";
  import StoreApplication from "../../web/src/lib/ui/store-application/StoreApplication.svelte";
  import { agentsRoute } from "../../web/src/lib/controller/agents-route-codec.js";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import {
    fixtureSession,
    fixtureRunners,
    fixtureJsonResponse as json,
  } from "./agent-execution-session.js";
  import type {
    AgentStatus,
    AgentEvent,
  } from "../../web/src/lib/controller/agent-execution-types.js";
  let {
    general = false,
    directory = false,
    multipleWorkspaces = false,
    modelFailure = false,
    catalogSession,
  }: {
    general?: boolean;
    directory?: boolean;
    multipleWorkspaces?: boolean;
    modelFailure?: boolean;
    catalogSession?: "live" | "remembered";
  } = $props();
  const base = untrack(() =>
    createInMemoryStoreController(directory ? "agents" : "branches"),
  );
  let requests = $state<Record<string, unknown>[]>([]);
  let catalogRequests = $state(0);
  let status: AgentStatus = untrack(() => ({
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
  }));
  const events: Record<string, AgentEvent[]> = {};
  const defaults = new AgentSettingsFixture();
  const manager = untrack(
    () =>
      new AgentExecution(
        () => true,
        async (input, init) => {
          const path = String(input);
          const settings = defaults.respond(path, init);
          if (settings) return settings;
          if (path.includes("/catalog")) {
            catalogRequests++;
            return modelFailure
              ? new Response(
                  JSON.stringify({ code: "agent_execution_unavailable" }),
                  {
                    status: 503,
                    headers: { "content-type": "application/json" },
                  },
                )
              : json({
                  runner: new URL(path, "http://localhost").searchParams.get(
                    "runner",
                  ),
                  facts: {
                    models: {
                      availableModels: [
                        { modelId: "fixture-model", name: "Fixture model" },
                      ],
                    },
                    configOptions: [
                      {
                        category: "thought_level",
                        options: [{ value: "high", name: "High" }],
                      },
                    ],
                  },
                });
          }
          if (path.includes("/targets"))
            return json(
              multipleWorkspaces
                ? [0, 1].map((n) => ({
                    id: `workspace-${n}`,
                    repository: base.state.repository.id,
                    name: n ? "Feature workspace" : "Original workspace",
                    folder: `/device/workspace-${n}`,
                    profile: "local_jj_git",
                    operation: String(n + 1).repeat(128),
                    commit: String(n + 1).repeat(40),
                  }))
                : [],
            );
          if (path.includes("/feedback"))
            return json({
              version: 1,
              profile: "local_jj_git",
              messages: [],
              write_unavailable: null,
            });
          if (path.includes("/events")) {
            const url = new URL(path, "http://localhost");
            const id = url.searchParams.get("session")!;
            const after = Number(url.searchParams.get("after"));
            return json({
              version: 1,
              events: (events[id] ?? []).filter(
                (e) => Number(e.sequence) > after,
              ),
              next: null,
            });
          }
          if (path.endsWith("/chat")) {
            const body = JSON.parse(String(init?.body));
            requests = [...requests, body];
            const session = fixtureSession({
              repository: "aa".repeat(16),
              operation: "11".repeat(64),
              commit: "22".repeat(20),
              status: "Online",
              permission: false,
              workspacePath: "/device/chat",
              lastActivityAt: Date.now(),
              preferences: status.setups[0]!.preferences,
              modeReady: true,
              advertisedModels: false,
            });
            session.conversation_id = body.conversation_id;
            session.setup_id = body.agent_ids[0];
            session.title = status.setups.find(
              (a) => a.id === body.agent_ids[0],
            )!.name;
            status = {
              ...status,
              sessions: [session],
              conversations: [
                {
                  id: body.conversation_id,
                  origin: body.origin,
                  home: session.repository,
                  discussion_thread: null,
                  participants: [session.id],
                },
              ],
            };
            events[session.id] = ["prompt", "activity", "record"].map(
              (type, index) => ({
                sequence: index + 1,
                observed_at_millis: Date.now(),
                committed: true,
                payload: {
                  type,
                  requestId: "turn",
                  text: body.text,
                  update: {
                    sessionUpdate: "agent_message_chunk",
                    content: { text: "Hello from the reusable agent." },
                  },
                  record: { status: "completed" },
                },
              }),
            );
            return json({
              request_id: body.request_id,
              conversation_id: body.conversation_id,
              agents: [
                {
                  version: 1,
                  request_id: body.request_id,
                  session_id: session.id,
                  state: "completed",
                  error: null,
                },
              ],
            });
          }
          if (path.endsWith("/control")) {
            const body = JSON.parse(String(init?.body));
            requests = [...requests, body];
            status.live_agents = [
              {
                agent_id: body.agent_id,
                state: body.start ? "idle" : "offline",
                enabled: body.start,
                generation: 1,
                error: null,
                restart_required: false,
                inference_host: null,
              },
            ];
            return json({
              version: 1,
              request_id: body.request_id,
              session_id: null,
              state: body.start ? "starting" : "stopping",
              error: null,
            });
          }
          return json(status);
        },
      ),
  );
  const controller = {
    ...base,
    execution: manager,
    get state() {
      return directory ? { ...base.state, route: agentsRoute() } : base.state;
    },
  };
  const storage = untrack(createMemoryStoryStorage);
  $effect(() => {
    if (general) untrack(() => manager.chat.showGeneral());
  });
  onDestroy(() => manager.destroy());
</script>

<StoreApplication {controller} layoutStorage={storage} initiallyCollapsed />
<output aria-label="Agent fleet requests">{JSON.stringify(requests)}</output>
<output aria-label="Model catalog requests">{catalogRequests}</output>
