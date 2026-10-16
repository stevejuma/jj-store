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
  import StoreProfileSettings from "./StoreProfileSettings.svelte";
  import StoreCustomEmojiSettings from "./StoreCustomEmojiSettings.svelte";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreServiceSettings from "./StoreServiceSettings.svelte";
  import StoreIntegrations from "./StoreIntegrations.svelte";
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
  const selectedTab = $derived(controller.state.route.globalSettings?.tab);
  function selectTab(tab: string) {
    controller.actions.selectGlobalSettings(section, tab);
  }
  const repository = $derived(controller.state.route.repository);
  $effect(() => {
    const selected = section;
    repository;
    if (
      [
        "appearance",
        "agents",
        "compute",
        "profile",
        "custom-emoji",
        "integrations",
      ].includes(selected)
    )
      return;
    return untrack(() => observeStoreSettings(controller, selected));
  });
</script>

<AppShell.Body layout="regions" label="Store settings">
  <div
    class="store-global-settings-body"
    role="region"
    aria-label="Settings content"
  >
    {#if section === "profile"}
      <StoreSettingsPage
        section="profile"
        onRefresh={async () => {
          await controller.directory?.observe();
        }}
        refreshLabel="Refresh profile"
        refreshDisabled={controller.directory?.state.busy}
      >
        <StoreProfileSettings management={controller.directory} />
      </StoreSettingsPage>
    {:else if section === "custom-emoji"}
      <StoreSettingsPage
        section="custom-emoji"
        onRefresh={async () => {
          await controller.directory?.observe();
        }}
        refreshLabel="Refresh custom emoji"
        refreshDisabled={controller.directory?.state.busy}
      >
        <StoreCustomEmojiSettings management={controller.directory} />
      </StoreSettingsPage>
    {:else if section === "appearance"}
      <StoreAppearance bind:active={() => selectedTab ?? "theme", selectTab} />
    {:else if section === "service"}
      <StoreServiceSettings
        {controller}
        bind:active={() => selectedTab ?? "overview", selectTab}
      />
    {:else if section === "integrations"}
      <StoreIntegrations {controller} />
    {:else if section === "agents"}
      <StoreNotice
        noticeKey="capture-repository"
        title="Could not open repository"
        message={repositoryError}
      />
      <StoreAgentSettings
        bind:tab={() => selectedTab ?? "runtimes", selectTab}
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
      <StoreComputeSettings
        management={controller.execution?.compute}
        bind:active={() => selectedTab ?? "models", selectTab}
      />
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
