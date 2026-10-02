<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import "./store-collaboration.css";
  let {
    label = "Loading collaboration activity",
    agents = false,
  }: { label?: string; agents?: boolean } = $props();
</script>

<div
  class="store-collaboration-skeleton"
  class:store-agent-session-list={agents}
  data-kind={agents ? "agent" : undefined}
  role="status"
  aria-label={label}
  aria-busy="true"
>
  {#each [0, 1, 2, 3, 4] as row}
    <div class="store-collaboration-skeleton__row" aria-hidden="true">
      <Skeleton class="store-collaboration-skeleton__avatar" />
      <div class="store-collaboration-skeleton__copy">
        <Skeleton class="store-collaboration-skeleton__title" />
        <Skeleton class="store-collaboration-skeleton__line" />
      </div>
    </div>
  {/each}
  <span class="store-collaboration-skeleton__label">{label}</span>
</div>

<style>
  .store-collaboration-skeleton {
    display: grid;
  }

  .store-collaboration-skeleton__row {
    display: flex;
    gap: 0.65rem;
    min-height: 4.5rem;
    padding: 0.75rem;
    border-bottom: 1px solid var(--border);
  }

  .store-collaboration-skeleton[data-kind="agent"] {
    gap: 4px;
  }

  .store-collaboration-skeleton[data-kind="agent"]
    .store-collaboration-skeleton__row {
    align-items: center;
    box-sizing: border-box;
    gap: 0.35rem;
    min-height: 5rem;
    padding: 0.7rem 0.65rem;
    border-bottom: 0;
  }

  :global(.store-collaboration-skeleton__avatar) {
    width: 2rem;
    height: 2rem;
    border-radius: var(--radius);
  }

  .store-collaboration-skeleton__copy {
    display: grid;
    flex: 1;
    align-content: center;
    gap: 0.5rem;
  }

  :global(.store-collaboration-skeleton__title) {
    width: 45%;
    height: 0.85rem;
  }

  :global(.store-collaboration-skeleton__line) {
    width: 76%;
    height: 0.7rem;
  }

  .store-collaboration-skeleton__label {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
  }
</style>
