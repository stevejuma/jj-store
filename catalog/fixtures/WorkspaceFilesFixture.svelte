<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreApplication from "../../web/src/lib/ui/store-application/StoreApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import { agentsRoute } from "../../web/src/lib/controller/agents-route-codec.js";
  import {
    fixtureExecutionStatus,
    fixtureExecutionSetup,
  } from "./agent-execution-status.js";
  import { fixtureJsonResponse as json } from "./agent-execution-session.js";
  import { AgentSettingsFixture } from "./agent-settings-fixture.svelte.js";
  import { eventsResponse } from "./agent-execution-events.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import type { WorkspaceBrowser } from "../../web/src/lib/conversations/workspace-browser.svelte.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    controlReads = false,
    routeNavigation = false,
    missingBase = false,
    baseName = "main",
  }: {
    controlReads?: boolean;
    routeNavigation?: boolean;
    missingBase?: boolean;
    baseName?: string;
  } = $props();
  let holdNextRead = $state(false);
  let releaseRead = $state<(() => void) | undefined>();
  const base = createInMemoryStoreController("agents"),
    repository = "aa".repeat(16);
  const operation = "11".repeat(64),
    commit = "22".repeat(20);
  const status = fixtureExecutionStatus({
    repository,
    operation,
    commit,
    setup: fixtureExecutionSetup(null),
    status: "Online",
    preparing: false,
    alternativeAgent: false,
    permission: false,
    workspacePath: "/fixture/repository/agent-workspace",
    lastActivityAt: 1,
    sessionPreferences: undefined,
    agentModeReady: true,
    advertisedModels: true,
  });
  let route = $state(agentsRoute("session", status.sessions[0].id));
  const settings = new AgentSettingsFixture();
  const paths = ["README.md", "src/example.ts", "src/unchanged.ts"];
  const files = {
    "README.md": "# Workspace example\n",
    "src/example.ts": "export const greeting = 'café 🦀';\n",
    "src/unchanged.ts": "export const unchanged = true;\n",
  };
  const manager = new AgentExecution(
    () => true,
    async (input, init) => {
      const url = String(input);
      const configured = settings.respond(url, init);
      if (configured) return configured;
      if (url.includes("/workspace/inspect")) {
        if (holdNextRead) {
          holdNextRead = false;
          await new Promise<void>((resolve) => (releaseRead = resolve));
          releaseRead = undefined;
        }
        const request = JSON.parse(String(init?.body)) as Parameters<
          WorkspaceBrowser["read"]
        >[0];
        const changed = request.query.view === "changes";
        const selected = request.path as keyof typeof files | null;
        return json({
          version: 1,
          session: request.session,
          repository,
          workspace: status.sessions[0].workspace_name,
          location: status.sessions[0].workspace,
          operation,
          commit,
          selection: "fixture-selection",
          references: [{ name: baseName, commit, remote: null }],
          comparison: changed
            ? {
                range:
                  request.query.view === "changes"
                    ? request.query.range
                    : "branch",
                before_operation: operation,
                before_commit: "33".repeat(20),
                after_operation: operation,
                after_commit: commit,
                current_files: true,
                request_id: null,
                unavailable:
                  missingBase &&
                  request.query.view === "changes" &&
                  !request.query.base
                    ? "Choose a comparison base. No default branch is configured."
                    : null,
              }
            : null,
          incomplete: null,
          files: (changed ? ["src/example.ts"] : paths).map((path) => ({
            path,
            kind: "file",
            bytes: "40",
            fingerprint: "44".repeat(32),
            status: changed ? "modified" : null,
          })),
          next: null,
          file: selected
            ? {
                path: selected,
                contents: files[selected],
                before: changed ? "export const greeting = 'Hello';\n" : null,
                fingerprint: "44".repeat(32),
                unavailable: null,
              }
            : null,
        });
      }
      if (url.includes("/feedback"))
        return json({ messages: [], profile: "local_jj_git" });
      if (url.includes("/revisions")) return json({ entries: [], next: null });
      if (url.includes("/events"))
        return json(eventsResponse(false, false, false, undefined, false));
      if (url.includes("/targets")) return json([]);
      if (url.includes("/stream"))
        return new Response(
          new ReadableStream({
            start(c) {
              init?.signal?.addEventListener("abort", () => c.close(), {
                once: true,
              });
            },
          }),
          { headers: { "content-type": "text/event-stream" } },
        );
      return json(status);
    },
  );
  manager.onNavigate = (view, id, related) =>
    (route = agentsRoute(view, id, related));
  const controller = {
    ...base,
    execution: manager,
    get state() {
      return { ...base.state, route };
    },
  };
  const storage = untrack(createMemoryStoryStorage);
  onDestroy(() => manager.destroy());
</script>

{#if routeNavigation}
  <nav aria-label="Fixture pages">
    <Button
      onclick={() => (route = agentsRoute("session", status.sessions[0].id))}
      >Fixture inbox</Button
    >
    <Button
      onclick={() =>
        (route = { ...base.state.route, page: "projects", agentsHub: false })}
      >Fixture projects</Button
    >
    <Button
      onclick={() =>
        (route = { ...base.state.route, page: "files", agentsHub: false })}
      >Fixture repository</Button
    >
    <Button onclick={() => (route = agentsRoute("session", "ff".repeat(16)))}
      >Fixture other session</Button
    >
  </nav>
{/if}
{#if controlReads}
  {#if releaseRead}<Button onclick={() => releaseRead?.()}
      >Complete workspace read</Button
    >
  {:else}<Button onclick={() => (holdNextRead = true)}
      >Hold next workspace read</Button
    >{/if}
{/if}
<StoreApplication {controller} layoutStorage={storage} initiallyCollapsed />
