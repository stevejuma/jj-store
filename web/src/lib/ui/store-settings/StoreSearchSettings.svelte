<script lang="ts">
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import StoreSearchHealth from "./StoreSearchHealth.svelte";
  import StoreSearchConfiguration from "./StoreSearchConfiguration.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let active = $state("configuration");
  let {
    controller,
    onClose,
  }: { controller: StoreController; onClose?: () => void } = $props();
</script>

<StoreSettingsPage
  section="search"
  {onClose}
  bind:value={active}
  tabs={[
    { value: "configuration", label: "Configuration" },
    { value: "scopes", label: "File scopes" },
    { value: "health", label: "Health" },
  ]}
>
  {#each ["configuration", "scopes"] as section}
    <Tabs.Content value={section}
      >{#if active === section}<StoreSearchConfiguration
          section={section as "configuration" | "scopes"}
          settings={controller.state.settings.searchSettings}
          supported={controller.state.settings.capabilities
            ?.local_search_management === true}
          onRefresh={controller.actions.refreshSearchSettings}
          onDraft={controller.actions.setSearchDraft}
          onCancel={controller.actions.cancelSearchDraft}
          onSave={controller.actions.saveSearchSettings}
          onControl={controller.actions.controlSearch}
        />{/if}</Tabs.Content
    >
  {/each}
  <Tabs.Content value="health"><StoreSearchHealth {controller} /></Tabs.Content>
</StoreSettingsPage>
