<script lang="ts">
  import { fixtureExecutionStatus } from "./agent-execution-status.js";
  import { eventsResponse } from "./agent-execution-events.js";
  import * as discussion from "./agent-discussion-fixture.js";
  import AgentExecutionCommandOutput from "./AgentExecutionCommandOutput.svelte";
  import { commandReceipts } from "./agent-command-receipts.js";
  import { revisionFixture } from "./agent-execution-revisions.js";
  import {
    fixtureJsonResponse as json,
    prepareSessionFixture,
    fixtureAgentTargets,
  } from "./agent-execution-session.js";
  import { AppShell } from "@lapismd/design-core/shell";
  import CommunityAppShell from "../../web/src/lib/ui/community-app-shell/CommunityAppShell.svelte";
  import { createStoreShell } from "../../web/src/lib/ui/store-application/create-store-shell.js";
  import { agentsRoute } from "../../web/src/lib/controller/agents-route-codec.js";
  import StoreAgentDetailsSidebar from "../../web/src/lib/ui/store-agent-execution/StoreAgentDetailsSidebar.svelte";
  import StoreAgentExecutionNotice from "../../web/src/lib/ui/store-agent-execution/StoreAgentExecutionNotice.svelte";
  import type { StoreRoute } from "../../web/src/lib/controller/contracts.js";
  import { onDestroy, untrack } from "svelte";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import type {
    AgentStatus,
    AgentSession,
  } from "../../web/src/lib/controller/agent-execution-types.js";
  import StoreAgentExecutionHost from "../../web/src/lib/ui/store-agent-execution/StoreAgentExecutionHost.svelte";
  import StoreAgentLaunchButton from "../../web/src/lib/ui/store-agent-execution/StoreAgentLaunchButton.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  let {
    permission = false,
    fail = false,
    staleSave = false,
    preparing = false,
    emptySessions = false,
    alternativeAgent = false,
    holdHistory = false,
    lastActivityAt = Date.now(),
    workspacePath = "/device/agent-workspaces/helper",
    advertisedModels = false,
    savedModel = null,
    sessionPreferences,
    agentAssociated = true,
    agentModeReady = true,
    holdConfiguration = false,
    uncertainConfiguration = false,
    resumeFailure = false,
    retainedError = false,
    longHistory = false,
    chatFeedback,
    agentReplies = false,
    unknownReply = false,
    feedbackWriteUnavailable = false,
    replayHistory = false,
    revisionIndexRequired = false,
    status = "Online",
  }: {
    permission?: boolean;
    agentReplies?: boolean;
    unknownReply?: boolean;
    feedbackWriteUnavailable?: boolean;
    replayHistory?: boolean;
    fail?: boolean;
    staleSave?: boolean;
    preparing?: boolean;
    emptySessions?: boolean;
    alternativeAgent?: boolean;
    holdHistory?: boolean;
    lastActivityAt?: number | null;
    workspacePath?: string;
    advertisedModels?: boolean;
    savedModel?: string | null;
    sessionPreferences?: AgentSession["preferences"];
    agentAssociated?: boolean;
    agentModeReady?: boolean;
    holdConfiguration?: boolean;
    uncertainConfiguration?: boolean;
    resumeFailure?: boolean;
    retainedError?: boolean;
    longHistory?: boolean;
    chatFeedback?: "completed" | "working";
    revisionIndexRequired?: boolean;
    status?:
      "Online" | "Offline" | "Starting" | "Working" | "Unknown" | "Empty";
  } = $props();
  const setup = {
    id: "helper",
    name: "Repository helper",
    avatar: "🤖",
    description: "Repository review and edits",
    instructions: "Keep changes focused.",
    runner: "codex",
    preferences: {
      model: untrack(() => savedModel),
      effort: null,
      mode: "ask",
    },
  };
  let staleSaveRemaining = untrack(() => staleSave);
  let resolveConfiguration = $state<(() => void) | undefined>();
  let resolveHistory = $state<(() => void) | undefined>();
  let commands = $state<Record<string, unknown>[]>([]);
  let lastCommand = $state<Record<string, unknown> | null>(null);
  let lastOpened = $state<{
    repository: string;
    commit: string;
    operation: string;
  }>();
  const repository = "11".repeat(16),
    operation = "22".repeat(64),
    commit = "33".repeat(20);
  let fixtureStatus = $state<AgentStatus>(
    untrack(() =>
      fixtureExecutionStatus({
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
      }),
    ),
  );
  untrack(() =>
    prepareSessionFixture(
      fixtureStatus,
      emptySessions,
      agentAssociated,
      retainedError,
    ),
  );
  const receipts = untrack(() =>
    commandReceipts(() => fixtureStatus.sessions[0], uncertainConfiguration),
  );
  const revisions = untrack(() =>
    revisionFixture(repository, operation, revisionIndexRequired),
  );
  const discussionFixture = untrack(() =>
    discussion.agentDiscussionFixture(
      agentReplies,
      unknownReply,
      feedbackWriteUnavailable,
    ),
  );
  const manager = untrack(
    () =>
      new AgentExecution(
        () => true,
        async (input, init) => {
          const path = String(input);
          const discussion = await discussionFixture(path, init);
          if (discussion) return discussion;
          const revision = revisions(path);
          if (revision) return revision;
          if (path.includes("/targets"))
            return json(fixtureAgentTargets(repository, operation, commit));
          if (path.endsWith("/setups")) {
            const body = JSON.parse(String(init?.body));
            if (staleSaveRemaining) {
              staleSaveRemaining = false;
              fixtureStatus.revision = Number(fixtureStatus.revision) + 1;
            }
            if (
              Number(body.expected_revision) !== Number(fixtureStatus.revision)
            )
              return json({ code: "agent_selection_changed" }, 409);
            fixtureStatus = {
              ...fixtureStatus,
              revision: Number(fixtureStatus.revision) + 1,
              setups: [
                ...fixtureStatus.setups.filter(
                  (s) => s.id !== (body.setup?.id ?? body.remove),
                ),
                ...(body.setup ? [body.setup] : []),
              ],
            };
            return json(fixtureStatus);
          }
          if (path.endsWith("/command") || path.endsWith("/launch")) {
            if (fail) throw new TypeError("Connection lost");
            const body = JSON.parse(String(init?.body));
            if (path.endsWith("/command")) {
              lastCommand = body;
              commands = [...commands, body];
              if (body.action.action === "configure") {
                if (holdConfiguration)
                  await new Promise<void>((resolve) => {
                    resolveConfiguration = resolve;
                  });
              }
            }
            return json(receipts.dispatch(body, resumeFailure));
          }
          if (path.includes("/outcome"))
            return json(
              receipts.outcome(
                new URL(path, "http://fixture").searchParams.get("request") ??
                  "",
              ),
            );
          if (path.includes("/events")) {
            if (holdHistory)
              await new Promise<void>((resolve) => {
                resolveHistory = resolve;
              });
            return json(
              eventsResponse(
                permission,
                retainedError,
                longHistory,
                chatFeedback,
                replayHistory,
              ),
            );
          }
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
          if (status === "Unknown") throw new TypeError("Status unavailable");
          return json(fixtureStatus);
        },
        () => "22".repeat(16),
      ),
  );
  let route = $state<StoreRoute>(agentsRoute());
  const shell = untrack(() => createStoreShell(undefined, false));
  untrack(() => {
    if (replayHistory) discussion.seedReplayDrafts(manager.discussions.drafts);
    manager.onNavigate = (view, id, relatedId) =>
      (route = agentsRoute(view, id, relatedId));
    if (status === "Unknown") manager.value = fixtureStatus;
  });
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
    <div>
      {#if resolveHistory}<Button
          onclick={() => {
            resolveHistory?.();
            resolveHistory = undefined;
          }}>Resolve fixture history</Button
        >{/if}
      {#if resolveConfiguration}<Button
          onclick={() => {
            resolveConfiguration?.();
            resolveConfiguration = undefined;
          }}>Resolve fixture configuration</Button
        >{/if}
      <AgentExecutionCommandOutput {commands} {lastCommand} />
      <StoreAgentLaunchButton {manager} /><Button
        onclick={() => manager.openSession(fixtureStatus.sessions[0].id)}
        >Open fixture conversation</Button
      ><Button onclick={() => manager.directory()}>Manage fixture agents</Button
      >
    </div>
    {#if lastOpened}<output aria-label="Last opened revision"
        >{JSON.stringify(lastOpened)}</output
      >{/if}
    <StoreAgentExecutionHost
      {manager}
      {route}
      onReview={(repository, commit, operation) => {
        lastOpened = { repository, commit, operation };
      }}
    />
  </AppShell.Main>
  <StoreAgentDetailsSidebar {manager} {shell} {route} />
</CommunityAppShell>
<StoreAgentExecutionNotice
  {manager}
  activeSession={route.agentView === "session"
    ? route.executionSessionId
    : undefined}
/>
