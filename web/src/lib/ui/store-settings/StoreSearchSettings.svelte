<script lang="ts">
  import { untrack } from "svelte";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import StoreSearchHealth from "./StoreSearchHealth.svelte";
  import StoreSearchConfiguration from "./StoreSearchConfiguration.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import { observeStoreSettings } from "../../controller/settings-refresh.js";
  let active = $state("configuration");
  let {
    controller,
    value,
    onSelect,
    embedded = false,
    visible = true,
  }: {
    controller: StoreController;
    value?: string;
    onSelect?: (value: string) => void;
    embedded?: boolean;
    visible?: boolean;
  } = $props();
  const selected = $derived(value ?? active);
  const settings = $derived(controller.state.settings);
  const supported = $derived(
    selected === "health"
      ? settings.capabilities?.search
      : settings.capabilities?.local_search_management,
  );
  const repository = $derived(controller.state.route.repository);
  $effect(() => {
    const section = selected === "health" ? "search_health" : "search";
    repository;
    if (!visible) return;
    return untrack(() => observeStoreSettings(controller, section));
  });
</script>

<StoreSettingsPage
  {embedded}
  section="search"
  value={selected}
  onSelect={onSelect ?? ((next) => (active = next))}
  onRefresh={supported
    ? selected === "health"
      ? () => controller.actions.refreshSettings("search_health")
      : controller.actions.refreshSearchSettings
    : undefined}
  refreshLabel={selected === "health" ? "Refresh status" : "Refresh settings"}
  refreshDisabled={selected === "health"
    ? ["loading", "refreshing"].includes(settings.searchStatusState)
    : settings.searchSettings.changing}
  tabs={[
    ...(settings.capabilities?.local_search_management &&
    controller.state.repository.permission === "admin"
      ? [
          { value: "configuration", label: "Configuration" },
          { value: "scopes", label: "File scopes" },
          { value: "advanced", label: "Advanced" },
        ]
      : []),
    ...(settings.capabilities?.search
      ? [{ value: "health", label: "Health" }]
      : []),
  ]}
>
  {#each ["configuration", "scopes", "advanced"] as section}
    <Tabs.Content value={section}
      >{#if selected === section}<StoreSearchConfiguration
          section={section as "configuration" | "scopes" | "advanced"}
          settings={controller.state.settings.searchSettings}
          supported={controller.state.settings.capabilities
            ?.local_search_management === true}
          onDraft={controller.actions.setSearchDraft}
          onCancel={controller.actions.cancelSearchDraft}
          onSave={controller.actions.saveSearchSettings}
          onControl={controller.actions.controlSearch}
        />{/if}</Tabs.Content
    >
  {/each}
  <Tabs.Content value="health"><StoreSearchHealth {controller} /></Tabs.Content>
</StoreSettingsPage>
