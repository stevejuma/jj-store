<script lang="ts">
  import { untrack } from "svelte";
  import { observeStoreSettings } from "../../controller/settings-refresh.js";
  import { AppShell } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import { settingsNavigationReason } from "./settings-navigation.js";
  import { storeErrorMessage } from "../../controller/store-error.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreAppearance from "../store-appearance/StoreAppearance.svelte";
  import StoreComputeSettings from "./StoreComputeSettings.svelte";
  import StoreAgentSettings from "./StoreAgentSettings.svelte";
  import StoreServiceSettings from "./StoreServiceSettings.svelte";
  import type { StoreSettingsSection } from "./model.js";

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
    if (["appearance", "agents", "compute"].includes(selected)) return;
    return untrack(() => observeStoreSettings(controller, selected));
  });
</script>

<AppShell.Body layout="regions" label="Store settings">
  <div
    class="store-global-settings-body"
    role="region"
    aria-label="Settings content"
  >
    {#if section === "appearance"}
      <StoreAppearance />
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
    {/if}
  </div>
</AppShell.Body>

<style>
  .store-global-settings-body {
    display: flex;
    flex: 1;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
  }
</style>
