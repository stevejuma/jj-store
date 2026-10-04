<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreStackDiffHunk from "./StoreStackDiffHunk.svelte";
  import CommunityDiffControls from "../community-diff-controls/CommunityDiffControls.svelte";
  import {
    workspaceSourceFiles,
    workspaceAggregateSource,
    workspaceHunkSelection,
  } from "../../controller/stack-workspace/files.js";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import Columns from "@lucide/svelte/icons/columns-2";
  import { TARGET_HISTORY_ORIGIN } from "../../controller/stack-workspace/target-history.svelte.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type {
    WorkspaceCommand,
    WorkspaceOpenDiff,
  } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    diff,
    onCommand,
  }: {
    controller: StackWorkspaceController;
    diff: WorkspaceOpenDiff;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
  let viewMode = $state<"unified" | "split">("unified"),
    wrap = $state(false);
  const file = $derived(
    (diff.origin === TARGET_HISTORY_ORIGIN
      ? (controller.targetHistory.commits.find(
          (commit) => commit.id === diff.source,
        )?.files ?? [])
      : controller.snapshot
        ? workspaceSourceFiles(controller.snapshot, diff.source)
        : []
    ).find((item) => item.path === diff.path),
  );
  const selection = $derived(
    diff.origin === TARGET_HISTORY_ORIGIN ||
      workspaceAggregateSource(diff.source)
      ? null
      : {
          source: diff.source,
          path: diff.path,
          ...(diff.hunks ? { hunks: diff.hunks } : {}),
        },
  );
  function choose(hunk: string, checked: boolean) {
    if (!file) return;
    const current = diff.hunks ?? file.hunks.map((item) => item.id);
    const hunks = checked
      ? [...new Set([...current, hunk])]
      : current.filter((value) => value !== hunk);
    controller.selectFile(diff.source, file.path, hunks);
  }
  const hasSelection = $derived(
    Boolean(
      selection && (selection.hunks === undefined || selection.hunks.length),
    ),
  );
</script>

<ColumnCanvas.Column
  resizable
  id={`diff-${diff.origin}`}
  aria-label={`Selected changes from ${diff.origin}`}
>
  <ColumnCanvas.Header
    ><ColumnCanvas.Title>{file?.path ?? "Diff"}</ColumnCanvas.Title
    ><ColumnCanvas.HeaderActions>
      {#if file}
        <CommunityDiffControls bind:viewMode bind:wrap />
        {#if selection && diff.source !== "unassigned"}
          <Tooltip.Provider
            ><Tooltip.Root
              ><Tooltip.Trigger>
                {#snippet child({ props })}
                  <Button
                    {...props}
                    variant="ghost"
                    size="icon-sm"
                    aria-label="Split selected changes"
                    disabled={controller.locked || !hasSelection}
                    onclick={() =>
                      selection &&
                      onCommand({
                        kind: "split",
                        commit: diff.source,
                        message: "",
                        selection,
                      })}><Columns aria-hidden="true" /></Button
                  >
                {/snippet}
              </Tooltip.Trigger><Tooltip.Content
                >Split selected changes</Tooltip.Content
              ></Tooltip.Root
            ></Tooltip.Provider
          >
        {/if}
      {/if}
      <ColumnCanvas.Close
        aria-label="Close Diff column"
        onclick={() => {
          if (diff.origin === TARGET_HISTORY_ORIGIN)
            controller.targetHistory.diff = null;
          else controller.closeDiff(diff.origin);
        }}
      /></ColumnCanvas.HeaderActions
    ></ColumnCanvas.Header
  >
  <ColumnCanvas.Body scrollStrategy="styled">
    {#if file}
      <section
        class="stack-workspace__hunks"
        aria-label={`Diff hunks from ${diff.origin} · ${file.path}`}
      >
        {#each file.hunks as hunk (hunk.id)}
          <StoreStackDiffHunk
            {hunk}
            drag={controller.drag}
            origin={diff.origin}
            path={file.path}
            selection={diff.origin === TARGET_HISTORY_ORIGIN
              ? null
              : workspaceHunkSelection(
                  controller.snapshot!,
                  diff.source,
                  file.path,
                  hunk.id,
                )}
            checked={!selection?.hunks || selection.hunks.includes(hunk.id)}
            selectable={Boolean(selection)}
            disabled={controller.locked}
            {viewMode}
            {wrap}
            onChecked={(checked) => choose(hunk.id, checked)}
            onAssign={(selection) =>
              onCommand({ kind: "transfer", selection, target: "" })}
          />
        {/each}
      </section>
    {/if}
  </ColumnCanvas.Body>
</ColumnCanvas.Column>
