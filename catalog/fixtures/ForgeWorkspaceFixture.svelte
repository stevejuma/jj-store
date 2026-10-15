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
  import type { ForgeCheck } from "../../web/src/lib/controller/forge/types.js";
  let {
    panelOpen = true,
    workspacePullRequest = false,
    apiPullRequests = true,
    failPreview = false,
    unknownSubmit = false,
    pushState = "changes",
    apiCheckState = "running",
    apiChecksComplete = true,
  }: {
    panelOpen?: boolean;
    workspacePullRequest?: boolean;
    apiPullRequests?: boolean;
    failPreview?: boolean;
    unknownSubmit?: boolean;
    pushState?: "changes" | "synced" | "unknown";
    apiCheckState?: ForgeCheck["state"];
    apiChecksComplete?: boolean;
  } = $props();
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
    const apiHead =
      pushState === "unknown"
        ? "api2"
        : (pushState === "synced" ? "2" : "4").repeat(40);
    const heads = new Map([
      ["api1", "1".repeat(40)],
      ["api2", apiHead],
      ["ui1", "3".repeat(40)],
    ]);
    for (const stack of snapshot.stacks) {
      for (const commit of stack.commits)
        commit.id = heads.get(commit.id) ?? commit.id;
      for (const head of stack.heads)
        head.commit = heads.get(head.commit) ?? head.commit;
    }
    const fixture = workspaceForgeTransport(
      workspacePullRequest,
      apiPullRequests,
      failPreview,
      unknownSubmit,
      {
        apiCheckState,
        apiChecksComplete,
        apiLocalHead: pushState === "unknown" ? undefined : apiHead,
      },
    );
    const forge = new ForgeController(fixture.transport);
    forge.data = fixture.initial;
    forge.ci = fixture.ci;
    if (panelOpen) forge.select("feature/api", "inline");
    const adapter = Object.assign(new MockStackWorkspaceAdapter(snapshot), {
      forge,
      close: () => forge.close(),
    });
    return new StackWorkspaceController(adapter);
  });
  onMount(() => {
    void stackWorkspace.refresh().then(() => {
      const source = stackWorkspace.snapshot?.stacks.find(
        (stack) => stack.id === "api",
      )?.heads[0]?.commit;
      if (panelOpen && source)
        stackWorkspace.selectFile(source, "src/api.ts", undefined, "api");
    });
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
