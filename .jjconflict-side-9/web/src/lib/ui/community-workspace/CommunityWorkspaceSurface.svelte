<script lang="ts">
  import type { Snippet } from "svelte";
  import type {
    CommunityWorkspaceModel,
    CommunityWorkspaceHandlers,
    CommunityMessageModel,
  } from "./model.js";
  import type { CommunityCustomEmoji } from "../../community/contracts.js";
  import CommunityConversation from "../community-conversation/CommunityConversation.svelte";
  let {
    model,
    communityEmojis = [],
    readOnly = false,
    readOnlyReason,
    history,
    onLoadOlder,
    handlers = {},
    messageSupplement,
    messageMetadata,
  }: {
    model: CommunityWorkspaceModel;
    communityEmojis?: readonly CommunityCustomEmoji[];
    readOnly?: boolean;
    readOnlyReason?: string;
    history?: {
      key?: string;
      complete?: boolean;
      hasNewer?: boolean;
      hasOlder: boolean;
      loading: boolean;
    };
    onLoadOlder?: () => Promise<void>;
    handlers?: CommunityWorkspaceHandlers;
    messageSupplement?: Snippet<[CommunityMessageModel]>;
    messageMetadata?: Snippet<[CommunityMessageModel]>;
  } = $props();
  const rooms = $derived(
    model.navigationSections.flatMap((section) => section.rooms),
  );
  const room = $derived(rooms.find((room) => room.id === model.activeRoomId));
</script>

{#if room}<CommunityConversation
    {room}
    {rooms}
    messages={model.messages}
    members={model.members}
    currentMemberId={model.currentMemberId}
    {communityEmojis}
    disabled={readOnly}
    disabledReason={readOnlyReason}
    {history}
    {onLoadOlder}
    {messageSupplement}
    {messageMetadata}
    onSend={(content) => handlers.onSendMessage?.(room!.id, content)}
    onReply={handlers.onReply}
    onOpenThread={handlers.onOpenThread}
    onToggleReaction={handlers.onToggleReaction}
    onAddReaction={handlers.onAddReaction}
    onSaveMessageEdit={handlers.onSaveMessageEdit}
    onCopyMessageLink={handlers.onCopyMessageLink}
  />{/if}
