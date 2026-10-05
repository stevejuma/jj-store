<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityProjectsRepositoryLogFiles from "../community-projects/CommunityProjectsRepositoryLogFiles.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { workspaceFileDrag } from "../../controller/stack-workspace/file-drag.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let {
    controller,
    source,
    ariaLabel,
    selectedPath = null,
    readOnly = false,
    onSelectPath,
  }: {
    controller: StackWorkspaceController;
    source: string;
    ariaLabel: string;
    selectedPath?: string | null;
    readOnly?: boolean;
    onSelectPath: (path: string) => void;
  } = $props();
  const state = $derived(controller.details.fileState(source));
  $effect(() => {
    const selected = source;
    controller.snapshot?.revision;
    controller.snapshot?.readKey;
    untrack(() => {
      void controller.details.loadFiles(selected);
    });
    return () => {
      controller.details.cancelFiles(selected);
    };
  });
</script>

<StoreNotice
  noticeKey={`stack-files:${source}`}
  title="Changed files unavailable"
  message={state.error}
  lifetime="condition"
/>
{#if state.error}<Button
    size="sm"
    variant="outline"
    disabled={state.loading}
    onclick={() => void controller.details.loadFiles(source)}
    >Retry changed files</Button
  >{/if}
<CommunityProjectsRepositoryLogFiles
  title="Changed files"
  {ariaLabel}
  compact
  scrollStrategy="styled"
  files={controller.filesFor(source).map((file) => ({
    path: file.path,
    additions: file.hunks.length,
    deletions: file.hunks.length,
  }))}
  {selectedPath}
  {onSelectPath}
  loading={state.loading}
  initialLoading={controller.details.lazy && !state.loaded && !state.error}
  hasMore={Boolean(state.nextCursor) && !state.error}
  onLoadMore={() => void controller.details.loadFiles(source, true)}
  fileDrag={readOnly ? {} : workspaceFileDrag(controller, () => source)}
/>
