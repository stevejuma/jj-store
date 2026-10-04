<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as InputGroup from "@lapismd/design-core/shadcn/input-group";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import HistoryIcon from "@lucide/svelte/icons/history";
  import SearchIcon from "@lucide/svelte/icons/search";
  import XIcon from "@lucide/svelte/icons/x";
  import type {
    RepositoryLogDevice,
    RepositoryLogRevision,
  } from "../../projects/repository-log.js";
  import { buildRepositoryLogGraph } from "../../projects/repository-log-graph.js";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraphCopy from "../community-projects/CommunityProjectsRevisionGraphCopy.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  let {
    parents,
    revisions,
    devices = [],
    now,
    disabled = false,
    onSelect,
  }: {
    parents: readonly string[];
    revisions: readonly RepositoryLogRevision[];
    devices?: readonly RepositoryLogDevice[];
    now?: number;
    disabled?: boolean;
    onSelect(id: string): void;
  } = $props();
  const id = $props.id();
  const gridId = `${id}-parents`;
  let open = $state(false),
    query = $state(""),
    active = $state(0);
  let searchInput = $state<HTMLInputElement | null>(null);
  const rows = $derived(buildRepositoryLogGraph(revisions, devices));
  const options = $derived(
    parents.map((id) => {
      const row = rows.find((row) => row.revision.eventId === id);
      return {
        id,
        row,
        label: row?.revision.message || `Operation ${id.slice(0, 12)}`,
        keywords: [
          id,
          row?.revision.message,
          row?.revision.author,
          row?.revision.authorEmail,
        ]
          .filter(Boolean)
          .join(" ")
          .toLocaleLowerCase(),
      };
    }),
  );
  const visible = $derived(
    options.filter((option) =>
      option.keywords.includes(query.trim().toLocaleLowerCase()),
    ),
  );
  const selected = $derived(visible[Math.min(active, visible.length - 1)]);
  function handleOpen(next: boolean) {
    open = next;
    if (next) {
      query = "";
      active = 0;
    }
  }
  function choose(id: string) {
    open = false;
    onSelect(id);
  }
  function navigate(event: KeyboardEvent) {
    if (!visible.length || event.isComposing) return;
    const index = Math.min(active, visible.length - 1);
    if (event.key === "ArrowDown") active = (index + 1) % visible.length;
    else if (event.key === "ArrowUp")
      active = (index - 1 + visible.length) % visible.length;
    else if (event.key === "Home") active = 0;
    else if (event.key === "End") active = visible.length - 1;
    else if (event.key === "Enter") choose(visible[index]!.id);
    else return;
    event.preventDefault();
    if (event.key !== "Enter")
      document
        .getElementById(`${gridId}-${active}`)
        ?.scrollIntoView({ block: "nearest" });
  }
</script>

<Popover.Root {open} onOpenChange={handleOpen}>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        {disabled}
        variant="outline"
        size="sm"
        class="store-comparison-parent-picker__trigger"
        role="combobox"
        aria-label="Comparison parent"
        aria-expanded={open}
      >
        <HistoryIcon data-icon="inline-start" aria-hidden="true" /><span
          >Compare parent</span
        ><ChevronDownIcon data-icon="inline-end" aria-hidden="true" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    align="end"
    role="dialog"
    aria-label="Choose comparison parent"
    onOpenAutoFocus={(event) => {
      event.preventDefault();
      searchInput?.focus();
    }}
    class="store-comparison-parent-picker__command"
    style="--ui-popover-width: min(42rem, calc(100vw - 2rem)); --ui-popover-padding: 0.5rem 0.75rem 0.625rem; --ui-popover-gap: 0.5rem"
  >
    <Popover.Header class="store-comparison-parent-picker__header">
      <Popover.Title>Choose comparison parent</Popover.Title>
      <Popover.Close
        >{#snippet child({ props })}<Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="Close comparison parent selector"
            ><XIcon aria-hidden="true" /></Button
          >{/snippet}</Popover.Close
      >
    </Popover.Header>
    <InputGroup.Root>
      <InputGroup.Input
        bind:ref={searchInput}
        bind:value={query}
        oninput={() => {
          active = 0;
        }}
        onkeydown={navigate}
        role="combobox"
        aria-label="Search comparison parents"
        placeholder="Search comparison parents"
        aria-autocomplete="list"
        aria-expanded="true"
        aria-haspopup="grid"
        aria-controls={gridId}
        aria-activedescendant={selected
          ? `${gridId}-${visible.indexOf(selected)}`
          : undefined}
      />
      <InputGroup.Addon><SearchIcon aria-hidden="true" /></InputGroup.Addon>
    </InputGroup.Root>
    <div
      id={gridId}
      role="grid"
      aria-label="Recorded parents"
      class="store-comparison-parent-picker__results"
    >
      {#each visible as option, index (option.id)}
        <div
          role="row"
          id={`${gridId}-${index}`}
          aria-selected={selected?.id === option.id}
          class="store-comparison-parent-picker__option"
        >
          <span role="gridcell" class="store-comparison-parent-picker__card">
            <Button
              variant="ghost"
              class="store-comparison-parent-picker__select"
              aria-label={`Select comparison parent ${option.label}`}
              onclick={() => choose(option.id)}
            />
            {#if option.row}<CommunityProjectsRevisionGraphCopy
                row={option.row}
                {now}
              />
            {:else}<span class="store-comparison-parent-picker__fallback"
                ><CommunityProjectsGitAuthorAvatar
                  name="Unknown author"
                  size="small"
                /><span>{option.label}</span></span
              >{/if}
          </span>
          <span role="gridcell"
            ><CommunityProjectsRevisionGraphIds
              commitId={option.id}
              commitLabel="operation ID"
              placement="inline"
            /></span
          >
        </div>
      {:else}<div role="row">
          <span role="gridcell">No matching comparison parents.</span>
        </div>{/each}
    </div>
  </Popover.Content>
</Popover.Root>

<style>
  .store-comparison-parent-picker__results {
    display: grid;
    gap: 0.25rem;
    max-height: min(24rem, 54dvh);
    overflow: auto;
  }
  .store-comparison-parent-picker__option {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-height: 4.75rem;
    padding-inline: 0.5rem;
    border-radius: var(--radius-sm);
  }
  .store-comparison-parent-picker__option[aria-selected="true"],
  .store-comparison-parent-picker__option:hover {
    background: var(--accent);
  }
  .store-comparison-parent-picker__card {
    position: relative;
    display: flex;
    flex: 1;
    min-width: 0;
    align-self: stretch;
    align-items: center;
  }
  :global(.store-comparison-parent-picker__select[data-ui-component="button"]) {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }
  .store-comparison-parent-picker__card :global(.community-revision-graph-copy),
  .store-comparison-parent-picker__fallback {
    position: relative;
    pointer-events: none;
  }
  .store-comparison-parent-picker__card
    :global(.community-revision-graph-message__id) {
    pointer-events: auto;
  }
  .store-comparison-parent-picker__fallback {
    display: flex;
    align-items: center;
    min-width: 0;
    gap: 0.55rem;
  }
  :global(.store-comparison-parent-picker__trigger) {
    min-width: 0;
    justify-content: flex-start;
  }
  :global(.store-comparison-parent-picker__trigger > span) {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  :global(
    .store-comparison-parent-picker__header[data-ui-part="popover-header"]
  ) {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 0.25rem;
    min-height: 0;
    margin: 0;
    padding: 0;
  }
</style>
