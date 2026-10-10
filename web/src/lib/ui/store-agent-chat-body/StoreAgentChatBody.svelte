<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  import { SystemMessage } from "@lapismd/design-core/ai/chat";
  import CommunityMessageMeta from "../community-message/CommunityMessageMeta.svelte";
  import { tick, untrack } from "svelte";
  import "./chat-body.css";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import ChatDisclosureIcon from "./ChatDisclosureIcon.svelte";
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import ChatCommentBatch from "./ChatCommentBatch.svelte";
  import ChatCollapsedComments from "./ChatCollapsedComments.svelte";
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
  import ChatNavigation from "./ChatNavigation.svelte";
  import ChatCommentArea from "./ChatCommentArea.svelte";
  import ChatReplyContext from "./ChatReplyContext.svelte";
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
    view = "document",
    loading = false,
    emptyState,
    beforeItems,
    afterItems,
    supplement: itemSupplement,
    metadataSupplement: itemMetadata,
    onOpenThread,
    messageActions,
    logLabel = "Conversation messages",
    ...hostOptions
  }: ChatBodyProps = $props();
  const ownedComments = new ChatBodyComments(() => ({
    ...hostOptions,
    items,
    author,
    onSend,
    onReact,
    reactionOptions,
  }));
  const comments = $derived(commentState ?? ownedComments);
  let root = $state<HTMLElement | null>(null);
  let previousLast = "";
  let composing = $state(false);
  let renderedView = $state<"document" | "classic">("document");
  $effect(() => {
    if (!composing) renderedView = view;
  });
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

{#snippet reactionBar(item: import("./model.js").ChatBodyItem)}
  {#if reactions[item.id]?.length}<CommunityReactions
      mode="bar"
      reactions={[...reactions[item.id]]}
      label={`Reactions to ${item.label}`}
      onToggle={onReact ? (emoji) => onReact?.(item.id, emoji) : undefined}
    />{/if}
{/snippet}
<div
  class="chat-body-reading-surface"
  data-conversation-view={renderedView}
  oncompositionstartcapture={() => (composing = true)}
  oncompositionendcapture={() => (composing = false)}
>
  <div class="chat-body-scroll-region">
    <ScrollArea
      class="store-agent-chat-body"
      bind:viewportRef={root}
      role="region"
      aria-label={label}
      data-comment-target={comments.active?.anchor.blockId}
      data-composing={!!composer}
    >
      {#if loading}<div
          class="chat-history-loading"
          role="status"
          aria-label="Loading previous messages"
        >
          <Button variant="ghost" size="xs" disabled>
            <Spinner
              data-icon="inline-start"
              role="presentation"
              aria-hidden="true"
            />
            Loading previous messages
          </Button>
        </div>{/if}
      {@render beforeItems?.()}
      <div
        role="log"
        aria-label={logLabel}
        aria-live="polite"
        aria-busy={loading}
      >
        <ol class="chat-document">
          {#each items as item (item.id)}
            <li data-chat-item={item.id} data-kind={item.kind} tabindex="-1">
              {#if item.dayLabel}<SystemMessage variant="divider"
                  >{item.dayLabel}</SystemMessage
                >{/if}
              {#if item.commentBatchId}<ChatCommentBatch
                  state={comments}
                  batchId={item.commentBatchId}
                />{/if}
              {#if item.replyContext}<div
                  class="chat-document-inset chat-reply-context"
                >
                  <ChatReplyContext state={comments} {item} />
                </div>{/if}
              {#if item.external}
                {@render itemSupplement?.(item)}
              {:else if renderedView === "classic" && item.kind === "message" && item.author}
                <CommunityMessage
                  author={item.author}
                  message={item.message ?? {
                    timestamp: "",
                    ...item.timestamp,
                    id: item.id,
                    authorId: item.author.id,
                    content: item.blocks.map((b) => b.text).join("\n"),
                  }}
                  {onOpenThread}
                  onReply={() =>
                    item.blocks[0] &&
                    comments.open(item.blocks[0], { from: 0, to: 0 })}
                >
                  {#snippet content()}<ChatContent
                      blocks={item.blocks}
                      state={comments}
                      reactionTarget={item.id}
                      {messageActions}
                      reply={!!item.replyContext}
                      {customBlock}
                    />{/snippet}
                  {#snippet supplement()}{@render itemSupplement?.(
                      item,
                    )}{@render reactionBar(item)}{/snippet}
                  {#snippet metadataSupplement()}{@render itemMetadata?.(
                      item,
                    )}{/snippet}
                </CommunityMessage>
              {:else if item.kind === "message" && item.blocks.length}
                <article
                  class="chat-message"
                  data-message-id={item.id}
                  aria-label={item.author
                    ? `Message from ${item.author.name}`
                    : item.label}
                >
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
                      {messageActions}
                      reply={item.role === "assistant" && !!item.replyContext}
                      {customBlock}
                    />
                    {@render itemSupplement?.(item)}
                    {#if item.message || item.timestamp || itemMetadata}<CommunityMessageMeta
                        message={item.message ?? {
                          id: item.id,
                          authorId: item.author?.id ?? "",
                          content: "",
                          timestamp: "",
                          ...item.timestamp,
                        }}
                        {onOpenThread}
                      >
                        {#snippet supplement()}{@render itemMetadata?.(
                            item,
                          )}{/snippet}
                      </CommunityMessageMeta>{/if}
                    {@render reactionBar(item)}
                  </div>
                </article>
              {:else if item.kind === "tools"}<div class="chat-document-inset">
                  <ChatTools
                    {item}
                    state={comments}
                    {customBlock}
                    onInspectTool={hostOptions.onInspectTool}
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
                          {messageActions}
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
              {#if !item.external && item.kind !== "message"}{@render itemSupplement?.(
                  item,
                )}{@render itemMetadata?.(item)}{@render reactionBar(item)}{/if}
            </li>
          {/each}
        </ol>
        {@render afterItems?.()}
      </div>
      {#if !items.length && !loading}
        {#if emptyState}{@render emptyState()}{:else}
          <Empty.Root>
            <Empty.Header>
              <Empty.Media variant="icon"
                ><MessageSquareIcon aria-hidden="true" /></Empty.Media
              >
              <Empty.Title>No captured messages.</Empty.Title>
            </Empty.Header>
          </Empty.Root>
        {/if}
      {/if}
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
          {@render composer(renderedView)}
        </div>{/if}
    </ScrollArea>
    <ChatNavigation {items} root={root ?? undefined} {comments} />
  </div>
  {#if footer}<div class="chat-body-footer">{@render footer()}</div>{/if}
</div>
