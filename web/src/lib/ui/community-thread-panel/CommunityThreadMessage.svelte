<script lang="ts">
  import { useAppearance } from "../../appearance/store.svelte.js";
  const appearance = useAppearance();
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import {
    memberById,
    type CommunityMessageModel,
    type CommunityMemberModel,
  } from "../community-workspace/model.js";
  import type { ThreadPanelProps } from "./props.js";
  let {
    panel,
    message,
    isContinuation,
    members,
    focused,
    startEditing,
  }: {
    panel: ThreadPanelProps;
    message: CommunityMessageModel;
    isContinuation: boolean;
    members: readonly CommunityMemberModel[];
    focused?: string | undefined;
    startEditing(id: string): void;
  } = $props();
</script>

{#snippet supplement()}{#if panel.messageSupplement}{@render panel.messageSupplement(
      message,
    )}{/if}{/snippet}

<CommunityMessage
  supplement={panel.messageSupplement ? supplement : undefined}
  {message}
  highlighted={message.id === focused}
  {isContinuation}
  author={memberById(members, message.authorId)}
  {members}
  rooms={panel.rooms ?? []}
  replyParticipants={(message.replyParticipantIds ?? []).map((id) =>
    memberById(members, id),
  )}
  density={appearance?.density ?? "compact"}
  communityEmojis={panel.communityEmojis ?? []}
  onToggleReaction={panel.onToggleReaction}
  onAddReaction={panel.onAddReaction}
  onCopyLink={panel.onCopyMessageLink}
  onCopyMessage={panel.onCopyMessage}
  {...panel.onCopyEvent === undefined ? {} : { onCopyEvent: panel.onCopyEvent }}
  {...(panel.onEditMessage === undefined &&
    panel.onSaveMessageEdit === undefined) ||
  message.authorId !== panel.currentMemberId
    ? {}
    : {
        onEdit: panel.onSaveMessageEdit
          ? () => startEditing(message.id)
          : panel.onEditMessage,
      }}
  {...panel.onDeleteMessage === undefined ||
  (message.authorId !== panel.currentMemberId && !panel.canDeleteAny)
    ? {}
    : { onDelete: panel.onDeleteMessage }}
  {...panel.onReportMessage === undefined
    ? {}
    : { onReport: panel.onReportMessage }}
  {...panel.onOpenProfile === undefined
    ? {}
    : { onOpenProfile: panel.onOpenProfile }}
  {...panel.onOpenRoom === undefined ? {} : { onOpenRoom: panel.onOpenRoom }}
  {...panel.onMessageMember === undefined
    ? {}
    : { onMessageMember: panel.onMessageMember }}
  {...panel.onWaveMember === undefined
    ? {}
    : { onWaveMember: panel.onWaveMember }}
  {...panel.onViewMemberActivity === undefined
    ? {}
    : { onViewMemberActivity: panel.onViewMemberActivity }}
  {...message.id === panel.thread.rootMessageId || panel.onReply === undefined
    ? {}
    : {
        onReply: (id) => {
          panel.onReply?.(id);
        },
        replyActionLabel: "Reply",
      }}
/>
