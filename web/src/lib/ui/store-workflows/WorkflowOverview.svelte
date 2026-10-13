<script lang="ts">
  import Plus from "@lucide/svelte/icons/plus";
  import WorkflowCard from "./WorkflowCard.svelte";
  import WorkflowSkeleton from "./WorkflowSkeleton.svelte";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import type { WorkflowHistoryEntry } from "./workflow-history.svelte.js";
  let {
    entries,
    loading,
    available,
    writable,
    pending,
    repositoryName,
    canWrite,
    onOpen,
    onToggle,
    onRemove,
    onRun,
  }: {
    entries: WorkflowHistoryEntry[];
    loading: boolean;
    available: boolean;
    writable: boolean;
    pending: boolean;
    repositoryName(id: string): string;
    canWrite(home: string): boolean;
    onOpen(entry: WorkflowHistoryEntry | null, duplicate?: boolean): void;
    onToggle(entry: WorkflowHistoryEntry): void;
    onRemove(entry: WorkflowHistoryEntry): void;
    onRun(entry: WorkflowHistoryEntry): void;
  } = $props();
</script>

{#if !available}<StoreWorkEmpty
    title="Workflows unavailable"
    description="This connection does not support native workflow metadata."
  />
{:else}<div class="workflow-grid">
    {#if loading}<span class="sr-only" role="status">Loading workflows</span
      >{#each [1, 2, 3] as key (key)}<WorkflowSkeleton />{/each}
    {:else}{#each entries as entry (`${entry.home}:${entry.workflow.id}`)}<WorkflowCard
          workflow={entry.workflow}
          repositoryName={repositoryName(entry.workflow.definition.repository)}
          busy={entry.work.state.writing || !!entry.work.state.pending}
          writable={canWrite(entry.home)}
          onOpen={() => onOpen(entry)}
          onToggle={() => onToggle(entry)}
          onDuplicate={() => onOpen(entry, true)}
          onRemove={() => onRemove(entry)}
          onRun={() => onRun(entry)}
        />{/each}
      {#if writable}<button
          class="workflow-create-card"
          disabled={pending}
          onclick={() => onOpen(null)}
          ><span class="workflow-create-card__icon"
            ><Plus aria-hidden="true" /></span
          ><span>Create workflow</span></button
        >{/if}{/if}
  </div>{/if}
