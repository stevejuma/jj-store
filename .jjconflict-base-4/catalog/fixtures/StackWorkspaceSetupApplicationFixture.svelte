<script lang="ts">
  import { onMount, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import {
    cloneWorkspace,
    type WorkspaceSnapshot,
    type StackWorkspaceAdapter,
  } from "../../web/src/lib/controller/stack-workspace/types.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { workspaceStoryFixture } from "./stack-workspace/data.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let { page = "workspace" }: { page?: "branches" | "workspace" } = $props();
  const controller = untrack(() => createInMemoryStoreController("stacks"));
  const layoutStorage = untrack(createMemoryStoryStorage);
  const stackWorkspace = untrack(() => {
    const original = workspaceStoryFixture();
    const target = original.target;
    let snapshot: WorkspaceSnapshot = {
      ...original,
      repository: controller.state.route.repository,
      active: null,
      base: null,
      target: null,
      targetOptions: [target],
      workspaces: [],
      capabilities: [],
      canUndo: false,
    };
    const adapter: StackWorkspaceAdapter = {
      read: async () => cloneWorkspace(snapshot),
      chooseTarget: async (request) => {
        if (
          request.target.name !== target.name ||
          request.target.commit !== target.commit
        )
          throw new Error("Target changed");
        snapshot = {
          ...snapshot,
          target,
          base: target.name,
          revision: snapshot.revision + 1,
        };
        return cloneWorkspace(snapshot);
      },
      readTargetHistory: async () => ({ commits: [] }),
      preview: async () => {
        throw new Error("No executor");
      },
      apply: async () => {
        throw new Error("No executor");
      },
      status: async () => ({ status: "unknown" }),
    };
    const value = new StackWorkspaceController(adapter);
    value.page = page;
    return value;
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
