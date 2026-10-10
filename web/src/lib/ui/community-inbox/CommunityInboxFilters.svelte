<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import ListFilterIcon from "@lucide/svelte/icons/list-filter";
  import CheckIcon from "@lucide/svelte/icons/check";
  import XIcon from "@lucide/svelte/icons/x";
  import type { SessionInboxItem } from "./model.js";
  import {
    inboxFacets,
    inboxFilterOptions,
    emptyInboxFilters,
    facetLabels,
    type InboxFilters,
    type InboxFacet,
  } from "./inbox-filters.js";
  let {
    items,
    filters = $bindable(emptyInboxFilters()),
  }: { items: readonly SessionInboxItem[]; filters?: InboxFilters } = $props();
  let open = $state(false),
    query = $state("");
  const count = $derived(
    inboxFacets.reduce((count, key) => count + filters[key].length, 0),
  );
  function toggle(facet: InboxFacet, id: string) {
    filters = {
      ...filters,
      [facet]: filters[facet].includes(id)
        ? filters[facet].filter((value) => value !== id)
        : [...filters[facet], id],
    };
  }
  function options(facet: InboxFacet) {
    const loaded = inboxFilterOptions(items, facet);
    // A refresh may temporarily omit a selected choice; keep it removable.
    return [
      ...loaded,
      ...filters[facet]
        .filter((id) => !loaded.some((o) => o.id === id))
        .map((id) => ({ id, label: id || "No repository" })),
    ];
  }
</script>

<Popover.Root
  {open}
  onOpenChange={(next) => {
    open = next;
    if (next) query = "";
  }}
>
  <Popover.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        variant={count ? "default" : "ghost"}
        size="icon-sm"
        aria-label="Filter loaded conversations"
        title="Filter loaded conversations"
        aria-pressed={count > 0}><ListFilterIcon aria-hidden="true" /></Button
      >{/snippet}</Popover.Trigger
  >
  <Popover.Content
    align="end"
    role="dialog"
    aria-label="Conversation filters"
    style="--ui-popover-width: min(24rem, calc(100vw - 2rem));"
  >
    <Popover.Header
      style="flex-direction: row; align-items: center; justify-content: space-between;"
    >
      <Popover.Title role="heading" aria-level={2}
        >Conversation filters</Popover.Title
      >
      <Popover.Close
        >{#snippet child({ props })}<Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="Close filters"><XIcon aria-hidden="true" /></Button
          >{/snippet}</Popover.Close
      >
    </Popover.Header>
    <p class="inbox-filter-scope">
      Filters apply to loaded conversations. Select several values in each
      group.
    </p>
    <CommandView.Root label="Conversation filters">
      <CommandView.Input
        bind:value={query}
        placeholder="Find filters..."
        aria-label="Find conversation filters"
      />
      <CommandView.List>
        <CommandView.Empty>No matching filters</CommandView.Empty>
        {#each inboxFacets as facet (facet)}
          {#if options(facet).length}<CommandView.Group
              heading={facetLabels[facet]}
            >
              {#each options(facet) as option (option.id)}
                <CommandView.Item
                  value={`${facet}:${option.id} ${option.label}`}
                  aria-label={`${facetLabels[facet]}: ${option.label}`}
                  data-checked={filters[facet].includes(option.id)
                    ? ""
                    : undefined}
                  onSelect={() => toggle(facet, option.id)}
                >
                  <CommandView.ItemLabel>{option.label}</CommandView.ItemLabel>
                  {#if filters[facet].includes(option.id)}<span
                      data-ui-component="command-view"
                      data-ui-part="trailing"
                      ><CheckIcon aria-label="Selected" /></span
                    >{/if}
                </CommandView.Item>
              {/each}
            </CommandView.Group>{/if}
        {/each}
      </CommandView.List>
    </CommandView.Root>
    <Button
      variant="ghost"
      size="sm"
      disabled={!count}
      onclick={() => (filters = emptyInboxFilters())}>Clear filters</Button
    >
  </Popover.Content>
</Popover.Root>

<style>
  .inbox-filter-scope {
    margin: 0;
    font-size: var(--community-text-secondary);
    color: var(--muted-foreground);
  }
</style>
