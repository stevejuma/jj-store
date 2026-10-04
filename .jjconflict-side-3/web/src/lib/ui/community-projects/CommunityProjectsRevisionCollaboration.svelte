<script lang="ts">
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  import type { RevisionCollaboration } from "../../collaboration/revision-collaboration.js";
  import CommunityMemberAvatarGroup from "../community-member-avatar-group/CommunityMemberAvatarGroup.svelte";

  let { activity }: { activity: RevisionCollaboration } = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<span
  class="community-projects-revision-collaboration"
  data-ui-part="revision-collaboration"
  onclick={(event) => event.stopPropagation()}
>
  {#if activity.members.length}<CommunityMemberAvatarGroup
      members={activity.members.slice(0, 3)}
      label="Collaboration participants"
      avatarSize="xsmall"
    />{/if}
  {#if activity.replyCount > 0}<span
      class="community-projects-revision-collaboration__replies"
      aria-label={`${activity.replyCount} review ${activity.replyCount === 1 ? "reply" : "replies"}`}
      ><MessageSquareIcon aria-hidden="true" />{activity.replyCount}
      {activity.replyCount === 1 ? "reply" : "replies"}</span
    >{/if}
</span>

<style>
  .community-projects-revision-collaboration {
    display: inline-flex;
    min-width: 0;
    flex: 0 0 auto;
    align-items: center;
    gap: 0.4rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  .community-projects-revision-collaboration__replies {
    display: inline-flex;
    align-items: center;
    gap: 0.2rem;
    white-space: nowrap;
  }

  .community-projects-revision-collaboration__replies :global(svg) {
    width: 0.8rem;
    height: 0.8rem;
  }
</style>
