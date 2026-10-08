<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CornerUpLeftIcon from "@lucide/svelte/icons/corner-up-left";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import XIcon from "@lucide/svelte/icons/x";
  import MessageSquarePlusIcon from "@lucide/svelte/icons/message-square-plus";
  import CornerDownRightIcon from "@lucide/svelte/icons/corner-down-right";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import ChatCommentDelivery from "./ChatCommentDelivery.svelte";
  import ChatCommentEditor from "./ChatCommentEditor.svelte";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import "./comment-card.css";
  let {
    state,
    threadId,
    placement = "inline",
    commentIds,
  }: {
    state: ChatBodyComments;
    threadId: string;
    placement?: "inline" | "queue" | "summary" | "history" | "reply";
    commentIds?: readonly string[];
  } = $props();
  const thread = $derived(state.threads.find((t) => t.id === threadId));
  const comments = $derived.by(() => {
    const entries =
      thread?.comments.filter(
        (c) =>
          !c.replyTo &&
          (!commentIds
            ? !c.historical
            : commentIds.includes(c.id) ||
              (!!c.messageId &&
                !!c.revision &&
                commentIds.includes(`${c.messageId}:${c.revision.id}`))),
      ) ?? [];
    return placement === "summary"
      ? entries.filter((c) => c.text.trim()).slice(-1)
      : entries;
  });
  const source = $derived(
    placement === "queue" || placement === "history" || placement === "reply",
  );
  const editable = $derived(placement === "inline" || placement === "queue");
  const available = $derived(thread ? state.isAvailable(thread) : false);
  const hasDraft = $derived(
    thread?.comments.some(
      (c) => c.status === "draft" && c.author.id === state.author.id,
    ),
  );
  async function reply(opener: HTMLElement) {
    if (!thread) return;
    if (placement === "summary") await state.goToSource(thread, opener);
    state.begin(thread, "inline", "", hasDraft);
  }
</script>

{#if thread}
  <section
    class:chat-comment-context={source}
    class="chat-comment-area"
    data-chat-comment-area
    data-thread-id={threadId}
    data-placement={placement}
    onpointerenter={() => (state.highlightedThread = threadId)}
    onpointerleave={() => {
      if (state.highlightedThread === threadId)
        state.highlightedThread = undefined;
    }}
    role="group"
    aria-label={`Comments on ${thread.label}`}
  >
    {#if source}<blockquote
        class="chat-comment-quote"
        data-chat-quote={thread.anchor.quote}
      >
        {#if thread.sourceKind === "markdown" || thread.sourceKind === "context"}
          <CommunityMarkdownContent
            content={thread.anchor.quote}
            fullWidth
            promptElements={thread.sourceKind === "context"}
          />
        {:else}<code>{thread.anchor.quote}</code>{/if}
      </blockquote>{/if}
    {#if !available}<p class="chat-comment-unavailable">
        Original content unavailable or changed.
      </p>{/if}
    <div class="chat-comment-list">
      {#each comments as comment (comment.id)}
        <article
          class="chat-comment-card"
          aria-label={`Comment by ${comment.author.name}`}
          data-status={comment.status}
          data-comment-id={comment.id}
          tabindex="-1"
        >
          <CommunityMemberAvatar
            member={comment.author}
            size="xsmall"
            decorative
          />
          <div class="chat-comment-content">
            {#if editable && (comment.status === "draft" || comment.status === "sending")}
              <ChatCommentEditor
                {state}
                {thread}
                {comment}
                location={placement === "queue" ? "queue" : "inline"}
              />
            {:else}<CommunityMarkdownContent
                content={comment.text}
                fullWidth
              />{/if}
          </div>
          <div class="chat-comment-links">
            {#if (placement === "inline" || placement === "summary") && !comment.replyTo && (comment.status === "draft" || comment.status === "sending" || comment.batchId)}
              <Button
                variant="ghost"
                size="icon-xs"
                aria-label={comment.batchId
                  ? "View message"
                  : "Go to queued comment"}
                title={comment.batchId
                  ? "View message"
                  : "Go to queued comment"}
                onclick={() => state.goToMessage(thread!, comment)}
                ><CornerDownRightIcon /></Button
              >
            {/if}
            {#if source || placement === "summary"}
              <Button
                variant="ghost"
                size="icon-xs"
                disabled={!available &&
                  !(thread.source && state.props().onNavigateSource)}
                aria-label={`Go to source: ${thread.label}`}
                title="Go to source"
                onclick={(event) =>
                  state.goToSource(thread!, event.currentTarget)}
                ><CornerUpLeftIcon /></Button
              >
            {/if}
            {#if placement === "inline" && !comment.historical && comment.revision && comment.author.id === state.author.id && (comment.status === "sent" || comment.status === "replied")}
              <Button
                variant="ghost"
                size="icon-xs"
                disabled={state.sending || !available}
                aria-label="Edit comment"
                title="Edit comment"
                onclick={() => state.editSent(thread!, comment)}
                ><PencilIcon /></Button
              >
            {/if}
            {#if comment.status === "draft"}
              <Button
                variant="ghost"
                size="icon-xs"
                aria-label="Remove pending comment"
                title="Remove pending comment"
                onclick={() => state.remove(comment.id)}><XIcon /></Button
              >
            {/if}
            {#if (placement === "inline" || placement === "summary") && comment.id === comments.at(-1)?.id}
              <Button
                variant="ghost"
                size="icon-xs"
                disabled={state.sending || !available}
                aria-label={hasDraft ? "Add another comment" : "Reply"}
                title={hasDraft ? "Add another comment" : "Reply"}
                onclick={(event) => reply(event.currentTarget)}
                ><MessageSquarePlusIcon /></Button
              >
            {/if}
          </div>
          <ChatCommentDelivery {state} {comment} />
        </article>
      {/each}
    </div>
  </section>
{/if}
