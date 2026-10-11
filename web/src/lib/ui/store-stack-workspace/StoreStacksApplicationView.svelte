<script lang="ts">
  import { untrack, type Snippet } from "svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { stacksRouteView } from "../../controller/store-stacks-navigation.js";
  import StoreStackWorkspace from "./StoreStackWorkspace.svelte";
  let {
    controller,
    workspace,
    history,
    onRepositorySettings,
  }: {
    controller: StoreController;
    workspace: StackWorkspaceController;
    history: Snippet;
    onRepositorySettings?: () => void;
  } = $props();
  const route = $derived(controller.state.route);
  const view = $derived(stacksRouteView(route));
  $effect(() => {
    const next = view,
      owner = workspace;
    if (next === "branches" || next === "workspace")
      untrack(() => {
        owner.page = next;
      });
  });
  // Local commands can switch the canvas tab; reflect that without changing history views.
  $effect(() => {
    const next = workspace.page;
    untrack(() => {
      if (route.page === "stacks" && (route.stackTab ?? "workspace") !== next)
        void controller.actions.selectStacksView(next);
    });
  });
</script>

<StoreStackWorkspace
  controller={workspace}
  showPicker={false}
  focusedView={view}
  historyMode={route.stackHistory ?? "live"}
  onHistoryMode={view === "log" || view === "oplog"
    ? (mode) => {
        void controller.actions.selectStacksView(view, mode);
      }
    : undefined}
  onSelectView={(next) => {
    void controller.actions.selectStacksView(next);
  }}
  onBack={() => {
    void controller.actions.leaveStacks();
  }}
  onSettings={() => (workspace.settingsOpen = true)}
  {history}
  {onRepositorySettings}
/>
