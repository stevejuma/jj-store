<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import StoreRepositoryActions from "../store-mutations/StoreRepositoryActions.svelte";

  let {
    controller,
    detailsOpen,
    toggleDetails,
    disabled = false,
  }: {
    controller: StoreController;
    detailsOpen: boolean;
    toggleDetails: () => void;
    disabled?: boolean;
  } = $props();
</script>

<StoreRepositoryActions {controller} {disabled} />
<Button
  variant={controller.state.status.refreshAvailable ? "outline" : "ghost"}
  size="icon-sm"
  aria-label={controller.state.status.refreshAvailable
    ? "Refresh snapshot (new history available)"
    : "Refresh snapshot"}
  title="Refresh snapshot"
  disabled={disabled || controller.state.status.connection !== "connected"}
  onclick={() => controller.actions.refresh()}
  ><RefreshCwIcon aria-hidden="true" /></Button
>
<Button
  variant="ghost"
  size="icon-sm"
  aria-label="Toggle details"
  {disabled}
  aria-expanded={detailsOpen}
  onclick={toggleDetails}
  ><CommunitySidebarToggleIcon side="right" expanded={detailsOpen} /></Button
>
