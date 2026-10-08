<script lang="ts">
  import CommunityEntityReference from "../community-markdown-content/CommunityEntityReference.svelte";
  import CommunityMemberHoverCard from "../community-member-hover-card/CommunityMemberHoverCard.svelte";
  import { contentMentionFallbackMember } from "../../community/content-mentions.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  let {
    href,
    memberId,
    label,
    state,
  }: {
    href?: string;
    memberId?: string;
    label: string;
    state: ChatBodyComments;
  } = $props();
  const member = $derived(
    memberId
      ? (state.props().members?.find((m) => m.id === memberId) ??
          contentMentionFallbackMember(memberId, label))
      : undefined,
  );
</script>

{#if href}<CommunityEntityReference
    {href}
    {label}
  />{:else if member}<CommunityMemberHoverCard
    {member}
    trigger="mention"
    triggerLabel={`@${member.name}`}
  />{/if}
