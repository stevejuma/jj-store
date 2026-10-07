<!-- Community conversation row, with separate owned repository navigation. -->
<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import { formatCommunityFullDate } from "../community-relative-time/model.js";
  import type { SessionInboxItem } from "./model.js";
  let {
    item,
    active,
    onSelect,
  }: {
    item: SessionInboxItem;
    active: boolean;
    onSelect: () => void;
  } = $props();
  const presence = $derived(
    item.status === "Online" || item.status === "Working"
      ? "online"
      : item.status === "Starting" || item.status === "Unknown"
        ? "unknown"
        : "offline",
  );
</script>

<div class="lc-community-inbox__item-row">
  <Button
    variant="ghost"
    class="lc-community-inbox__item"
    data-active={active ? "" : undefined}
    aria-pressed={active}
    data-ui-part="inbox-conversation"
    aria-label={`Open session ${item.author.name} · ${item.id}${item.status === "Draft" ? "" : `, ${item.status}`}`}
    onclick={onSelect}
  >
    <CommunityMemberAvatar
      member={item.author}
      size="small"
      shape="rounded"
      showPresence={item.status !== "Draft"}
      {presence}
      decorative
    />
    <span class="lc-community-inbox__item-body">
      <span class="lc-community-inbox__item-heading"
        ><strong>{item.author.name}</strong></span
      >
      <span class="lc-community-inbox__context">{item.context}</span>
      {#if item.lastActivityAt !== undefined}<span
          class="lc-community-inbox__context"
        >
          <CommunityRelativeTime
            timestamp={item.lastActivityAt}
            ariaLabel={`Last activity: ${formatCommunityFullDate(item.lastActivityAt)}`}
          />
        </span>
      {:else if item.excerpt}<span class="lc-community-inbox__excerpt"
          >{item.excerpt}</span
        >{/if}
    </span>
  </Button>
  {#if item.repository}<Tooltip.Provider
      ><Tooltip.Root>
        {@const repository = item.repository}
        <Tooltip.Trigger
          >{#snippet child({ props })}
            <a
              {...props}
              href={repository.href}
              class="lc-community-inbox__repository"
              data-ui-component="button"
              data-variant="ghost"
              data-size="icon-sm"
              aria-label={repository.label}
            >
              <FolderGitIcon aria-hidden="true" />
            </a>
          {/snippet}</Tooltip.Trigger
        >
        <Tooltip.Content role="tooltip">{repository.label}</Tooltip.Content>
      </Tooltip.Root></Tooltip.Provider
    >{/if}
</div>
