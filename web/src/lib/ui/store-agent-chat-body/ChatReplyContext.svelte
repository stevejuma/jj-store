<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CornerUpLeftIcon from "@lucide/svelte/icons/corner-up-left";
  import ChatCommentArea from "./ChatCommentArea.svelte";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import type { ChatBodyItem } from "./model.js";
  let { item, state }: { item: ChatBodyItem; state: ChatBodyComments } =
    $props();
  const context = $derived(item.replyContext);
  const thread = $derived(
    state.threads.find((thread) => thread.id === context?.threadId),
  );
  const compact = $derived.by(() => {
    if (!context || !thread || !state.isAvailable(thread)) return false;
    const entries = thread.comments.filter((comment) =>
      context.commentIds.includes(comment.id),
    );
    if (!entries.length || entries.length !== context.commentIds.length)
      return false;
    const items = state.props().items;
    const index = items.findIndex((candidate) => candidate.id === item.id);
    if (index < 0) return false;
    const batchId = items
      .slice(0, index)
      .reverse()
      .find((candidate) => candidate.role === "user")?.commentBatchId;
    return (
      !!batchId &&
      !!state.disclosures[`batch:${batchId}`] &&
      entries.every((comment) => comment.batchId === batchId)
    );
  });
</script>

{#if context && thread}
  {#if compact}
    <div
      class="chat-reply-source"
      data-placement="reply"
      role="group"
      aria-label={`Reply context: ${thread.label}`}
    >
      <span
        >Reply to {context.commentIds.length}
        {context.commentIds.length === 1 ? "comment" : "comments"}</span
      >
      <Button
        variant="ghost"
        size="icon-xs"
        aria-label={`Go to source: ${thread.label}`}
        title={`Go to source: ${thread.label}`}
        onclick={(event) => state.goToSource(thread!, event.currentTarget)}
        ><CornerUpLeftIcon /></Button
      >
    </div>
  {:else}
    <ChatCommentArea
      {state}
      threadId={context.threadId}
      commentIds={context.commentIds}
      placement="reply"
    />
  {/if}
{/if}

<style>
  .chat-reply-source {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--muted-foreground);
    font-size: var(--mira-markdown-font-size, var(--mira-font-size, 1rem));
  }
</style>
