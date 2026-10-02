<script lang="ts">
  import { onMount } from "svelte";
  import { AppShell } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreAppearance from "../store-appearance/StoreAppearance.svelte";
  import StoreRepositorySettings from "./StoreRepositorySettings.svelte";
  import StoreSyncSettings from "./StoreSyncSettings.svelte";
  import StoreSearchSettings from "./StoreSearchSettings.svelte";
  import StoreServiceSettings from "./StoreServiceSettings.svelte";
  import type { StoreSettingsSection } from "./model.js";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";

  let {
    controller,
    section,
  }: {
    controller: StoreController;
    section: StoreSettingsSection;
  } = $props();
  onMount(() => {
    const timer = setInterval(() => {
      if (
        !document.hidden &&
        section !== "appearance" &&
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
    {:else if section === "repository"}
      <StoreRepositorySettings {controller} />
    {:else if section === "sync"}
      <StoreSyncSettings
        settings={controller.state.settings.syncSettings}
        supported={Boolean(
          controller.state.settings.capabilities?.local_sync_management,
        )}
        onRefresh={controller.actions.refreshSyncSettings}
        onControl={controller.actions.controlSync}
      />
    {:else}
      <StoreSearchSettings {controller} />
    {/if}
  </AppShell.Body.Content>
</AppShell.Body>
