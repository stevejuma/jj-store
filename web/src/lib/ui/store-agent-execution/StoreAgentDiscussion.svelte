<script lang="ts">
  import { useAgentExecution } from "./context.js";
  import { agentMember } from "../community-agents/model.js";
  import { mentionedAgentIds } from "../../community/agent-mentions.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import CommunityThreadTreeRow from "../community-thread-panel/CommunityThreadTreeRow.svelte";
  import { projectThreadTree } from "../community-thread-panel/thread-tree-layout.js";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  import { agentMessageTimestamp } from "../../collaboration/presentation.js";
  import type { AgentDiscussions } from "../../controller/agent-discussions.svelte.js";
  import type {
    CommunityMemberModel,
    CommunityMessageModel,
  } from "../community-workspace/model.js";
  import type { ThreadPanelProps } from "../community-thread-panel/props.js";
  import type { ReactionTarget } from "../../controller/thread-types.js";
  let {
    discussions,
    sessionId,
    repository,
    row,
    author,
    onClose,
  }: {
    discussions: AgentDiscussions;
    sessionId: string;
    repository: string;
    row: { id: string; text: string };
    author: CommunityMemberModel;
    onClose(): void;
  } = $props();
  const manager = useAgentExecution();
  const key = $derived(`${sessionId}:${row.id}`);
  const feedback = $derived(discussions.message(sessionId, row.id));
  const threads = $derived(discussions.reader(repository));
  const unavailable = $derived(
    discussions.values[sessionId]?.write_unavailable,
  );
  const blocked = $derived(threads.busy || !!threads.pending);
  const context = $derived(discussions.contexts[key]);
  const selected = $derived(
    feedback?.replies.find(
      (r) => r.message.message_id === (context?.edit ?? context?.reply),
    ),
  );
  const targets = $derived(
    Object.fromEntries(
      (feedback?.replies ?? []).map((r) => [
        r.message.message_id,
        {
          type: "human_message",
          thread_id: r.thread_id,
          message_id: r.message.message_id,
          revision: r.revision,
        } satisfies ReactionTarget,
      ]),
    ),
  );
  const members = $derived(
    (feedback?.replies ?? []).map((r) => ({
      id: r.message.actor,
      name: discussions.authorName(repository, r.message.actor),
      handle: "",
      initials: "YO",
    })),
  );
  const tree = $derived.by(() => {
    const replies = feedback?.replies ?? [];
    const parents = new Map(
      replies.flatMap(
        (r) =>
          [
            [r.revision.id, r.message.message_id],
            ...(r.message.previous
              ? [[r.message.previous.id, r.message.message_id]]
              : []),
          ] as [string, string][],
      ),
    );
    return projectThreadTree(
      replies.map((r) => ({
        id: r.message.message_id,
        authorId: r.message.actor,
        content: r.message.body ?? "",
        ...agentMessageTimestamp(r.message.accepted_at_millis),
        edited: !!r.message.previous,
        parentMessageId: r.message.reply_to
          ? parents.get(r.message.reply_to.id)
          : undefined,
        reactions: threads.reactionModels(targets[r.message.message_id]),
      })),
      key,
    );
  });
  function choose(id: string, edit = false) {
    const reply = feedback?.replies.find((r) => r.message.message_id === id);
    if (!reply || blocked || unavailable) return;
    discussions.contexts[key] = {
      [edit ? "edit" : "reply"]: id,
      revision: reply.revision.id,
      thread: reply.thread_id,
    };
    if (edit) discussions.drafts[key] = reply.message.body ?? "";
  }
  const panel = $derived<ThreadPanelProps>({
    thread: { rootMessageId: key, messages: [...tree.messages] },
    members,
    currentMemberId: discussions.actor(repository),
    communityEmojis: threads.emojis,
    onReply: blocked || unavailable ? undefined : (id) => choose(id),
    onEditMessage:
      blocked || unavailable ? undefined : (id) => choose(id, true),
    onToggleReaction:
      blocked || unavailable
        ? undefined
        : (id, emoji, custom) =>
            discussions.toggle(repository, targets[id], emoji, custom, true),
    onAddReaction:
      blocked || unavailable
        ? undefined
        : (id, emoji, custom) =>
            discussions.toggle(repository, targets[id], emoji, custom),
  });
  const status = $derived(
    unavailable ||
      (!feedback
        ? "Waiting for this message to be published. Your reply draft is retained."
        : threads.pending
          ? "Check the original reply status before saving another reply."
          : discussions.errors[sessionId]),
  );
</script>

<section
  aria-label={`Replies to ${author.name}`}
  class="agent-message-discussion"
>
  {#each tree.messages as message (message.id)}
    <CommunityThreadTreeRow
      panel={{ ...panel, messageMetadata }}
      {message}
      {members}
      isContinuation={false}
      treeRow={tree.rowsById.get(message.id)}
      transcriptComment
      startEditing={(id) => choose(id, true)}
    />
  {/each}
  {#snippet messageMetadata(message: CommunityMessageModel)}
    <StoreMessageReactions
      {threads}
      target={targets[message.id]}
      disabled={blocked || !!unavailable}
      showControls={false}
    />
  {/snippet}
  {#key discussions.recovered[key]}<CommunityChatComposer
      bind:value={discussions.drafts[key]}
      members={[...members, ...(manager?.value?.setups ?? []).map(agentMember)]}
      audienceKey={context?.edit
        ? undefined
        : JSON.stringify([
            discussions.actor(repository),
            repository,
            sessionId,
            row.id,
          ])}
      initialAgentIds={author.acpAgentId ? [author.acpAgentId] : []}
      context={context?.edit
        ? { kind: "edit", id: context.edit, label: "Editing reply" }
        : {
            kind: "transcript-reply",
            target: {
              authorName: selected
                ? discussions.authorName(repository, selected.message.actor)
                : author.name,
              content: selected?.message.body ?? row.text,
            },
          }}
      onCancelContext={() => {
        if (context) delete discussions.contexts[key];
        else onClose();
      }}
      ariaLabel="Agent message reply"
      placeholder={`Reply to ${author.name}`}
      submitLabel={context?.edit ? "Save changes" : "Save reply"}
      busy={threads.busy}
      composerStatus={status ? { type: "warning", message: status } : undefined}
      onRetryComposerStatus={discussions.errors[sessionId]
        ? () => discussions.load(sessionId)
        : undefined}
      onSubmit={feedback && !unavailable && !blocked
        ? async (submission) => {
            const ids = mentionedAgentIds(submission.content);
            if (!ids.length || context?.edit)
              return discussions.save(sessionId, row.id, submission.content);
            if (!manager?.available())
              throw new Error("Agent execution is unavailable.");
            manager.chat.show(
              {
                scope: `agent-reply:${repository}:${sessionId}:${row.id}`,
                home: repository,
                thread: feedback?.threads[0] ?? null,
                agent_target: feedback.target,
                context: JSON.stringify({
                  session: sessionId,
                  message: row.id,
                  content: row.text,
                }),
              },
              [],
              undefined,
              false,
            );
            await manager.chat.send(submission.content, ids);
          }
        : undefined}
    />{/key}
  {#if threads.pending && discussions.awaiting(sessionId, row.id)}
    <Button
      variant="secondary"
      size="sm"
      onclick={() => discussions.recover(sessionId, row.id)}
      >Check reply status</Button
    >
    <Button
      variant="secondary"
      size="sm"
      onclick={() => discussions.recover(sessionId, row.id, true)}
      >Retry same reply</Button
    >
  {/if}
</section>

<style>
  .agent-message-discussion {
    margin-inline-start: 2.5rem;
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
</style>
