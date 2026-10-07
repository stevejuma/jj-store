<script lang="ts">
  import { untrack } from "svelte";
  import { observeStoreSettings } from "../../controller/settings-refresh.js";
  import { AppShell } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import { settingsNavigationReason } from "./settings-navigation.js";
  import { storeErrorMessage } from "../../controller/store-error.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreAppearance from "../store-appearance/StoreAppearance.svelte";
  import StoreRepositorySettings from "./StoreRepositorySettings.svelte";
  import StoreSyncReconciliation from "./StoreSyncReconciliation.svelte";
  import StoreStorageSettings from "./StoreStorageSettings.svelte";
  import StoreSyncSettings from "./StoreSyncSettings.svelte";
  import StoreSearchSettings from "./StoreSearchSettings.svelte";
  import StoreComputeSettings from "./StoreComputeSettings.svelte";
  import StoreAgentSettings from "./StoreAgentSettings.svelte";
  import StoreServiceSettings from "./StoreServiceSettings.svelte";
  import type { StoreSettingsSection } from "./model.js";
  import StoreConfigurationEditor from "./StoreConfigurationEditor.svelte";
  import StoreDeploymentConfiguration from "./StoreDeploymentConfiguration.svelte";
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
  let repositoryError = $state("");
  async function openCaptureRepository(id: string) {
    repositoryError = "";
    try {
      await controller.actions.openRepository(id);
      if (
        controller.state.route.repository === id &&
        controller.state.status.connection !== "failed"
      )
        onClose?.();
      else
        repositoryError =
          controller.state.status.error ||
          "Repository is unavailable. Refresh repositories before trying again.";
    } catch (cause) {
      repositoryError = storeErrorMessage(cause);
    }
  }
  const repository = $derived(controller.state.route.repository);
  $effect(() => {
    const selected = section;
    repository;
    if (["appearance", "agents", "search"].includes(selected)) return;
    return untrack(() => observeStoreSettings(controller, selected));
  });
</script>

<AppShell.Body layout="regions" label="Store settings">
  <AppShell.Body.Content label="Settings content">
    {#if section === "appearance"}
      <StoreSettingsPage section="appearance"
        ><StoreAppearance /></StoreSettingsPage
      >
    {:else if section === "service"}
      <StoreServiceSettings {controller} />
    {:else if section === "agents"}
      <StoreNotice
        noticeKey="capture-repository"
        title="Could not open repository"
        message={repositoryError}
      />
      <StoreAgentSettings
        manager={controller.execution}
        management={controller.capture}
        runtimes={controller.execution?.tooling}
        onRepository={(id) => void openCaptureRepository(id)}
        repositoryNavigationReason={settingsNavigationReason(
          controller.state.settings,
        )}
        repositoryName={(id) =>
          controller.state.repositories.find((r) => r.id === id)?.name ??
          "Previously bound repository"}
      />
    {:else if section === "compute"}
      <StoreComputeSettings management={controller.execution?.compute} />
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
        onRefreshConfiguration={controller.configuration?.observe}
        configurationBusy={controller.configuration?.state.saving ?? false}
        onRetry={controller.actions.retryStorage}
        >{#snippet configuration()}<StoreDeploymentConfiguration
            management={controller.configuration}
            section="storage"
          />{/snippet}</StoreStorageSettings
      >
    {:else if section === "sync"}
      <StoreSyncSettings
        settings={controller.state.settings.syncSettings}
        supported={Boolean(
          controller.state.settings.capabilities?.local_sync_management,
        )}
        onRefresh={controller.actions.refreshSyncSettings}
        onRefreshConfiguration={controller.configuration?.observe}
        configurationBusy={controller.configuration?.state.saving ?? false}
        onControl={controller.actions.controlSync}
        onWait={controller.actions.waitForSync}
        onStopWait={controller.actions.stopSyncWait}
        >{#snippet configuration()}<StoreConfigurationEditor
            management={controller.configuration}
            section="sync"
            repository={controller.state.repository.profile === "git_sha1_jj045"
              ? ""
              : controller.state.repository.id}
            repositoryName={controller.state.repository.name}
          />{/snippet}
        {#snippet reconciliation()}<StoreSyncReconciliation
            {controller}
          />{/snippet}</StoreSyncSettings
      >
    {:else}
      <StoreSearchSettings {controller} />
    {/if}
  </AppShell.Body.Content>
</AppShell.Body>
