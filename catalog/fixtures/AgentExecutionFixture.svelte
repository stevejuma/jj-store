<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import AgentExecutionPendingControls from "./AgentExecutionPendingControls.svelte";
  import {
    staleResumeOutcome,
    seedStaleResume,
    cancelledTurnEvents,
  } from "./agent-session-recovery.js";
  import * as lifecycle from "./agent-execution-lifecycle.svelte.js";
  import { AgentExecutionConfigurationFixture } from "./agent-execution-configuration.svelte.js";
  import {
    fixtureExecutionStatus,
    fixtureExecutionSetup,
  } from "./agent-execution-status.js";
  import { appendChatToolEvents } from "./agent-chat-tool-events.js";
  import { eventsResponse } from "./agent-execution-events.js";
  import * as discussion from "./agent-discussion-fixture.js";
  import AgentExecutionCommandOutput from "./AgentExecutionCommandOutput.svelte";
  import { commandReceipts } from "./agent-command-receipts.js";
  import { agentFreshnessFixture } from "./agent-freshness.js";
  import { revisionFixture } from "./agent-execution-revisions.js";
  import {
    fixtureJsonResponse as json,
    prepareSessionFixture,
    fixtureAgentTargets,
  } from "./agent-execution-session.js";
  import ApplicationStory from "./ApplicationStory.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { agentsRoute } from "../../web/src/lib/controller/agents-route-codec.js";
  import type { StoreRoute } from "../../web/src/lib/controller/contracts.js";
  import { untrack } from "svelte";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import type {
    AgentStatus,
    AgentSession,
  } from "../../web/src/lib/controller/agent-execution-types.js";
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
    staleResumeReceipt = false,
    retainedError = false,
    longHistory = false,
    chatFeedback,
    chatTools = false,
    agentReplies = false,
    unknownReply = false,
    feedbackWriteUnavailable = false,
    replayHistory = false,
    revisionIndexRequired = false,
    status = "Online",
    observeRuntime = false,
  }: {
    permission?: boolean;
    observeRuntime?: boolean;
    agentReplies?: boolean | "refresh-failed";
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
    staleResumeReceipt?: boolean;
    retainedError?: boolean;
    longHistory?: boolean;
    chatTools?: boolean | "grouped";
    chatFeedback?: "completed" | "working";
    revisionIndexRequired?: boolean;
    status?:
      "Online" | "Offline" | "Starting" | "Working" | "Unknown" | "Empty";
  } = $props();
  const setup = untrack(() => fixtureExecutionSetup(savedModel));
  let resolveConfiguration = $state<(() => void) | undefined>();
  let resolveHistory = $state<(() => void) | undefined>();
  let commands = $state<Record<string, unknown>[]>([]);
  let reads = $state<Record<string, number>>({});
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
  const configuration = untrack(
    () =>
      new AgentExecutionConfigurationFixture(
        () => fixtureStatus,
        (value) => (fixtureStatus = value),
        staleSave,
      ),
  );
  const feed = untrack(() => agentFreshnessFixture());
  const receipts = untrack(() =>
    commandReceipts(
      () => fixtureStatus.sessions[0],
      uncertainConfiguration,
      feed.changed,
    ),
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
  let route = $state<StoreRoute>(agentsRoute());
  const manager = untrack(
    () =>
      new AgentExecution(
        () => true,
        async (input, init) => {
          const path = String(input);
          if (!init?.body) reads = { ...reads, [path]: (reads[path] ?? 0) + 1 };
          const discussion = await discussionFixture(path, init);
          if (discussion) return discussion;
          const revision = revisions(path);
          if (revision) return revision;
          if (path.includes("/targets"))
            return json(fixtureAgentTargets(repository, operation, commit));
          const configured = configuration.respond(path, init);
          if (configured) return configured;
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
            if (staleResumeReceipt)
              return json(staleResumeOutcome(fixtureStatus.sessions[0]));
            else
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
            const result = eventsResponse(
              permission,
              retainedError,
              longHistory,
              chatFeedback,
              replayHistory,
            );
            result.events.push(...cancelledTurnEvents(lastCommand));
            if (chatTools)
              appendChatToolEvents(result.events, chatTools === "grouped");
            return json(result);
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
        feed.freshness,
      ),
  );
  untrack(() =>
    seedStaleResume(manager, fixtureStatus.sessions[0], staleResumeReceipt),
  );
  const { shell, inboxStorage } = untrack(lifecycle.fixtureAgentShell);
  untrack(() => {
    if (replayHistory) discussion.seedReplayDrafts(manager.discussions.drafts);
    manager.onNavigate = (view, id, relatedId) =>
      (route = agentsRoute(view, id, relatedId));
    if (status === "Unknown") manager.value = fixtureStatus;
  });
  lifecycle.own(manager, receipts, feed.freshness, shell);
  const base = untrack(() => createInMemoryStoreController("agents"));
  const application = {
    ...base,
    execution: manager,
    get state() {
      return {
        ...base.state,
        route,
        repositories: [
          {
            ...base.state.repository,
            id: repository,
            name: "Fixture repository",
          },
        ],
      };
    },
    actions: {
      ...base.actions,
      async openRoute(next: StoreRoute) {
        if (!next.agentsHub && next.commitId && next.snapshotOperationId)
          lastOpened = {
            repository: next.repository,
            commit: next.commitId,
            operation: next.snapshotOperationId,
          };
        route = next;
      },
    },
  };
</script>

<ApplicationStory controller={application} layoutStorage={inboxStorage} />
<FixtureControls
  ><div data-story-controls>
    <AgentExecutionPendingControls
      {manager}
      sessionId={fixtureStatus.sessions[0]?.id}
      controls={configuration.controls}
      bind:history={resolveHistory}
      bind:configuration={resolveConfiguration}
    />
    <AgentExecutionCommandOutput {commands} {lastCommand} {lastOpened} />
  </div></FixtureControls
>
<output hidden data-testid="inbox-read-counts">{JSON.stringify(reads)}</output>
