<script lang="ts">
  import {
    toggleStackBranch,
    toggleTargetBranch,
  } from "../../controller/stack-workspace/selection-actions.js";
  import { onDestroy, untrack } from "svelte";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import CommunityRepositoryBranchCommand from "../community-repository-branch-command/CommunityRepositoryBranchCommand.svelte";
  import StoreStackUnstaged from "./StoreStackUnstaged.svelte";
  import StoreStackWorkspaceSpacer from "./StoreStackWorkspaceSpacer.svelte";
  import StoreForgePanel from "../store-forge/StoreForgePanel.svelte";
  import StoreStackLane from "./StoreStackLane.svelte";
  import StoreStackTargetHistory from "./StoreStackTargetHistory.svelte";
  import StoreStackWorkspaceDiff from "./StoreStackWorkspaceDiff.svelte";
  import StoreStackWorkspaceDetailsColumn from "./StoreStackWorkspaceDetailsColumn.svelte";
  import { stackDetailsColumns } from "../../controller/stack-workspace/details-columns.js";
  import {
    appliedStacks,
    browsingStack,
    isAppliedBranch,
    stackBookmarks,
  } from "../../controller/stack-workspace/presentation.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceCommand } from "../../controller/stack-workspace/types.js";
  import { TARGET_HISTORY_ORIGIN } from "../../controller/stack-workspace/target-history.svelte.js";
  let {
    controller,
    showStackActions = true,
    onCommand,
  }: {
    controller: StackWorkspaceController;
    showStackActions?: boolean;
    onCommand: (command: WorkspaceCommand) => void;
  } = $props();
  let scrollViewport = $state<HTMLElement | null>(null);
  let registeredStacks = $state<string[]>([]);
  let registeredDiffs = $state<string[]>([]);
  const snapshot = $derived(controller.snapshot!);
  const stacks = $derived(
    controller.page === "workspace"
      ? appliedStacks(snapshot)
      : [browsingStack(snapshot, controller.selectedStack)].filter(
          (stack): stack is NonNullable<typeof stack> => Boolean(stack),
        ),
  );
  const details = $derived(
    stacks.flatMap((stack) =>
      stackDetailsColumns(controller, stack).map((detail) => ({
        ...detail,
        stack: stack.id,
      })),
    ),
  );
  const canvas = ColumnCanvas.createColumnCanvasController({
    trailingSpacerWidth: 240,
    columns: {
      branches: {
        resizable: true,
        defaultWidth: 320,
        minWidth: 260,
        maxWidth: 520,
        pathLevel: 0,
        collapsible: true,
      },
      [TARGET_HISTORY_ORIGIN]: {
        resizable: true,
        defaultWidth: 390,
        minWidth: 310,
        maxWidth: 640,
        pathLevel: 0,
        collapsible: true,
      },
      "forge-pr": {
        resizable: true,
        defaultWidth: 480,
        minWidth: 360,
        maxWidth: 800,
        pathLevel: 0,
        closeable: false,
      },
      working: {
        resizable: true,
        defaultWidth: 330,
        minWidth: 260,
        maxWidth: 520,
        pathLevel: 0,
        collapsible: true,
      },
    },
  });
  $effect(() => {
    canvas.trailingSpacerWidth = controller.page === "workspace" ? 390 : 240;
  });
  $effect(() => {
    const ids = snapshot.stacks.map((stack) => stack.id);
    untrack(() => {
      for (const id of ids)
        canvas.ensureColumn(`stack-${id}`, {
          resizable: true,
          defaultWidth: 390,
          minWidth: 310,
          maxWidth: 640,
          pathLevel: 0,
          collapsible: true,
        });
    });
    registeredStacks = ids;
  });
  $effect(() => {
    const origins = [
      ...controller.diffs.map((diff) => diff.origin),
      ...details.map((detail) => detail.origin),
      ...(controller.targetHistory.diff ? [TARGET_HISTORY_ORIGIN] : []),
    ];
    untrack(() => {
      for (const origin of origins) {
        const id = `diff-${origin}`;
        canvas.ensureColumn(id, {
          resizable: true,
          defaultWidth: 580,
          minWidth: 360,
          maxWidth: 980,
          pathLevel: 0,
          closeable: true,
        });
        canvas.open(id);
      }
    });
    registeredDiffs = origins;
  });
  onDestroy(() => {
    void canvas.dispose();
  });
  function drop(
    event: DragEvent,
    target: "workspace" | "working" | "stack" | "commit",
    id = "",
  ) {
    const command = controller.drag.drop(
      event,
      { kind: target, id },
      snapshot,
      controller.locked,
    );
    if (command) onCommand(command);
  }
</script>

{#snippet branchSpacer()}<StoreStackWorkspaceSpacer
    {controller}
    {onCommand}
  />{/snippet}

<ScrollArea
  orientation="horizontal"
  scrollStrategy="styled"
  fillContent
  bind:viewportRef={scrollViewport}
  class="stack-workspace__canvas-scroll"
  ondragend={() => controller.drag.clear()}
>
  <ColumnCanvas.Root
    displayMode="fixed"
    controller={canvas}
    {scrollViewport}
    class="stack-workspace__canvas"
    trailingSpacer={controller.page === "workspace" ? branchSpacer : undefined}
  >
    {#if controller.page === "workspace"}<StoreStackUnstaged
        {controller}
        onDrop={(event) => drop(event, "workspace")}
        onChangesDrop={(event) => drop(event, "working")}
      />
      {#each controller.diffs.filter((diff) => diff.origin === "working" && registeredDiffs.includes(diff.origin)) as diff (diff.origin)}
        <StoreStackWorkspaceDiff {controller} {diff} {onCommand} />
      {/each}
    {/if}
    {#if controller.page === "branches"}
      <ColumnCanvas.Column
        resizable
        fixed
        id="branches"
        aria-label="Branches"
        title="Branches"
        count={stackBookmarks(snapshot).length}
      >
        <ColumnCanvas.Body scrollStrategy="styled">
          <CommunityRepositoryBranchCommand
            hasMore={Boolean(snapshot.branchesNextCursor)}
            loading={controller.branches.loading}
            onLoadMore={() => void controller.branches.load()}
            groupWorkspaceBookmarks
            scrollStrategy="styled"
            targetBookmark={snapshot.target
              ? {
                  name: snapshot.target.name,
                  targets: [snapshot.target.commit],
                  description: snapshot.target.message,
                  author: snapshot.target.author,
                  authorEmail: snapshot.target.email,
                  updatedAt: snapshot.target.createdAt,
                }
              : undefined}
            bookmarks={stackBookmarks(snapshot)}
            selected={controller.targetHistory.active
              ? (snapshot.target?.name ?? "")
              : (controller.selectedBranch ?? "")}
            onSelect={(name) => {
              if (name === snapshot.target?.name) {
                toggleTargetBranch(controller);
                return;
              }
              const stack =
                snapshot.stacks.find((item) => item.name === name) ??
                snapshot.stacks.find((item) =>
                  item.heads.some((head) => head.name === name),
                );
              if (stack) toggleStackBranch(controller, stack.id, name);
              else
                controller.validation =
                  snapshot.branchUnavailableReasons?.[name] ??
                  "This branch is unavailable.";
            }}
          />
        </ColumnCanvas.Body>
      </ColumnCanvas.Column>
      {#if controller.targetHistory.active}
        <StoreStackTargetHistory {controller} />
        {#each controller.targetHistory.diff && registeredDiffs.includes(TARGET_HISTORY_ORIGIN) ? [controller.targetHistory.diff] : [] as diff (diff.origin)}
          <StoreStackWorkspaceDiff {controller} {diff} {onCommand} />
        {/each}
      {/if}
    {/if}
    {#each stacks.filter( (stack) => registeredStacks.includes(stack.id) ) as stack (stack.id)}<StoreStackLane
        {controller}
        {stack}
        {showStackActions}
        applied={[stack.name, ...(stack.heads[0]?.aliases ?? [])].some((name) =>
          isAppliedBranch(snapshot, name),
        )}
        {onCommand}
        onDrop={drop}
      />
      {#each details.filter((detail) => detail.stack === stack.id && registeredDiffs.includes(detail.origin)) as detail (detail.origin)}
        <StoreStackWorkspaceDetailsColumn
          {controller}
          origin={detail.origin}
          branch={detail.branch}
          diff={detail.diff}
          {onCommand}
        />
      {/each}
    {/each}
    {#if controller.forge?.open}<ColumnCanvas.Column
        id="forge-pr"
        resizable
        aria-label="Pull request details"
        ><StoreForgePanel controller={controller.forge} /></ColumnCanvas.Column
      >{/if}
  </ColumnCanvas.Root>
</ScrollArea>
