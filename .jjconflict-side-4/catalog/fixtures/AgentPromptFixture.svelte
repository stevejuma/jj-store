<script lang="ts">
  import StoreAgentTranscript from "../../web/src/lib/ui/store-collaboration/StoreAgentTranscript.svelte";
  import StoreAgentContext from "../../web/src/lib/ui/store-collaboration/StoreAgentContext.svelte";
  import CommunityMarkdownContent from "../../web/src/lib/ui/community-markdown-content/CommunityMarkdownContent.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { agentDetail } from "./collaboration.js";
  import { agentMember } from "../../web/src/lib/collaboration/presentation.js";
  import { agentPromptMarkup } from "./agent-prompt-markup.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  let {
    content = agentPromptMarkup,
    fallback = false,
    ordinary = false,
    occurredAt = 1_704_164_702_456,
  }: {
    content?: string;
    fallback?: boolean;
    ordinary?: boolean;
    occurredAt?: number | null;
  } = $props();
  const base = createInMemoryStoreController("agents");
  const controller: StoreController = {
    ...base,
    get state() {
      return {
        ...base.state,
        agentSession: { ...agentDetail, activity: [] },
        agentTranscript: [
          {
            sequence: 0,
            content,
            media_type: "text" as const,
            occurred_at_millis: occurredAt ?? undefined,
          },
        ],
      };
    },
  };
  const activity = $derived([
    {
      ...agentDetail.activity[0]!,
      type: "message" as const,
      occurred_at_millis: occurredAt ?? undefined,
      content: { media_type: "text" as const, content },
    },
  ] as typeof agentDetail.activity);
</script>

<main class="agent-prompt-fixture">
  <h1>Agent conversation</h1>
  {#if ordinary}<CommunityMarkdownContent {content} />
  {:else if fallback}<StoreAgentContext {controller} showHeader={false} />
  {:else}<StoreAgentTranscript
      {controller}
      {activity}
      agent={agentMember(agentDetail.session)}
      member={base.state.member}
      sessionId={agentDetail.session.session_id}
      interactive={false}
    />{/if}
</main>

<style>
  .agent-prompt-fixture {
    max-width: 48rem;
    height: 46rem;
    margin: auto;
    overflow: auto;
    padding: 1rem;
  }
</style>
