<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { onDestroy, untrack, tick } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import {
    createAppearanceStore,
    provideAppearance,
  } from "../../web/src/lib/appearance/store.svelte.js";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import type {
    AgentOutcome,
    AgentEvent,
    AgentAction,
  } from "../../web/src/lib/controller/agent-execution-types.js";
  import StoreAgentConversation from "../../web/src/lib/ui/store-agent-execution/StoreAgentConversation.svelte";
  import {
    fixtureExecutionSetup,
    fixtureExecutionStatus,
  } from "./agent-execution-status.js";
  import { eventsResponse } from "./agent-execution-events.js";
  import { inlineCommentTransport } from "./agent-inline-comment-transport.js";
  let {
    view = "document",
    delayed = false,
  }: { view?: "document" | "classic"; delayed?: boolean } = $props();
  const appearance = createAppearanceStore();
  untrack(() => appearance.update({ conversationView: view }));
  provideAppearance(appearance);
  const status = untrack(() =>
    fixtureExecutionStatus({
      repository: "11".repeat(16),
      operation: "22".repeat(64),
      commit: "33".repeat(20),
      setup: fixtureExecutionSetup(null),
      status: "Working",
      preparing: false,
      alternativeAgent: false,
      permission: false,
      workspacePath: "/fixture/queued-agent",
      lastActivityAt: 1791670000000,
      agentModeReady: true,
      advertisedModels: false,
    }),
  );
  const session = status.sessions[0];
  const receipts = new Map<string, AgentOutcome>();
  let commands = $state<Record<string, unknown>[]>([]),
    mounted = $state(true),
    rejectNext = $state(false);
  let active = "55".repeat(16);
  const pending: string[] = [];
  const comments = inlineCommentTransport({
    observe: () => {},
    resume: () => {},
  });
  const manager: AgentExecution = new AgentExecution(
    () => true,
    async (input, init): Promise<Response> => {
      const path = new URL(String(input), "http://fixture");
      if (path.pathname.endsWith("/agents/command")) {
        const body = JSON.parse(String(init?.body)) as {
          request_id: string;
          session_id: string;
          action: AgentAction;
        };
        commands = [...commands, body as unknown as Record<string, unknown>];
        if (body.action.action === "prompt" && rejectNext) {
          rejectNext = false;
          return Response.json(
            { code: "agent_feedback_limit" },
            { status: 413 },
          );
        }
        if (body.action.action === "prompt") {
          pending.push(body.request_id);
          const frame = {
            type: "prompt",
            requestId: body.request_id,
            text: body.action.text,
            sentContext: {
              version: 1,
              delivery: "prepared",
              wireText: body.action.text,
              wireContext: "",
              blocks: [],
            },
          };
          append(frame);
          const outcome: AgentOutcome = {
            version: 1,
            request_id: body.request_id,
            session_id: session.id,
            state: delayed ? "dispatching" : "queued",
            error: null,
            comment_publication: body.action.comments
              ? { request_id: body.request_id, state: "published", batch: null }
              : null,
          };
          receipts.set(body.request_id, outcome);
          if (!delayed)
            append({
              ...frame,
              type: "context_delivery",
              sentContext: { ...frame.sentContext, delivery: "queued" },
            });
          return Response.json(outcome);
        }
        if (body.action.action === "cancel") {
          const target = body.action.target;
          const index = pending.indexOf(target);
          if (index >= 0) pending.splice(index, 1);
          else if (active === target) {
            active = "";
            session.state = "idle";
          }
          const targetOutcome: AgentOutcome = {
            version: 1,
            request_id: target,
            session_id: session.id,
            state: "cancelled",
            error: null,
          };
          receipts.set(target, targetOutcome);
          append({
            type: "error",
            requestId: target,
            code: "dispatch_not_sent",
          });
        }
        const outcome: AgentOutcome = {
          version: 1,
          request_id: body.request_id,
          session_id: session.id,
          state: "accepted",
          error: null,
        };
        receipts.set(body.request_id, outcome);
        return Response.json(outcome);
      }
      if (path.pathname.endsWith("/agents/outcome"))
        return Response.json(receipts.get(path.searchParams.get("request")!));
      if (path.pathname.endsWith("/agents/events"))
        return Response.json({
          version: 1,
          next: null,
          events: manager.events.filter(
            (e) =>
              Number(e.sequence) > Number(path.searchParams.get("after") ?? 0),
          ),
        });
      if (path.pathname.endsWith("/agents")) return Response.json(status);
      if (path.pathname.endsWith("/agents/targets")) return Response.json([]);
      if (path.pathname.endsWith("/agents/chatgpt"))
        return Response.json({ supported: false, accounts: [], attempts: [] });
      const collaboration = await comments.respond(String(input), init);
      return (
        collaboration ??
        Response.json({
          messages: [],
          profile: "native",
          write_unavailable: null,
        })
      );
    },
    () => "22".repeat(16),
  );
  manager.value = structuredClone(status);
  manager.selected = session.id;
  manager.events = eventsResponse(false, false, false, "working").events;
  void manager.discussions.load(session.id);
  function append(payload: Record<string, unknown>) {
    const event: AgentEvent = {
      sequence: Number(manager.events.at(-1)?.sequence ?? 0) + 1,
      observed_at_millis: null,
      committed: false,
      payload,
    };
    manager.events = [...manager.events, event];
    session.last_sequence = event.sequence;
  }
  async function acknowledge() {
    for (const request of pending) {
      const outcome = receipts.get(request)!;
      receipts.set(request, { ...outcome, state: "queued" });
      append({
        type: "context_delivery",
        requestId: request,
        sentContext: { version: 1, delivery: "queued" },
      });
    }
    await manager.updates.status();
  }
  async function complete() {
    if (active)
      append({
        type: "record",
        requestId: active,
        record: { status: "completed" },
      });
    active = pending.shift() ?? "";
    session.state = active ? "running" : "idle";
    if (active) {
      receipts.set(active, { ...receipts.get(active)!, state: "started" });
      append({ type: "started", requestId: active });
    }
    await manager.updates.status();
  }
  async function review() {
    await manager.discussions.load(session.id);
    await tick();
    const conversation = manager.conversations.get(`session:${session.id}`);
    const block = conversation.projection.items
      .filter((i) => i.role === "assistant")
      .flatMap((i) => i.blocks)
      .find((b) => b.commentable);
    if (!block) return;
    conversation.comments.open(
      block,
      { from: 0, to: 0 },
      "Review this output next",
    );
    conversation.comments.active = undefined;
  }
  onDestroy(() => manager.destroy());
</script>

<main class="queue-fixture">
  <h1>Queued agent messages</h1>
  <div
    class="fixture-controls"
    role="group"
    aria-label="Queue fixture controls"
  >
    <Button
      onclick={() =>
        appearance.update({
          conversationView:
            appearance.live.conversationView === "document"
              ? "classic"
              : "document",
        })}>Switch view</Button
    >
    <Button onclick={() => (mounted = !mounted)}>Toggle chat</Button>
    <FixtureControls floating={false}
      ><Button onclick={() => void complete()}>Complete active turn</Button
      ></FixtureControls
    >
    <Button onclick={() => void acknowledge()}>Acknowledge queue</Button>
    <Button onclick={() => void review()}>Add review comment</Button>
    <Button onclick={() => (rejectNext = true)}>Reject next send</Button>
  </div>
  <output aria-label="Submitted queue commands" hidden
    >{JSON.stringify(commands)}</output
  >

  {#if mounted}<StoreAgentConversation {manager} />{/if}
</main>

<style>
  .queue-fixture {
    height: 100vh;
    display: flex;
    flex-direction: column;
    padding: 1rem;
    gap: 0.5rem;
  }
  h1 {
    font-size: 1.25rem;
    margin: 0;
  }
  .fixture-controls {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
  }
  .queue-fixture :global(.agent-conversation) {
    min-height: 0;
    flex: 1;
  }
</style>
