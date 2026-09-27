<script lang="ts">
  import {
    FileListing,
    FileListingViewModeToggle,
    type FileListingViewMode,
  } from "@lapismd/design-core/diff";

  let {
    files,
    selectedPath = "",
    onSelectPath,
  }: {
    files: readonly { path: string }[];
    selectedPath?: string | undefined;
    onSelectPath: (path: string) => void;
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
    {onSelectPath}
  >
    {#snippet fileMeta()}
      <span class="sr-only">Repository file</span>
    {/snippet}
  </FileListing>
</div>
