<script lang="ts">
  import {
    FileListing,
    FileListingViewModeToggle,
    type FileListingViewMode,
  } from "@lapismd/design-core/diff";
  import "./repository-file-tree-resize.css";

  let {
    files,
    selectedPath = "",
    hasMore = false,
    loading = false,
    onSelectPath,
    onLoadMore,
  }: {
    files: readonly { path: string }[];
    selectedPath?: string | undefined;
    hasMore?: boolean;
    loading?: boolean;
    onSelectPath: (path: string) => void;
    onLoadMore?: () => void;
  } = $props();

  let mode = $state<FileListingViewMode>("tree");
</script>

<div class="community-projects__file-tree">
  <div class="community-projects__file-tree-actions">
    <span class="community-projects__file-tree-title">Files</span>
    <span class="community-projects__file-tree-count">{files.length}</span>
    <FileListingViewModeToggle bind:mode />
  </div>
  <FileListing
    files={[...files]}
    {mode}
    selectedPath={selectedPath || null}
    virtualize
    {hasMore}
    {loading}
    loadingLabel="Loading more repository files"
    {onSelectPath}
    {onLoadMore}
  >
    {#snippet fileMeta()}
      <span class="sr-only">Repository file</span>
    {/snippet}
  </FileListing>
</div>
