<script lang="ts">
  import { onMount, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { ForgeController } from "../../web/src/lib/controller/forge/controller.svelte.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { MockStackWorkspaceAdapter } from "./stack-workspace/adapter.js";
  import { workspaceStoryFixture } from "./stack-workspace/data.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { workspaceForgeTransport } from "./forge/workspace-transport.js";
  let { panelOpen = true }: { panelOpen?: boolean } = $props();
  const controller = untrack(() => {
    const value = createInMemoryStoreController("stacks");
    value.actions.selectStacksView("workspace");
    return value;
  });
  const layoutStorage = untrack(createMemoryStoryStorage);
  const stackWorkspace = untrack(() => {
    const snapshot = workspaceStoryFixture();
    snapshot.repository = controller.state.route.repository;
    snapshot.folder = "/Users/demo/code/repository";
    snapshot.workspaces[0]!.applied = ["ui", "api"];
    const fixture = workspaceForgeTransport();
    const forge = new ForgeController(fixture.transport);
    forge.data = fixture.initial;
    forge.ci = fixture.ci;
    if (panelOpen) forge.select("feature/api");
    const adapter = Object.assign(new MockStackWorkspaceAdapter(snapshot), {
      forge,
      close: () => forge.close(),
    });
    return new StackWorkspaceController(adapter);
  });
  onMount(() => {
    void stackWorkspace.refresh();
    if (panelOpen) void stackWorkspace.forge!.act({ kind: "checks" });
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
    initiallyCollapsed={true}
  />
</div>
