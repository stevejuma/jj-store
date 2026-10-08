<script lang="ts">
  import { tick } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import type { ChatBodyBlock } from "./model.js";
  import { anchorKey } from "./anchors.js";
  let {
    state,
    block,
    threadId,
  }: { state: ChatBodyComments; block: ChatBodyBlock; threadId?: string } =
    $props();
  let root: HTMLElement;
  const thread = $derived(state.threads.find((t) => t.id === threadId));
  const active = $derived(
    state.active?.anchor.blockId === block.id &&
      (threadId ? state.active.threadId === threadId : !state.active.threadId),
  );
  let focusedKey = "";
  function focusDraft() {
    if (!active || !root.isConnected) return;
    const content = root.querySelector<HTMLElement>('[role="textbox"]');
    if (!content) return;
    content.focus();
    const range = document.createRange();
    range.selectNodeContents(content);
    range.collapse(false);
    const selection = window.getSelection();
    selection?.removeAllRanges();
    selection?.addRange(range);
  }
  $effect(() => {
    const key = active && state.active ? anchorKey(state.active.anchor) : "";
    if (key && focusedKey !== key) {
      focusedKey = key;
      void tick().then(focusDraft);
    } else if (!key) focusedKey = "";
  });
  function keydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      event.preventDefault();
      event.stopPropagation();
      state.close();
    }
    if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
      event.preventDefault();
      event.stopPropagation();
      state.submit();
    }
  }
  function shortcuts(node: HTMLFormElement) {
    node.addEventListener("keydown", keydown, true);
    return {
      destroy: () => node.removeEventListener("keydown", keydown, true),
    };
  }
</script>

<section
  bind:this={root}
  class="chat-comment-area"
  data-chat-comment-area
  role="group"
  aria-label={`Comments on ${block.label}`}
>
  {#if thread}
    <blockquote class="chat-comment-quote">{thread.anchor.quote}</blockquote>
    {#each thread.comments as comment (comment.id)}
      <article aria-label={`Comment by ${comment.author}`}>
        <strong>{comment.author}</strong>
        <CommunityMarkdownContent content={comment.text} />
      </article>
    {/each}
    {#if !active}<Button
        variant="ghost"
        size="sm"
        onclick={() => state.reveal(thread.anchor, block.label, thread.id)}
        >Reply</Button
      >{/if}
  {/if}
  {#if active}
    <form
      aria-label="Inline comment composer"
      use:shortcuts
      onsubmit={(event) => {
        event.preventDefault();
        state.submit();
      }}
    >
      <p class="chat-comment-caption">Comment on {block.label}</p>
      {#if !thread}<blockquote class="chat-comment-quote">
          {state.active?.anchor.quote}
        </blockquote>{/if}
      <CommunityChatComposer
        bind:value={state.draft}
        mode="form"
        showSubmitButton={false}
        ariaLabel={`Comment on ${block.label}`}
        placeholder="Write a comment"
        clearOnSubmit={false}
        onSubmit={() => state.submit()}
      />
      <div class="chat-comment-actions">
        <span>⌘ / Ctrl + Enter to add</span>
        <Button variant="ghost" size="sm" onclick={() => state.close(true)}
          >Cancel</Button
        >
        <Button
          size="sm"
          disabled={!state.draft.trim()}
          onclick={() => state.submit()}
        >
          {thread ? "Add reply" : "Add comment"}
        </Button>
      </div>
    </form>
  {/if}
</section>

<style>
  .chat-comment-area {
    padding: 0.8rem 1rem;
    border-inline-start: 3px solid var(--primary);
    background: var(--muted);
    border-radius: var(--radius);
    font-family: var(--font-sans, system-ui);
    font-size: 0.875rem;
    white-space: normal;
    color: var(--foreground);
  }
  .chat-comment-area article {
    margin-block: 0.75rem;
  }
  .chat-comment-quote {
    margin: 0 0 0.75rem;
    padding-inline-start: 0.75rem;
    border-inline-start: 2px solid var(--border);
    max-height: 5rem;
    overflow: auto;
    font-size: 0.8rem;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  .chat-comment-caption {
    margin: 0 0 0.5rem;
    font-weight: 600;
  }
  .chat-comment-actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    justify-content: flex-end;
    margin-block-start: 0.5rem;
    flex-wrap: wrap;
  }
  .chat-comment-actions span {
    margin-inline-end: auto;
    font-size: 0.75rem;
    color: var(--muted-foreground);
  }
</style>
