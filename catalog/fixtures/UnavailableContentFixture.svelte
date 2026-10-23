<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import ApplicationStory from "./ApplicationStory.svelte";
  import { onDestroy, untrack } from "svelte";
  import StoreWorkspaceInspector from "../../web/src/lib/ui/store-agent-workspace/StoreWorkspaceInspector.svelte";
  import { createThreadPanel } from "../../web/src/lib/ui/store-threads/thread-panel.svelte.js";
  import { createStoreShell } from "../../web/src/lib/ui/store-application/create-store-shell.js";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import { agentsRoute } from "../../web/src/lib/controller/agents-route-codec.js";
  import { createThreadsController } from "../../web/src/lib/controller/production-threads.svelte.js";
  import type {
    StoreController,
    StoreRoute,
  } from "../../web/src/lib/controller/contracts.js";
  import { LocalClient } from "../../web/src/lib/api/local-client.js";
  import { WorkspaceBrowser } from "../../web/src/lib/conversations/workspace-browser.svelte.js";
  import { ConversationSession } from "../../web/src/lib/conversations/session.svelte.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import {
    fixtureExecutionStatus,
    fixtureExecutionSetup,
  } from "./agent-execution-status.js";
  import { AgentSettingsFixture } from "./agent-settings-fixture.svelte.js";
  import { createThreadTransport, THREAD_IDS } from "./thread-transport.js";
  let {
    surface = "session",
  }: {
    surface?: "session" | "device" | "agents" | "thread" | "review" | "file";
  } = $props();
  const selected = untrack(() => surface);
  const storage = createMemoryStoryStorage();
  const shell = createStoreShell(storage, false);
  if (selected === "file") shell.right.open();
  const status = fixtureExecutionStatus({
    repository: THREAD_IDS.repo,
    operation: "22".repeat(64),
    commit: "33".repeat(20),
    setup: fixtureExecutionSetup(null),
    status: "Online",
    preparing: false,
    alternativeAgent: false,
    permission: false,
    workspacePath: "/fixture/workspace",
    lastActivityAt: 1,
    sessionPreferences: undefined,
    agentModeReady: true,
    advertisedModels: true,
  });
  status.sessions = [];
  const settings = new AgentSettingsFixture();
  const manager = new AgentExecution(
    () => selected !== "device" && selected !== "agents",
    async (input, init) =>
      settings.respond(String(input), init) ?? Response.json(status),
  );
  manager.value = status;
  let route = $state<StoreRoute>(
    selected === "agents"
      ? agentsRoute()
      : agentsRoute("session", "ff".repeat(16)),
  );
  manager.onNavigate = (view, id, related) =>
    (route = agentsRoute(view, id, related));
  const base = createInMemoryStoreController("reviews");
  const transport = createThreadTransport();
  transport.transport.thread = async () => {
    throw new Error("The selected Thread could not be loaded.");
  };
  const threadRoute = {
    ...base.state.route,
    page: "threads" as const,
    repository: THREAD_IDS.repo,
    threadId: THREAD_IDS.thread,
  };
  const threads = createThreadsController({
    directory: false,
    source: () => transport.transport,
    repository: () => THREAD_IDS.repo,
    principal: () => THREAD_IDS.actor,
    route: () => threadRoute,
    select: (id) => (threadRoute.threadId = id ?? THREAD_IDS.thread),
  });
  const controller: StoreController = {
    ...base,
    threads,
    execution: manager,
    get state() {
      return {
        ...base.state,
        route:
          selected === "thread"
            ? threadRoute
            : selected === "review"
              ? { ...base.state.route, reviewThreadId: "ff".repeat(16) }
              : route,
        reviewThread: undefined,
      };
    },
    actions: {
      ...base.actions,
      async openRoute(next: StoreRoute) {
        route = next;
      },
    },
  };
  createThreadPanel(
    () => {},
    () => {},
  );
  const browser = new WorkspaceBrowser(
    new LocalClient("", async () => Response.json({})),
    new ConversationSession("fixture"),
    () => {},
  );
  browser.fileError = "This file is unavailable or no longer accessible.";
  browser.listing.selected = "src/missing.ts";
  let fileOpen = $state(true);
  $effect(() => {
    if (selected === "thread") void threads.restore();
    return () => threads.clear();
  });
  onDestroy(() => {
    manager.destroy();
    browser.dispose();
    void shell.dispose();
  });
</script>

{#if selected === "file"}
  <AppShell.Root controller={shell} style="--ui-shell-height: 36rem">
    <AppShell.Main />
    <AppShell.Sidebar side="right" label="Workspace file">
      {#if fileOpen}<StoreWorkspaceInspector
          {browser}
          onClose={() => (fileOpen = false)}
        />{/if}
    </AppShell.Sidebar>
  </AppShell.Root>
{:else}
  <ApplicationStory {controller} layoutStorage={storage} />
{/if}
