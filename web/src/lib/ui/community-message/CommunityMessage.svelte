<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { useAppearance } from "../../appearance/store.svelte.js";
  const appearance = useAppearance();
  import { Message, MessageBubble } from "@lapismd/design-core/ai/chat";
  import CommunityReactions from "../community-reactions/CommunityReactions.svelte";
  import { ConfirmDialog } from "@lapismd/design-core/forms/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityMessageActions from "../community-message-actions/CommunityMessageActions.svelte";
  import { createMessageActionHover } from "../community-message-actions/message-action-hover.svelte.js";

  import type {
    CommunityCustomEmoji,
    CommunityMessageReport,
  } from "../../community/contracts.js";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import { communityMemberAvatarShape } from "../community-member-avatar/model.js";
  import CommunityMemberHoverCard from "../community-member-hover-card/CommunityMemberHoverCard.svelte";
  import CommunityMessageMeta from "./CommunityMessageMeta.svelte";
  import CommunityReportDialog from "../community-report-dialog/CommunityReportDialog.svelte";
  import type {
    CommunityEmojiReference,
    CommunityMemberModel,
    CommunityMessageModel,
    CommunityRoomModel,
  } from "../community-workspace/model.js";

  import { memberById } from "../community-workspace/model.js";
  import { messageActivityContent } from "./message-activity.js";

  import type { CommunityMessageProps } from "./props.js";
  import "./community-message.css";
  import "./message-local.css";

  let {
    message,
    author,
    members = [],
    rooms = [],
    isContinuation = false,
    highlighted = false,
    replyParticipants = [],
    communityEmojis = [],
    reactionOptions,
    density = appearance?.density ?? "compact",
    onReply,
    onOpenThread,
    onToggleReaction,
    onAddReaction,
    onCopyLink,
    onCopyMessage,
    onCopyEvent,
    onEdit,
    onDelete,
    onReport,
    onOpenProfile,
    onOpenRoom,
    onMessageMember,
    onWaveMember,
    onViewMemberActivity,
    currentMemberId,
    replyActionLabel = "Reply in thread",
    supplement,
    metadataSupplement,
    avatarShape,
  }: CommunityMessageProps = $props();
  const resolvedAvatarShape = $derived(
    avatarShape ?? communityMemberAvatarShape(author),
  );

  const hover = createMessageActionHover();
  let deleteOpen = $state(false);
  let deletePending = $state(false);
  let deleteError = $state("");
  let reportOpen = $state(false);

  const isSelected = $derived(
    message.reactions?.some((reaction) => reaction.selected) ?? false,
  );
  const authorName = $derived(
    author.id === currentMemberId ? "You" : author.name,
  );
  const activityMembers = $derived(
    currentMemberId === undefined
      ? members
      : members.map((member) =>
          member.id === currentMemberId ? { ...member, name: "You" } : member,
        ),
  );
  const activityContent = $derived(
    messageActivityContent(message.activity, activityMembers),
  );

  const replyAction = $derived(
    message.activity === undefined ? (onReply ?? onOpenThread) : undefined,
  );
  const canCopyEvent = $derived(
    message.activity === undefined && onCopyEvent !== undefined,
  );

  async function confirmDelete(): Promise<void> {
    if (onDelete === undefined || deletePending) return;
    deletePending = true;
    deleteError = "";
    try {
      await onDelete(message.id);
      deleteOpen = false;
    } catch (cause) {
      deleteError =
        cause instanceof Error
          ? cause.message
          : "The message could not be deleted";
    } finally {
      deletePending = false;
    }
  }
</script>

<Message
  sender="assistant"
  {density}
  class="community-message"
  data-selected={isSelected ? "" : undefined}
  data-highlighted={highlighted ? "" : undefined}
  data-message-id={message.id}
  data-continuation={isContinuation ? "" : undefined}
  data-actions-visible={hover.visible ? "" : undefined}
  aria-label={`Message from ${authorName}`}
  {...hover.handlers}
>
  {#snippet avatar()}
    {#if isContinuation}
      <span
        class="community-message__continuation-time"
        data-ui-part="continuation-time"
        aria-hidden="true"
      >
        {message.timestamp}
      </span>
    {:else}
      <CommunityMemberHoverCard
        member={author}
        avatarShape={resolvedAvatarShape}
        {...onOpenProfile === undefined ? {} : { onOpenProfile }}
        {...onMessageMember === undefined ? {} : { onMessage: onMessageMember }}
        {...onWaveMember === undefined ? {} : { onWave: onWaveMember }}
        {...onViewMemberActivity === undefined
          ? {}
          : { onViewActivity: onViewMemberActivity }}
      />
    {/if}
  {/snippet}
  {#snippet name()}
    {#if !isContinuation}
      <span class="community-message__author">{authorName}</span>
      {#if author.role}
        <span class="community-message__role">{author.role}</span>
      {/if}
    {/if}
  {/snippet}

  <MessageBubble variant="ghost" {density}>
    {#if message.activity}
      <CommunityMarkdownContent
        class="community-message__content community-message__activity"
        content={activityContent}
        mentionPubkeys={message.activity.targetMemberId === undefined
          ? []
          : [message.activity.targetMemberId]}
        members={activityMembers}
        {rooms}
        {communityEmojis}
        {...onOpenProfile === undefined
          ? {}
          : {
              onOpenMember: (member: CommunityMemberModel) =>
                onOpenProfile(memberById(members, member.id)),
            }}
        {...onOpenRoom === undefined ? {} : { onOpenRoom }}
        {...onMessageMember === undefined
          ? {}
          : {
              onMessageMember: (member: CommunityMemberModel) =>
                onMessageMember(memberById(members, member.id)),
            }}
        {...onWaveMember === undefined
          ? {}
          : {
              onWaveMember: (member: CommunityMemberModel) =>
                onWaveMember(memberById(members, member.id)),
            }}
      />
    {:else}
      <CommunityMarkdownContent
        class="community-message__content"
        content={message.content}
        mentionPubkeys={message.mentionPubkeys ?? []}
        {members}
        {rooms}
        {communityEmojis}
        {...onOpenProfile === undefined ? {} : { onOpenMember: onOpenProfile }}
        {...onMessageMember === undefined ? {} : { onMessageMember }}
        {...onWaveMember === undefined ? {} : { onWaveMember }}
        {...onOpenRoom === undefined ? {} : { onOpenRoom }}
      />
    {/if}
  </MessageBubble>

  {#if message.activity === undefined}
    <CommunityMessageActions
      id={message.id}
      {authorName}
      visible={hover.visible}
      {communityEmojis}
      {reactionOptions}
      {onAddReaction}
      onReply={replyAction}
      {replyActionLabel}
      {onCopyLink}
      {onCopyMessage}
      onCopyEvent={canCopyEvent ? onCopyEvent : undefined}
      {onEdit}
      onDelete={onDelete
        ? () => {
            deleteOpen = true;
          }
        : undefined}
      onReport={onReport
        ? () => {
            reportOpen = true;
          }
        : undefined}
    />
  {/if}

  {#if message.reactions?.length}
    <CommunityReactions
      mode="bar"
      reactions={message.reactions}
      label={`Reactions to ${authorName}'s message`}
      onToggle={(emoji) => {
        const reaction = message.reactions?.find(
          (candidate) => candidate.emoji === emoji,
        );
        if (reaction?.customEmojiUrl)
          onToggleReaction?.(message.id, emoji, {
            shortcode: emoji,
            url: reaction.customEmojiUrl,
          });
        else onToggleReaction?.(message.id, emoji);
      }}
    />
  {/if}

  {#snippet metadata()}
    {#if supplement}
      <div
        class="community-message__supplement"
        data-ui-part="message-supplement"
      >
        {@render supplement()}
      </div>
    {/if}
    <CommunityMessageMeta
      {message}
      {members}
      {isContinuation}
      {replyParticipants}
      {onOpenThread}
      {onOpenProfile}
      {onMessageMember}
      {onWaveMember}
      {onViewMemberActivity}
      supplement={metadataSupplement}
    />
  {/snippet}
</Message>

{#if onReport}
  <CommunityReportDialog
    bind:open={reportOpen}
    onSubmit={(report) => onReport(message.id, report)}
  />
{/if}

{#if onDelete}
  <ConfirmDialog
    bind:open={deleteOpen}
    pending={deletePending}
    destructive
    onConfirm={async () => {
      await confirmDelete();
      return !deleteError;
    }}
  >
    {#snippet title()}Delete message?{/snippet}
    {#snippet description()}
      This removes your message from the conversation. This action cannot be
      undone.
    {/snippet}
    {#snippet confirmLabel()}
      {deletePending ? "Deleting…" : "Delete message"}
    {/snippet}

    <StoreNotice
      noticeKey={`delete-message:${message.id}`}
      title="Could not delete message"
      message={deleteError}
      pending={deletePending}
    />
  </ConfirmDialog>
{/if}
