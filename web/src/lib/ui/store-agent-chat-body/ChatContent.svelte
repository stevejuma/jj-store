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
    reply = false,
  }: {
    blocks: readonly ChatBodyBlock[];
    state: ChatBodyComments;
    customBlock?: ChatBodyProps["customBlock"];
    showHeaders?: boolean;
    reactionTarget?: string;
    reply?: boolean;
  } = $props();
</script>

<div class="chat-content-flow">
  {#each blocks as block, index (block.id)}
    {#if block.kind === "custom" || block.kind === "context"}
      <section
        class="chat-custom"
        data-kind={block.kind}
        role="group"
        aria-label={block.label}
        use:customInteraction={{ block, state }}
        data-highlighted={state.forBlocks([block]).length > 0}
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
        {#each state.visibleThreads.filter( (t) => validAnchor(block, t.anchor) ) as thread (thread.id)}
          <ChatCommentArea {state} threadId={thread.id} />
        {/each}
      </section>
    {:else}<ChatTextBlock
        {block}
        {state}
        {reactionTarget}
        reply={reply && index === 0}
        showHeader={showHeaders}
      />{/if}
  {/each}
</div>

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
  .chat-content-flow {
    display: grid;
    gap: 0.5rem;
    min-width: 0;
  }
</style>
