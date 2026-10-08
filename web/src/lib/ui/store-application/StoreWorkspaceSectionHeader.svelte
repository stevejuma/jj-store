<script lang="ts">
  import FolderRootIcon from "@lucide/svelte/icons/folder-root";
  import StoreAgentHeaderActions from "../store-agent-execution/StoreAgentHeaderActions.svelte";
  import StoreWorkspacePicker from "./StoreWorkspacePicker.svelte";
  import StoreComparisonParentPicker from "./StoreComparisonParentPicker.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    disabled,
  }: { controller: StoreController; disabled: boolean } = $props();
</script>

<header
  class="store-application__section-header"
  aria-label="Workspace section header"
>
  <div>
    <FolderRootIcon aria-hidden="true" />
    <h1 class="store-application__title">Workspace</h1>
  </div>
  <div class="store-application__section-controls">
    {#if controller.state.route.page === "agents"}
      <StoreAgentHeaderActions
        manager={controller.execution}
        context={{ repository: controller.state.repository.id }}
      />
    {/if}
    {#if controller.state.comparisonParents.length}
      <StoreComparisonParentPicker
        parents={controller.state.comparisonParents}
        revisions={controller.state.graph.revisions}
        devices={controller.state.graph.devices}
        now={controller.state.now}
        {disabled}
        onSelect={(id) => void controller.actions.selectComparisonParent(id)}
      />
    {/if}
    <StoreWorkspacePicker {controller} {disabled} />
  </div>
</header>
