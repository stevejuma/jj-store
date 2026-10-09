<!-- Adapted from Community navigation's expandable item rows and indented children. -->
<script lang="ts">
  import { untrack } from "svelte";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import MessagesSquareIcon from "@lucide/svelte/icons/messages-square";
  import CommunityInboxItem from "./CommunityInboxItem.svelte";
  import { groupSessionInboxItems, type SessionInboxItem } from "./model.js";
  let {
    items,
    activeId,
    onSelect,
  }: {
    items: readonly SessionInboxItem[];
    activeId?: string | null;
    onSelect: (id: string) => void;
  } = $props();
  const groups = $derived(groupSessionInboxItems(items));
  let openGroups = $state<Record<string, boolean>>({});
  let revealedId: string | null | undefined;
  $effect(() => {
    const id = activeId;
    const group = groups.find((group) =>
      group.items.some((item) => item.id === id),
    );
    if (group && id !== revealedId)
      untrack(() => {
        revealedId = id;
        openGroups = { ...openGroups, [group.id]: true };
      });
  });
</script>

{#if groups.length}
  <nav aria-label="Session chats by repository" class="inbox-session-tree">
    {#each groups as group (group.id)}
      {@const open = openGroups[group.id] ?? true}
      <Collapsible.Root
        {open}
        onOpenChange={(value) =>
          (openGroups = { ...openGroups, [group.id]: value })}
        role="group"
        class="inbox-session-tree__group"
        aria-label={`${group.name} sessions`}
        data-repository-id={group.repositoryId}
      >
        <Collapsible.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="ghost"
              class="inbox-session-tree__repository"
              aria-label={`${open ? "Collapse" : "Expand"} ${group.name} sessions`}
              title={group.name}
            >
              {#if group.repositoryId}<FolderGitIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />{:else}<MessagesSquareIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />{/if}
              <span>{group.name}</span>
              <ChevronRightIcon data-icon="inline-end" aria-hidden="true" />
            </Button>
          {/snippet}
        </Collapsible.Trigger>
        <Collapsible.Content>
          <div class="inbox-session-tree__children">
            {#each group.items as item (item.id)}
              <CommunityInboxItem
                {item}
                active={item.id === activeId}
                onSelect={() => onSelect(item.id)}
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
  :global(.inbox-session-tree__repository[data-ui-component="button"]) {
    --ui-button-height: 2.25rem;
    --ui-button-gap: 0.65rem;
    --ui-button-padding-inline: 0.625rem;
    width: 100%;
    min-width: 0;
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
  :global(
    .inbox-session-tree__repository[aria-expanded="true"] > svg:last-child
  ) {
    rotate: 90deg;
  }
  .inbox-session-tree__children {
    display: grid;
    gap: 0.125rem;
    padding: 0.125rem 0 0.25rem 1rem;
  }
</style>
