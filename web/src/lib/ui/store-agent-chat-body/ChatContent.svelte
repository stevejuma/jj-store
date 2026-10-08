<script lang="ts">
  import ChatHoverActions from "./ChatHoverActions.svelte";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import ChatTextBlock from "./ChatTextBlock.svelte";
  import ChatCommentArea from "./ChatCommentArea.svelte";
  import type { ChatBodyBlock, ChatBodyProps } from "./model.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import { validAnchor } from "./anchors.js";
  import { customInteraction } from "./custom-interaction.js";
  let {
    blocks,
    state,
    customBlock,
    showHeaders = true,
    reactionTarget,
  }: {
    blocks: readonly ChatBodyBlock[];
    state: ChatBodyComments;
    customBlock?: ChatBodyProps["customBlock"];
    showHeaders?: boolean;
    reactionTarget?: string;
  } = $props();
</script>

{#each blocks as block (block.id)}
  {#key `${block.id}:${block.revision}`}
    {#if block.kind === "custom" || block.kind === "context"}
      <section
        class="chat-custom"
        data-kind={block.kind}
        role="group"
        aria-label={block.label}
        use:customInteraction={{ block, state }}
        data-highlighted={state.active?.anchor.blockId === block.id}
      >
        {#if block.kind === "context"}<CommunityMarkdownContent
            content={block.text}
            promptElements
            fullWidth
          />
        {:else if customBlock}{@render customBlock(block)}
        {:else}<CommunityMarkdownContent content={block.text} fullWidth />{/if}
        <ChatHoverActions
          {block}
          {state}
          {reactionTarget}
          onComment={() => state.open(block, { from: 0, to: 0 })}
        />
        {#each state.threads.filter( (t) => validAnchor(block, t.anchor) ) as thread (thread.id)}
          <ChatCommentArea {state} {block} threadId={thread.id} />
        {/each}
        {#if state.active?.anchor.blockId === block.id && !state.active.threadId}
          <ChatCommentArea {state} {block} />
        {/if}
      </section>
    {:else}<ChatTextBlock
        {block}
        {state}
        {reactionTarget}
        showHeader={showHeaders}
      />{/if}
  {/key}
{/each}

<style>
  .chat-custom {
    position: relative;
    min-width: 0;
    padding: 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }
  .chat-custom[data-kind="context"] {
    padding: 0;
    border: 0;
  }
  .chat-custom[data-highlighted="true"] {
    border-color: var(--primary);
    border-inline-start-width: 3px;
    background: color-mix(in srgb, var(--primary) 10%, transparent);
  }
</style>
