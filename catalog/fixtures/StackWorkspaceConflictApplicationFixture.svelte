<script lang="ts">
  import { onMount, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { MockStackWorkspaceAdapter } from "./stack-workspace/adapter.js";
  import {
    fixtureFile,
    workspaceStoryFixture,
  } from "./stack-workspace/data.js";
  let {
    writable = true,
    membershipChanged = false,
  }: { writable?: boolean; membershipChanged?: boolean } = $props();
  const controller = untrack(() => createInMemoryStoreController("stacks"));
  const layoutStorage = untrack(createMemoryStoryStorage);
  const fixture = untrack(() => {
    const snapshot = workspaceStoryFixture();
    snapshot.repository = controller.state.route.repository;
    snapshot.conflicts = [
      {
        path: "src/settings.ts",
        left: "export const enabled = true;\n",
        base: "export const enabled = false;\n",
        right: "export const enabled = null;\n",
      },
    ];
    snapshot.workspaceWarning =
      "This workspace contains native JJ conflicts. Your branch membership and unfinished work are retained.";
    snapshot.membershipChanged = membershipChanged;
    if (membershipChanged) snapshot.capabilities.push("reconcile_workspace");
    if (!writable) snapshot.capabilities = [];
    const files = snapshot.workspaces[0]!.unassigned;
    files[0]!.content = { status: "unavailable", reason: "conflict" };
    files[0]!.hunks = [];
    const metadata = fixtureFile("src/metadata.ts", []);
    const left = {
      type: "file" as const,
      id: "1".repeat(40),
      executable: false,
      copy: { kind: "placeholder" as const },
    };
    const right = { ...left, id: "2".repeat(40) };
    metadata.content = { status: "unloaded" };
    metadata.change = {
      before: [null],
      after: [left, null, right],
      sources: [],
    };
    const resolved = fixtureFile("src/resolved.ts", []);
    resolved.content = { status: "unavailable", reason: "conflict" };
    resolved.change = {
      before: [left, null, right],
      after: [right],
      sources: [],
    };
    files.push(metadata, resolved);
    const adapter = new MockStackWorkspaceAdapter(snapshot);
    return { adapter, workspace: new StackWorkspaceController(adapter) };
  });
  const stackWorkspace = fixture.workspace;
  async function recover() {
    fixture.adapter.snapshot.conflicts = [];
    fixture.adapter.snapshot.workspaceWarning = "";
    for (const file of fixture.adapter.snapshot.workspaces[0]!.unassigned) {
      file.change = undefined;
      file.content = { status: "ready" };
    }
    fixture.adapter.snapshot.revision++;
    await stackWorkspace.refresh();
  }
  onMount(() => {
    void stackWorkspace.refresh();
    return () => {
      stackWorkspace.close();
      controller.actions.destroy();
    };
  });
</script>

<div class="stack-conflict-application-fixture">
  <Button variant="ghost" onclick={recover}>Simulate conflict recovery</Button>
  <StoreRepositoryApplication
    {controller}
    {stackWorkspace}
    {layoutStorage}
    displayMode="desktop"
    initiallyCollapsed={false}
  />
</div>

<style>
  .stack-conflict-application-fixture {
    height: 100vh;
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
  }
</style>
