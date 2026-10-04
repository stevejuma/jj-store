<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  let { label }: { label: string } = $props();
</script>

<div role="status" aria-label={label} aria-busy="true">
  <dl class="store-settings__rows" aria-hidden="true">
    {#each [0, 1] as row (row)}
      <div>
        <dt><Skeleton class="store-capture-loading__label" /></dt>
        <dd class="store-settings__stack">
          <Skeleton class="store-capture-loading__value" />
          <Skeleton class="store-capture-loading__detail" />
        </dd>
      </div>
    {/each}
  </dl>
</div>

<style>
  :global(.store-capture-loading__label),
  :global(.store-capture-loading__value),
  :global(.store-capture-loading__detail) {
    height: 1rem;
    max-width: 100%;
  }
  :global(.store-capture-loading__label) {
    width: 10rem;
  }
  :global(.store-capture-loading__value) {
    width: 16rem;
  }
  :global(.store-capture-loading__detail) {
    width: 12rem;
  }
</style>
