<script lang="ts">
  import { onMount } from "svelte";
  import { AppShell, AppShellController } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreStackWorkspace from "../../web/src/lib/ui/store-stack-workspace/StoreStackWorkspace.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { localStacksCommand } from "../../web/src/lib/controller/stack-workspace/local-state.js";
  import {
    cloneWorkspace,
    type StackWorkspaceAdapter,
  } from "../../web/src/lib/controller/stack-workspace/types.js";
  import { workspaceFixture, fixtureCommit } from "./stack-workspace/data.js";
  const snapshot = workspaceFixture();
  const names = [
    "cloud-observability",
    "cloud-runner",
    "workspace/experiment",
    "export-json",
    "query-filtering",
    "bootstrap-cleanup",
    "readme-polish",
  ];
  snapshot.stacks = names.map((name) => ({
    id: name,
    name,
    base: "trunk",
    heads: [{ name, commit: name }],
    commits: [],
    conflicts: false,
  }));
  snapshot.workspaces[0]!.applied = names;
  const heads = [
    { name: names[0]!, commit: names[0]! },
    { name: names[1]!, commit: names[1]!, aliases: [names[2]!] },
  ];
  snapshot.stacks[0]!.heads = heads;
  snapshot.stacks[0]!.topologyHeads = heads;
  snapshot.stacks[0]!.appliedBranches = names.slice(0, 3);
  snapshot.topology = {
    complete: true,
    integrated: names.slice(5),
    lanes: [
      {
        heads: [
          { names: [names[0]!], commit: names[0]! },
          { names: names.slice(1, 3), commit: names[1]! },
        ],
        applied: names.slice(0, 3),
      },
      ...names.slice(3, 5).map((name) => ({
        heads: [{ names: [name], commit: name }],
        applied: [name],
      })),
    ],
  };
  let release!: () => void;
  let loading = $state(true);
  const ready = new Promise<void>((resolve) => {
    release = resolve;
  });
  const adapter: StackWorkspaceAdapter = {
    read: async () => cloneWorkspace(snapshot),
    readTargetHistory: async () => ({ commits: [] }),
    readStackHistory: async ({ head }) => {
      await ready;
      const ids = head === names[0] ? names.slice(0, 2) : [head];
      return {
        commits: ids.map((id, index) => {
          const parent = ids[index + 1] ?? snapshot.target.commit;
          return {
            ...fixtureCommit(id, `Commit ${id}`, []),
            graph: {
              parents: [parent],
              edges: [{ target: parent, kind: "direct" as const }],
              bookmarks: [],
              workspaces: [],
              conflicted: false,
              divergent: false,
            },
          };
        }),
      };
    },
    preview: async (request) => ({
      request,
      title: "Unapply stack",
      summary: JSON.stringify(localStacksCommand(snapshot, request.command)),
      conflicts: [],
    }),
    apply: async () => ({ status: "unknown" }),
    status: async () => ({ status: "unknown" }),
  };
  const controller = new StackWorkspaceController(adapter);
  const shell = new AppShellController();
  onMount(() => {
    void controller.refresh();
    return () => controller.close();
  });
</script>

<Button
  disabled={!loading}
  onclick={() => {
    loading = false;
    release();
  }}>Finish loading fixture history</Button
>
<div style="height: 850px">
  <AppShell.Root controller={shell} displayMode="desktop">
    <AppShell.Main><StoreStackWorkspace {controller} /></AppShell.Main>
  </AppShell.Root>
</div>
