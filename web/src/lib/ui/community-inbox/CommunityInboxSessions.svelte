<!-- Adapted from Community navigation's expandable item rows and indented children. -->
<script lang="ts">
  import { untrack } from "svelte";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import PinIcon from "@lucide/svelte/icons/pin";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import MessagesSquareIcon from "@lucide/svelte/icons/messages-square";
  import CommunityInboxItem from "./CommunityInboxItem.svelte";
  import { groupSessionInboxItems, type SessionInboxItem } from "./model.js";
  let {
    items,
    activeId,
    onSelect,
    loading = false,
    kind = "session",
    pinned = [],
    archived = [],
    onPin,
    onArchive,
  }: {
    items: readonly SessionInboxItem[];
    activeId?: string | null;
    onSelect: (id: string) => void;
    loading?: boolean;
    kind?: "session" | "thread";
    pinned?: readonly string[];
    archived?: readonly string[];
    onPin?: (id: string) => void;
    onArchive?: (id: string) => void;
  } = $props();
  const groups = $derived(groupSessionInboxItems(items, pinned));
  let openGroups = $state<Record<string, boolean>>({});
  let revealedId: string | null | undefined;
  $effect(() => {
    const id = activeId;
    const group = groups.find((group) =>
      group.items.some((item) => (item.targetId ?? item.id) === id),
    );
    if (group && id !== revealedId)
      untrack(() => {
        revealedId = id;
        openGroups = { ...openGroups, [group.id]: true };
      });
  });
</script>

{#if loading && !groups.length}
  <div
    class="inbox-session-tree"
    role="status"
    aria-label="Loading sessions"
    aria-busy="true"
  >
    <span class="sr-only">Loading sessions</span>
    <div class="inbox-session-tree__skeleton-groups" aria-hidden="true">
      {#each [3, 2] as count, index (index)}
        <div>
          <Skeleton class="inbox-session-tree__skeleton-header" />
          <div class="inbox-session-tree__children">
            {#each Array(count) as _, row (row)}
              <div class="inbox-session-tree__skeleton-row">
                <Skeleton class="inbox-session-tree__skeleton-avatar" />
                <div class="inbox-session-tree__skeleton-text">
                  <Skeleton class="inbox-session-tree__skeleton-name" />
                  <Skeleton class="inbox-session-tree__skeleton-context" />
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  </div>
{:else if groups.length}
  <nav
    aria-label={`${kind === "thread" ? "Threads" : "Session chats"} by repository`}
    class="inbox-session-tree"
  >
    {#each groups as group (group.id)}
      {@const open = openGroups[group.id] ?? true}
      <Collapsible.Root
        {open}
        onOpenChange={(value) =>
          (openGroups = { ...openGroups, [group.id]: value })}
        role="group"
        class="inbox-session-tree__group"
        aria-label={`${group.name} ${kind === "thread" ? "Threads" : "sessions"}`}
        data-repository-id={group.repositoryId}
      >
        <div class="inbox-session-tree__header">
          <Collapsible.Trigger>
            {#snippet child({ props })}
              <Button
                {...props}
                variant="ghost"
                class="inbox-session-tree__repository"
                aria-label={`${open ? "Collapse" : "Expand"} ${group.name} ${kind === "thread" ? "Threads" : "sessions"}`}
                title={group.name}
              >
                {#if group.pinned}<PinIcon
                    aria-hidden="true"
                  />{:else if group.repositoryId}<FolderGitIcon
                    data-icon="inline-start"
                    aria-hidden="true"
                  />{:else}<MessagesSquareIcon
                    data-icon="inline-start"
                    aria-hidden="true"
                  />{/if}
                <span>{group.name}</span>
              </Button>
            {/snippet}
          </Collapsible.Trigger>
          {#if group.repositoryId}
            <Tooltip.Provider
              ><Tooltip.Root>
                <Tooltip.Trigger>
                  {#snippet child({ props })}
                    <a
                      {...props}
                      href={`#/repos/${encodeURIComponent(group.repositoryId!)}/files`}
                      class="inbox-session-tree__action"
                      data-ui-component="button"
                      data-variant="ghost"
                      data-size="icon-xs"
                      aria-label={`Open ${group.name}`}
                    >
                      <FolderGitIcon aria-hidden="true" />
                    </a>
                  {/snippet}
                </Tooltip.Trigger>
                <Tooltip.Content role="tooltip"
                  >Open {group.name}</Tooltip.Content
                >
              </Tooltip.Root></Tooltip.Provider
            >
          {/if}
          <Collapsible.Trigger>
            {#snippet child({ props })}
              <Button
                {...props}
                variant="ghost"
                size="icon-xs"
                class="inbox-session-tree__toggle"
                aria-label={`Toggle ${group.name} ${kind === "thread" ? "Threads" : "sessions"}`}
              >
                <ChevronRightIcon aria-hidden="true" />
              </Button>
            {/snippet}
          </Collapsible.Trigger>
        </div>
        <Collapsible.Content>
          <div class="inbox-session-tree__children">
            {#each group.items as item (item.id)}
              <CommunityInboxItem
                {item}
                active={(item.targetId ?? item.id) === activeId}
                onSelect={() => onSelect(item.targetId ?? item.id)}
                pinned={pinned.includes(item.targetId ?? item.id)}
                archived={archived.includes(item.targetId ?? item.id)}
                onPin={onPin
                  ? () => onPin(item.targetId ?? item.id)
                  : undefined}
                onArchive={onArchive
                  ? () => onArchive(item.targetId ?? item.id)
                  : undefined}
              />
            {/each}
          </div>
        </Collapsible.Content>
      </Collapsible.Root>
    {/each}
  </nav>
{/if}

<style>
  .inbox-session-tree {
    display: grid;
    min-width: 0;
    grid-template-columns: minmax(0, 1fr);
    gap: 0.25rem;
    padding-inline: 0.5rem;
  }
  :global(.inbox-session-tree__group) {
    min-width: 0;
  }
  .inbox-session-tree__header {
    display: flex;
    min-width: 0;
    align-items: center;
    gap: 0.125rem;
    padding-inline-end: 0.25rem;
    border-radius: var(--ui-button-radius, calc(var(--radius) * 0.8));
    background: var(--muted);
  }
  .inbox-session-tree__header:hover,
  .inbox-session-tree__header:focus-within {
    background: color-mix(in srgb, var(--foreground) 4%, var(--muted));
  }
  .inbox-session-tree__header
    :global(.inbox-session-tree__repository[data-ui-component="button"]),
  .inbox-session-tree__header
    :global(.inbox-session-tree__toggle[data-ui-component="button"]) {
    background: transparent;
  }
  .inbox-session-tree__action {
    flex: none;
  }
  .inbox-session-tree__action:hover,
  .inbox-session-tree__action:focus-visible {
    background: color-mix(in srgb, var(--foreground) 12%, var(--muted));
  }
  .inbox-session-tree__header :global(.inbox-session-tree__toggle:hover),
  .inbox-session-tree__header
    :global(.inbox-session-tree__toggle:focus-visible) {
    background: color-mix(in srgb, var(--foreground) 12%, var(--muted));
  }
  :global(.inbox-session-tree__repository[data-ui-component="button"]) {
    --ui-button-height: 2.25rem;
    --ui-button-gap: 0.65rem;
    --ui-button-padding-inline: 0.625rem;
    width: 100%;
    min-width: 0;
    flex: 1;
    justify-content: flex-start;
    text-align: start;
  }
  :global(.inbox-session-tree__repository > span) {
    min-width: 0;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  :global(.inbox-session-tree__toggle[aria-expanded="true"] > svg) {
    rotate: 90deg;
  }
  .inbox-session-tree__children {
    display: grid;
    gap: 0.125rem;
    padding: 0.125rem 0 0.25rem 1rem;
  }
  .inbox-session-tree__skeleton-groups {
    display: grid;
    gap: 0.25rem;
  }
  :global(.inbox-session-tree__skeleton-header) {
    height: 2.25rem;
    width: 100%;
  }
  .inbox-session-tree__skeleton-row {
    box-sizing: border-box;
    display: grid;
    min-height: 3.25rem;
    grid-template-columns: 1.5rem minmax(0, 1fr);
    align-items: center;
    gap: 0.5rem;
    padding: 0.375rem 0.625rem;
  }
  :global(.inbox-session-tree__skeleton-avatar) {
    width: 1.5rem;
    height: 1.5rem;
  }
  .inbox-session-tree__skeleton-text {
    display: grid;
    gap: 0.125rem;
    min-width: 0;
  }
  :global(.inbox-session-tree__skeleton-name) {
    width: 45%;
    height: 1rem;
  }
  :global(.inbox-session-tree__skeleton-context) {
    width: 80%;
    height: 0.875rem;
  }
</style>
