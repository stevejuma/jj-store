<script lang="ts">
  import "./conversation.css";
  import {
    Layout,
    LayoutScrollButton,
    MessageList,
    SystemMessage,
    type ComposerStatus,
  } from "@lapismd/design-core/ai/chat";
  import { type Snippet } from "svelte";
  import CommunityMessage from "../community-message/CommunityMessage.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import {
    communityMessageDayLabel,
    communityMessageStartsDay,
  } from "./conversation-day.js";
  import { isMessageContinuation } from "../community-message/grouping.js";
  import {
    memberById,
    type CommunityRoomModel,
    type CommunityMemberModel,
    type CommunityMessageModel,
  } from "../community-workspace/model.js";
  import type { CommunityCustomEmoji } from "../../community/contracts.js";
  let {
    room,
    rooms = [],
    messages,
    members,
    onSend,
    onReply,
    onOpenThread,
    onToggleReaction,
    onAddReaction,
    onSaveMessageEdit,
    onCopyMessageLink,
    currentMemberId,
    communityEmojis = [],
    disabled = false,
    disabledReason = "Messaging is unavailable",
    history,
    onLoadOlder,
    composerStatus,
    messageSupplement,
    footer,
  }: {
    room: CommunityRoomModel;
    rooms?: readonly CommunityRoomModel[];
    messages: CommunityMessageModel[];
    members: CommunityMemberModel[];
    onSend?: (content: string) => void | Promise<void>;
    onReply?: (id: string) => void;
    onOpenThread?: (id: string) => void;
    onToggleReaction?: (
      id: string,
      emoji: string,
      custom?: { shortcode: string; url: string },
    ) => void | Promise<void>;
    onAddReaction?: (
      id: string,
      emoji: string,
      custom?: { shortcode: string; url: string },
    ) => void | Promise<void>;
    onSaveMessageEdit?: (id: string, text: string) => void | Promise<void>;
    onCopyMessageLink?: (id: string) => void;
    currentMemberId?: string;
    communityEmojis?: readonly CommunityCustomEmoji[];
    disabled?: boolean;
    disabledReason?: string;
    history?: { hasOlder: boolean; loading: boolean };
    onLoadOlder?: () => Promise<void>;
    composerStatus?: ComposerStatus;
    messageSupplement?: Snippet<[CommunityMessageModel]>;
    footer?: Snippet;
  } = $props();
  import { useAppearance } from "../../appearance/store.svelte.js";
  import { useDrafts } from "../../appearance/drafts.svelte.js";
  const appearance = useAppearance();
  const drafts = useDrafts();
  const draft = $derived(drafts.get(`conversation:${room.id}`));
  const editing = $derived(
    messages.find((message) => message.id === draft.editing),
  );
  function startEditing(message: CommunityMessageModel) {
    draft.editing = message.id;
    draft.value = message.content;
  }
  async function submit(content: string) {
    if (editing && onSaveMessageEdit) {
      await onSaveMessageEdit(editing.id, content);
      draft.editing = undefined;
    } else await onSend?.(content);
  }
</script>

<div
  class="community-conversation"
  data-room-kind={room.kind}
  data-composer-placement="overlay"
  data-footer={footer ? "custom" : "composer"}
>
  <Layout
    density={appearance?.density ?? "compact"}
    scrollMode="history"
    conversationKey={room.id}
    aria-label={`${room.name} conversation`}
    isEmpty={messages.length === 0}
  >
    {#snippet scrollButton({
      isVisible,
      hasNewMessages,
      scrollToBottom,
    })}<LayoutScrollButton
        {isVisible}
        {hasNewMessages}
        onClick={scrollToBottom}
      />{/snippet}
    {#snippet emptyState()}{#if history?.loading}<CommunitySkeleton
          layout="messages"
        />{:else}<div class="community-conversation__empty">
          <strong>Start the conversation in {room.name}</strong><span
            >Messages in this thread will appear here.</span
          >
        </div>{/if}{/snippet}
    {#snippet composer()}{#if footer}{@render footer()}{:else}
        <CommunityChatComposer
          bind:value={draft.value}
          {members}
          rooms={rooms.length ? rooms : [room]}
          {communityEmojis}
          {disabled}
          {disabledReason}
          {composerStatus}
          placeholder={`Message #${room.name}`}
          context={editing
            ? { kind: "edit", id: editing.id, label: "Editing message" }
            : undefined}
          onCancelContext={() => {
            draft.editing = undefined;
            draft.value = "";
          }}
          onSubmit={(submission) => submit(submission.content)}
        />
      {/if}{/snippet}
    <MessageList
      density={appearance?.density ?? "compact"}
      aria-label={`${room.name} messages`}
      conversationKey={room.id}
      latestMessageId={messages.at(-1)?.id ?? ""}
      scrollToTopAction={onLoadOlder}
      hasOlder={history?.hasOlder ?? false}
      loading={history?.loading ?? false}
    >
      {#each messages as message, index (message.id)}
        {#if communityMessageStartsDay(messages, index)}<SystemMessage
            variant="divider">{communityMessageDayLabel(message)}</SystemMessage
          >{/if}
        {#snippet supplement()}{#if messageSupplement}{@render messageSupplement(
              message,
            )}{/if}{/snippet}
        <CommunityMessage
          {message}
          author={memberById(members, message.authorId)}
          {members}
          rooms={rooms.length ? rooms : [room]}
          density={appearance?.density ?? "compact"}
          {currentMemberId}
          {communityEmojis}
          isContinuation={isMessageContinuation(messages, index, {
            startsNewGroup: communityMessageStartsDay(messages, index),
          })}
          replyParticipants={(message.replyParticipantIds ?? []).map((id) =>
            memberById(members, id),
          )}
          onReply={disabled ? undefined : onReply}
          replyActionLabel={disabled ? "View replies" : "Reply in thread"}
          {onOpenThread}
          onToggleReaction={disabled ? undefined : onToggleReaction}
          onAddReaction={disabled ? undefined : onAddReaction}
          onCopyLink={onCopyMessageLink}
          onCopyMessage={(id) => {
            const content = messages.find((m) => m.id === id)?.content;
            if (content) void navigator.clipboard.writeText(content);
          }}
          onEdit={!disabled &&
          onSaveMessageEdit &&
          message.authorId === currentMemberId
            ? () => startEditing(message)
            : undefined}
          supplement={messageSupplement ? supplement : undefined}
        />
      {/each}
    </MessageList>
  </Layout>
</div>
