<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import StoreAgentSentContext from "./StoreAgentSentContext.svelte";
  import StoreAgentDiscussion from "./StoreAgentDiscussion.svelte";
  import StoreMessageActions from "../store-collaboration/StoreMessageActions.svelte";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  import { agentMessageTimestamp } from "../../collaboration/presentation.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let {
    manager,
    row,
    author,
    children,
    openReply = false,
    sessionId,
  }: {
    manager: AgentExecution;
    sessionId?: string;
    row: {
      id: string;
      text: string;
      kind: "user" | "assistant";
      observedAtMillis?: number;
      value?: Record<string, unknown>;
    };
    author: CommunityMemberModel;
    children?: Snippet;
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
</script>

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
  members={[author]}
  highlighted={row.kind === "user"}
  avatarShape={row.kind === "assistant" ? "rounded" : "circle"}
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
  {#snippet supplement()}<StoreMessageActions
      onReply={() => (expanded = !expanded)}
      reactionDisabled={disabled || !!unavailable}
      communityEmojis={threads.emojis}
      onReact={feedback && !disabled && !unavailable
        ? (emoji, custom) =>
            discussions.toggle(
              session.repository,
              feedback.target,
              emoji,
              custom,
            )
        : undefined}
      authorName={author.name}
      items={[{ label: "Copy message", value: row.text }]}
    />{/snippet}
  {#snippet metadataSupplement()}
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
</CommunityMessage>
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
