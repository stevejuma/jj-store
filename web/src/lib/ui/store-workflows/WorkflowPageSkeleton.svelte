<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import WorkflowSkeleton from "./WorkflowSkeleton.svelte";
  let { view = "workflows" }: { view?: "workflows" | "history" | "detail" } =
    $props();
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
      <div class="workflow-page-skeleton__sections">
        {#each [1, 2, 3] as key (key)}<Skeleton class="h-4 w-24" />{/each}
      </div>
    </div>
  {:else}<div class="workflow-history-loading" aria-hidden="true">
      <Skeleton class="h-6 w-32" /><Skeleton
        class="h-4 w-16"
      />{#each [1, 2, 3] as key (key)}<div
          class="community-projects__task-row workflow-history-row"
        >
          <Skeleton class="size-4 rounded-full" /><Skeleton
            class="h-4 w-3/4"
          /><Skeleton
            class="h-3 w-24 community-projects__task-repository"
          /><Skeleton class="size-6 rounded-full" /><Skeleton
            class="h-3 w-16"
          /><Skeleton class="h-3 w-20 workflow-page-skeleton__time" />
        </div>{/each}
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
  .workflow-page-skeleton__sections {
    display: grid;
    gap: 1.5rem;
    padding-block: 1rem;
  }
  @media (max-width: 48rem) {
    .workflow-page-skeleton :global(.workflow-page-skeleton__time) {
      display: none;
    }
  }
</style>
