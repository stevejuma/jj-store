<script lang="ts">
  import StoreStackDropGuide from "./StoreStackDropGuide.svelte";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import {
    FileListing,
    FileListingViewModeToggle,
    type FileListingViewMode,
  } from "@lapismd/design-core/diff";
  import GitBranchPlus from "@lucide/svelte/icons/git-branch-plus";
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
  const workspace = $derived(activeWorkspace(snapshot));
  const files = $derived(
    workspace.unassigned.map((file) => ({
      path: file.path,
      additions: file.hunks.length,
      deletions: file.hunks.length,
    })),
  );
</script>

<ColumnCanvas.Column
  id="working"
  columnTitle="Unstaged"
  sticky
  resizable
  aria-label="Unstaged files"
>
  <ColumnCanvas.Header class="stack-workspace__unstaged-header">
    <ColumnCanvas.Title>Unstaged</ColumnCanvas.Title><ColumnCanvas.Count
      >{files.length}</ColumnCanvas.Count
    >
    <ColumnCanvas.HeaderActions>
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
    <FileListing
      {files}
      {mode}
      virtualize
      selectedPath={controller.diffs.find(
        (diff) => diff.origin === "working" && diff.source === "unassigned",
      )?.path ?? null}
      onSelectPath={(path) => controller.selectFile("unassigned", path)}
      {...workspaceFileDrag(controller, () => "unassigned")}
    />
  </ColumnCanvas.Body>
</ColumnCanvas.Column>
