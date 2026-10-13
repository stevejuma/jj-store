<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import ListFilter from "@lucide/svelte/icons/list-filter";
  import X from "@lucide/svelte/icons/x";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import {
    workflowStates,
    type WorkflowSelection,
  } from "../../controller/workflows-route-codec.js";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
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
  const filtered = $derived(
    !!(
      selection.scope ||
      selection.source ||
      selection.workflow ||
      selection.state
    ),
  );
</script>

<Popover.Root bind:open>
  <Popover.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        type="button"
        variant={filtered ? "default" : "ghost"}
        size="icon-sm"
        aria-label="Filters for history"
        aria-pressed={filtered}><ListFilter aria-hidden="true" /></Button
      >{/snippet}</Popover.Trigger
  >
  <Popover.Content
    align="end"
    role="dialog"
    aria-label="History filters"
    style="--ui-popover-width: min(24rem, calc(100vw - 2rem)); max-height: min(80vh, 40rem); overflow-y: auto;"
  >
    <Popover.Header
      style="flex-direction: row; align-items: center; justify-content: space-between;"
      ><Popover.Title>History filters</Popover.Title><Popover.Close
        >{#snippet child({ props })}<Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="Close filters"><X aria-hidden="true" /></Button
          >{/snippet}</Popover.Close
      ></Popover.Header
    >
    <div class="store-work__picker">
      <StoreWorkField label="Metadata home" compact
        ><StoreRepositoryPicker
          label="Metadata home filter"
          repositories={homes}
          value={selection.scope ?? ""}
          controlled
          allowEmpty
          emptyLabel="All accessible homes"
          onChange={(scope) =>
            onChange({ scope: scope || undefined, workflow: undefined })}
        /></StoreWorkField
      >
      <StoreWorkField label="Workflow" compact
        ><StoreRepositoryPicker
          label="Workflow filter"
          repositories={entries.map((entry) => ({
            id: `${entry.home}:${entry.workflow.id}`,
            name: entry.workflow.definition.name,
            description: homes.find((home) => home.id === entry.home)?.name,
          }))}
          value={selection.workflow
            ? `${selection.scope}:${selection.workflow}`
            : ""}
          controlled
          allowEmpty
          emptyLabel="All workflows"
          searchLabel="Find a workflow"
          groupLabel="Workflows"
          emptyResult="No matching loaded workflows."
          unavailableLabel="Selected workflow"
          onChange={(value) => {
            const [scope, workflow] = value.split(":");
            onChange({ workflow, scope: value ? scope : selection.scope });
          }}
        /></StoreWorkField
      >
      <StoreWorkField label="Repository" compact
        ><StoreRepositoryPicker
          label="Source repository filter"
          {repositories}
          value={selection.source ?? ""}
          controlled
          allowEmpty
          emptyLabel="All repositories"
          onChange={(source) => onChange({ source: source || undefined })}
        /></StoreWorkField
      >
      <StoreWorkField label="Status" compact
        ><StoreRepositoryPicker
          label="Run status filter"
          repositories={workflowStates.map((id) => ({
            id,
            name: id.replaceAll("_", " "),
          }))}
          value={selection.state ?? ""}
          controlled
          allowEmpty
          emptyLabel="All statuses"
          searchLabel="Find a status"
          groupLabel="Statuses"
          emptyResult="No matching statuses."
          unavailableLabel="Selected status"
          onChange={(state) =>
            onChange({
              state: (state as WorkflowSelection["state"]) || undefined,
            })}
        /></StoreWorkField
      >
      {#if filtered}<Button
          variant="ghost"
          onclick={() =>
            onChange({
              scope: undefined,
              source: undefined,
              workflow: undefined,
              state: undefined,
            })}>Clear filters</Button
        >{/if}
    </div>
  </Popover.Content>
</Popover.Root>
