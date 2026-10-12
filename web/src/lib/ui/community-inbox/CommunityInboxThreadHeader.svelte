<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { Snippet } from "svelte";
  let {
    listLabel,
    plural,
    expanded,
    threadTitle,
    threadMember,
    threadStatus,
    threadActions,
    threadAvatar,
    openSidebar,
    onToggle,
    actionsElement = $bindable(),
  }: {
    listLabel: string;
    plural: string;
    expanded: boolean;
    threadTitle?: string;
    threadMember?: CommunityMemberModel;
    threadStatus?: string;
    threadActions?: Snippet<[{ openSidebar: () => void }]>;
    threadAvatar?: Snippet;
    openSidebar: () => void;
    onToggle: () => void;
    actionsElement?: HTMLDivElement;
  } = $props();
</script>

<header class="lc-community-inbox__thread-header">
  <div class="lc-community-inbox__thread-identity">
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label={`Toggle ${listLabel.toLowerCase()}`}
      title={`Toggle ${listLabel.toLowerCase()}`}
      aria-pressed={expanded}
      onclick={onToggle}
      ><CommunitySidebarToggleIcon side="left" {expanded} /></Button
    >
    {#if threadAvatar}{@render threadAvatar()}{:else if threadMember}<CommunityMemberAvatar
        member={threadMember}
        size="small"
        shape="rounded"
        decorative
      />{/if}
    <div>
      <h1 title={threadTitle ?? plural}>
        {threadTitle ?? plural}
      </h1>
      {#if threadStatus}<p role="status">{threadStatus}</p>{/if}
    </div>
  </div>
  <div class="lc-community-inbox__thread-actions" bind:this={actionsElement}>
    {#if threadActions}{@render threadActions({ openSidebar })}{/if}
  </div>
</header>
