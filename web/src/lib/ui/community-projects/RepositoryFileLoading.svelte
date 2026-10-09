<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  let { path, isMarkdown = false }: { path: string; isMarkdown?: boolean } =
    $props();
</script>

<div
  class="community-projects__blob-skeleton"
  role="status"
  aria-label={`Loading ${path}`}
>
  {#if isMarkdown}
    <Skeleton class="community-projects__markdown-skeleton-title" />
    <Skeleton class="community-projects__markdown-skeleton-copy" />
    <Skeleton class="community-projects__markdown-skeleton-copy" />
    <Skeleton class="community-projects__markdown-skeleton-heading" />
    <Skeleton class="community-projects__markdown-skeleton-copy" />
    <Skeleton class="community-projects__markdown-skeleton-code" />
  {:else}
    <div class="community-projects__files-loading-code">
      {#each [0, 1, 2, 3, 4, 5, 6] as row (row)}
        <div>
          <Skeleton />
          <Skeleton />
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .community-projects__blob-skeleton
    > :global(.community-projects__markdown-skeleton-title) {
    width: 70%;
    height: 1.75rem;
  }
  .community-projects__blob-skeleton
    > :global(.community-projects__markdown-skeleton-heading) {
    width: 50%;
    height: 1.25rem;
  }
  .community-projects__blob-skeleton
    > :global(.community-projects__markdown-skeleton-copy) {
    width: 100%;
    height: 0.875rem;
  }
  .community-projects__blob-skeleton
    > :global(.community-projects__markdown-skeleton-code) {
    width: 100%;
    height: 5rem;
  }
  .community-projects__files-loading-code {
    display: grid;
    min-height: 24rem;
    align-content: start;
    gap: 0.65rem;
    padding: 1rem 0.75rem;
  }

  .community-projects__files-loading-code > div {
    display: grid;
    grid-template-columns: 1.5rem minmax(0, 1fr);
    gap: 0.8rem;
  }

  .community-projects__files-loading-code > div > :global(*:first-child) {
    width: 1rem;
    height: 0.7rem;
  }

  .community-projects__files-loading-code > div > :global(*:last-child) {
    width: min(42rem, 82%);
    height: 0.7rem;
  }

  .community-projects__files-loading-code
    > div:nth-child(3n)
    > :global(*:last-child) {
    width: min(30rem, 58%);
  }

  .community-projects__blob-skeleton {
    display: grid;
    width: 100%;
    min-height: 28rem;
    box-sizing: border-box;
    align-content: start;
    gap: 0.8rem;
    padding: clamp(1.5rem, 4vw, 3rem);
  }
</style>
