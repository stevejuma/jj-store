<!-- Owned Community conversation row beneath its repository header. -->
<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
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
    aria-label={item.label
      ? `Open Thread ${item.label}`
      : `Open session ${item.author.name} · ${item.id}${item.status === "Draft" ? "" : `, ${item.status}`}`}
    onclick={onSelect}
  >
    <CommunityMemberAvatar
      member={item.author}
      size="xsmall"
      shape="rounded"
      showPresence={item.status !== "Draft"}
      {presence}
      decorative
    />
    <span class="lc-community-inbox__item-body">
      <span class="lc-community-inbox__item-heading"
        ><strong title={item.label ?? item.author.name}
          >{item.label ?? item.author.name}</strong
        ></span
      >
      <span class="lc-community-inbox__item-detail">
        <span class="lc-community-inbox__context"
          >{item.label ? item.author.name : item.context}</span
        >
        {#if item.lastActivityAt !== undefined}<span
            class="lc-community-inbox__time"
          >
            <CommunityRelativeTime
              timestamp={item.lastActivityAt}
              ariaLabel={`Last activity: ${formatCommunityFullDate(item.lastActivityAt)}`}
            />
          </span>{/if}
      </span>
      {#if item.lastActivityAt === undefined && item.excerpt}<span
          class="lc-community-inbox__excerpt">{item.excerpt}</span
        >{/if}
    </span>
  </Button>
</div>
