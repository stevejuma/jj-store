<script lang="ts">
  import { tick, untrack } from "svelte";
  import "./chat-body.css";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import ChatDisclosureIcon from "./ChatDisclosureIcon.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import ChatCommentBatch from "./ChatCommentBatch.svelte";
  import ChatCollapsedComments from "./ChatCollapsedComments.svelte";
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
  import ChatNavigation from "./ChatNavigation.svelte";
  import ChatCommentArea from "./ChatCommentArea.svelte";
  import ChatContent from "./ChatContent.svelte";
  import ChatTools from "./ChatTools.svelte";
  import { ChatBodyComments } from "./comments.svelte.js";
  import type { ChatBodyProps } from "./model.js";
  import { rawToolBlock } from "./tool-blocks.js";
  import { validAnchor } from "./anchors.js";
  let {
    items,
    author,
    onSend,
    reactions = {},
    reactionOptions,
    onReact,
    composer,
    footer,
    label = "Agent chat body",
    customBlock,
    commentState,
    onInspectTool,
  }: ChatBodyProps = $props();
  const comments =
    untrack(() => commentState) ??
    new ChatBodyComments(() => ({
      items,
      author,
      onSend,
      onInspectTool,
      onReact,
      reactionOptions,
    }));
  let root = $state<HTMLElement | null>(null);
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
    comments.visibleThreads.filter(
      (thread) => !blocks.some((block) => validAnchor(block, thread.anchor)),
    ),
  );
  $effect(() => {
    comments.root = root;
    return () => {
      if (comments.root === root) comments.root = null;
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
  <div class="chat-body-scroll-region">
    <ScrollArea
      class="store-agent-chat-body"
      bind:viewportRef={root}
      role="region"
      aria-label={label}
      data-comment-target={comments.active?.anchor.blockId}
      data-composing={!!composer}
    >
      <ol class="chat-document">
        {#each items as item (item.id)}
          <li data-chat-item={item.id} data-kind={item.kind} tabindex="-1">
            {#if item.commentBatchId}<ChatCommentBatch
                state={comments}
                batchId={item.commentBatchId}
              />{/if}
            {#if item.replyContext}<div
                class="chat-document-inset chat-reply-context"
              >
                <ChatCommentArea
                  state={comments}
                  threadId={item.replyContext.threadId}
                  commentIds={item.replyContext.commentIds}
                  placement="reply"
                />
              </div>{/if}
            {#if item.kind === "message" && item.blocks.length}
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
                    reactionTarget={item.id}
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
            {:else if item.kind !== "message"}
              <div class="chat-document-inset">
                <Collapsible.Root
                  open={comments.disclosures[item.id] ??
                    item.defaultExpanded ??
                    false}
                  onOpenChange={(open) =>
                    (comments.disclosures[item.id] = open)}
                >
                  <Collapsible.Trigger class="chat-context-trigger"
                    ><span>{item.label}</span><ChatDisclosureIcon
                    /></Collapsible.Trigger
                  >
                  <Collapsible.Content
                    >{#if comments.disclosures[item.id] ?? item.defaultExpanded ?? false}<ChatContent
                        blocks={item.blocks}
                        state={comments}
                        reactionTarget={item.id}
                        {customBlock}
                      />{/if}</Collapsible.Content
                  >
                </Collapsible.Root>
                {#if !(comments.disclosures[item.id] ?? item.defaultExpanded ?? false)}<ChatCollapsedComments
                    state={comments}
                    blocks={item.blocks}
                  />{/if}
              </div>
            {/if}
            {#if reactions[item.id]?.length}<div class="chat-document-inset">
                <CommunityReactions
                  mode="bar"
                  reactions={[...reactions[item.id]]}
                  label={`Reactions to ${item.label}`}
                  onToggle={(emoji) => onReact?.(item.id, emoji)}
                />
              </div>{/if}
          </li>
        {/each}
      </ol>
      {#if !items.length}<p class="chat-empty">No captured messages.</p>{/if}
      {#if stale.length}<aside
          class="chat-stale"
          aria-label="Comments on unavailable content"
        >
          {#each stale as thread (thread.id)}<ChatCommentArea
              state={comments}
              threadId={thread.id}
              placement="history"
            />{/each}
        </aside>{/if}
      {#if composer}<div class="chat-document-composer">
          <ChatCommentBatch state={comments} />
          {@render composer()}
        </div>{/if}
    </ScrollArea>
    <ChatNavigation {items} root={root ?? undefined} {comments} />
  </div>
  {#if footer}<div class="chat-body-footer">{@render footer()}</div>{/if}
</div>
