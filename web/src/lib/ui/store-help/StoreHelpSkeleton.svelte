<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
</script>

<div
  class="store-help-skeleton"
  role="status"
  aria-label="Loading documentation"
  aria-busy="true"
>
  <span class="sr-only">Loading documentation</span>
  <div class="store-help-skeleton__chrome" aria-hidden="true">
    <Skeleton class="store-help-skeleton__brand" />
    <Skeleton class="store-help-skeleton__search" />
  </div>
  <div class="store-help-skeleton__columns" aria-hidden="true">
    <div class="store-help-skeleton__navigation">
      {#each Array(10) as _, index (index)}
        <Skeleton class="store-help-skeleton__nav-row" />
      {/each}
    </div>
    <div class="store-help-skeleton__content">
      <div class="store-help-skeleton__article">
        <Skeleton class="store-help-skeleton__heading" />
        {#each Array(4) as _, index (index)}
          <div class="store-help-skeleton__paragraph">
            <Skeleton class="store-help-skeleton__line" />
            <Skeleton class="store-help-skeleton__line" />
            <Skeleton class="store-help-skeleton__short" />
          </div>
        {/each}
      </div>
    </div>
    <div class="store-help-skeleton__contents">
      {#each Array(6) as _, index (index)}
        <Skeleton class="store-help-skeleton__nav-row" />
      {/each}
    </div>
  </div>
</div>

<style>
  .store-help-skeleton {
    height: 100%;
    overflow: hidden;
    background: var(--store-theme-gradient, var(--sidebar));
  }
  .store-help-skeleton__chrome {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 2.5rem;
    padding: 0.25rem 0.75rem;
    background: var(--sidebar);
  }
  :global(.store-help-skeleton__brand) {
    width: 10rem;
    height: 1rem;
  }
  :global(.store-help-skeleton__search) {
    width: 14rem;
    height: 2rem;
  }
  .store-help-skeleton__columns {
    display: flex;
    gap: 0.75rem;
    padding: 0.5rem;
    height: calc(100% - 2.5rem);
  }
  .store-help-skeleton__navigation,
  .store-help-skeleton__contents {
    display: grid;
    align-content: start;
    gap: 1rem;
    padding: 0.75rem 0.5rem;
    flex: 0 0 18rem;
  }
  .store-help-skeleton__contents {
    flex-basis: 14rem;
    padding-inline: 0.25rem 0;
  }
  :global(.store-help-skeleton__nav-row) {
    height: 0.875rem;
    width: 70%;
  }
  .store-help-skeleton__content {
    flex: 1;
    min-width: 0;
    border: 1px solid color-mix(in srgb, var(--foreground) 11%, transparent);
    border-radius: 1rem;
    background: var(--background);
  }
  .store-help-skeleton__article {
    max-width: 88ch;
    margin: 0 auto;
    padding: 1.5rem 2rem;
  }
  :global(.store-help-skeleton__heading) {
    height: 1.75rem;
    width: 65%;
    margin-bottom: 2rem;
  }
  .store-help-skeleton__paragraph {
    display: grid;
    gap: 0.625rem;
    margin-bottom: 2rem;
  }
  :global(.store-help-skeleton__line) {
    height: 0.875rem;
    width: 100%;
  }
  :global(.store-help-skeleton__short) {
    height: 0.875rem;
    width: 70%;
  }
  @media (max-width: 76.2344em) {
    .store-help-skeleton__navigation {
      display: none;
    }
  }
  @media (max-width: 59.9844em) {
    .store-help-skeleton__contents {
      display: none;
    }
    :global(.store-help-skeleton__search) {
      width: 6rem;
    }
    .store-help-skeleton__article {
      padding: 1rem;
    }
  }
</style>
