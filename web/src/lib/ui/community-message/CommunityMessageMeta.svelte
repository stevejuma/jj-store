<script lang="ts">
  import { MessageMetadata } from "@lapismd/design-core/ai/chat";
  import type { Snippet } from "svelte";

  import type {
    CommunityMemberModel,
    CommunityMessageModel,
  } from "../community-workspace/model.js";

  import CommunityMessageReplySummary from "./CommunityMessageReplySummary.svelte";
  import {
    bundleMessageRowMembers,
    implicitTaggedMentionMembers,
    taggedMentionAvatarsLabel,
  } from "./implicit-mentions.js";

  let {
    message,
    members = [],
    isContinuation = false,
    replyParticipants = [],
    onOpenThread,
    onOpenProfile,
    onMessageMember,
    onWaveMember,
    onViewMemberActivity,
    supplement,
  }: {
    message: CommunityMessageModel;
    members?: readonly CommunityMemberModel[];
    isContinuation?: boolean;
    supplement?: Snippet;
    replyParticipants?: readonly CommunityMemberModel[];
    onOpenThread?: ((messageId: string) => void) | undefined;
    onOpenProfile?: ((member: CommunityMemberModel) => void) | undefined;
    onMessageMember?:
      ((member: CommunityMemberModel) => void | Promise<void>) | undefined;
    onWaveMember?:
      ((member: CommunityMemberModel) => void | Promise<void>) | undefined;
    onViewMemberActivity?:
      ((member: CommunityMemberModel) => void | Promise<void>) | undefined;
  } = $props();

  const replyLabel = $derived.by(() => {
    const replyCount = message.replyCount ?? 0;
    return (
      message.replyLabel ??
      `${replyCount} ${replyCount === 1 ? "reply" : "replies"}`
    );
  });
  const showReplySummary = $derived(
    (message.replyCount ?? 0) > 0 && onOpenThread !== undefined,
  );
  const implicitMentions = $derived(
    implicitTaggedMentionMembers(message, members),
  );
  const rowParticipants = $derived(
    showReplySummary
      ? bundleMessageRowMembers(replyParticipants, implicitMentions)
      : implicitMentions,
  );
</script>

<div class="community-message__metadata">
  {#if !isContinuation || message.edited}
    {#snippet edited()}<span>Edited</span>{/snippet}
    <MessageMetadata
      timestamp={isContinuation ? undefined : message.timestamp}
      footer={message.edited ? edited : undefined}
    />
  {/if}
  {#if showReplySummary || implicitMentions.length}
    <CommunityMessageReplySummary
      label={showReplySummary ? replyLabel : undefined}
      avatarLabel={showReplySummary
        ? "Reply participants"
        : taggedMentionAvatarsLabel(implicitMentions)}
      participants={rowParticipants}
      onOpenThread={showReplySummary && onOpenThread
        ? () => onOpenThread(message.id)
        : undefined}
      {onOpenProfile}
      {onMessageMember}
      {onWaveMember}
      {onViewMemberActivity}
    />
  {/if}
  {#if supplement}
    <div
      class="community-message__metadata-supplement"
      data-ui-part="metadata-supplement"
    >
      {@render supplement()}
    </div>
  {/if}
</div>

<style>
  .community-message__metadata {
    display: flex;
    flex-wrap: wrap;
    min-height: 1.5rem;
    align-items: center;
    gap: 0.4rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
    line-height: 1;
  }
  .community-message__metadata-supplement {
    display: inline-flex;
    align-items: center;
    min-width: 0;
    gap: 0.4rem;
    flex-wrap: wrap;
  }

  .community-message__metadata
    :global([data-ui-component="ai-chat-message-metadata"]) {
    min-height: 1.5rem;
    margin-top: 0;
    line-height: 1;
  }
</style>
