<script lang="ts">
  import {
    FileListing,
    FileListingViewModeToggle,
    type FileListingFile,
    type FileListingViewMode,
  } from "@lapismd/design-core/diff";
  import {
    ScrollArea,
    type ScrollAreaStrategy,
  } from "@lapismd/design-core/shadcn/scroll-area";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import CommunityProjectsRepositorySkeleton from "./CommunityProjectsRepositorySkeleton.svelte";
  import type { ComponentProps } from "svelte";

  let {
    files,
    scrollStrategy = "auto",
    selectedPath = null,
    title = "Changed files",
    ariaLabel = title,
    compact = false,
    error = "",
    hasMore = false,
    initialLoading = false,
    loading = false,
    loadingLabel = "Loading changed files",
    onSelectPath,
    onLoadMore,
    fileDrag = {},
  }: {
    files: FileListingFile[];
    scrollStrategy?: ScrollAreaStrategy;
    selectedPath?: string | null;
    title?: string | undefined;
    ariaLabel?: string | undefined;
    compact?: boolean | undefined;
    error?: string | undefined;
    hasMore?: boolean | undefined;
    initialLoading?: boolean | undefined;
    loading?: boolean | undefined;
    loadingLabel?: string | undefined;
    onSelectPath: (path: string) => void;
    onLoadMore?: (() => void) | undefined;
    fileDrag?: Pick<
      ComponentProps<typeof FileListing>,
      | "fileDraggable"
      | "onFileDragStart"
      | "directoryDraggable"
      | "onDirectoryDragStart"
    >;
  } = $props();

  let mode = $state<FileListingViewMode>("tree");
</script>

{#snippet body()}
  {#if initialLoading}
    <CommunityProjectsRepositorySkeleton
      layout="changed-files"
      label={loadingLabel}
      count={4}
    />
  {:else}
    <FileListing
      {files}
      {mode}
      {selectedPath}
      virtualize={!(compact && scrollStrategy === "styled")}
      {hasMore}
      {loading}
      loadingLabel="Loading more changed files"
      {onSelectPath}
      {onLoadMore}
      {...fileDrag}
    />
  {/if}
{/snippet}

<section
  class="community-revision-graph-files"
  class:community-revision-graph-files--compact={compact}
  class:community-revision-graph-files--inline-styled={compact &&
    scrollStrategy === "styled"}
  data-ui-part="changed-files"
  aria-label={ariaLabel}
  aria-busy={initialLoading || loading}
>
  <div class="community-revision-graph-files__header">
    <span class="community-revision-graph-files__title">{title}</span>
    <span
      class="community-revision-graph-files__count"
      data-ui-part="changed-files-count"
    >
      {#if initialLoading}<Skeleton />{:else}{files.length}{/if}
    </span>

    <FileListingViewModeToggle bind:mode />
  </div>
  {#if error}
    <p class="community-revision-graph-files__status">{error}</p>
  {/if}
  <div
    class="community-revision-graph-files__body"
    class:community-revision-graph-files__body--styled={scrollStrategy ===
      "styled"}
  >
    {#if scrollStrategy === "styled" && !compact}<ScrollArea
        scrollStrategy="styled"
        class="community-revision-graph-files__scroll"
        >{@render body()}</ScrollArea
      >
    {:else}{@render body()}{/if}
  </div>
</section>

<style>
  .community-revision-graph-files {
    --ui-diff-row-hover: color-mix(
      in srgb,
      var(--foreground) 7%,
      var(--background)
    );
    --ui-diff-selected: color-mix(
      in srgb,
      var(--foreground) 12%,
      var(--background)
    );
    display: flex;
    min-width: 0;
    min-height: 0;
    flex: 1 1 auto;
    flex-direction: column;
  }

  .community-revision-graph-files__header {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    gap: 0.5rem;
    padding: 0.45rem 0.65rem;
    border-bottom: 1px solid color-mix(in srgb, currentColor 14%, transparent);
  }

  .community-revision-graph-files__title {
    min-width: 0;
    flex: 1 1 auto;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: var(--community-text-body);
    font-weight: 600;
  }

  .community-revision-graph-files__header
    :global(.community-revision-graph-files__review) {
    flex: 0 0 auto;
  }

  .community-revision-graph-files__count {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
    padding: 0.05rem 0.4rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--foreground) 7%, var(--background));
    color: var(--foreground);
    font-size: var(--community-text-secondary);
    font-weight: 600;
  }

  .community-revision-graph-files__count
    :global([data-ui-component="skeleton"]) {
    width: 1rem;
    height: 0.7rem;
  }

  .community-revision-graph-files__status {
    margin: 0;
    padding: 0.25rem 0.65rem 0;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  .community-revision-graph-files :global(.ui-diff-file-listing) {
    gap: 0.2rem;
  }

  .community-revision-graph-files :global(.ui-diff-file-listing__row) {
    color: var(--muted-foreground);
  }

  .community-revision-graph-files :global(.ui-diff-file-listing__row--selected),
  .community-revision-graph-files
    :global(.ui-diff-file-listing__row--selected:hover) {
    color: var(--foreground);
  }

  .community-revision-graph-files
    :global(.ui-diff-file-listing__row--selected .ui-diff-file-listing__icon) {
    border-radius: 4px;
    background: var(--background);
    box-shadow: 0 0 0 1px var(--border);
  }

  .community-revision-graph-files__body {
    min-height: 0;
    flex: 1 1 auto;
    padding: 0.25rem 0.35rem 0.35rem;
    overflow: auto;
  }

  .community-revision-graph-files__body--styled {
    overflow: hidden;
  }
  :global(.community-revision-graph-files__scroll) {
    height: 100%;
    min-height: 0;
  }

  .community-revision-graph-files--compact {
    flex: 0 1 auto;
    max-height: 16rem;
    overflow: hidden;
    border: 1px solid color-mix(in srgb, currentColor 14%, transparent);
    border-radius: 0.5rem;
    background: var(--background);
  }

  .community-revision-graph-files--compact
    .community-revision-graph-files__body {
    flex: 0 1 auto;
    max-height: 12rem;
  }

  /* Inline lists use their column's styled viewport and natural row height. */
  .community-revision-graph-files--inline-styled,
  .community-revision-graph-files--inline-styled
    .community-revision-graph-files__body {
    max-height: none;
    overflow: visible;
  }
</style>
