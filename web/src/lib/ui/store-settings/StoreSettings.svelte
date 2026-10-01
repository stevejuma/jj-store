<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreRepositorySettings from "./StoreRepositorySettings.svelte";
  import StoreSyncSettings from "./StoreSyncSettings.svelte";
  import StoreSearchSettings from "./StoreSearchSettings.svelte";
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
    onClose: () => void;
  } = $props();
</script>

<AppShell.Body layout="regions" label="Store settings">
  <AppShell.Body.Content label="Settings content">
    {#if section === "service"}
      <StoreSettingsPage section="service" {onClose}
        ><StoreServiceSettings {controller} /></StoreSettingsPage
      >
    {:else if section === "repository"}
      <StoreRepositorySettings {controller} {onClose} />
    {:else if section === "sync"}
      <StoreSyncSettings
        {onClose}
        settings={controller.state.settings.syncSettings}
        supported={Boolean(
          controller.state.settings.capabilities?.local_sync_management,
        )}
        onRefresh={controller.actions.refreshSyncSettings}
        onControl={controller.actions.controlSync}
      />
    {:else}
      <StoreSearchSettings {controller} {onClose} />
    {/if}
  </AppShell.Body.Content>
</AppShell.Body>
