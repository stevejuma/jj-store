<script lang="ts">
  import { AppShell, AppShellController } from "@lapismd/design-core/shell";
  import { onMount, untrack } from "svelte";
  import StoreStackWorkspace from "../../web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { MockStackWorkspaceAdapter } from "./stack-workspace/adapter.js";
  import {
    fixtureCommit,
    fixtureFile,
    workspaceFixture,
  } from "./stack-workspace/data.js";
  let { page = "workspace" }: { page?: "workspace" | "branches" } = $props();
  const shell = new AppShellController();
  const controller = untrack(() => {
    const snapshot = workspaceFixture();
    const branches = Array.from({ length: 12 }, (_, i) => `fork-${i}`);
    const definitions: [string, string[]][] = [
      ["tip", ["merge"]],
      ["merge", branches],
      ...branches.map((id): [string, string[]] => [id, ["base"]]),
      ["base", []],
    ];
    const commits = definitions.map(([id, parents]) => ({
      ...fixtureCommit(id, `Graph ${id}`, [
        fixtureFile(`${id}.ts`, ["change"]),
      ]),
      graph: {
        parents,
        edges: parents.map((target) => ({ target, kind: "direct" as const })),
        bookmarks: [],
        workspaces: [],
        conflicted: false,
        divergent: false,
      },
    }));
    snapshot.stacks = [
      {
        id: "api",
        name: "feature/api",
        base: "main",
        conflicts: false,
        heads: [{ name: "feature/api", commit: "tip" }],
        commits: [...commits].reverse(),
      },
    ];
    snapshot.workspaces[0]!.applied = ["api"];
    const adapter = new MockStackWorkspaceAdapter(snapshot);
    adapter.readTargetHistory = async () => ({
      commits,
      total: commits.length,
    });
    const value = new StackWorkspaceController(adapter);
    value.page = page;
    return value;
  });
  onMount(() => {
    void controller.refresh();
    return () => controller.close();
  });
</script>

<div style="height: 46rem">
  <AppShell.Root
    controller={shell}
    desktopMinMainWidth={0}
    style="--ui-shell-height: 100%; --ui-shell-main-block-inset: 0px"
  >
    <AppShell.Main><StoreStackWorkspace {controller} /></AppShell.Main>
  </AppShell.Root>
</div>
