<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import XIcon from "@lucide/svelte/icons/x";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import type { RepositoryChoice } from "../../controller/contracts.js";
  let {
    branch,
    branches = [],
    onSelectBranch,
  }: {
    branch: string;
    branches?: readonly RepositoryChoice[];
    onSelectBranch: (value: string | undefined) => void;
  } = $props();
  let open = $state(false),
    tab = $state("bookmark"),
    query = $state("");
  const label = "Repository bookmark or change";
  const heading = "Switch snapshot";
  const visibleRefs = $derived(
    branches.filter((choice) => choice.kind === tab),
  );
  const selected = $derived(branches.find((choice) => choice.value === branch));
  function handleOpen(next: boolean) {
    open = next;
    if (next) {
      query = "";
      tab = selected?.kind ?? "bookmark";
    }
  }
  function choose(value: string) {
    open = false;
    onSelectBranch(value);
  }
</script>

<Popover.Root {open} onOpenChange={handleOpen}>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        variant="outline"
        class={["community-projects__file-branch-trigger", props.class]}
        role="combobox"
        aria-label={label}
      >
        <GitBranchIcon aria-hidden="true" />
        <span
          >{(selected?.kind === "change"
            ? selected.label.split(" · ").slice(0, 2).join(" · ")
            : selected?.label) || "Select snapshot"}</span
        >
        <ChevronDownIcon
          class="community-repository-branch-command__chevron"
          aria-hidden="true"
        />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    class="community-repository-branch-command"
    align="start"
    style="--ui-popover-width: 22rem; --ui-popover-padding: 0.5rem 0.75rem 0.625rem; --ui-popover-gap: 0.25rem"
  >
    <Popover.Header class="community-repository-branch-command__header">
      <Popover.Title
        class="community-repository-branch-command__title"
        role="heading"
        aria-level={2}>{heading}</Popover.Title
      >
      <Popover.Close>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="Close snapshot switcher"
          >
            <XIcon />
            <span class="sr-only">Close</span>
          </Button>
        {/snippet}
      </Popover.Close>
    </Popover.Header>
    <CommandView.Root label={heading}>
      <CommandView.Input
        bind:value={query}
        placeholder={"Find bookmark or change"}
        aria-label={"Find bookmark or change"}
      />
      {#if true}
        <CommandView.Filters
          bind:value={tab}
          label="Snapshot kinds"
          tabs={[
            { id: "bookmark", label: "Bookmarks" },
            { id: "change", label: "Changes" },
          ]}
        />
      {/if}
      <CommandView.List>
        <CommandView.Empty>No matching snapshots</CommandView.Empty>
        <CommandView.Group
          heading={tab === "bookmark" ? "Bookmarks" : "Changes"}
        >
          {#each visibleRefs as option (option.value)}
            <CommandView.Item
              value={option.value}
              keywords={[option.label, ...(option.searchTerms ?? [])]}
              onSelect={() => choose(option.value)}
            >
              <CommandView.ItemIcon>
                {#if option.value === branch}
                  <CheckIcon aria-hidden="true" />
                {/if}
              </CommandView.ItemIcon>
              <CommandView.ItemLabel>{option.label}</CommandView.ItemLabel>
              {#if option.kind === "bookmark" && option.targets.length !== 1}
                <span data-ui-component="command-view" data-ui-part="trailing">
                  <CommunityChip variant="muted" size="compact" ariaHidden
                    >{option.targets.length
                      ? `${option.targets.length} targets`
                      : "deleted"}</CommunityChip
                  >
                </span>
              {/if}
            </CommandView.Item>
          {/each}
        </CommandView.Group>
      </CommandView.List>
    </CommandView.Root>
  </Popover.Content>
</Popover.Root>

<style>
  :global(
    .community-repository-branch-command__header[data-ui-part="popover-header"]
  ) {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 0.25rem;
    min-height: 0;
    padding: 0;
    margin: 0;
  }

  :global(
    .community-repository-branch-command__title[data-ui-part="popover-title"]
  ) {
    margin: 0;
    font-weight: var(--font-weight-bold, 700);
    line-height: 1.2;
  }

  :global(
    .community-repository-branch-command[data-ui-part="popover-content"]
  ) {
    --ui-command-view-list-max-height: min(18rem, 50dvh);
    --ui-command-view-list-padding: 0.25rem;
    --ui-command-view-gap: 0.25rem;
  }

  :global(.community-repository-branch-command [data-ui-part="item-icon"]),
  :global(.community-repository-branch-command [data-ui-part="item-icon"] svg) {
    width: 0.75rem;
    height: 0.75rem;
  }

  :global(.community-projects__file-branch-trigger > span) {
    flex: 1;
    text-align: start;
  }

  :global(.community-repository-branch-command__chevron) {
    margin-inline-start: auto;
    flex-shrink: 0;
  }

  :global(
    .community-repository-branch-command[data-ui-part="popover-content"]
      [data-ui-component="command-view"][data-ui-part="root"]
  ) {
    overflow: visible;
  }

  :global(
    .community-repository-branch-command[data-ui-part="popover-content"]
      [data-ui-component="command-view"][data-ui-part="input-wrapper"]
  ) {
    padding: 0.25rem 0.25rem 0.5rem;
  }

  :global(
    .community-repository-branch-command[data-ui-part="popover-content"]
      [data-ui-component="command-view"][data-ui-part="input-wrapper"]
      > [data-ui-component="input-group"][data-ui-part="input-group"]
  ) {
    border: 1px solid
      var(
        --ui-input-group-border-color,
        var(--ui-input-border-color, var(--input))
      );
    border-radius: 999px;
    background: var(--background);
    box-shadow: none;
  }

  :global(
    .community-repository-branch-command[data-ui-part="popover-content"]
      [data-ui-component="command-view"][data-ui-part="input-wrapper"]
      > [data-ui-component="input-group"][data-ui-part="input-group"]:focus-within
  ) {
    border-color: transparent;
    outline: 3px solid color-mix(in srgb, var(--ring) 50%, transparent);
    outline-offset: 0;
    box-shadow: none;
  }
</style>
