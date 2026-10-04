<script lang="ts">
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { communityMemberAvatarShape } from "../community-member-avatar/model.js";

  let {
    members,
    label = "Members",
    avatarSize = "small",
  }: {
    members: readonly CommunityMemberModel[];
    label?: string;
    avatarSize?: "xsmall" | "small";
  } = $props();
</script>

<span
  class="community-member-avatar-group"
  data-avatar-size={avatarSize}
  role="group"
  aria-label={label}
>
  {#each members as member (member.id)}
    <span
      class="community-member-avatar-group__member"
      data-shape={communityMemberAvatarShape(member)}
      title={member.name}
    >
      <CommunityMemberAvatar {member} size={avatarSize} />
    </span>
  {/each}
</span>

<style>
  .community-member-avatar-group {
    --community-member-avatar-group-overlap: 0.5rem;

    display: flex;
    align-items: center;
    flex-wrap: wrap;
    padding-inline-start: var(--community-member-avatar-group-overlap);
  }

  .community-member-avatar-group[data-avatar-size="xsmall"] {
    --community-member-avatar-group-overlap: 0.4rem;
  }

  .community-member-avatar-group__member {
    position: relative;
    margin-inline-start: calc(
      -1 * var(--community-member-avatar-group-overlap)
    );
    border-radius: 50%;
    box-shadow: 0 0 0 2px var(--background);
  }

  .community-member-avatar-group__member[data-shape="rounded"] {
    border-radius: 25%;
  }
</style>
