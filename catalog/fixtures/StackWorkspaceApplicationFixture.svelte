<script lang="ts">
  import { onMount, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { MockStackWorkspaceAdapter } from "./stack-workspace/adapter.js";
  import { workspaceStoryFixture } from "./stack-workspace/data.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    page = "workspace",
    linked = true,
    continuousStack = false,
    targetBranch = "main",
    writable = true,
  }: {
    page?: "branches" | "workspace";
    linked?: boolean;
    continuousStack?: boolean;
    targetBranch?: string;
    writable?: boolean;
  } = $props();
  const controller = untrack(() => createInMemoryStoreController("stacks"));
  const layoutStorage = untrack(createMemoryStoryStorage);
  const stackWorkspace = untrack(() => {
    const snapshot = workspaceStoryFixture();
    snapshot.base = snapshot.target.name = targetBranch;
    snapshot.target.commit = `${targetBranch}-history-240`;
    snapshot.target.message = `${targetBranch === "main" ? "Main" : targetBranch} history 240`;
    for (const stack of snapshot.stacks) stack.base = targetBranch;
    if (continuousStack)
      snapshot.stacks[0]!.heads = [snapshot.stacks[0]!.heads[0]!];
    snapshot.targetOptions = [
      snapshot.target,
      ...snapshot.stacks.map((stack) => ({
        ...snapshot.target,
        name: stack.name,
        commit: stack.heads[0]!.commit,
        message: stack.commits.at(-1)?.message ?? stack.name,
      })),
    ];
    snapshot.repository = controller.state.route.repository;
    if (!linked) {
      snapshot.folder = null;
      snapshot.availability = "unlinked";
    }
    if (!writable) snapshot.capabilities = [];
    const value = new StackWorkspaceController(
      new MockStackWorkspaceAdapter(snapshot),
    );
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

<div class="stack-workspace-application-fixture">
  <StoreRepositoryApplication
    {controller}
    {stackWorkspace}
    {layoutStorage}
    displayMode="desktop"
    initiallyCollapsed={false}
  />
</div>

<style>
  .stack-workspace-application-fixture {
    height: 100vh;
    min-height: 36rem;
  }
</style>
