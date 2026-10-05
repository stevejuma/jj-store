<script lang="ts">
  import { onMount, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
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
  }: { failFirstPatch?: boolean; unavailable?: boolean } = $props();
  const controller = untrack(() => createInMemoryStoreController("stacks"));
  const layoutStorage = untrack(createMemoryStoryStorage);
  const stackWorkspace = untrack(() => {
    const original = workspaceStoryFixture();
    original.repository = controller.state.route.repository;
    const snapshot = cloneWorkspace(original);
    snapshot.capabilities = [];
    snapshot.canUndo = false;
    snapshot.readKey = "read-only-catalog-operation";
    for (const stack of snapshot.stacks)
      for (const commit of stack.commits) commit.files = [];
    let failed = false;
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
    return new StackWorkspaceController(adapter);
  });
  onMount(() => {
    void stackWorkspace.refresh();
    return () => {
      stackWorkspace.close();
      controller.actions.destroy();
    };
  });
</script>

<div style="height: 100vh; min-height: 36rem">
  <StoreRepositoryApplication
    {controller}
    {stackWorkspace}
    {layoutStorage}
    displayMode="desktop"
    initiallyCollapsed={false}
  />
</div>
