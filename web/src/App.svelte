<script lang="ts">
  import { ownApplicationNotifications } from "./lib/notifications/notification-context.svelte.js";
  import StoreNotificationHost from "./lib/ui/store-notifications/StoreNotificationHost.svelte";
  import { onDestroy, untrack } from "svelte";
  import { createServerConnections } from "./lib/connections/owner.svelte.js";
  import { provideServerConnections } from "./lib/connections/context.js";
  import StoreConnectionDialog from "./lib/ui/store-connections/StoreConnectionDialog.svelte";
  import StoreServerSwitcher from "./lib/ui/store-connections/StoreServerSwitcher.svelte";
  import StoreHelp from "./lib/ui/store-help/StoreHelp.svelte";
  import StoreConnection from "./lib/ui/store-application/StoreConnection.svelte";
  import StoreRepositoryApplication from "./lib/ui/store-application/StoreRepositoryApplication.svelte";

  import { ownAppearance } from "./lib/appearance/runtime.svelte.js";
  import type { AppearanceStore } from "./lib/appearance/store.svelte.js";
  let { appearance }: { appearance?: AppearanceStore } = $props();
  ownAppearance(untrack(() => appearance));
  const connections = createServerConnections();
  provideServerConnections(connections);
  onDestroy(() => connections.destroy());
  const notifications = ownApplicationNotifications(
    () => connections.controller,
  );
</script>

{#each connections.workspaces as workspace (workspace.controller)}
  <div
    class="connection-workspace"
    hidden={connections.selected !== workspace.id}
    inert={connections.selected !== workspace.id}
  >
    {#if workspace.controller.state.authenticated || workspace.controller.state.installationReady}
      <StoreRepositoryApplication
        controller={workspace.controller}
        stackWorkspace={workspace.controller.stackWorkspace}
        layoutStorage={workspace.layoutStorage}
      />
    {:else}
      <StoreConnection
        status={workspace.controller.state.status}
        accessMode={workspace.controller.state.accessMode}
        repositories={workspace.controller.state.repositories}
        storageReady={workspace.controller.state.storageReady}
        onConnect={workspace.controller.actions.connect}
      />
    {/if}
  </div>
{/each}
{#if !connections.workspaces.some((workspace) => workspace.id === connections.selected)}
  <div class="connection-locked">
    <aside aria-label="Server selection">
      <StoreServerSwitcher compact /><StoreHelp compact sidebar />
    </aside>
    <main>
      <h1>
        {connections.records.find((value) => value.id === connections.selected)
          ?.name ?? "Server unavailable"}
      </h1>
      <p>{connections.error || "Sign in to use this server."}</p>
    </main>
  </div>
{/if}
<StoreConnectionDialog owner={connections} />

<StoreNotificationHost owner={notifications} />

<style>
  .connection-workspace {
    height: 100%;
    min-height: 0;
  }
  .connection-workspace[hidden] {
    display: none;
  }
  .connection-locked {
    display: flex;
    height: 100%;
    background: var(--background);
  }
  .connection-locked aside {
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    width: 4rem;
    background: var(--sidebar);
    border-inline-end: 1px solid var(--border);
  }
  .connection-locked main {
    display: grid;
    align-content: center;
    justify-content: center;
    padding: 1.5rem;
    flex: 1;
  }
</style>
