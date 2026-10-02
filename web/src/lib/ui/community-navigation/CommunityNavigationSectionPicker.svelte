<script lang="ts">
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import { navigationItemIcon } from "./icons.js";
  import type {
    CommunityNavigationSectionModel,
    CommunityNavigationItemModel,
  } from "./model.js";
  let {
    section,
    activeItemId,
    onSelect,
  }: {
    section: CommunityNavigationSectionModel;
    activeItemId?: string;
    onSelect: (item: CommunityNavigationItemModel) => void;
  } = $props();
  let query = $state("");
  let input = $state<HTMLInputElement | null>(null);
  $effect(() => input?.focus());
  const items = $derived(
    section.id === "projects"
      ? section.items
      : [
          ...new Map(
            section.items
              .flatMap((item) => [item, ...(item.children ?? [])])
              .map((item) => [item.id, item]),
          ).values(),
        ],
  );
</script>

{#snippet itemIcon(item: CommunityNavigationItemModel)}
  {@const Icon = navigationItemIcon(item)}<Icon aria-hidden="true" />
{/snippet}
<CommandView.Root label={`${section.label} navigation`} loop>
  <CommandView.Input
    bind:ref={input}
    value={query}
    placeholder={`Search ${section.label.toLocaleLowerCase()}`}
    aria-label={`Search ${section.label}`}
    oninput={(event) => {
      query = event.currentTarget.value;
    }}
  />
  <CommandView.List>
    <CommandView.Empty
      >No matching {section.label.toLocaleLowerCase()}</CommandView.Empty
    >
    {#if section.items.length}
      <CommandView.Group heading={section.label}>
        {#each items as item (item.id)}
          <CommandView.Item
            value={`${item.label} ${item.id}`}
            data-active={activeItemId === item.id ? "" : undefined}
            onSelect={() => onSelect(item)}
          >
            <CommandView.ItemIcon>
              {@render itemIcon(item)}
            </CommandView.ItemIcon>
            <CommandView.ItemLabel>{item.label}</CommandView.ItemLabel>
            {#if item.description}<CommandView.ItemDescription
                >{item.description}</CommandView.ItemDescription
              >{/if}
          </CommandView.Item>
        {/each}
      </CommandView.Group>
    {/if}
    {#if section.action}<CommandView.Group heading="Actions">
        <CommandView.Item
          value={section.action.label}
          onSelect={() => onSelect(section.action!)}
        >
          <CommandView.ItemIcon
            >{@render itemIcon(section.action)}</CommandView.ItemIcon
          >
          <CommandView.ItemLabel>{section.action.label}</CommandView.ItemLabel>
        </CommandView.Item>
      </CommandView.Group>{/if}
  </CommandView.List>
</CommandView.Root>
