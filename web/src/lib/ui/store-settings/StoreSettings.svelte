<script lang="ts">
  import { onMount } from "svelte";
  import { AppShell } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreAppearance from "../store-appearance/StoreAppearance.svelte";
  import StoreRepositorySettings from "./StoreRepositorySettings.svelte";
  import StoreSyncReconciliation from "./StoreSyncReconciliation.svelte";
  import StoreStorageSettings from "./StoreStorageSettings.svelte";
  import StoreSyncSettings from "./StoreSyncSettings.svelte";
  import StoreSearchSettings from "./StoreSearchSettings.svelte";
  import StoreAgentSettings from "./StoreAgentSettings.svelte";
  import StoreServiceSettings from "./StoreServiceSettings.svelte";
  import type { StoreSettingsSection } from "./model.js";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";

  let {
    controller,
    section,
    onClose,
  }: {
    controller: StoreController;
    section: StoreSettingsSection;
    onClose?: () => void;
  } = $props();
  onMount(() => {
    const timer = setInterval(() => {
      if (
        !document.hidden &&
        section !== "appearance" &&
        section !== "agents" &&
        controller.state.status.connection === "connected"
      )
        void controller.actions.refreshSettings();
    }, 5000);
    return () => clearInterval(timer);
  });
</script>

<AppShell.Body layout="regions" label="Store settings">
  <AppShell.Body.Content label="Settings content">
    {#if section === "appearance"}
      <StoreSettingsPage section="appearance"
        ><StoreAppearance /></StoreSettingsPage
      >
    {:else if section === "service"}
      <StoreSettingsPage section="service"
        ><StoreServiceSettings {controller} /></StoreSettingsPage
      >
    {:else if section === "agents"}
      <StoreAgentSettings
        management={controller.capture}
        repositoryName={(id) =>
          controller.state.repositories.find((r) => r.id === id)?.name ??
          "Previously bound repository"}
      />
    {:else if section === "repository"}
      <StoreRepositorySettings {controller} {onClose} />
    {:else if section === "storage"}
      <StoreStorageSettings
        settings={controller.state.settings.storageSettings}
        supported={Boolean(
          controller.state.settings.capabilities?.local_storage_management &&
          !controller.state.route.projectsHub &&
          controller.state.route.repository === controller.state.repository.id,
        )}
        onRefresh={controller.actions.refreshStorage}
        onStart={controller.actions.startStorage}
        onRetry={controller.actions.retryStorage}
      />
    {:else if section === "sync"}
      <StoreSyncSettings
        settings={controller.state.settings.syncSettings}
        supported={Boolean(
          controller.state.settings.capabilities?.local_sync_management,
        )}
        onRefresh={controller.actions.refreshSyncSettings}
        onControl={controller.actions.controlSync}
        onWait={controller.actions.waitForSync}
        onStopWait={controller.actions.stopSyncWait}
        >{#snippet reconciliation()}<StoreSyncReconciliation
            {controller}
          />{/snippet}</StoreSyncSettings
      >
    {:else}
      <StoreSearchSettings {controller} />
    {/if}
  </AppShell.Body.Content>
</AppShell.Body>
