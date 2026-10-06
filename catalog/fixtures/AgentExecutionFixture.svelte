<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import type { AgentStatus } from "../../web/src/lib/controller/agent-execution-types.js";
  import StoreAgentExecutionHost from "../../web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte";
  import StoreAgentLaunchButton from "../../web/src/lib/ui/store-agent-execution/StoreAgentLaunchButton.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    permission = false,
    fail = false,
  }: { permission?: boolean; fail?: boolean } = $props();
  const setup = {
    id: "helper",
    name: "Repository helper",
    avatar: "🤖",
    instructions: "Keep changes focused.",
    runner: "codex",
    preferences: { model: null, effort: null, mode: "ask" },
  };
  const repository = "11".repeat(16),
    operation = "22".repeat(64),
    commit = "33".repeat(20);
  let state = $state<AgentStatus>({
    version: 1,
    runtime: "ready",
    revision: 0,
    setups: [setup],
    sessions: [
      {
        id: "44".repeat(16),
        repository,
        target_id: "fixture",
        runner: "codex",
        title: "Repository helper",
        avatar: "🤖",
        workspace: "/device/agent-workspaces/helper",
        base_commit: commit,
        review_commit: commit,
        operation,
        state: untrack(() => permission) ? "running" : "idle",
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
          },
          modeReady: true,
        },
        preferences: setup.preferences,
        last_sequence: 2,
        published_sequence: 1,
        last_error: null,
      },
    ],
    runners: [
      {
        id: "codex",
        label: "Codex",
        available: true,
        qualified: false,
        install_hint: null,
      },
    ],
  });
  const json = (body: unknown, status = 200) =>
    new Response(JSON.stringify(body), {
      status,
      headers: { "content-type": "application/json" },
    });
  const manager = untrack(
    () =>
      new AgentExecution(
        () => true,
        async (input, init) => {
          const path = String(input);
          if (path.includes("/targets"))
            return json([
              {
                id: "fixture",
                repository,
                name: "Fixture",
                folder: "/device/source",
                profile: "local_jj_git",
                operation,
                commit,
              },
            ]);
          if (path.endsWith("/setups")) {
            const body = JSON.parse(String(init?.body));
            state = {
              ...state,
              revision: Number(state.revision) + 1,
              setups: [
                ...state.setups.filter((s) => s.id !== body.setup?.id),
                ...(body.setup ? [body.setup] : []),
              ],
            };
            return json(state);
          }
          if (path.endsWith("/command") || path.endsWith("/launch")) {
            if (fail) throw new TypeError("Connection lost");
            const body = JSON.parse(String(init?.body));
            return json({
              version: 1,
              request_id: body.request_id,
              session_id: state.sessions[0].id,
              state: "queued",
              error: null,
            });
          }
          if (path.includes("/outcome"))
            return json({
              version: 1,
              request_id: new URL(path, "http://fixture").searchParams.get(
                "request",
              ),
              session_id: state.sessions[0].id,
              state: "outcome_unknown",
              error: "connection_lost",
            });
          if (path.includes("/events"))
            return json({
              version: 1,
              next: null,
              events: [
                {
                  sequence: 1,
                  committed: true,
                  payload: { type: "started", requestId: "55".repeat(16) },
                },
                {
                  sequence: 2,
                  committed: false,
                  payload: permission
                    ? {
                        type: "permission",
                        permissionId: "exact-permission",
                        requestId: "55".repeat(16),
                        request: {
                          toolCall: { title: "Edit source" },
                          options: [
                            {
                              optionId: "allow-exact",
                              kind: "allow_once",
                              name: "Allow once",
                            },
                            {
                              optionId: "reject-exact",
                              kind: "reject_once",
                              name: "Reject once",
                            },
                          ],
                        },
                      }
                    : {
                        type: "activity",
                        requestId: "55".repeat(16),
                        update: {
                          sessionUpdate: "agent_message_chunk",
                          content: { text: "Ready to help." },
                        },
                      },
                },
              ],
            });
          if (path.includes("/stream"))
            return new Response(
              new ReadableStream({
                start(controller) {
                  init?.signal?.addEventListener(
                    "abort",
                    () => controller.close(),
                    { once: true },
                  );
                },
              }),
              { headers: { "content-type": "text/event-stream" } },
            );
          return json(state);
        },
      ),
  );
  onDestroy(() => manager.destroy());
</script>

<StoreAgentLaunchButton {manager} />
<Button
  onclick={async () => {
    await manager.show();
    manager.launchOpen = false;
    await manager.select(state.sessions[0].id);
  }}>Open fixture conversation</Button
>
<StoreAgentExecutionHost {manager} />
