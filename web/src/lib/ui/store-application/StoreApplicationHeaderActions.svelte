<script lang="ts">
  import StoreTerminalToggle from "../store-terminal/StoreTerminalToggle.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageCircleIcon from "@lucide/svelte/icons/message-circle";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import StoreRepositoryActions from "../store-mutations/StoreRepositoryActions.svelte";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";

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

{#if !isStacksRoute(controller.state.route)}
  {#if controller.state.route.page !== "threads"}<StoreRepositoryActions
      {controller}
      {disabled}
    />
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label="Refresh snapshot"
      title="Refresh snapshot"
      disabled={disabled || controller.state.status.connection !== "connected"}
      onclick={() => controller.actions.refresh()}
      ><RefreshCwIcon aria-hidden="true" /></Button
    >{/if}
  <StoreTerminalToggle {disabled} />
{/if}
{#if controller.execution}<Button
    variant="ghost"
    size="icon-sm"
    aria-label="Open chat"
    title="Chat"
    aria-expanded={controller.execution.chat.open}
    disabled={disabled || !controller.execution.available()}
    onclick={() =>
      controller.execution?.chat.open
        ? controller.execution.chat.close()
        : controller.execution?.chat.showContext(controller)}
    ><MessageCircleIcon aria-hidden="true" /></Button
  >{/if}
<span class="community-app-shell__sidebar-action">
  <Button
    variant="default"
    size="icon-sm"
    aria-label="Toggle details"
    {disabled}
    aria-expanded={detailsOpen}
    onclick={toggleDetails}
    ><CommunitySidebarToggleIcon side="right" expanded={detailsOpen} /></Button
  >
</span>
