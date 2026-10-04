<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import CommunityRepositoryBranchCommand from "../community-repository-branch-command/CommunityRepositoryBranchCommand.svelte";
  import StoreStackUnstaged from "./StoreStackUnstaged.svelte";
  import StoreStackLane from "./StoreStackLane.svelte";
  import StoreStackWorkspaceDiff from "./StoreStackWorkspaceDiff.svelte";
  import {
    activeWorkspace,
    appliedStacks,
    stackBookmarks,
  } from "../../controller/stack-workspace/presentation.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceCommand } from "../../controller/stack-workspace/types.js";
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
  const workspace = $derived(activeWorkspace(snapshot));
  const stacks = $derived(
    controller.page === "workspace"
      ? appliedStacks(snapshot)
      : snapshot.stacks.filter(
          (stack) => stack.id === controller.selectedStack,
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
    const origins = controller.diffs.map((diff) => diff.origin);
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
        id="branches"
        aria-label="Branches"
        title="Branches"
        count={stackBookmarks(snapshot).length}
      >
        <ColumnCanvas.Body scrollStrategy="styled">
          <CommunityRepositoryBranchCommand
            groupWorkspaceBookmarks
            scrollStrategy="styled"
            bookmarks={stackBookmarks(snapshot)}
            selected={controller.selectedBranch ?? ""}
            onSelect={(name) => {
              const stack = snapshot.stacks.find((item) =>
                item.heads.some((head) => head.name === name),
              );
              if (stack) controller.selectStack(stack.id, name);
            }}
          />
        </ColumnCanvas.Body>
      </ColumnCanvas.Column>
    {/if}
    {#each stacks.filter( (stack) => registeredStacks.includes(stack.id) ) as stack (stack.id)}<StoreStackLane
        {controller}
        {stack}
        {showStackActions}
        applied={workspace.applied.includes(stack.id)}
        {onCommand}
        onDrop={drop}
      />
      {#each controller.diffs.filter((diff) => diff.origin === `stack-${stack.id}` && registeredDiffs.includes(diff.origin)) as diff (diff.origin)}
        <StoreStackWorkspaceDiff {controller} {diff} {onCommand} />
      {/each}
    {/each}
  </ColumnCanvas.Root>
</ScrollArea>
