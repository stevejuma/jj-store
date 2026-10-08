<script lang="ts">
  import { tick, untrack } from "svelte";
  import "./chat-body.css";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
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
      threads,
      onAddComment,
      onReply,
      reactions,
      reactionOptions,
      onReact,
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
  <div class="chat-body-scroll-region">
    <section
      class="store-agent-chat-body"
      bind:this={root}
      aria-label={label}
      data-ui-component="store-agent-chat-body"
      data-comment-target={comments.active?.anchor.blockId}
      data-composing={!!composer}
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
            {:else}
              <div class="chat-document-inset">
                <Collapsible.Root
                  open={comments.disclosures[item.id] ??
                    item.defaultExpanded ??
                    false}
                  onOpenChange={(open) =>
                    (comments.disclosures[item.id] = open)}
                >
                  <Collapsible.Trigger class="chat-context-trigger"
                    ><span>{item.label}</span><ChevronRightIcon
                      size={14}
                    /></Collapsible.Trigger
                  >
                  <Collapsible.Content
                    ><ChatContent
                      blocks={item.blocks}
                      state={comments}
                      reactionTarget={item.id}
                      {customBlock}
                    /></Collapsible.Content
                  >
                </Collapsible.Root>
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
      {#if stale.length || unavailableDraft}<aside
          class="chat-stale"
          aria-label="Comments on unavailable content"
        >
          <p>
            The original content for these comments is unavailable or has
            changed.
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
      {#if composer}<div class="chat-document-composer">
          {@render composer()}
        </div>{/if}
    </section>
    <ChatNavigation {items} {root} {comments} />
  </div>
  {#if footer}<div class="chat-body-footer">{@render footer()}</div>{/if}
</div>
