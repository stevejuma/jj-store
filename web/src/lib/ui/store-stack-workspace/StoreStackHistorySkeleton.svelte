<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import RepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  let {
    label,
    count = 4,
    decorative = false,
  }: { label: string; count?: number; decorative?: boolean } = $props();
</script>

<div
  class="stack-history-skeleton"
  data-stack-history-skeleton
  role={decorative ? undefined : "status"}
  aria-label={decorative ? undefined : label}
  aria-busy={decorative ? undefined : "true"}
>
  <div aria-hidden="true" inert>
    <div class="stack-history-skeleton__heading">
      <Skeleton /><Skeleton />
      <div><Skeleton /><Skeleton /></div>
    </div>
    <div class="stack-history-skeleton__actions"><Skeleton /><Skeleton /></div>
    <RepositorySkeleton layout="graph" {label} {count} />
  </div>
</div>

<style>
  .stack-history-skeleton {
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }
  .stack-history-skeleton__heading {
    display: grid;
    gap: 0.65rem;
    padding: 1rem 0.75rem 1rem 2rem;
  }
  .stack-history-skeleton__heading > :global([data-ui-component="skeleton"]) {
    width: 65%;
    height: 0.8rem;
  }
  .stack-history-skeleton__heading > div,
  .stack-history-skeleton__actions {
    display: flex;
    gap: 0.5rem;
  }
  .stack-history-skeleton__heading
    > div
    :global([data-ui-component="skeleton"]) {
    width: 6rem;
    height: 1.25rem;
  }
  .stack-history-skeleton__actions {
    --ui-skeleton-background: color-mix(
      in srgb,
      var(--foreground) 12%,
      var(--background)
    );
    padding: 0.5rem;
    border-block: 1px solid var(--border);
    background: var(--muted);
  }
  .stack-history-skeleton__actions :global([data-ui-component="skeleton"]) {
    width: 5rem;
    height: 1.5rem;
  }
  @media (prefers-reduced-motion: reduce) {
    .stack-history-skeleton :global([data-ui-component="skeleton"]) {
      animation: none;
    }
  }
</style>
