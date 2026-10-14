<script lang="ts">
  import StoreTerminalToggle from "../store-terminal/StoreTerminalToggle.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageCircleIcon from "@lucide/svelte/icons/message-circle";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  import type { Snippet } from "svelte";
  import StoreWorkspaceToggle from "../store-agent-workspace/StoreWorkspaceToggle.svelte";

  let {
    controller,
    detailsOpen,
    toggleDetails,
    disabled = false,
    hidden = false,
    menu,
  }: {
    controller: StoreController;
    detailsOpen: boolean;
    toggleDetails: () => void;
    disabled?: boolean;
    hidden?: boolean;
    menu?: Snippet;
  } = $props();
  const current = $derived(controller.state.route);
</script>

{#if !hidden}
  {#if (current.agentsHub && ["session", "sessions"].includes(current.agentView ?? "")) || (current.page === "threads" && current.threadId)}
    <StoreWorkspaceToggle />
  {:else if !current.agentsHub || controller.execution?.chat.open}
    {#if !isStacksRoute(controller.state.route)}
      {#if controller.state.route.page !== "threads"}
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="Refresh snapshot"
          title="Refresh snapshot"
          disabled={disabled ||
            controller.state.status.connection !== "connected"}
          onclick={() => controller.actions.refresh()}
          ><RefreshCwIcon aria-hidden="true" /></Button
        >{/if}
      {#if controller.state.accessMode === "local_trusted"}<StoreTerminalToggle
          {disabled}
        />{/if}
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
        ><CommunitySidebarToggleIcon
          side="right"
          expanded={detailsOpen}
        /></Button
      >
    </span>
  {/if}
{/if}
{@render menu?.()}
