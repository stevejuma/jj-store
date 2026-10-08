<script lang="ts">
  import { tick, untrack } from "svelte";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import ChatNavigation from "./ChatNavigation.svelte";
  import ChatCommentArea from "./ChatCommentArea.svelte";
  import ChatContent from "./ChatContent.svelte";
  import ChatTools from "./ChatTools.svelte";
  import { ChatBodyComments } from "./comments.svelte.js";
  import type { ChatBodyProps, ChatBodyBlock } from "./model.js";
  import { rawToolBlock } from "./tool-blocks.js";
  import { validAnchor } from "./anchors.js";
  let {
    items,
    threads = [],
    onAddComment,
    onReply,
    label = "Agent chat body",
    customBlock,
    commentState,
    onInspectTool,
  }: ChatBodyProps = $props();
  const comments =
    untrack(() => commentState) ??
    new ChatBodyComments(() => ({
      items,
      threads,
      onAddComment,
      onReply,
      label,
    }));
  let root = $state<HTMLElement>();
  let previousLast = "";
  const blocks = $derived(
    items.flatMap((item) => [
      ...item.blocks,
      ...(item.tools ?? []).flatMap((tool) => [
        ...tool.blocks,
        rawToolBlock(tool),
      ]),
    ]),
  );
  const stale = $derived(
    threads.filter(
      (thread) => !blocks.some((block) => validAnchor(block, thread.anchor)),
    ),
  );
  const unavailableDraft = $derived.by((): ChatBodyBlock | undefined => {
    const active = comments.active;
    if (!active || blocks.some((block) => validAnchor(block, active.anchor)))
      return;
    return {
      id: active.anchor.blockId,
      revision: active.anchor.revision,
      label: active.label,
      kind: "custom",
      text: active.anchor.quote,
    };
  });
  $effect.pre(() => {
    const last = items.at(-1)?.id ?? "";
    const scrollport = root;
    if (previousLast && previousLast !== last && scrollport) {
      const top = scrollport.scrollTop;
      const bottom =
        scrollport.scrollHeight - scrollport.clientHeight - top < 8;
      void tick().then(() => {
        scrollport.scrollTop =
          bottom && !untrack(() => comments.active)
            ? scrollport.scrollHeight
            : top;
      });
    }
    previousLast = last;
  });
</script>

<div class="chat-body-reading-surface">
  <section
    class="store-agent-chat-body"
    bind:this={root}
    aria-label={label}
    data-ui-component="store-agent-chat-body"
    data-comment-target={comments.active?.anchor.blockId}
  >
    <ol class="chat-document">
      {#each items as item (item.id)}
        <li data-chat-item={item.id} data-kind={item.kind} tabindex="-1">
          {#if item.kind === "message"}
            <article class="chat-message" aria-label={item.label}>
              <div class="chat-message-avatar">
                {#if item.author}<CommunityMemberAvatar
                    member={item.author}
                    size="xsmall"
                    decorative
                  />{/if}
              </div>
              <div class="chat-message-content">
                <span class="sr-only">{item.author?.name ?? item.role}</span
                ><ChatContent
                  blocks={item.blocks}
                  state={comments}
                  {customBlock}
                />
              </div>
            </article>
          {:else if item.kind === "tools"}<div class="chat-document-inset">
              <ChatTools
                {item}
                state={comments}
                {customBlock}
                {onInspectTool}
              />
            </div>
          {:else}
            <div class="chat-document-inset">
              <Collapsible.Root
                open={comments.disclosures[item.id] ??
                  item.defaultExpanded ??
                  false}
                onOpenChange={(open) => (comments.disclosures[item.id] = open)}
              >
                <Collapsible.Trigger class="chat-context-trigger"
                  >{item.label}<ChevronRightIcon
                    size={14}
                  /></Collapsible.Trigger
                >
                <Collapsible.Content
                  ><ChatContent
                    blocks={item.blocks}
                    state={comments}
                    {customBlock}
                  /></Collapsible.Content
                >
              </Collapsible.Root>
            </div>
          {/if}
        </li>
      {/each}
    </ol>
    {#if !items.length}<p class="chat-empty">No captured messages.</p>{/if}
    {#if stale.length || unavailableDraft}<aside
        class="chat-stale"
        aria-label="Comments on unavailable content"
      >
        <p>
          The original content for these comments is unavailable or has changed.
        </p>
        {#if unavailableDraft}<ChatCommentArea
            state={comments}
            block={unavailableDraft}
          />{/if}
        {#each stale as thread (thread.id)}<blockquote>
            {thread.anchor.quote}
          </blockquote>
          {#each thread.comments as comment (comment.id)}<CommunityMarkdownContent
              content={comment.text}
            />{/each}
        {/each}
      </aside>{/if}
  </section>
  <ChatNavigation {items} {root} {comments} />
</div>

<style>
  .chat-body-reading-surface {
    height: 100%;
    min-height: 0;
    min-width: 0;
    position: relative;
  }
  .store-agent-chat-body {
    height: 100%;
    min-height: 0;
    overflow: auto;
    overscroll-behavior: contain;
    color: var(--foreground);
    background: var(--background);
    font-family: var(--font-sans, system-ui);
    font-size: 1rem;
  }
  .chat-document {
    list-style: none;
    margin: 0 auto;
    padding: 1.25rem 2.75rem 4rem 1rem;
    max-width: 62rem;
  }
  .chat-document > li {
    margin-block-end: 1.1rem;
  }
  .chat-message {
    display: grid;
    grid-template-columns: 1.5rem minmax(0, 1fr);
    gap: 0.5rem;
  }
  .chat-message-avatar {
    padding-block-start: 0.25rem;
  }
  .chat-message-content {
    min-width: 0;
  }
  .chat-document-inset {
    margin-inline-start: 2rem;
    min-width: 0;
  }
  :global(.chat-context-trigger) {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    border: 0;
    border-radius: var(--radius);
    background: transparent;
    color: var(--muted-foreground);
    cursor: pointer;
    padding: 0.4rem;
    font: inherit;
    text-align: start;
  }
  :global(.chat-context-trigger:hover) {
    background: var(--muted);
  }
  :global(.chat-context-trigger:focus-visible) {
    outline: 2px solid var(--ring);
  }
  :global(.chat-context-trigger[data-state="open"] svg) {
    rotate: 90deg;
  }
  .chat-empty,
  .chat-stale {
    padding: 1.5rem;
  }
  .chat-stale {
    border: 1px solid var(--border);
    margin: 1rem;
    border-radius: var(--radius);
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
  }
  @media (max-width: 600px) {
    .chat-document {
      padding: 1rem 2.25rem 3rem 0.75rem;
    }
    .chat-message {
      grid-template-columns: 1.5rem minmax(0, 1fr);
      gap: 0.5rem;
    }
    .chat-document-inset {
      margin-inline-start: 2rem;
    }
  }
</style>
