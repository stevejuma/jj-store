<script lang="ts">
  import { onMount, untrack } from "svelte";
  import ApplicationStory from "./ApplicationStory.svelte";
  import FixtureControls from "./FixtureControls.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import { projectStackTopology } from "../../web/src/lib/controller/stack-workspace/topology-projection.js";
  import { localStacksCommand } from "../../web/src/lib/controller/stack-workspace/local-state.js";
  import {
    cloneWorkspace,
    type StackWorkspaceAdapter,
  } from "../../web/src/lib/controller/stack-workspace/types.js";
  import { workspaceFixture, fixtureCommit } from "./stack-workspace/data.js";
  let {
    partial = false,
    shared = false,
  }: { partial?: boolean; shared?: boolean } = $props();
  const initiallyPartial = untrack(() => partial);
  const initiallyShared = untrack(() => shared);
  const snapshot = workspaceFixture();
  snapshot.target.name = "trunk";
  snapshot.base = "trunk";
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
  snapshot.stacks[2]!.heads[0]!.commit = names[1]!;
  const inventory = [
    {
      heads: [
        {
          names: [names[0]!],
          commit: names[0]!,
          base_commit: names[1]!,
          shared: false,
        },
        {
          names: names.slice(1, 3),
          commit: names[1]!,
          base_commit: "fork",
          shared: false,
        },
      ],
      applied: [initiallyPartial ? names[1]! : names[0]!],
      unapply: [initiallyPartial ? names[1]! : names[0]!],
    },
    ...names.slice(3, 5).map((name) => ({
      heads: [
        { names: [name], commit: name, base_commit: "fork", shared: false },
      ],
      applied: [name],
      unapply: [name],
    })),
  ];
  if (initiallyShared) {
    inventory[0]!.heads[1]!.shared = true;
    inventory[1]!.heads[0]!.base_commit = names[1]!;
    inventory[1]!.heads.push(inventory[0]!.heads[1]!);
  }
  snapshot.workspaces[0]!.applied = [
    initiallyPartial ? names[1]! : names[0]!,
    ...names.slice(3, 5),
  ];
  snapshot.bookmarks = snapshot.stacks.map((stack) => ({
    name: stack.name,
    targets: [stack.heads[0]!.commit],
  }));
  projectStackTopology(snapshot, {
    complete: true,
    integrated: names.slice(5),
    inventory,
    lanes: initiallyPartial
      ? [
          { ...inventory[0]!, heads: inventory[0]!.heads.slice(1) },
          ...inventory.slice(1),
        ]
      : inventory,
  });
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
      const ids =
        head === names[0] || (initiallyShared && head === names[3])
          ? [head, names[1]!, "cloud-config"]
          : head === names[1]
            ? [names[1]!, "cloud-config"]
            : [head];
      return {
        commits: ids.map((id, index) => {
          const parent = ids[index + 1] ?? "fork";
          return {
            ...fixtureCommit(
              id,
              id === names[0]
                ? "add deployment health summary"
                : id === names[1]
                  ? "add deployment preview command"
                  : id === "cloud-config"
                    ? "prepare cloud deployment config"
                    : `Commit ${id}`,
              [],
            ),
            graph: {
              parents: [parent],
              edges: [
                {
                  target: parent,
                  kind:
                    parent === "fork"
                      ? ("missing" as const)
                      : ("direct" as const),
                },
              ],
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
    apply: async (request) => {
      executed = JSON.stringify(localStacksCommand(snapshot, request.command));
      return { status: "unknown" };
    },
    status: async () => ({ status: "unknown" }),
  };
  let executed = $state("");
  const controller = new StackWorkspaceController(adapter);
  const application = untrack(() => createInMemoryStoreController("stacks"));
  snapshot.repository = application.state.route.repository;
  onMount(() => {
    void controller.refresh();
    return () => controller.close();
  });
</script>

<ApplicationStory
  controller={application}
  stackWorkspace={controller}
  initiallyCollapsed={false}
/>
<FixtureControls>
  <output aria-label="Executed membership command">{executed}</output>
  <Button
    disabled={!loading}
    onclick={() => {
      loading = false;
      release();
    }}>Finish loading fixture history</Button
  >
</FixtureControls>
