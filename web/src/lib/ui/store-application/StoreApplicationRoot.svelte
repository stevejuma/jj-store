<script lang="ts">
  import { ownApplicationNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../../ui/store-notifications/StoreNotificationHost.svelte";
  import { untrack } from "svelte";
  import type { ServerConnectionsView } from "../../connections/presentation.js";
  import type { AppShellDisplayMode } from "@lapismd/design-core/shell";
  import { provideServerConnections } from "../../connections/context.js";
  import StoreConnectionDialog from "../../ui/store-connections/StoreConnectionDialog.svelte";
  import StoreServerSwitcher from "../../ui/store-connections/StoreServerSwitcher.svelte";
  import StoreHelp from "../../ui/store-help/StoreHelp.svelte";
  import StoreConnection from "./StoreConnection.svelte";
  import StoreRepositoryApplication from "./StoreRepositoryApplication.svelte";

  import { ownAppearance } from "../../appearance/runtime.svelte.js";
  import type { AppearanceStore } from "../../appearance/store.svelte.js";
  let {
    connections,
    appearance,
    initiallyCollapsed = true,
    desktopChrome,
    displayMode = "auto",
  }: {
    connections: ServerConnectionsView;
    appearance?: AppearanceStore;
    initiallyCollapsed?: boolean;
    desktopChrome?: boolean;
    displayMode?: AppShellDisplayMode;
  } = $props();
  ownAppearance(untrack(() => appearance));
  provideServerConnections(untrack(() => connections));
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
        {initiallyCollapsed}
        {displayMode}
        {desktopChrome}
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
    width: 100%;
    min-width: 0;
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
