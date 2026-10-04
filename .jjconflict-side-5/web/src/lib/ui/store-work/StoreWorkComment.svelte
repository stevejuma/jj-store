<script lang="ts">
  // @community-copy CommunityProjects task Activity (Apache-2.0), native comment adapter.
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import StoreMessageActions from "../store-collaboration/StoreMessageActions.svelte";
  import StoreMessageReactions from "../store-threads/StoreMessageReactions.svelte";
  import { workMember, workMessage } from "./work-presentation.js";
  import type { WorkController } from "../../controller/work-types.js";
  import type { Model } from "../../api/client.js";
  import type { CommunityMemberModel } from "../../community/workspace-model.js";
  let {
    work,
    comment,
    members,
    writable,
    disabled,
    onReply,
    onEdit,
    onRemove,
  }: {
    work: WorkController;
    comment: Model<"WorkCommentV1">;
    members: CommunityMemberModel[];
    writable: boolean;
    disabled: boolean;
    onReply?: (() => void) | undefined;
    onEdit?: (() => void) | undefined;
    onRemove?: (() => void) | undefined;
  } = $props();
  const reactions = $derived(work.reactions);
  const target = $derived({
    type: "work_comment" as const,
    task_id: comment.task,
    comment_id: comment.id,
    version: comment.version,
  });
  const canReact = $derived(
    writable && comment.body !== null && reactions !== undefined,
  );
  const pendingTarget = $derived.by(() => {
    const pending = reactions?.pending;
    if (pending?.kind !== "reactions") return undefined;
    if (
      pending.request.command.type !== "set" &&
      pending.request.command.type !== "clear"
    )
      return undefined;
    const original = pending.request.command.target;
    return original.type === "work_comment" &&
      original.task_id === comment.task &&
      original.comment_id === comment.id
      ? original
      : undefined;
  });
  const reactionDisabled = $derived(
    !writable || !!reactions?.busy || !!reactions?.pending,
  );
</script>

<CommunityMessage
  message={{
    ...workMessage(comment),
    reactions: comment.body !== null ? reactions?.reactionModels(target) : [],
  }}
  onToggleReaction={canReact
    ? (_id, emoji, custom) => {
        if (!reactionDisabled)
          void reactions!.toggle(target, emoji, custom, true).catch(() => {});
      }
    : undefined}
  author={workMember(comment.actor.created_by)}
  {members}
>
  {#snippet metadataSupplement()}
    {#if reactions && (comment.body !== null || pendingTarget)}<StoreMessageReactions
        threads={reactions}
        target={pendingTarget ?? target}
        disabled={reactionDisabled || !writable}
        showControls={false}
      />{/if}
  {/snippet}
  {#snippet supplement()}
    {#if comment.reply_to && !work.state.comments.some((c) => c.id === comment.reply_to)}<p
        class="store-work__meta"
        title={comment.reply_to}
      >
        Reply to {comment.reply_to.slice(0, 8)} (earlier page)
      </p>{/if}
    {#if comment.version > 1}<span
        class="sr-only"
        title={comment.actor.updated_by}
        >Last edited by {comment.actor.updated_by}</span
      >{/if}
    <StoreMessageActions
      {disabled}
      {reactionDisabled}
      communityEmojis={reactions?.emojis ?? []}
      onReact={canReact
        ? (emoji, custom) => {
            if (!reactionDisabled)
              void reactions!.toggle(target, emoji, custom).catch(() => {});
          }
        : undefined}
      authorName={workMember(comment.actor.created_by).name}
      items={[
        { label: "Copy message", value: comment.body },
        { label: "Copy comment ID", value: comment.id },
        {
          label: "Copy author ID",
          value: comment.actor.created_by,
        },
        { label: "Copy task ID", value: comment.task },
        {
          label: "Copy parent comment ID",
          value: comment.reply_to,
        },
      ]}
      {onReply}
      {onEdit}
      {onRemove}
    />
  {/snippet}
</CommunityMessage>
