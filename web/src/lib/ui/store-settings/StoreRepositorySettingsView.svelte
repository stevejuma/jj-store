<script lang="ts">
  import { untrack } from "svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type {
    RepositorySettingsSection,
    RepositorySettingsTab,
  } from "../../controller/repository-settings-navigation.js";
  import {
    repositorySettingsSections,
    supportedSettingsDestination,
  } from "./repository-settings-model.js";
  import { observeStoreSettings } from "../../controller/settings-refresh.js";
  import StoreViewTabs from "../store-application/StoreViewTabs.svelte";
  import StoreRepositorySettings from "./StoreRepositorySettings.svelte";
  import StoreSearchSettings from "./StoreSearchSettings.svelte";
  import StoreStorageSettings from "./StoreStorageSettings.svelte";
  import StoreSyncSettings from "./StoreSyncSettings.svelte";
  import StoreSyncReconciliation from "./StoreSyncReconciliation.svelte";
  import StoreConfigurationEditor from "./StoreConfigurationEditor.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  let {
    controller,
    visible = true,
  }: { controller: StoreController; visible?: boolean } = $props();
  const view = $derived(controller.state);
  const ready = $derived(
    view.status.connection === "connected" &&
      view.repository.id === view.route.repository &&
      Boolean(view.settings.capabilities),
  );
  const sections = $derived(repositorySettingsSections(view));
  const destination = $derived(
    supportedSettingsDestination(view, controller.configuration?.available()),
  );
  const activeSection = $derived(destination.section);
  const refresh = $derived(
    destination.section === "folders" &&
      view.repository.profile !== "git_sha1_jj045"
      ? controller.actions.refreshWorkspaceLinks
      : destination.section === "search"
        ? destination.tab === "health"
          ? () => controller.actions.refreshSettings("search_health")
          : controller.actions.refreshSearchSettings
        : destination.section === "storage"
          ? controller.actions.refreshStorage
          : destination.section === "sync"
            ? destination.tab === "configuration"
              ? controller.configuration?.observe
              : controller.actions.refreshSyncSettings
            : undefined,
  );
  const refreshBusy = $derived(
    view.settings.workspaceLinking ||
      view.settings.workspacePicking ||
      view.settings.searchSettings.changing ||
      view.settings.storageSettings.changing ||
      view.settings.syncSettings.changing ||
      Boolean(controller.configuration?.state.saving),
  );
  let notice = $state("");
  let visited = $state<RepositorySettingsSection[]>([]);
  $effect(() => {
    if (visible && ready && !visited.includes(destination.section))
      visited = [...visited, destination.section];
  });
  function select(
    section: RepositorySettingsSection,
    tab?: RepositorySettingsTab,
  ) {
    void controller.actions.selectRepositorySettings(section, tab);
  }
  $effect(() => {
    const { section, tab, unsupported } = destination;
    if (!visible || !ready || !unsupported) return;
    // Wait for Sync's first response before deciding whether Progress exists.
    if (
      section === "sync" &&
      ["idle", "loading", "refreshing"].includes(
        view.settings.syncSettings.state,
      )
    )
      return;
    untrack(() => {
      notice =
        "That setting is not available for this repository. Showing supported settings.";
      void controller.actions.selectRepositorySettings(section, tab, true);
    });
  });
  $effect(() => {
    const section = activeSection;
    if (!visible || !ready || section === "search") return;
    return untrack(() =>
      observeStoreSettings(
        controller,
        section === "folders" ? "repository" : section,
      ),
    );
  });
</script>

<StoreViewTabs
  value={destination.section}
  sections={ready ? sections : []}
  label="Repository settings categories"
  navigationLabel="Repository settings navigation"
  onSelect={(value) => select(value as RepositorySettingsSection)}
>
  {#snippet leading()}<Tooltip.Provider
      ><Tooltip.Root
        ><Tooltip.Trigger>
          {#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon"
              style="--ui-button-radius: 999px"
              aria-label="Back to repository"
              onclick={() => void controller.actions.leaveRepositorySettings()}
              ><ArrowLeftIcon aria-hidden="true" /></Button
            >{/snippet}
        </Tooltip.Trigger><Tooltip.Content>Back to repository</Tooltip.Content
        ></Tooltip.Root
      ></Tooltip.Provider
    >{/snippet}
  {#snippet trailing()}{#if refresh}<div
        class="store-repository-settings__actions"
      >
        <Tooltip.Provider
          ><Tooltip.Root
            ><Tooltip.Trigger>
              {#snippet child({ props })}<Button
                  {...props}
                  variant="ghost"
                  size="icon-sm"
                  aria-label={destination.section === "search" &&
                  destination.tab === "health"
                    ? "Refresh status"
                    : "Refresh settings"}
                  disabled={refreshBusy}
                  onclick={() => void refresh?.()}
                  ><RefreshIcon aria-hidden="true" /></Button
                >{/snippet}
            </Tooltip.Trigger><Tooltip.Content
              >{destination.section === "search" && destination.tab === "health"
                ? "Refresh status"
                : "Refresh settings"}</Tooltip.Content
            ></Tooltip.Root
          ></Tooltip.Provider
        >
      </div>{/if}{/snippet}
</StoreViewTabs>
<StoreNotice
  noticeKey="repository-settings-destination"
  title="Settings location changed"
  message={notice}
  severity="info"
  lifetime="timed"
/>
{#if !ready}<div
    role="status"
    aria-label="Loading repository settings"
    class="store-repository-settings__loading"
  >
    <Skeleton class="h-12" /><Skeleton class="h-24" />
  </div>{/if}
{#each visited as section (section)}
  <div hidden={!visible || destination.section !== section}>
    {#if ["general", "folders", "advanced"].includes(section)}
      <StoreRepositorySettings
        {controller}
        section={section as "general" | "folders" | "advanced"}
        visible={visible && destination.section === section}
      />
    {:else if section === "search"}
      <StoreSearchSettings
        embedded
        {controller}
        value={destination.section === section ? destination.tab : undefined}
        visible={visible && destination.section === section}
        onSelect={(tab) => select("search", tab as RepositorySettingsTab)}
      />
    {:else if section === "storage"}
      <StoreStorageSettings
        embedded
        settings={view.settings.storageSettings}
        supported
        value={destination.section === section ? destination.tab : undefined}
        onSelect={(tab) => select("storage", tab as RepositorySettingsTab)}
        onRefresh={controller.actions.refreshStorage}
        onStart={controller.actions.startStorage}
        onRetry={controller.actions.retryStorage}
      />
    {:else if section === "sync"}
      <StoreSyncSettings
        embedded
        configurationSupported={controller.configuration?.available() ?? false}
        settings={view.settings.syncSettings}
        supported
        value={destination.section === section ? destination.tab : undefined}
        onSelect={(tab) => select("sync", tab as RepositorySettingsTab)}
        onRefresh={controller.actions.refreshSyncSettings}
        onControl={controller.actions.controlSync}
        onWait={controller.actions.waitForSync}
        onStopWait={controller.actions.stopSyncWait}
        onRefreshConfiguration={controller.configuration?.observe}
        configurationBusy={controller.configuration?.state.saving ?? false}
      >
        {#snippet configuration()}{#if controller.configuration?.available()}<StoreConfigurationEditor
              management={controller.configuration}
              section="sync"
              repository={view.repository.id}
              repositoryName={view.repository.name}
            />{/if}{/snippet}
        {#snippet reconciliation()}<StoreSyncReconciliation
            {controller}
          />{/snippet}
      </StoreSyncSettings>
    {/if}
  </div>
{/each}

<style>
  .store-repository-settings__loading {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  [hidden] {
    display: none;
  }
  .store-repository-settings__actions {
    margin-inline-start: auto;
  }
</style>
