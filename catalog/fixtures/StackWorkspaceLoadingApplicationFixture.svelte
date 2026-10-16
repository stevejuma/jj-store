<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
  import type { StoreStacksView } from "../../web/src/lib/controller/store-stacks-navigation.js";
  import type {
    StoreController,
    StoreLoadStatus,
  } from "../../web/src/lib/controller/contracts.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { MockStackWorkspaceAdapter } from "./stack-workspace/adapter.js";
  import { workspaceStoryFixture } from "./stack-workspace/data.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";

  import { repositoryColumnCanvasStorageKey } from "../../web/src/lib/ui/layout-storage.js";
  let {
    view = "workspace",
    selectedHistory = false,
    historyWidth,
    holdHistory = false,
  }: {
    view?: StoreStacksView;
    selectedHistory?: boolean;
    historyWidth?: number;
    holdHistory?: boolean;
  } = $props();
  const base = untrack(() => createInMemoryStoreController("stacks"));
  untrack(() => void base.actions.selectStacksView(view));
  let phase = $state<
    "connecting" | "discovering" | "bound" | "reading" | "ready"
  >("connecting");
  let stackWorkspace = $state<StackWorkspaceController>();
  let complete: (() => void) | undefined;
  const histories = new Map<string, () => void>();
  let reject: ((error: Error) => void) | undefined;
  const controller: StoreController = {
    ...base,
    get stackWorkspaceLoading() {
      return phase === "discovering";
    },
    get state() {
      return {
        ...base.state,
        route: selectedHistory
          ? { ...base.state.route, inspectedOperationId: "a".repeat(128) }
          : base.state.route,
        status: {
          ...base.state.status,
          connection:
            phase === "connecting"
              ? ("connecting" as const)
              : ("connected" as const),
          navigation: (phase === "ready" || phase === "discovering"
            ? "ready"
            : "loading") as StoreLoadStatus,
        },
      };
    },
  };
  const storage = untrack(() => {
    const saved = createMemoryStoryStorage();
    if (historyWidth)
      saved.setItem(
        repositoryColumnCanvasStorageKey(view === "oplog" ? "oplog" : "log"),
        JSON.stringify({
          version: 2,
          columns: {
            revisions: { collapsed: false, width: historyWidth },
            diff: { collapsed: false, width: 400 },
          },
          pairSplits: [],
        }),
      );
    return saved;
  });
  function discover() {
    const snapshot = workspaceStoryFixture();
    snapshot.repository = base.state.route.repository;
    const seed = {
      ...snapshot,
      active: null,
      target: null,
      workspaces: [],
      stacks: [],
    };
    const adapter = Object.assign(
      new MockStackWorkspaceAdapter(snapshot),
      holdHistory
        ? {
            async readStackHistory(
              request: import("../../web/src/lib/controller/stack-workspace/types.js").WorkspaceStackHistoryRequest,
            ) {
              await new Promise<void>((resolve) =>
                histories.set(request.stack, resolve),
              );
              return {
                commits: [
                  ...snapshot.stacks.find(
                    (stack) => stack.id === request.stack,
                  )!.commits,
                ].reverse(),
              };
            },
          }
        : {},
    );
    adapter.read = async () => {
      await new Promise<void>((resolve, fail) => {
        complete = resolve;
        reject = fail;
      });
      return structuredClone(snapshot);
    };
    stackWorkspace = new StackWorkspaceController(adapter, {
      initial: seed,
      initialLoaded: false,
    });
    phase = "bound";
  }
  async function read() {
    phase = "reading";
    await stackWorkspace!.refresh();
    phase = "ready";
  }
  onDestroy(() => {
    stackWorkspace?.close();
    base.actions.destroy();
  });
</script>

<div
  style="display: flex; flex-direction: column; height: 100vh; min-height: 36rem"
>
  <div
    role="group"
    aria-label="Loading fixture controls"
    style="display: flex; flex: none; gap: 0.25rem"
  >
    <Button onclick={() => (phase = "discovering")}>Connect repository</Button>
    <Button onclick={discover}>Discover workspace binding</Button>
    <Button onclick={read} disabled={!stackWorkspace || phase === "reading"}
      >Start workspace read</Button
    >
    <Button onclick={() => complete?.()}>Complete workspace read</Button>
    {#if holdHistory}<Button
        onclick={() => {
          for (const finish of histories.values()) finish();
          histories.clear();
        }}>Complete stack histories</Button
      >{/if}
    <Button
      onclick={() =>
        reject?.(new Error("The workspace read failed. Try again."))}
      >Fail workspace read</Button
    >
  </div>
  <div style="display: flex; flex: 1; min-height: 0">
    <StoreRepositoryApplication
      {controller}
      {stackWorkspace}
      layoutStorage={storage}
      displayMode="desktop"
      initiallyCollapsed={historyWidth !== undefined}
    />
  </div>
</div>
