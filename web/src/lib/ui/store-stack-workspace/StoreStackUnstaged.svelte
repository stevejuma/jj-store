<script lang="ts">
  import { untrack } from "svelte";
  import StoreStackDropGuide from "./StoreStackDropGuide.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import CommunityProjectsRepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import {
    FileListing,
    FileChangeStats,
    FileListingViewModeToggle,
    type FileListingViewMode,
  } from "@lapismd/design-core/diff";
  import GitBranchPlus from "@lucide/svelte/icons/git-branch-plus";
  import TriangleAlert from "@lucide/svelte/icons/triangle-alert";
  import { activeWorkspace } from "../../controller/stack-workspace/presentation.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { workspaceFileDrag } from "../../controller/stack-workspace/file-drag.js";
  let {
    controller,
    onDrop,
    onChangesDrop,
  }: {
    controller: StackWorkspaceController;
    onDrop: (event: DragEvent) => void;
    onChangesDrop: (event: DragEvent) => void;
  } = $props();
  let mode = $state<FileListingViewMode>("tree");
  const snapshot = $derived(controller.snapshot!);
  const readState = $derived(controller.details.fileState("unassigned"));
  $effect(() => {
    snapshot.revision;
    snapshot.readKey;
    snapshot.active;
    untrack(() => {
      void controller.details.loadFiles("unassigned");
    });
    return () => controller.details.cancelFiles("unassigned");
  });
  const changes = $derived(
    controller.details.lazy
      ? controller.filesFor("unassigned")
      : activeWorkspace(snapshot).unassigned,
  );
  const conflictedPaths = $derived(
    new Set([
      ...snapshot.conflicts.map((conflict) => conflict.path),
      ...changes
        .filter((file) =>
          file.change
            ? file.change.after.length > 1
            : file.content?.status === "unavailable" &&
              file.content.reason === "conflict",
        )
        .map((file) => file.path),
    ]),
  );
  const files = $derived(
    changes.map((file) => ({
      path: file.path,
      additions: file.hunks.length,
      deletions: file.hunks.length,
    })),
  );
</script>

<ColumnCanvas.Column
  id="working"
  columnTitle="Unstaged"
  fixed
  resizable
  aria-label="Unstaged files"
>
  <ColumnCanvas.Header class="stack-workspace__unstaged-header">
    <ColumnCanvas.Title>Unstaged</ColumnCanvas.Title><ColumnCanvas.Count
      >{files.length}</ColumnCanvas.Count
    >
    <ColumnCanvas.HeaderActions>
      {#if readState.error}<Button
          size="sm"
          variant="outline"
          onclick={() => void controller.details.loadFiles("unassigned")}
          >Retry unstaged files</Button
        >{/if}
      <FileListingViewModeToggle bind:mode />
      <Tooltip.Provider
        ><Tooltip.Root>
          <Tooltip.Trigger>
            {#snippet child({ props })}
              <Button
                {...props}
                variant="ghost"
                size="icon-sm"
                aria-label="Browse stacks to apply"
                disabled={controller.locked}
                ondragover={(event) => event.preventDefault()}
                ondrop={onDrop}
                onclick={() => (controller.page = "branches")}
                ><GitBranchPlus aria-hidden="true" /></Button
              >
            {/snippet}
          </Tooltip.Trigger>
          <Tooltip.Content>Browse stacks to apply</Tooltip.Content>
        </Tooltip.Root></Tooltip.Provider
      >
      <ColumnCanvas.Toggle aria-label="Collapse Unstaged" />
    </ColumnCanvas.HeaderActions>
  </ColumnCanvas.Header>
  <ColumnCanvas.Body
    scrollStrategy="styled"
    class="stack-workspace__unstaged-body stack-workspace__drop-target"
    data-drop-target-state={controller.drag.state({ kind: "working" })}
    ondragover={(event) =>
      controller.drag.over(
        event,
        { kind: "working" },
        snapshot,
        controller.locked,
      )}
    ondragleave={(event) => controller.drag.leave(event, { kind: "working" })}
    ondrop={onChangesDrop}
  >
    <StoreStackDropGuide feedback={controller.drag.for({ kind: "working" })} />
    <StoreNotice
      noticeKey="stack-files:unassigned"
      title="Unstaged files unavailable"
      message={readState.error}
      lifetime="condition"
    />
    {#if controller.details.lazy && !readState.loaded && !readState.error}
      <CommunityProjectsRepositorySkeleton
        layout="changed-files"
        label="Loading unstaged files"
        count={4}
      />
    {:else}
      <FileListing
        {files}
        {mode}
        loading={readState.loading}
        hasMore={Boolean(readState.nextCursor) && !readState.error}
        onLoadMore={() => void controller.details.loadFiles("unassigned", true)}
        virtualize
        selectedPath={controller.diffs.find(
          (diff) => diff.origin === "working" && diff.source === "unassigned",
        )?.path ?? null}
        onSelectPath={(path) => controller.selectFile("unassigned", path)}
        {...workspaceFileDrag(controller, () => "unassigned")}
      >
        {#snippet fileMeta({ file, path })}
          {#if conflictedPaths.has(path)}
            <span
              data-workspace-conflict
              role="img"
              aria-label="Conflicted file"
              title="Unresolved conflict"
            >
              <TriangleAlert size={14} aria-hidden="true" />
            </span>
          {:else}
            <FileChangeStats
              additions={file.additions}
              deletions={file.deletions}
            />
          {/if}
        {/snippet}
      </FileListing>
    {/if}
  </ColumnCanvas.Body>
</ColumnCanvas.Column>

<style>
  :global(
    .stack-workspace__unstaged-body
      [data-ui-part="file-row"]:has([data-workspace-conflict])
  ) {
    background: var(--jj-store-warning-background);
    color: var(--foreground);
  }
  :global(
    .stack-workspace__unstaged-body
      [data-ui-part="file-row"][aria-current="true"]:has(
        [data-workspace-conflict]
      )
  ) {
    box-shadow: inset 2px 0 var(--jj-store-warning-foreground);
  }
  [data-workspace-conflict] {
    display: inline-flex;
    margin-inline-start: auto;
    color: var(--jj-store-warning-foreground);
  }
</style>
