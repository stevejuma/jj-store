<script lang="ts">
  import { AppShell, AppShellController } from "@lapismd/design-core/shell";
  import { onMount, untrack } from "svelte";
  import StoreStackWorkspaceConflict from "../../web/src/lib/ui/store-stack-workspace/StoreStackWorkspaceConflict.svelte";
  import StoreStackWorkspace from "../../web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceSnapshot } from "../../web/src/lib/controller/stack-workspace/types.js";
  import {
    MockStackWorkspaceAdapter,
    type FixtureFault,
  } from "./stack-workspace/adapter.js";
  import { workspaceStoryFixture } from "./stack-workspace/data.js";
  let {
    showStackActions = true,
    profile = "git_sha1_jj045",
    availability = "ready",
    fault = "none",
    empty = false,
    immutable = false,
    page = "workspace",
    loading = false,
    narrow = false,
    context = false,
  }: {
    showStackActions?: boolean;
    profile?: WorkspaceSnapshot["profile"];
    availability?: WorkspaceSnapshot["availability"];
    fault?: FixtureFault;
    empty?: boolean;
    immutable?: boolean;
    page?: "branches" | "workspace";
    loading?: boolean;
    narrow?: boolean;
    context?: boolean;
  } = $props();
  const shell = new AppShellController();
  const controller = untrack(() => {
    const snapshot = workspaceStoryFixture(profile);
    snapshot.availability = availability;
    if (context) {
      const lines = Array.from(
        { length: 60 },
        (_, index) => `context_${index + 1}`,
      );
      const full =
        `@@ -1,60 +1,60 @@\n` +
        lines
          .map((line, index) =>
            index === 20 || index === 40
              ? `-${line}\n+changed_${index + 1}\n`
              : ` ${line}\n`,
          )
          .join("");
      snapshot.workspaces[0]!.unassigned[0] = {
        path: "src/context.ts",
        patch: full,
        hunks: [20, 40].map((at, index) => ({
          id: `native:${String(index + 1).repeat(64)}`,
          label: `@@ -${at + 1},1 +${at + 1},1 @@`,
          patch: `@@ -${at + 1},1 +${at + 1},1 @@\n-context_${at + 1}\n+changed_${at + 1}\n`,
        })),
      };
    }
    if (empty) {
      snapshot.stacks = [];
      snapshot.workspaces[0]!.applied = [];
    }
    if (immutable) snapshot.stacks[0]!.commits[0]!.immutable = true;
    const value = new StackWorkspaceController(
      new MockStackWorkspaceAdapter(snapshot, fault),
    );
    value.page = page;
    return value;
  });
  onMount(() => {
    if (!loading) void controller.refresh();
    return () => controller.close();
  });
</script>

<div
  class="stack-workspace-fixture"
  style:max-width={narrow ? "390px" : undefined}
>
  <AppShell.Root
    controller={shell}
    desktopMinMainWidth={0}
    style="--ui-shell-height: 100%; --ui-shell-main-block-inset: 0px"
  >
    <AppShell.Main
      ><StoreStackWorkspace {controller} {showStackActions} /></AppShell.Main
    >
    <StoreStackWorkspaceConflict
      {controller}
      bind:open={controller.conflictOpen}
    />
  </AppShell.Root>
</div>

<style>
  .stack-workspace-fixture {
    height: 48rem;
    min-height: 30rem;
  }
</style>
