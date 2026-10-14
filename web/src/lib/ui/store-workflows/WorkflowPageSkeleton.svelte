<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import WorkflowSkeleton from "./WorkflowSkeleton.svelte";
  import WorkflowNavigation from "./WorkflowNavigation.svelte";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  import "./workflow-console.css";
  import "./workflow-history-list.css";
  let {
    view = "workflows",
    listOnly = false,
    selection,
    onChange = () => {},
    sourceRepository,
  }: {
    view?: "workflows" | "history" | "detail";
    listOnly?: boolean;
    selection?: WorkflowSelection;
    sourceRepository?: string;
    onChange?: (fields: WorkflowSelection) => void;
  } = $props();
</script>

<div
  class="workflow-page-skeleton"
  data-view={view}
  aria-busy="true"
  aria-label={view === "detail" ? "Loading run details" : `Loading ${view}`}
>
  <span class="sr-only" role="status"
    >{view === "detail" ? "Loading run details" : `Loading ${view}`}</span
  >
  {#if view === "workflows"}<div class="workflow-grid">
      {#each [1, 2, 3] as key (key)}<WorkflowSkeleton />{/each}
    </div>
  {:else if view === "detail"}<div
      class="community-projects__task-detail-header"
      aria-hidden="true"
    >
      <Skeleton class="size-8 rounded-full" />
      <div class="workflow-page-skeleton__title">
        <Skeleton class="h-6 w-48" /><Skeleton class="h-3 w-32" />
      </div>
      <div class="workflow-run-actions">
        <Skeleton class="size-8 rounded-full" /><Skeleton
          class="size-8 rounded-full"
        /><Skeleton class="size-8 rounded-full" />
      </div>
    </div>
    <div class="community-projects__task-body" aria-hidden="true">
      <div class="community-projects__task-meta">
        {#each [1, 2, 3, 4, 5, 6, 7] as key (key)}<div>
            <Skeleton class="h-4 w-20" /><Skeleton
              class="h-5 w-48 max-w-full"
            />
          </div>{/each}
      </div>
      <div class="workflow-console workflow-page-skeleton__console">
        <div class="workflow-console-header">
          <Skeleton class="h-5 w-40" /><Skeleton class="h-8 w-44" /><Skeleton
            class="size-8"
          />
        </div>
        <div class="workflow-console-body">
          {#each [1, 2, 3] as key (key)}<div
              class="workflow-page-skeleton__step"
            >
              <Skeleton class="size-4 rounded-full" /><Skeleton
                class="h-4 w-40"
              /><Skeleton class="h-3 w-8" />
            </div>{/each}
        </div>
      </div>
    </div>
  {:else}<div
      class="workflow-history-skeleton"
      data-list-only={listOnly || undefined}
    >
      {#if !listOnly}<div class="workflow-history-skeleton__sidebar">
          {#if selection && !sourceRepository}<WorkflowNavigation
              {selection}
              {onChange}
            />{/if}
          <Skeleton class="h-5 w-24" /><Skeleton class="h-8 w-full" />
          {#each [1, 2] as group (group)}<Skeleton class="h-8 w-full" />
            <div class="workflow-history-skeleton__children">
              {#each [1, 2] as item (item)}<Skeleton
                  class="h-8 w-full"
                />{/each}
            </div>
          {/each}
        </div>{/if}
      <div class="workflow-history-skeleton__list">
        {#if !listOnly}<div class="workflow-history-heading">
            <Skeleton class="size-8" /><Skeleton class="h-5 w-36" />
          </div>
          <div class="workflow-history-card__header">
            <Skeleton class="h-5 w-16" /><Skeleton
              class="h-8 w-80 max-w-full"
            />
          </div>{/if}
        {#each [1, 2, 3] as key (key)}<div class="workflow-history-row">
            <Skeleton class="size-4 rounded-full" /><Skeleton
              class="size-7 rounded-full"
            />
            <div class="workflow-history-row__summary">
              <Skeleton class="h-5 w-40 max-w-full" /><Skeleton
                class="h-4 w-64 max-w-full"
              />
            </div>
            <Skeleton class="h-5 w-16 workflow-history-row__source" />
            <div class="workflow-history-row__time">
              <Skeleton class="h-3 w-20" /><Skeleton class="h-3 w-12" />
            </div>
            <Skeleton class="size-6 workflow-history-row__menu" />
          </div>{/each}
      </div>
    </div>{/if}
</div>

<style>
  .workflow-page-skeleton {
    min-width: 0;
  }
  .workflow-page-skeleton__title {
    display: grid;
    gap: 0.5rem;
  }
  .workflow-page-skeleton__step {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 10px;
    padding: 5px 10px;
  }
  .workflow-page-skeleton__step > :global(:last-child) {
    margin-left: auto;
  }
  .workflow-history-skeleton {
    display: grid;
    grid-template-columns: 16rem minmax(0, 1fr);
  }
  .workflow-history-skeleton[data-list-only] {
    grid-template-columns: minmax(0, 1fr);
  }
  .workflow-history-skeleton__sidebar {
    display: grid;
    gap: 0.5rem;
    padding: 1rem;
    border-right: 1px solid var(--border);
    align-content: start;
  }
  .workflow-history-skeleton__children {
    display: grid;
    gap: 0.25rem;
    padding-left: 1rem;
  }
  .workflow-history-skeleton__list {
    min-width: 0;
    padding: 0.5rem 1.25rem;
  }
  .workflow-history-skeleton[data-list-only] .workflow-history-skeleton__list {
    padding: 0;
  }
  @media (max-width: 48rem) {
    .workflow-history-skeleton {
      grid-template-columns: minmax(0, 1fr);
    }
    .workflow-history-skeleton__sidebar {
      display: none;
    }
  }
</style>
