<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import "./repository-files.css";
  let {
    path,
    isMarkdown = false,
    decorative = false,
  }: {
    path: string;
    isMarkdown?: boolean;
    decorative?: boolean;
  } = $props();
</script>

<div
  class="community-projects__file-loading"
  role={decorative ? undefined : "status"}
  aria-label={decorative ? undefined : `Loading ${path}`}
  aria-busy={decorative ? undefined : "true"}
>
  <div class="community-projects__file-loading-content" aria-hidden="true">
    <div class="community-projects__file-panel-info">
      <Skeleton class="file-loading__toggle" />
      <Skeleton class="file-loading__avatar" />
      <Skeleton class="file-loading__summary" />
      <Skeleton class="file-loading__revision" />
    </div>
    <div class="community-projects__file-panel-stats">
      <Skeleton class="file-loading__stats" />
    </div>
    <section class="community-projects__file-content-card">
      <header class="community-projects__file-content-header">
        {#if path}<span>{path}</span>{:else}<Skeleton
            class="file-loading__filename"
          />{/if}
        <div class="file-loading__controls">
          {#each isMarkdown ? [0, 1, 2, 3] : [0, 1, 2] as action (action)}<Skeleton
            />{/each}
        </div>
      </header>
      {#if isMarkdown}<div class="file-loading__markdown">
          <Skeleton class="file-loading__title" />
          <Skeleton class="file-loading__copy" />
          <Skeleton class="file-loading__copy" />
          <Skeleton class="file-loading__heading" />
          <Skeleton class="file-loading__copy" />
          <Skeleton class="file-loading__code" />
        </div>
      {:else}<div class="file-loading__source">
          {#each [0, 1, 2, 3, 4, 5, 6] as row (row)}<div>
              <Skeleton /><Skeleton />
            </div>{/each}
        </div>{/if}
    </section>
  </div>
</div>

<style>
  .community-projects__file-loading,
  .community-projects__file-loading-content {
    min-width: 0;
    width: 100%;
  }
  .community-projects__file-loading-content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  :global(.file-loading__toggle) {
    width: 2rem;
    height: 2rem;
    flex: none;
  }
  :global(.file-loading__avatar) {
    width: 1.75rem;
    height: 1.75rem;
    flex: none;
    border-radius: 50%;
  }
  :global(.file-loading__summary) {
    width: min(18rem, 55%);
    height: 0.8rem;
    flex: 1;
    min-width: 0;
  }
  :global(.file-loading__revision) {
    width: 5.5rem;
    height: 1.4rem;
    flex: none;
  }
  :global(.file-loading__stats) {
    width: min(14rem, 100%);
    height: 1rem;
  }
  :global(.file-loading__filename) {
    width: min(8rem, 30%);
    height: 0.75rem;
  }
  .file-loading__controls {
    display: flex;
    flex: none;
    gap: 0.125rem;
  }
  .file-loading__controls :global([data-ui-component="skeleton"]) {
    width: 2.75rem;
    height: 1.75rem;
  }
  .file-loading__markdown {
    display: grid;
    min-height: 28rem;
    box-sizing: border-box;
    align-content: start;
    gap: 0.8rem;
    padding: clamp(1.5rem, 4vw, 3rem);
    max-width: 58rem;
    margin-inline: auto;
    width: 100%;
  }
  .file-loading__markdown > :global(.file-loading__title) {
    width: 70%;
    height: 1.75rem;
  }
  .file-loading__markdown > :global(.file-loading__heading) {
    width: 50%;
    height: 1.25rem;
  }
  .file-loading__markdown > :global(.file-loading__copy) {
    width: 100%;
    height: 0.875rem;
  }
  .file-loading__markdown > :global(.file-loading__code) {
    width: 100%;
    height: 5rem;
  }
  .file-loading__source {
    display: grid;
    min-height: 18rem;
    box-sizing: border-box;
    align-content: start;
    gap: 0.65rem;
    padding: 1rem 0.75rem;
  }
  .file-loading__source > div {
    display: grid;
    grid-template-columns: 1.5rem minmax(0, 1fr);
    gap: 0.8rem;
  }
  .file-loading__source > div > :global(*:first-child) {
    width: 1rem;
    height: 0.7rem;
  }
  .file-loading__source > div > :global(*:last-child) {
    width: min(42rem, 82%);
    height: 0.7rem;
  }
  .file-loading__source > div:nth-child(3n) > :global(*:last-child) {
    width: min(30rem, 58%);
  }
  @media (max-width: 36rem) {
    .file-loading__controls :global([data-ui-component="skeleton"]) {
      width: 1.75rem;
    }
  }
</style>
