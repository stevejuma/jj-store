<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import StoreMessageActions from "../store-collaboration/StoreMessageActions.svelte";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  import { agentMessageTimestamp } from "../../collaboration/presentation.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let {
    manager,
    row,
    author,
    children,
  }: {
    manager: AgentExecution;
    row: {
      id: string;
      text: string;
      kind: "user" | "assistant";
      observedAtMillis?: number;
    };
    author: CommunityMemberModel;
    children?: Snippet;
  } = $props();
  const session = $derived(manager.session!);
  const discussions = $derived(manager.discussions);
  const feedback = $derived(discussions.message(session.id, row.id));
  const threads = $derived(discussions.reader(session.repository));
  const key = $derived(`${session.id}:${row.id}`);
  const unavailable = $derived(
    discussions.values[session.id]?.write_unavailable,
  );
  let expanded = $state(false);
  $effect(() => {
    discussions.drafts[key] ??= "";
  });
  const disabled = $derived(
    threads.busy || !!threads.pending || !!discussions.errors[session.id],
  );
  async function save(body: string) {
    await discussions.save(session.id, row.id, body);
  }
</script>

<CommunityMessage
  promptElements
  message={{
    id: row.id,
    authorId: author.id,
    content: row.text,
    ...agentMessageTimestamp(row.observedAtMillis),
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
  <section
    aria-label={`Replies to ${author.name}`}
    class="agent-message-discussion"
  >
    {#each feedback?.replies ?? [] as reply (reply.revision.id)}
      <CommunityMessage
        message={{
          id: reply.message.message_id,
          authorId: reply.message.actor,
          content: reply.message.body ?? "",
          ...agentMessageTimestamp(reply.message.accepted_at_millis),
        }}
        author={{
          id: reply.message.actor,
          name: discussions.authorName(session.repository, reply.message.actor),
          handle: "",
          initials: "YO",
        }}
      />
    {/each}
    {#key discussions.recovered[key]}<CommunityChatComposer
        bind:value={discussions.drafts[key]}
        context={{
          kind: "transcript-reply",
          target: { authorName: author.name, content: row.text },
        }}
        onCancelContext={() => (expanded = false)}
        ariaLabel="Agent message reply"
        placeholder={`Reply to ${author.name}`}
        submitLabel="Save reply"
        busy={threads.busy}
        {disabled}
        {...feedback && !unavailable
          ? { onSubmit: (submission) => save(submission.content) }
          : {
              composerStatus: {
                type: "warning" as const,
                message:
                  unavailable ??
                  "Waiting for this message to be published. Your reply draft is retained.",
              },
            }}
      />{/key}
    {#if discussions.errors[session.id]}<p role="alert">
        {discussions.errors[session.id]}
      </p>{/if}
    {#if threads.pending && discussions.awaiting(session.id, row.id)}<Button
        variant="secondary"
        size="sm"
        onclick={() => discussions.recover(session.id, row.id)}
        >Check reply status</Button
      >
      <Button
        variant="secondary"
        size="sm"
        onclick={() => discussions.recover(session.id, row.id, true)}
        >Retry same reply</Button
      >{/if}
  </section>
{/if}

<style>
  .agent-message-discussion {
    margin-inline-start: 2.5rem;
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
</style>
