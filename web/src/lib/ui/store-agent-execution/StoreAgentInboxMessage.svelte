<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import StoreAgentSentContext from "./StoreAgentSentContext.svelte";
  import StoreAgentDiscussion from "./StoreAgentDiscussion.svelte";
  import StoreMessageActions from "../store-collaboration/StoreMessageActions.svelte";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
  import { agentMessageTimestamp } from "../../collaboration/presentation.js";
  import { createMessageActionHover } from "../community-message-actions/message-action-hover.svelte.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let {
    manager,
    row,
    author,
    children,
    content,
    openReply = false,
    sessionId,
  }: {
    manager: AgentExecution;
    sessionId?: string;
    row: {
      id: string;
      text: string;
      kind: "user" | "assistant" | "tool";
      observedAtMillis?: number;
      value?: Record<string, unknown>;
    };
    author: CommunityMemberModel;
    children?: Snippet;
    content?: Snippet;
    openReply?: boolean;
  } = $props();
  const session = $derived(
    (sessionId
      ? manager.value?.sessions.find((s) => s.id === sessionId)
      : manager.session)!,
  );
  const discussions = $derived(manager.discussions);
  const feedback = $derived(discussions.message(session.id, row.id));
  const threads = $derived(discussions.reader(session.repository));
  const key = $derived(`${session.id}:${row.id}`);
  const unavailable = $derived(
    discussions.values[session.id]?.write_unavailable,
  );
  let expanded = $state(false);
  $effect(() => {
    if (openReply) expanded = true;
  });
  $effect(() => {
    discussions.drafts[key] ??= "";
  });
  const disabled = $derived(threads.busy || !!threads.pending);
  const timing = $derived(agentMessageTimestamp(row.observedAtMillis));
  const toolHover = createMessageActionHover();
</script>

{#snippet actions()}<StoreMessageActions
    onReply={() => (expanded = !expanded)}
    reactionDisabled={disabled || !!unavailable}
    communityEmojis={threads.emojis}
    onReact={feedback && !disabled && !unavailable
      ? (emoji, custom) =>
          discussions.toggle(session.repository, feedback.target, emoji, custom)
      : undefined}
    authorName={author.name}
    items={[{ label: "Copy message", value: row.text }]}
  />{/snippet}
{#snippet metadata()}
  {#if row.kind === "user" && row.value?.sentContext}<StoreAgentSentContext
      value={row.value.sentContext}
    />{/if}
  {@render children?.()}
  {#if feedback && !unavailable}<StoreMessageReactions
      {threads}
      target={feedback.target}
      showControls={false}
    />{/if}
  {#if feedback?.replies.length}<Button
      variant="ghost"
      size="sm"
      onclick={() => (expanded = !expanded)}
    >
      {feedback.replies.length}
      {feedback.replies.length === 1 ? "reply" : "replies"}
    </Button>{/if}
{/snippet}

{#if row.kind === "tool"}
  <article
    class="store-agent-tool-row"
    aria-label={`Tool call: ${row.text}`}
    data-ui-component="agent-tool-message"
    data-message-id={row.id}
    data-actions-visible={toolHover.visible || undefined}
    {...toolHover.handlers}
  >
    {@render content?.()}
    {@render actions()}
    {#if feedback && !unavailable && threads.reactionModels(feedback.target).length}
      <CommunityReactions
        mode="bar"
        label={`Reactions to ${row.text}`}
        reactions={threads.reactionModels(feedback.target)}
        onToggle={(emoji) =>
          discussions.toggle(
            session.repository,
            feedback.target,
            emoji,
            undefined,
            true,
          )}
      />
    {/if}
    {@render metadata()}
  </article>
{:else}
  <CommunityMessage
    promptElements
    message={{
      id: row.id,
      authorId: author.id,
      content: row.text,
      ...timing,
      timestamp: timing.createdAt ? timing.timestamp : "",
      reactions:
        feedback && !unavailable ? threads.reactionModels(feedback.target) : [],
    }}
    {author}
    {content}
    members={[author]}
    highlighted={row.kind === "user"}
    avatarShape={row.kind !== "user" ? "rounded" : "circle"}
    onToggleReaction={feedback && !disabled && !unavailable
      ? (_id, emoji, custom) =>
          discussions.toggle(
            session.repository,
            feedback.target,
            emoji,
            custom,
            true,
          )
      : undefined}
  >
    {#snippet supplement()}{@render actions()}{/snippet}
    {#snippet metadataSupplement()}{@render metadata()}{/snippet}
  </CommunityMessage>
{/if}
{#if expanded}
  <StoreAgentDiscussion
    {discussions}
    sessionId={session.id}
    repository={session.repository}
    {row}
    {author}
    onClose={() => (expanded = false)}
  />
{/if}

<style>
  .store-agent-tool-row {
    position: relative;
    min-width: 0;
  }
</style>
