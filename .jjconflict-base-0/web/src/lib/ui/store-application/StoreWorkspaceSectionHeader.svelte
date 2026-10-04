<script lang="ts">
  import PanelsTopLeftIcon from "@lucide/svelte/icons/panels-top-left";
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
    <PanelsTopLeftIcon aria-hidden="true" />
    <h1 class="store-application__title">Workspace</h1>
  </div>
  <div class="store-application__section-controls">
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
