<script lang="ts">
  import { untrack } from "svelte";
  import StoreAgentTranscript from "../../web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte";
  import StoreReviewConversation from "../../web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte";
  import { createThreadsController } from "../../web/src/lib/controller/production-threads.svelte.js";
  import { principalMember } from "../../web/src/lib/collaboration/presentation.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { agentDetail, reviewThreads } from "./collaboration.js";
  import { createThreadTransport, THREAD_IDS } from "./thread-transport.js";
  let {
    surface = "agents",
    writable = true,
  }: {
    surface?: "agents" | "reviews";
    writable?: boolean;
  } = $props();
  const base = untrack(() => createInMemoryStoreController(surface));
  untrack(() => base.actions.selectReviewThread(reviewThreads[0]!.thread_id));
  const f = createThreadTransport();
  const threads = createThreadsController({
    source: () => f.transport,
    repository: () => THREAD_IDS.repo,
    principal: () => THREAD_IDS.actor,
    route: () => base.state.route,
    select: () => {},
  });
  const controller: StoreController = {
    threads,
    get state(): StoreController["state"] {
      return {
        ...base.state,
        repository: {
          ...base.state.repository,
          principal: THREAD_IDS.actor,
          permission: writable ? "write" : "read",
        },
      };
    },
    actions: base.actions,
  };
  const agent = {
    id: "captured-agent",
    name: "Codex",
    handle: "codex",
    initials: "CX",
    kind: "agent" as const,
  };
  const activity = agentDetail.activity.map((entry) =>
    entry.type === "message"
      ? {
          ...entry,
          reaction_target: {
            type: "agent_message" as const,
            session_id: agentDetail.session.session_id,
            message_id: entry.message_id,
            event_id: entry.event_id,
            batch: { kind: "event_batch" as const, id: "aa".repeat(32) },
            content: { kind: "content" as const, id: "bb".repeat(32) },
          },
        }
      : entry,
  );
  $effect(() => () => threads.clear());
</script>

{#if surface === "agents"}<StoreAgentTranscript
    {controller}
    {activity}
    {agent}
    member={principalMember(THREAD_IDS.actor)}
    sessionId={agentDetail.session.session_id}
  />{:else}<StoreReviewConversation
    {controller}
    showComposer={false}
    showDiff={false}
  />{/if}
