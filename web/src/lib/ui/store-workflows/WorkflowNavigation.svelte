<script lang="ts">
  import StoreViewTabs from "../store-application/StoreViewTabs.svelte";
  import StoreToolbarSearch from "../StoreToolbarSearch.svelte";
  import WorkflowFilters from "./WorkflowFilters.svelte";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  import type { WorkflowHistoryEntry } from "./workflow-history.svelte.js";
  let {
    selection,
    homes,
    repositories,
    entries,
    onChange,
  }: {
    selection: WorkflowSelection;
    homes: readonly StoreRepositorySummary[];
    repositories: readonly StoreRepositorySummary[];
    entries: WorkflowHistoryEntry[];
    onChange(fields: WorkflowSelection): void;
  } = $props();
  let open = $state(false);
  const tab = $derived(selection.tab ?? "workflows");
</script>

<div class="community-projects__tabs workflow-navigation">
  <StoreViewTabs
    value={tab}
    sections={[
      { value: "workflows", label: "Workflows" },
      { value: "history", label: "History" },
    ]}
    label="Workflow views"
    hideTabs={open}
    onSelect={(tab) =>
      onChange({
        tab: tab as WorkflowSelection["tab"],
        state: undefined,
        workflow: undefined,
        home: undefined,
        run: undefined,
        at: undefined,
        attempt: undefined,
      })}
  >
    {#snippet leading()}<StoreToolbarSearch
        label={`Search ${tab}`}
        scope={`workflows:${tab}`}
        query={selection.q ?? ""}
        bind:open
        onQuery={(q) => onChange({ q: q || undefined })}
      />{/snippet}
    {#snippet trailing()}{#if tab === "history"}<WorkflowFilters
          {selection}
          {homes}
          {repositories}
          {entries}
          {onChange}
        />{/if}{/snippet}
  </StoreViewTabs>
</div>
