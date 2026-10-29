<script lang="ts">
  import StoreTerminalToggle from "../store-terminal/StoreTerminalToggle.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import MessageCircleIcon from "@lucide/svelte/icons/message-circle";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    type AppShellController,
    type AppShellSidebarController,
  } from "@lapismd/design-core/shell";
  import { PanelToggle } from "@lapismd/design-core/shadcn/panel-toggle";
  import { isStacksRoute } from "../../controller/store-stacks-navigation.js";
  import type { Snippet } from "svelte";
  import StoreWorkspaceToggle from "../store-agent-workspace/StoreWorkspaceToggle.svelte";

  let {
    controller,
    shell,
    detailsRail,
    compact = false,
    disabled = false,
    hidden = false,
    menu,
  }: {
    controller: StoreController;
    shell: AppShellController;
    detailsRail: AppShellSidebarController;
    compact?: boolean;
    disabled?: boolean;
    hidden?: boolean;
    menu?: Snippet;
  } = $props();
  const current = $derived(controller.state.route);
  const mobileMode = $derived(shell.mobile.resolvedMode === "mobile");
  const detailsExpanded = $derived(
    mobileMode
      ? shell.mobile.stage === "right" &&
          shell.mobile.activePanelId("right") === shell.getPanelId(detailsRail)
      : !detailsRail.closed,
  );
  const detailsLabel = $derived(
    compact
      ? `${detailsExpanded ? "Close" : "Open"} details sidebar`
      : "Toggle details",
  );
  function toggleDetails(
    event: MouseEvent & { currentTarget: HTMLButtonElement },
  ) {
    if (mobileMode) {
      if (detailsExpanded) shell.mobile.showMain();
      else
        shell.mobile.show(
          "right",
          shell.getPanelId(detailsRail)!,
          event.currentTarget,
        );
    } else if (detailsRail.closed) detailsRail.open();
    else detailsRail.close();
  }
</script>

{#if !hidden}
  {#if (current.agentsHub && ["session", "sessions"].includes(current.agentView ?? "")) || (current.page === "threads" && current.threadId)}
    {#if !compact}<StoreWorkspaceToggle />{/if}
  {:else if !current.agentsHub || controller.execution?.chat.open}
    {#if !compact && !isStacksRoute(controller.state.route)}
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
    {#if !compact && controller.execution}<Button
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
    {#if !compact || !["workflows", "threads"].includes(current.page)}
      <PanelToggle
        side="right"
        iconVariant="standard"
        expanded={detailsExpanded}
        size="icon-sm"
        aria-label={detailsLabel}
        title={detailsLabel}
        aria-expanded={detailsExpanded}
        {disabled}
        onclick={toggleDetails}
      />
    {/if}
  {/if}
{/if}
{@render menu?.()}
