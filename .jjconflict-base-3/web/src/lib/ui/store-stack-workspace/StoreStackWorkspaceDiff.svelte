<script lang="ts">
  import { untrack } from "svelte";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreStackDiffHunk from "./StoreStackDiffHunk.svelte";
  import CommunityDiffControls from "../community-diff-controls/CommunityDiffControls.svelte";
  import CommunityProjectsRepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import {
    workspaceAggregateSource,
    workspaceHunkSelection,
  } from "../../controller/stack-workspace/files.js";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import Trash from "@lucide/svelte/icons/trash-2";
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
    controller.filesFor(diff.source).find((item) => item.path === diff.path),
  );
  const patchRead = $derived(
    controller.details.patchState(diff.source, diff.path),
  );
  const filesState = $derived(controller.details.fileState(diff.source));
  const readScope = $derived(
    JSON.stringify([
      controller.snapshot?.revision,
      controller.snapshot?.readKey,
    ]),
  );
  $effect(() => {
    const source = diff.source;
    readScope;
    untrack(() => {
      void controller.details.loadFiles(source);
    });
  });
  const patchIdentity = $derived(JSON.stringify([diff.source, diff.path]));
  $effect(() => {
    const [source, path] = JSON.parse(patchIdentity) as [string, string];
    return () => untrack(() => controller.details.cancelDiff(source, path));
  });
  $effect(() => {
    if (file?.content?.status === "unloaded") {
      const source = diff.source,
        path = diff.path;
      untrack(() => {
        void controller.details.loadDiff(source, path);
      });
    }
  });
  const executable = $derived(
    controller.can("transfer") ||
      controller.can("split") ||
      controller.can("create_commit"),
  );
  const selection = $derived(
    diff.origin === TARGET_HISTORY_ORIGIN ||
      workspaceAggregateSource(diff.source) ||
      !executable
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
    controller.selectFile(
      diff.source,
      file.path,
      hunks,
      diff.origin.startsWith("stack-") ? diff.origin.slice(6) : undefined,
    );
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
        {#if selection && diff.source === "unassigned" && controller.snapshot?.capabilities.includes("discard")}
          <Tooltip.Provider
            ><Tooltip.Root
              ><Tooltip.Trigger
                >{#snippet child({ props })}<Button
                    {...props}
                    variant="ghost"
                    size="icon-sm"
                    aria-label="Discard selected changes"
                    disabled={!controller.can("discard") || !hasSelection}
                    onclick={() =>
                      selection &&
                      onCommand({ kind: "discard", selections: [selection] })}
                    ><Trash aria-hidden="true" /></Button
                  >{/snippet}</Tooltip.Trigger
              ><Tooltip.Content>Discard selected changes</Tooltip.Content
              ></Tooltip.Root
            ></Tooltip.Provider
          >
        {/if}
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
                    disabled={!controller.can("split") || !hasSelection}
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
        onclick={() => controller.closeDiff(diff.origin)}
      /></ColumnCanvas.HeaderActions
    ></ColumnCanvas.Header
  >
  <ColumnCanvas.Body scrollStrategy="styled">
    <StoreNotice
      noticeKey={`stack-files:${diff.source}`}
      title="Changed files unavailable"
      message={filesState.error}
      lifetime="condition"
    />
    <StoreNotice
      noticeKey={`stack-patch:${diff.source}:${diff.path}`}
      title="Diff unavailable"
      message={patchRead.error}
      lifetime="condition"
    />
    {#if patchRead.error}<Button
        size="sm"
        variant="outline"
        onclick={() => void controller.details.loadDiff(diff.source, diff.path)}
        >Retry diff</Button
      >{/if}
    {#if controller.details.lazy && !file}
      {#if filesState.error}<Button
          size="sm"
          variant="outline"
          onclick={() => void controller.details.loadFiles(diff.source)}
          >Retry changed files</Button
        >
      {:else}<CommunityProjectsRepositorySkeleton
          layout="diff"
          label={`Loading diff for ${diff.path}`}
          count={3}
        />{/if}
    {:else if file?.content?.status === "unloaded" && !patchRead.error}
      <CommunityProjectsRepositorySkeleton
        layout="diff"
        label={`Loading diff for ${diff.path}`}
        count={3}
      />
    {:else if file?.content?.status === "unavailable"}
      <CommunityProjectsColumnAlert
        title="Text diff unavailable"
        description={`This file cannot be displayed as a text diff (${file.content.reason.replaceAll("_", " ")}).`}
      />
    {:else if file && !file.hunks.length && !patchRead.error}
      <CommunityProjectsColumnAlert
        title="No text changes"
        description="There are no changed text hunks for this file."
      />
    {:else if file}
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
            selection={diff.origin === TARGET_HISTORY_ORIGIN ||
            !executable ||
            (controller.details.lazy &&
              !/^native:[0-9a-f]{64}$/u.test(hunk.id)) ||
            !controller.can("transfer")
              ? null
              : workspaceHunkSelection(
                  controller.snapshot!,
                  diff.source,
                  file.path,
                  hunk.id,
                )}
            checked={!selection?.hunks || selection.hunks.includes(hunk.id)}
            selectable={Boolean(selection) &&
              (!controller.details.lazy ||
                /^native:[0-9a-f]{64}$/u.test(hunk.id))}
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
