<!-- Owned Community conversation row beneath its repository header. -->
<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as ButtonGroup from "@lapismd/design-core/shadcn/button-group";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import PinIcon from "@lucide/svelte/icons/pin";
  import ArchiveIcon from "@lucide/svelte/icons/archive";
  import ArchiveRestoreIcon from "@lucide/svelte/icons/archive-restore";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import { formatCommunityFullDate } from "../community-relative-time/model.js";
  import type { SessionInboxItem } from "./model.js";
  let {
    item,
    active,
    onSelect,
    pinned = false,
    archived = false,
    onPin,
    onArchive,
  }: {
    item: SessionInboxItem;
    active: boolean;
    onSelect: () => void;
    pinned?: boolean;
    archived?: boolean;
    onPin?: () => void;
    onArchive?: () => void;
  } = $props();
  const presence = $derived(
    item.status === "Online" || item.status === "Working"
      ? "online"
      : item.status === "Starting" || item.status === "Unknown"
        ? "unknown"
        : "offline",
  );
</script>

<div class="lc-community-inbox__item-row" data-pinned={pinned ? "" : undefined}>
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
  {#if onPin || onArchive}
    <div class="inbox-row-actions">
      <ButtonGroup.Root aria-label="Conversation actions">
        <Tooltip.Provider>
          {#if onPin}<Tooltip.Root
              ><Tooltip.Trigger
                >{#snippet child({ props })}<Button
                    {...props}
                    size="icon-xs"
                    variant="secondary"
                    aria-label={pinned
                      ? "Unpin conversation"
                      : "Pin conversation"}
                    aria-pressed={pinned}
                    onclick={onPin}><PinIcon aria-hidden="true" /></Button
                  >{/snippet}</Tooltip.Trigger
              ><Tooltip.Content
                >{pinned
                  ? "Unpin conversation"
                  : "Pin conversation"}</Tooltip.Content
              ></Tooltip.Root
            >{/if}
          {#if onPin && onArchive}<ButtonGroup.Separator />{/if}
          {#if onArchive}<Tooltip.Root
              ><Tooltip.Trigger
                >{#snippet child({ props })}<Button
                    {...props}
                    size="icon-xs"
                    variant="secondary"
                    aria-label={archived
                      ? "Restore on this device"
                      : "Archive on this device"}
                    onclick={onArchive}
                    >{#if archived}<ArchiveRestoreIcon
                        aria-hidden="true"
                      />{:else}<ArchiveIcon aria-hidden="true" />{/if}</Button
                  >{/snippet}</Tooltip.Trigger
              ><Tooltip.Content
                >{archived
                  ? "Restore on this device"
                  : "Archive on this device"}</Tooltip.Content
              ></Tooltip.Root
            >{/if}
        </Tooltip.Provider>
      </ButtonGroup.Root>
    </div>
  {/if}
</div>

<style>
  .lc-community-inbox__item-row {
    position: relative;
  }
  .inbox-row-actions {
    display: flex;
    position: absolute;
    inset-inline-end: 0.25rem;
    inset-block-start: 0.125rem;
    border-radius: var(--radius);
    background: var(--secondary);
    opacity: 0;
    pointer-events: none;
  }
  .lc-community-inbox__item-row:hover .inbox-row-actions,
  .lc-community-inbox__item-row:focus-within .inbox-row-actions {
    opacity: 1;
    pointer-events: auto;
  }
  @media (hover: none) {
    .inbox-row-actions {
      opacity: 1;
      pointer-events: auto;
    }
  }
</style>
