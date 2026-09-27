<script lang="ts">
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import "./repository-skeleton.css";

  let {
    layout,
    label,
    count = 6,
    showStatus = false,
  }: {
    layout:
      | "bookmarks"
      | "graph"
      | "file-tree"
      | "directory"
      | "source"
      | "changed-files"
      | "diff";
    label: string;
    count?: number;
    showStatus?: boolean;
  } = $props();

  const rows = $derived(Array.from({ length: Math.max(1, count) }));
</script>

<div
  class="community-projects-repository-skeleton"
  data-layout={layout}
  role="status"
  aria-label={label}
  aria-busy="true"
>
  <span class="sr-only">{label}</span>
  <div
    class="community-projects-repository-skeleton__content"
    aria-hidden="true"
  >
    {#if layout === "bookmarks"}
      <div class="community-projects-repository-skeleton__section-heading">
        <Skeleton />
      </div>
      {#each rows as _, index (index)}
        <div class="community-projects-repository-skeleton__bookmark-row">
          <div class="community-projects-repository-skeleton__copy">
            <Skeleton class="community-projects-repository-skeleton__title" />
            <Skeleton class="community-projects-repository-skeleton__line" />
            <div class="community-projects-repository-skeleton__chips">
              <Skeleton /><Skeleton />
            </div>
          </div>
          <div class="community-projects-repository-skeleton__identity">
            <Skeleton class="community-projects-repository-skeleton__avatar" />
            <Skeleton class="community-projects-repository-skeleton__meta" />
          </div>
        </div>
      {/each}
    {:else if layout === "graph"}
      {#if showStatus}
        <div class="community-projects-repository-skeleton__status">
          <Skeleton />
        </div>
      {/if}
      {#each rows as _, index (index)}
        <div class="community-projects-repository-skeleton__graph-row">
          <div class="community-projects-repository-skeleton__graph-lane">
            <Skeleton />
          </div>
          <Skeleton class="community-projects-repository-skeleton__avatar" />
          <div class="community-projects-repository-skeleton__copy">
            <Skeleton class="community-projects-repository-skeleton__title" />
            <Skeleton class="community-projects-repository-skeleton__meta" />
          </div>
          <Skeleton class="community-projects-repository-skeleton__id" />
        </div>
      {/each}
    {:else if layout === "file-tree" || layout === "changed-files"}
      <div class="community-projects-repository-skeleton__tree-heading">
        <Skeleton class="community-projects-repository-skeleton__title" />
        <Skeleton class="community-projects-repository-skeleton__count" />
        <div class="community-projects-repository-skeleton__tree-actions">
          <Skeleton /><Skeleton /><Skeleton />
        </div>
      </div>
      {#each rows as _, index (index)}
        <div
          class="community-projects-repository-skeleton__tree-row"
          style={`--repository-skeleton-depth: ${index % 3}`}
        >
          <Skeleton class="community-projects-repository-skeleton__file-icon" />
          <Skeleton class="community-projects-repository-skeleton__file-name" />
        </div>
      {/each}
    {:else if layout === "directory"}
      <div class="community-projects-repository-skeleton__directory-summary">
        <Skeleton class="community-projects-repository-skeleton__avatar" />
        <Skeleton
          class="community-projects-repository-skeleton__summary-copy"
        />
        <Skeleton class="community-projects-repository-skeleton__selector" />
      </div>
      <div class="community-projects-repository-skeleton__directory-head">
        <Skeleton /><Skeleton /><Skeleton />
      </div>
      {#each rows as _, index (index)}
        <div class="community-projects-repository-skeleton__directory-row">
          <Skeleton class="community-projects-repository-skeleton__file-icon" />
          <Skeleton class="community-projects-repository-skeleton__file-name" />
          <Skeleton
            class="community-projects-repository-skeleton__description"
          />
          <Skeleton class="community-projects-repository-skeleton__updated" />
        </div>
      {/each}
    {:else}
      {#if layout === "source"}
        <div class="community-projects-repository-skeleton__source-meta">
          <Skeleton class="community-projects-repository-skeleton__avatar" />
          <Skeleton
            class="community-projects-repository-skeleton__summary-copy"
          />
          <Skeleton class="community-projects-repository-skeleton__id" />
        </div>
        <div class="community-projects-repository-skeleton__source-stats">
          <Skeleton />
        </div>
      {/if}
      <div class="community-projects-repository-skeleton__code-card">
        <div class="community-projects-repository-skeleton__code-head">
          <Skeleton /><Skeleton />
        </div>
        <div class="community-projects-repository-skeleton__code-lines">
          {#each rows as _, index (index)}
            <div>
              <Skeleton
                class="community-projects-repository-skeleton__line-number"
              />
              <Skeleton
                class={[
                  "community-projects-repository-skeleton__code-line",
                  index % 3 === 2
                    ? "community-projects-repository-skeleton__code-line-short"
                    : "",
                ]
                  .filter(Boolean)
                  .join(" ")}
              />
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>
