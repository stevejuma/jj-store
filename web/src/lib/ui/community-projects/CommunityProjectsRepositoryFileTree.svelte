<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    FileListing,
    FileListingViewModeToggle,
    type FileListingDirectory,
    type FileListingViewMode,
  } from "@lapismd/design-core/diff";
  import "./repository-file-tree-resize.css";
  import "./repository-file-tree.css";

  let {
    files,
    directories = [],
    mode = "tree",
    collapsedDirectoryPaths,
    selectedPath = "",
    hasMore = false,
    loading = false,
    onSelectPath,
    onLoadMore,
    onModeChange,
    onDirectoryToggle,
    empty,
  }: {
    files: readonly { path: string }[];
    directories?: readonly FileListingDirectory[];
    mode?: FileListingViewMode;
    collapsedDirectoryPaths?: ReadonlySet<string>;
    selectedPath?: string | undefined;
    hasMore?: boolean;
    loading?: boolean;
    onSelectPath: (path: string) => void;
    onLoadMore?: () => void;
    onModeChange?: (mode: FileListingViewMode) => void;
    onDirectoryToggle?: (path: string) => void;
    empty?: Snippet;
  } = $props();
</script>

<div class="community-projects__file-tree">
  <div class="community-projects__file-tree-actions">
    <span class="community-projects__file-tree-title">Files</span>
    <span class="community-projects__file-tree-count">{files.length}</span>
    <FileListingViewModeToggle {mode} {onModeChange} />
  </div>
  {#if !files.length && !directories.length && empty}
    {@render empty()}
  {:else}
    <FileListing
      files={[...files]}
      directories={[...directories]}
      {mode}
      {collapsedDirectoryPaths}
      selectedPath={selectedPath || null}
      virtualize
      {hasMore}
      {loading}
      loadingLabel="Loading more repository files"
      {onSelectPath}
      {onLoadMore}
      {onDirectoryToggle}
    >
      {#snippet fileMeta()}
        <span class="sr-only">Repository file</span>
      {/snippet}
    </FileListing>
  {/if}
</div>
