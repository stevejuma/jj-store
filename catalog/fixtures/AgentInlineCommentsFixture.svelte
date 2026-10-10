<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { AppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityAppShell from "../../web/src/lib/ui/community-app-shell/CommunityAppShell.svelte";
  import StoreAgentConversation from "../../web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte";
  import ChatToolPanel from "../../web/src/lib/ui/store-agent-chat-body/ChatToolPanel.svelte";
  import { provideConversationInspection } from "../../web/src/lib/conversations/inspection.svelte.js";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import {
    createAppearanceStore,
    provideAppearance,
  } from "../../web/src/lib/appearance/store.svelte.js";
  import {
    fixtureExecutionSetup,
    fixtureExecutionStatus,
  } from "./agent-execution-status.js";
  import { fixtureJsonResponse as json } from "./agent-execution-session.js";
  import { eventsResponse } from "./agent-execution-events.js";
  import { appendChatToolEvents } from "./agent-chat-tool-events.js";
  import { fixtureAgentShell } from "./agent-execution-lifecycle.svelte.js";
  import {
    inlineCommentTransport,
    inlineSession,
    type InlineAgentCommand,
  } from "./agent-inline-comment-transport.js";
  import "../../web/src/lib/ui/store-agent-execution/agent-execution.css";

  let {
    status = "Online",
    unpublished = false,
    writeUnavailable = false,
    unknown = false,
    tools = false,
  }: {
    status?: "Online" | "Offline" | "Working";
    unpublished?: boolean;
    writeUnavailable?: boolean;
    unknown?: boolean;
    tools?: boolean | "grouped";
  } = $props();
  let mounted = $state(false);
  let commands = $state<InlineAgentCommand[]>([]);
  let checks = $state<string[]>([]);
  const appearance = createAppearanceStore();
  provideAppearance(appearance);
  const { shell } = untrack(fixtureAgentShell);
  const inspection = provideConversationInspection(shell, () => null);
  let value = $state(
    untrack(() =>
      fixtureExecutionStatus({
        repository: "11".repeat(16),
        operation: "22".repeat(64),
        commit: "33".repeat(20),
        setup: fixtureExecutionSetup(null),
        status,
        preparing: false,
        alternativeAgent: false,
        permission: false,
        workspacePath: "/fixture/workspace",
        lastActivityAt: 1791324003000,
        agentModeReady: true,
        advertisedModels: false,
      }),
    ),
  );
  const transport = untrack(() =>
    inlineCommentTransport({
      unpublished,
      writeUnavailable,
      unknown,
      observe: (command, check) => {
        if (command) commands = [...commands, command];
        if (check) checks = [...checks, check];
      },
      resume: () => {
        value.sessions[0].connected = true;
        value.sessions[0].state = "idle";
      },
    }),
  );
  const history = untrack(() => {
    const result = eventsResponse(
      false,
      false,
      false,
      status === "Working" ? "working" : "completed",
    );
    if (tools) {
      appendChatToolEvents(result.events, tools === "grouped");
      const prompt = result.events.find(
        (event) => (event.payload as { type?: string }).type === "prompt",
      )!;
      const payload = prompt.payload as { context?: string };
      prompt.payload = {
        ...payload,
        sentContext: { delivery: "sent", wireContext: payload.context },
      };
    }
    return result;
  });
  const manager = untrack(
    () =>
      new AgentExecution(
        () => true,
        async (input, init) => {
          const path = String(input);
          const response = await transport.respond(path, init);
          if (response) return response;
          if (path.includes("/events")) return json(history);
          if (path.includes("/stream")) {
            let closed = false;
            return new Response(
              new ReadableStream({
                start(controller) {
                  init?.signal?.addEventListener(
                    "abort",
                    () => {
                      if (!closed) {
                        closed = true;
                        controller.close();
                      }
                    },
                    { once: true },
                  );
                },
                cancel() {
                  closed = true;
                },
              }),
              { headers: { "content-type": "text/event-stream" } },
            );
          }
          return json(value);
        },
        () => "22".repeat(16),
      ),
  );
  async function open() {
    await manager.ensure();
    await manager.select(inlineSession);
    mounted = true;
  }
  onDestroy(() => {
    manager.destroy();
    void shell.dispose();
  });
</script>

<CommunityAppShell
  controller={shell}
  displayMode="desktop"
  desktopChrome={false}
  style="height: 100dvh"
>
  <AppShell.Main>
    <div class="inline-fixture">
      <div role="group" aria-label="Fixture controls">
        <Button onclick={open}>Open fixture conversation</Button>
        <Button onclick={() => (mounted = false)}
          >Leave fixture conversation</Button
        >
        <Button
          onclick={() =>
            appearance.update({
              conversationView:
                appearance.live.conversationView === "document"
                  ? "classic"
                  : "document",
            })}>Switch renderer</Button
        >
        <Button
          onclick={async () => {
            await manager.command({ action: "resume" });
          }}>Resume fixture session</Button
        >
        <Button
          onclick={() => {
            transport.failFeedback();
            void manager.discussions.load(inlineSession);
          }}>Fail feedback refresh</Button
        >
        <Button
          onclick={() => {
            transport.recoverFeedback();
            void manager.discussions.load(inlineSession);
          }}>Refresh captured feedback</Button
        >
        <Button onclick={() => transport.confirm()}
          >Confirm fixture request</Button
        >
      </div>
      {#if mounted}<StoreAgentConversation {manager} />{/if}
    </div>
  </AppShell.Main>
  {#if inspection.current}<AppShell.Sidebar
      side="right"
      sidebarController={shell.right}
    >
      <ChatToolPanel
        tool={inspection.current.tool}
        state={inspection.current.state}
        onClose={() => void inspection.close()}
      />
    </AppShell.Sidebar>{/if}
</CommunityAppShell>
<output hidden data-testid="comment-commands">{JSON.stringify(commands)}</output
>
<output hidden data-testid="comment-checks">{JSON.stringify(checks)}</output>
<output hidden data-testid="comment-feedback-error"
  >{manager.discussions.errors[inlineSession] ?? ""}</output
>

<style>
  .inline-fixture {
    height: 100%;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
</style>
