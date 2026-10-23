<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { onMount, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreRepositoryApplication from "./ApplicationStory.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { workspaceSourceFiles } from "../../web/src/lib/controller/stack-workspace/files.js";
  import {
    cloneWorkspace,
    type WorkspaceSourceRequest,
    type WorkspaceFile,
  } from "../../web/src/lib/controller/stack-workspace/types.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { MockStackWorkspaceAdapter } from "./stack-workspace/adapter.js";
  import { workspaceStoryFixture } from "./stack-workspace/data.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    failFirstPatch = false,
    unavailable = false,
    refreshDuringPatch = false,
    simulateOperationChange = false,
  }: {
    failFirstPatch?: boolean;
    unavailable?: boolean;
    refreshDuringPatch?: boolean;
    simulateOperationChange?: boolean;
  } = $props();
  const controller = untrack(() => createInMemoryStoreController("stacks"));
  const layoutStorage = untrack(createMemoryStoryStorage);
  let patchReads = $state(0);
  const fixture = untrack(() => {
    const original = workspaceStoryFixture();
    original.repository = controller.state.route.repository;
    const snapshot = cloneWorkspace(original);
    snapshot.capabilities = [];
    snapshot.canUndo = false;
    snapshot.readKey = "read-only-catalog-operation";
    for (const stack of snapshot.stacks)
      for (const commit of stack.commits) commit.files = [];
    let failed = false,
      refreshed = false,
      operation = 0;
    const adapter = Object.assign(new MockStackWorkspaceAdapter(snapshot), {
      async readSourceFiles(request: WorkspaceSourceRequest) {
        return {
          files: workspaceSourceFiles(original, request.source).map((file) => ({
            ...file,
            hunks: [],
            content: { status: "unloaded" as const },
          })),
        };
      },
      async readFileDiff(
        request: WorkspaceSourceRequest & { path: string },
      ): Promise<WorkspaceFile> {
        patchReads++;
        if (simulateOperationChange)
          await new Promise((resolve) => setTimeout(resolve, 0));
        if (refreshDuringPatch && !refreshed) {
          refreshed = true;
          await value.refresh();
          await new Promise((resolve) => setTimeout(resolve, 0));
        }
        if (failFirstPatch && !failed) {
          failed = true;
          throw new Error("The selected patch could not be loaded. Try again.");
        }
        const file = workspaceSourceFiles(original, request.source).find(
          (file) => file.path === request.path,
        )!;
        return unavailable
          ? {
              ...cloneWorkspace(file),
              hunks: [],
              content: { status: "unavailable", reason: "binary" },
            }
          : { ...cloneWorkspace(file), content: { status: "ready" } };
      },
    });
    const value = new StackWorkspaceController(adapter);
    return {
      controller: value,
      async advanceOperation() {
        await new Promise((resolve) => setTimeout(resolve, 0));
        adapter.snapshot.readKey = `catalog-operation-${++operation}`;
        return value.refresh();
      },
    };
  });
  const stackWorkspace = fixture.controller;
  onMount(() => {
    void stackWorkspace.refresh();
    return () => {
      stackWorkspace.close();
      controller.actions.destroy();
    };
  });
</script>

<div style="height: 100vh">
  <FixtureControls
    ><div data-story-controls>
      {#if simulateOperationChange}
        <Button onclick={() => fixture.advanceOperation()}
          >Simulate native operation change</Button
        >
        <output aria-label="Fixture patch reads">{patchReads}</output>
      {/if}
    </div></FixtureControls
  >
  <StoreRepositoryApplication
    {controller}
    {stackWorkspace}
    {layoutStorage}
    displayMode="auto"
    initiallyCollapsed={false}
  />
</div>
