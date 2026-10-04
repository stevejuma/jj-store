<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunitySearchCommand from "../community-search-command/CommunitySearchCommand.svelte";
  import type {
    CommunitySearchAction,
    CommunitySearchRow,
  } from "../community-search-command/model.js";
  import type {
    StoreSearchState,
    StoreSearchResult,
  } from "../../controller/contracts.js";
  let {
    open = $bindable(false),
    query,
    search,
    now,
    onQueryChange,
    onSelect,
    onLoadMore,
    onOpenChangeComplete,
    rows,
    status,
    errors,
    scopeLabel,
    activeScopeLabel,
    onScope,
    onClearScope,
    actions,
    sections,
    onSelectRow,
    canLoadMore,
  }: {
    open?: boolean;
    query: string;
    search: StoreSearchState;
    now: number;
    onQueryChange: (query: string) => void;
    onSelect: (result: StoreSearchResult) => void;
    onLoadMore: () => void;
    onOpenChangeComplete?: (open: boolean) => void;
    rows?: readonly CommunitySearchRow[];
    status?: string;
    errors?: readonly string[];
    scopeLabel?: string;
    activeScopeLabel?: string;
    onScope?: () => void;
    onClearScope?: () => void;
    actions?: readonly CommunitySearchAction[];
    sections?: readonly CommunitySearchAction[];
    onSelectRow?: (id: string) => void;
    canLoadMore?: boolean;
  } = $props();
</script>

<CommunitySearchCommand
  bind:open
  {query}
  results={rows ?? search.results}
  status={status ?? search.status}
  errors={errors ??
    (search.error
      ? [search.error]
      : !search.available
        ? ["Search is not configured for this repository."]
        : [])}
  {now}
  {onQueryChange}
  {onOpenChangeComplete}
  {scopeLabel}
  {activeScopeLabel}
  {onScope}
  {onClearScope}
  {actions}
  {sections}
  onSelect={(id) => {
    if (onSelectRow) onSelectRow(id);
    else {
      const result = search.results.find((r) => r.id === id);
      if (result) onSelect(result);
    }
  }}
>
  {#snippet footer()}{#if canLoadMore ?? search.canLoadMore}<Button
        variant="ghost"
        size="sm"
        disabled={(status ?? search.status) === "refreshing"}
        onclick={onLoadMore}>Load more results</Button
      >{/if}{#if search.isStale}<span>Indexing newer history</span
      >{/if}{#if search.degraded}<span>Search index degraded</span
      >{/if}{#if search.truncated}<span>Result limit reached</span
      >{/if}{/snippet}
</CommunitySearchCommand>
