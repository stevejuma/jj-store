<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { onMount, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreRepositoryApplication from "./ApplicationStory.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { MockStackWorkspaceAdapter } from "./stack-workspace/adapter.js";
  import type { StackWorkspaceAdapter } from "../../web/src/lib/controller/stack-workspace/types.js";
  import {
    fixtureFile,
    workspaceStoryFixture,
  } from "./stack-workspace/data.js";
  let {
    writable = true,
    membershipChanged = false,
    multiple = false,
    multisided = false,
    deletedVersion = false,
    unsupported = false,
    deferredConflict = false,
  }: {
    writable?: boolean;
    membershipChanged?: boolean;
    multiple?: boolean;
    multisided?: boolean;
    deletedVersion?: boolean;
    unsupported?: boolean;
    deferredConflict?: boolean;
  } = $props();
  let completeRead = $state<(() => void) | undefined>();
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
    if (multisided)
      snapshot.conflicts[0]!.terms = [
        "left\n",
        "base\n",
        "right\n",
        "base\n",
        "third\n",
      ];
    if (deletedVersion)
      snapshot.conflicts[0]!.terms = [
        "left\n",
        "base\n",
        "right\n",
        "base\n",
        null,
      ];
    if (unsupported)
      snapshot.conflicts[0]!.reason =
        "This structural or mode conflict cannot be resolved as text.";
    snapshot.workspaceWarning =
      "This workspace contains native JJ conflicts. Your branch membership and unfinished work are retained.";
    if (multiple)
      snapshot.conflicts.push({
        path: "src/output.ts",
        left: "export const output = 1;\n",
        base: "export const output = 0;\n",
        right: "export const output = 2;\n",
      });
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
    if (deferredConflict) {
      const readConflict: NonNullable<
        StackWorkspaceAdapter["readConflict"]
      > = async ({ path }) => {
        await new Promise<void>((resolve) => (completeRead = resolve));
        completeRead = undefined;
        const value = adapter.snapshot.conflicts.find(
          (item) => item.path === path,
        );
        if (!value) throw Error("Fixture conflict unavailable");
        return { ...value };
      };
      Object.assign(adapter, { readConflict });
    }
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
      completeRead?.();
      controller.actions.destroy();
    };
  });
</script>

<div class="stack-conflict-application-fixture">
  <FixtureControls
    ><div data-story-controls>
      <Button variant="ghost" onclick={recover}
        >Simulate conflict recovery</Button
      >
      {#if deferredConflict}<Button
          variant="ghost"
          disabled={!completeRead}
          onclick={() => completeRead?.()}>Complete conflict read</Button
        >{/if}
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

<style>
  .stack-conflict-application-fixture {
    height: 100vh;
  }
</style>
