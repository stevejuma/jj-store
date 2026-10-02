<script lang="ts">
  import type { Snippet } from "svelte";
  // CommunityProjectsActivityFilter composition, adapted to native recorded facts.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ListFilterIcon from "@lucide/svelte/icons/list-filter";
  import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
  import BotIcon from "@lucide/svelte/icons/bot";
  import XIcon from "@lucide/svelte/icons/x";
  import {
    nativeActivityFilters,
    type NativeActivityFilter,
  } from "../../controller/native-activity-filter.js";
  let {
    value = "all",
    onChange,
    scopeFiltered = false,
    children,
  }: {
    value?: NativeActivityFilter;
    onChange: (value: NativeActivityFilter) => void;
    scopeFiltered?: boolean;
    children?: Snippet;
  } = $props();
  let open = $state(false),
    query = $state("");
</script>

<Popover.Root
  {open}
  onOpenChange={(next) => {
    open = next;
    if (next) query = "";
  }}
>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        type="button"
        variant={value === "all" && !scopeFiltered ? "ghost" : "default"}
        size="icon-sm"
        aria-label="Filters for activity"
        title="Filters for activity"
        aria-pressed={value !== "all" || scopeFiltered}
      >
        <ListFilterIcon aria-hidden="true" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    align="end"
    role="dialog"
    aria-label="Activity filters"
    style="--ui-popover-width: 22rem; --ui-popover-padding: 0.5rem 0.75rem 0.625rem; --ui-popover-gap: 0.25rem"
  >
    <Popover.Header
      style="flex-direction: row; align-items: center; justify-content: space-between; gap: 0.25rem; padding: 0; margin: 0;"
    >
      <Popover.Title role="heading" aria-level={2}
        >Activity filters</Popover.Title
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
    {@render children?.()}
    <CommandView.Root label="Filter activity">
      <CommandView.Input
        bind:value={query}
        placeholder="Find an activity type..."
        aria-label="Find an activity type"
      />
      <CommandView.List>
        <CommandView.Empty>No matching activity types</CommandView.Empty>
        <CommandView.Group heading="Native activity">
          {#each nativeActivityFilters as option (option.id)}
            <CommandView.Item
              value={`${option.label} ${option.description}`}
              data-checked={value === option.id ? "" : undefined}
              onSelect={() => onChange(option.id)}
            >
              <CommandView.ItemIcon
                >{#if option.id === "agent"}<BotIcon
                    aria-hidden="true"
                  />{:else if option.id === "all"}<ListFilterIcon
                    aria-hidden="true"
                  />{:else}<GitCommitIcon
                    aria-hidden="true"
                  />{/if}</CommandView.ItemIcon
              >
              <CommandView.ItemLabel>{option.label}</CommandView.ItemLabel>
              <CommandView.ItemDescription
                >{option.description}</CommandView.ItemDescription
              >
              {#if value === option.id}<span
                  data-ui-component="command-view"
                  data-ui-part="trailing"><CheckIcon aria-hidden="true" /></span
                >{/if}
            </CommandView.Item>
          {/each}
        </CommandView.Group>
      </CommandView.List>
    </CommandView.Root>
  </Popover.Content>
</Popover.Root>
