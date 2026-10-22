<!-- Adapted CommunityApplication profile sidebar and CommunityAgentProfile. -->
<script lang="ts">
  import {
    AppShell,
    type AppShellController,
  } from "@lapismd/design-core/shell";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import { tick, untrack, onDestroy } from "svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { StoreRoute } from "../../controller/contracts.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityAgentProfile from "../community-agent-profile/CommunityAgentProfile.svelte";
  let {
    manager,
    shell,
    route,
  }: { manager: AgentExecution; shell: AppShellController; route: StoreRoute } =
    $props();
  const agent = $derived(
    route.agentView === "directory"
      ? manager.value?.setups.find((a) => a.id === manager.detailsId)
      : undefined,
  );
  const { context } = ownNotifications();
  let opener: HTMLElement | null = null;
  let previousId: string | undefined;
  $effect(() => {
    const id = agent?.id;
    untrack(() => {
      if (id === previousId) return;
      previousId = id;
      if (id) {
        opener =
          document.activeElement instanceof HTMLElement
            ? document.activeElement
            : null;
        shell.right.open();
        void tick().then(() => {
          if (agent?.id === id && shell.mobile.resolvedMode === "mobile")
            shell.mobile.show("right", "right", opener);
        });
      } else shell.right.close();
    });
  });
  function close() {
    manager.detailsId = "";
    shell.right.close();
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
    opener?.focus({ preventScroll: true });
  }
  onDestroy(() => {
    manager.detailsId = "";
    shell.right.close();
  });
  async function copy(id: string) {
    try {
      await navigator.clipboard.writeText(id);
      context.service.resolve(context.scope, "agent-id-copy");
      context.service.publish({
        scope: context.scope,
        key: "agent-id-copy",
        title: "Copied",
        message: "Agent ID copied",
        severity: "info",
        lifetime: "timed",
      });
    } catch {
      context.service.publish({
        scope: context.scope,
        key: "agent-id-copy",
        title: "Copy failed",
        message: "Could not copy the agent ID. Try again.",
        severity: "error",
        lifetime: "dismiss",
      });
    }
  }
</script>

{#if agent}<AppShell.Sidebar
    side="right"
    variant="attached"
    constraintPriority={0}
    constrainedPresentation="replace-main"
    label="Agent details sidebar"
    mobileLabel="Agent details"
    resizeLabel="Resize agent details sidebar"
    closeable
  >
    {#if agent}
      <AppShell.Sidebar.Header class="store-agent-details-header"
        ><strong>Agent details</strong><AppShell.Sidebar.Close
          label="Close agent details"
          onclick={close}
        /></AppShell.Sidebar.Header
      >
      <AppShell.Sidebar.Body>
        {#if manager.value?.live_agents?.find((live) => live.agent_id === agent.id)?.inference_host}<p
            class="store-agent-inference-host"
          >
            Inference host: {manager.value?.live_agents?.find(
              (live) => live.agent_id === agent.id,
            )?.inference_host}
          </p>{/if}
        {#if manager.localManagement() && manager.value?.live_agents?.find((live) => live.agent_id === agent.id)?.restart_required}
          <p>Configuration changed. Restart required.</p>
          <Button
            variant="outline"
            disabled={manager.busy || !!manager.pending}
            onclick={async () => {
              await manager.stopAgent(agent.id);
              if (!manager.pending && !manager.error)
                await manager.startAgent(agent.id);
            }}>Restart agent</Button
          >
        {/if}
        {#key agent.id}<CommunityAgentProfile
            {agent}
            management={manager.localManagement()}
            status={manager.value}
            unknown={!!manager.connectionError}
            pending={manager.busy || !!manager.pending}
            onEdit={() => (manager.editRequested = agent)}
            onStart={() => void manager.startAgent(agent.id)}
            onChat={() => manager.newChat(agent.id)}
            onSessions={() => manager.viewSessions(agent.id)}
            onActivity={() => manager.viewActivity(agent.id)}
            onStop={(id) => void manager.stopAgent(id)}
            onCopy={() => {
              void copy(agent.id);
            }}
          />{/key}
      </AppShell.Sidebar.Body>
    {/if}
  </AppShell.Sidebar>{/if}

<style>
  .store-agent-inference-host {
    overflow-wrap: anywhere;
    font-size: 0.75rem;
    color: var(--muted-foreground);
  }
  :global(.store-agent-details-header[data-ui-part="sidebar-header"]) {
    border-block-end: 0;
  }
</style>
