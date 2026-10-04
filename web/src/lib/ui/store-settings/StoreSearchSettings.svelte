<script lang="ts">
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import StoreSearchHealth from "./StoreSearchHealth.svelte";
  import StoreSearchConfiguration from "./StoreSearchConfiguration.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let active = $state("configuration");
  let { controller }: { controller: StoreController } = $props();
  const settings = $derived(controller.state.settings);
  const supported = $derived(
    active === "health"
      ? settings.capabilities?.search
      : settings.capabilities?.local_search_management,
  );
</script>

<StoreSettingsPage
  section="search"
  bind:value={active}
  onRefresh={supported
    ? active === "health"
      ? controller.actions.refreshSettings
      : controller.actions.refreshSearchSettings
    : undefined}
  refreshLabel={active === "health" ? "Refresh status" : "Refresh settings"}
  refreshDisabled={active === "health"
    ? ["loading", "refreshing"].includes(settings.searchStatusState)
    : settings.searchSettings.changing}
  tabs={[
    { value: "configuration", label: "Configuration" },
    { value: "scopes", label: "File scopes" },
    { value: "advanced", label: "Advanced" },
    { value: "health", label: "Health" },
  ]}
>
  {#each ["configuration", "scopes", "advanced"] as section}
    <Tabs.Content value={section}
      >{#if active === section}<StoreSearchConfiguration
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
