<script lang="ts">
  import "./store-search-command.css";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import FileCodeIcon from "@lucide/svelte/icons/file-code-2";
  import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
  import HistoryIcon from "@lucide/svelte/icons/history";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
  import SearchIcon from "@lucide/svelte/icons/search";
  import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
  import type { Component } from "svelte";
  import type {
    StoreSearchResult,
    StoreSearchResultKind,
    StoreSearchState,
  } from "../../controller/contracts.js";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";

  let {
    open = $bindable(false),
    query,
    search,
    now,
    onQueryChange,
    onSelect,
    onLoadMore,
  }: {
    open?: boolean;
    query: string;
    search: StoreSearchState;
    now: number;
    onQueryChange: (query: string) => void;
    onSelect: (result: StoreSearchResult) => void;
    onLoadMore: () => void;
  } = $props();

  let selectedValue = $state("");
  let searchInput = $state<HTMLInputElement | null>(null);
  const groups: readonly {
    kind: StoreSearchResultKind;
    heading: string;
  }[] = [
    { kind: "operation", heading: "Operations" },
    { kind: "commit", heading: "Commits" },
    { kind: "file", heading: "Files" },
  ];

  function iconFor(kind: StoreSearchResultKind): Component {
    if (kind === "operation") return HistoryIcon;
    if (kind === "commit") return GitCommitIcon;
    return FileCodeIcon;
  }

  function items(kind: StoreSearchResultKind) {
    return search.results.filter((result) => result.kind === kind);
  }
</script>

{#snippet resultItem(result: StoreSearchResult)}
  {@const Icon = iconFor(result.kind)}
  <CommandView.Item
    class="store-search-command__result"
    value={result.id}
    keywords={[result.title, result.description, result.author]}
    onSelect={() => onSelect(result)}
  >
    <CommandView.ItemIcon><Icon aria-hidden="true" /></CommandView.ItemIcon>
    <CommandView.ItemLabel>{result.title}</CommandView.ItemLabel>
    <CommandView.ItemDescription
      >{result.description}</CommandView.ItemDescription
    >
    <span data-ui-component="command-view" data-ui-part="trailing">
      <CommunityRelativeTime timestamp={result.timestamp} {now} />
    </span>
    {#if result.hasConflict}<span class="store-search-command__conflict"
        >Conflict</span
      >{/if}
  </CommandView.Item>
{/snippet}

<Dialog.Root bind:open>
  <Dialog.Content
    class="store-search-dialog"
    showCloseButton={false}
    aria-describedby={undefined}
  >
    <Dialog.Title class="sr-only">Search repository</Dialog.Title>
    <CommandView.Root
      class="store-search-command"
      label="Search repository"
      bind:value={selectedValue}
      shouldFilter={false}
    >
      <CommandView.Input
        bind:ref={searchInput}
        value={query}
        placeholder="Search commits, operations, and files"
        aria-label="Search repository"
        aria-controls="store-search-command-results"
        oninput={(event) => onQueryChange(event.currentTarget.value)}
      >
        {#snippet start()}<SearchIcon aria-hidden="true" />{/snippet}
      </CommandView.Input>
      <Dialog.Close>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="outline"
            size="sm"
            class="store-search-command__escape"
            aria-label="Close search"
            title="Close search">Esc</Button
          >
        {/snippet}
      </Dialog.Close>
      <CommandView.List id="store-search-command-results">
        {#if !search.available}
          <CommandView.Empty class="store-search-command__state">
            <TriangleAlertIcon aria-hidden="true" />
            <span role="status"
              >Search is not configured for this repository.</span
            >
          </CommandView.Empty>
        {:else if !query.trim()}
          <CommandView.Empty class="store-search-command__state">
            <SearchIcon aria-hidden="true" />
            <span role="status">Search the selected repository snapshot.</span>
          </CommandView.Empty>
        {:else if search.status === "loading"}
          <CommandView.Empty class="store-search-command__state">
            <LoaderCircleIcon
              class="store-search-command__spinner"
              aria-hidden="true"
            />
            <span role="status">Searching repository…</span>
          </CommandView.Empty>
        {:else if search.status === "failed" || search.status === "unavailable"}
          <CommandView.Empty class="store-search-command__state">
            <TriangleAlertIcon aria-hidden="true" />
            <span role="alert"
              >{search.error || "Repository search is unavailable."}</span
            >
          </CommandView.Empty>
        {:else if search.status === "empty"}
          <CommandView.Empty>No matching repository content</CommandView.Empty>
        {:else}
          {#each groups as group (group.kind)}
            {@const results = items(group.kind)}
            {#if results.length}<CommandView.Group heading={group.heading}>
                {#each results as result (result.id)}
                  {@render resultItem(result)}
                {/each}
              </CommandView.Group>{/if}
          {/each}
        {/if}
      </CommandView.List>
      {#if search.canLoadMore || search.isStale || search.degraded || search.truncated}
        <CommandView.Footer>
          {#if search.canLoadMore}<Button
              variant="ghost"
              size="sm"
              disabled={search.status === "refreshing"}
              onclick={onLoadMore}
              >{search.status === "refreshing"
                ? "Loading more…"
                : "Load more results"}</Button
            >{/if}
          {#if search.isStale}<span>Indexing newer history</span>{/if}
          {#if search.degraded}<span>Search index degraded</span>{/if}
          {#if search.truncated}<span>Result limit reached</span>{/if}
        </CommandView.Footer>
      {/if}
    </CommandView.Root>
  </Dialog.Content>
</Dialog.Root>
