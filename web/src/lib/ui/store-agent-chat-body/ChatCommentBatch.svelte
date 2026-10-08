<script lang="ts">
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import ChatDisclosureIcon from "./ChatDisclosureIcon.svelte";
  import ChatCommentArea from "./ChatCommentArea.svelte";
  import type { ChatBodyComments } from "./comments.svelte.js";
  let { state, batchId }: { state: ChatBodyComments; batchId?: string } =
    $props();
  const entries = $derived(
    batchId
      ? state.threads.flatMap((thread) =>
          thread.comments
            .filter(
              (comment) =>
                (comment.batchId === batchId ||
                  comment.batchIds?.includes(batchId)) &&
                !comment.replyTo,
            )
            .map((comment) => ({ thread, comment })),
        )
      : state.pending,
  );
  const ids = $derived([...new Set(entries.map((e) => e.thread.id))]);
  const key = $derived(`batch:${batchId ?? "pending"}`);
  const open = $derived(state.disclosures[key] ?? !batchId);
</script>

{#if entries.length}
  <div
    class="chat-comment-batch"
    role="group"
    aria-label={batchId ? "Sent comments" : "Pending comments"}
    data-chat-pending={!batchId || undefined}
  >
    <Collapsible.Root
      {open}
      onOpenChange={(value) => (state.disclosures[key] = value)}
    >
      <div class="chat-comment-batch-heading">
        <CommunityMemberAvatar
          member={entries[0].comment.author}
          size="xsmall"
          decorative
        />
        <Collapsible.Trigger
          class="chat-context-trigger"
          aria-label={`${open ? "Collapse" : "Expand"} ${entries.length} ${batchId ? "sent" : "pending"} comments`}
        >
          <span
            >{entries.length}
            {entries.length === 1 ? "comment" : "comments"}</span
          ><ChatDisclosureIcon />
        </Collapsible.Trigger>
      </div>
      <Collapsible.Content>
        <div class="chat-comment-batch-content">
          {#each ids as threadId (threadId)}
            <ChatCommentArea
              {state}
              {threadId}
              placement={batchId ? "history" : "queue"}
              commentIds={entries
                .filter((e) => e.thread.id === threadId)
                .map((e) => e.comment.id)}
            />
          {/each}
        </div>
      </Collapsible.Content>
    </Collapsible.Root>
  </div>
{/if}

<style>
  .chat-comment-batch-heading {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .chat-comment-batch-content {
    margin-inline-start: 2rem;
    padding-block-start: 0.5rem;
    display: grid;
    gap: 0.5rem;
  }
  .chat-comment-batch {
    margin-block-end: 0.5rem;
  }
</style>
