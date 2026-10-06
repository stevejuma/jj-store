<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import { stacksRouteView } from "../../controller/store-stacks-navigation.js";
  import type { StorePage } from "../community-navigation/model.js";
  import StoreRepositoryNavigation from "../store-application/StoreRepositoryNavigation.svelte";
  import StoreStackWorkspaceDetailsActions from "./StoreStackWorkspaceDetailsActions.svelte";
  let {
    controller,
    workspace,
  }: { controller: StoreController; workspace: StackWorkspaceController } =
    $props();
  const view = $derived(stacksRouteView(controller.state.route));
  const headings = {
    branches: "Branches/Stacks",
    workspace: "Workspace/Stacks",
    log: "JJ Log/Changes",
    oplog: "Op Log/Operations",
  };
  const sections: readonly { page: StorePage; label: string }[] = [
    { page: "branches", label: "Branches" },
    { page: "stacks", label: "Workspace" },
    { page: "log", label: "Commits" },
    { page: "oplog", label: "Op Log" },
  ];
</script>

<section class="stack-workspace-details" aria-label="Stack workspace details">
  <strong>{headings[view]}</strong>
  <StoreRepositoryNavigation
    page={view === "workspace" ? "stacks" : view}
    {sections}
    label="Stack workspace views"
    disabled={workspace.locked}
    onSelect={(page) => {
      if (workspace.locked) return;
      if (page === "stacks")
        void controller.actions.selectStacksView("workspace");
      else if (page === "branches" || page === "log" || page === "oplog")
        void controller.actions.selectStacksView(page);
    }}
  />
</section>
<StoreStackWorkspaceDetailsActions {workspace} />

<style>
  .stack-workspace-details {
    width: 100%;
    box-sizing: border-box;
  }
  strong {
    display: flex;
    min-height: 2rem;
    align-items: center;
    margin: 0.85rem 0.75rem 0;
    padding: 0 0.4rem 0.55rem;
    font-size: var(--community-text-secondary);
    font-weight: 500;
  }
</style>
