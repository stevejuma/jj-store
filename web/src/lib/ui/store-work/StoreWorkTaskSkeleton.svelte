<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import "./community-work-tasks.css";
  import "./task-detail-layout.css";
</script>

<div
  class="community-projects__task-detail task-skeleton"
  role="status"
  aria-label="Loading task details"
  aria-busy="true"
  data-layout="task-detail"
>
  <header class="community-projects__task-detail-header" aria-hidden="true">
    <Skeleton class="task-skeleton__title" /><Skeleton
      class="task-skeleton__subtitle"
    />
  </header>
  <div class="task-skeleton__body" aria-hidden="true">
    <div class="task-skeleton__metadata">
      {#each [0, 1, 2, 3] as row}<div><Skeleton /><Skeleton /></div>{/each}
    </div>
    <Skeleton class="task-skeleton__section" /><Skeleton
      class="task-skeleton__line"
    /><Skeleton class="task-skeleton__line" />
    <CommunitySkeleton
      layout="messages"
      count={2}
      label="Loading task comments"
    />
  </div>
</div>

<style>
  .task-skeleton {
    width: 100%;
  }
  .task-skeleton header {
    display: grid;
    gap: 0.4rem;
    padding-inline-start: 2.75rem;
  }
  :global(.task-skeleton__title) {
    width: 65%;
    height: 1.4rem;
  }
  :global(.task-skeleton__subtitle) {
    width: 35%;
    height: 0.85rem;
  }
  .task-skeleton__body {
    display: grid;
    align-content: start;
    gap: 1rem;
    padding: 1rem 1.5rem;
    overflow: hidden;
  }
  .task-skeleton__metadata {
    display: grid;
    gap: 1px;
    margin-bottom: 1rem;
  }
  .task-skeleton__metadata > div {
    display: grid;
    grid-template-columns: 30% 1fr;
    gap: 1rem;
    padding: 0.6rem 0;
  }
  .task-skeleton__metadata :global([data-ui-component="skeleton"]) {
    height: 1rem;
  }
  :global(.task-skeleton__section) {
    width: 7rem;
    height: 1rem;
  }
  :global(.task-skeleton__line) {
    width: 80%;
    height: 0.85rem;
  }
</style>
