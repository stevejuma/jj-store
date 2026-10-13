<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ChevronRight from "@lucide/svelte/icons/chevron-right";
  import { sectionDisclosure } from "../disclosure-layout.svelte.js";
  import "../store-work/community-work-disclosures.css";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import WorkflowRunEvidence from "./WorkflowRunEvidence.svelte";
  import WorkflowRunStatus from "./WorkflowRunStatus.svelte";
  import type {
    WorkflowPage,
    WorkflowRun,
  } from "../../controller/workflow-types.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  let {
    run,
    page,
    selection,
    loading,
    onChange,
    onMore,
  }: {
    run: WorkflowRun;
    page: WorkflowPage;
    selection: WorkflowSelection;
    loading: boolean;
    onChange(fields: WorkflowSelection): void;
    onMore(): void;
  } = $props();
  const attempts = sectionDisclosure(
    () => ["workflow", selection.home ?? "", run.id, "attempts"],
    false,
  );
  const steps = sectionDisclosure(
    () => ["workflow", selection.home ?? "", run.id, "steps"],
    false,
  );
  const outputs = sectionDisclosure(
    () => ["workflow", selection.home ?? "", run.id, "outputs"],
    false,
  );
  const stepRows = $derived(
    page.evidence.filter((row) => row.data.type === "step"),
  );
  const outputRows = $derived(
    page.evidence.filter((row) => row.data.type !== "step"),
  );
</script>

{#snippet heading(label: string, open: boolean)}
  <span>{label}</span>{#if open}<ChevronDown
      class="community-projects__task-section-chevron"
      aria-hidden="true"
    />{:else}<ChevronRight
      class="community-projects__task-section-chevron"
      aria-hidden="true"
    />{/if}
{/snippet}
<section aria-label="Attempts">
  <Collapsible.Root
    class="community-projects__task-activity"
    bind:open={attempts.open}
  >
    <Collapsible.Trigger class="community-projects__task-section-trigger"
      >{@render heading("Attempts", attempts.open)}</Collapsible.Trigger
    >
    <Collapsible.Content>
      <StoreRepositoryPicker
        label="Run attempt"
        repositories={run.attempts.map((row, index) => ({
          id: row.id,
          name: `Attempt ${index + 1} · ${row.status.replaceAll("_", " ")}`,
          description: row.host ?? "Unassigned",
        }))}
        value={selection.attempt ?? ""}
        controlled
        allowEmpty
        emptyLabel="All attempts"
        searchLabel="Find an attempt"
        groupLabel="Attempts"
        emptyResult="No matching attempts."
        unavailableLabel="Selected attempt"
        onChange={(attempt) =>
          onChange({
            attempt: attempt || undefined,
            at: String(page.position),
          })}
      />
      <ol class="workflow-run-attempts" aria-label="Recorded attempts">
        {#each run.attempts as row, index (row.id)}<li
            data-selected={row.id === selection.attempt || undefined}
          >
            <span title={row.id}>Attempt {index + 1}</span><WorkflowRunStatus
              status={row.status}
              label
            /><span>{row.host ?? "Unassigned"}</span>{#if row.result}<p>
                {row.result}
              </p>{/if}
          </li>{/each}
      </ol>
      {#if !run.attempts.length}<StoreWorkEmpty
          kind="activity"
          title="No attempts recorded"
          description="Execution attempts will appear here once the run is scheduled."
        />{/if}
    </Collapsible.Content>
  </Collapsible.Root>
</section>
<section aria-label="Steps">
  <Collapsible.Root
    class="community-projects__task-activity"
    bind:open={steps.open}
  >
    <Collapsible.Trigger class="community-projects__task-section-trigger"
      >{@render heading("Steps", steps.open)}</Collapsible.Trigger
    >
    <Collapsible.Content
      ><WorkflowRunEvidence
        rows={stepRows}
      />{#if !stepRows.length}<StoreWorkEmpty
          kind="tasks"
          title="No steps recorded"
          description="Step status updates will appear here as execution progresses."
        />{/if}</Collapsible.Content
    >
  </Collapsible.Root>
</section>
<section aria-label="Outputs">
  <Collapsible.Root
    class="community-projects__task-activity"
    bind:open={outputs.open}
  >
    <Collapsible.Trigger class="community-projects__task-section-trigger"
      >{@render heading("Outputs", outputs.open)}</Collapsible.Trigger
    >
    <Collapsible.Content
      ><WorkflowRunEvidence
        rows={outputRows}
      />{#if !outputRows.length}<StoreWorkEmpty
          kind="activity"
          title="No outputs recorded"
          description="Logs, artifacts and agent transcripts will appear here when reported."
        />{/if}</Collapsible.Content
    >
  </Collapsible.Root>
</section>
{#if page.next}<Button variant="outline" disabled={loading} onclick={onMore}
    >More evidence</Button
  >{/if}
