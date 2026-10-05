<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
</script>

<div
  class="store-conflict-skeleton"
  role="status"
  aria-label="Loading conflict file"
  aria-busy="true"
>
  <span class="sr-only">Loading conflict file</span>
  <div aria-hidden="true">
    <div class="store-conflict-skeleton__header">
      <Skeleton style="width: 9rem; height: 0.875rem" />
      <Skeleton style="width: 5rem; height: 0.875rem" />
    </div>
    <div class="store-conflict-skeleton__panes">
      {#each [0, 1, 2] as pane (pane)}
        <div class="store-conflict-skeleton__pane">
          <Skeleton
            style="width: 50%; height: 0.875rem; margin-bottom: 0.5rem"
          />
          {#each [74, 88, 64, 80, 48, 88, 72, 54, 80, 64] as width, row (row)}
            <div class="store-conflict-skeleton__line">
              <Skeleton style="width: 0.875rem; height: 0.75rem" />
              <Skeleton style={`width: ${width}%; height: 0.75rem`} />
            </div>
          {/each}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .store-conflict-skeleton {
    width: 100%;
    min-height: 20rem;
    overflow: hidden;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--background);
  }
  .store-conflict-skeleton__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.625rem;
    border-bottom: 1px solid var(--border);
  }
  .store-conflict-skeleton__panes {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
  .store-conflict-skeleton__pane {
    display: grid;
    min-width: 0;
    gap: 0.75rem;
    padding: 0.75rem;
    border-inline-end: 1px solid var(--border);
  }
  .store-conflict-skeleton__pane:last-child {
    border-inline-end: 0;
  }
  .store-conflict-skeleton__line {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>
