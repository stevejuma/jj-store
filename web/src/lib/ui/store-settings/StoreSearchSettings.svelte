<script lang="ts">
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import StoreSearchHealth from "./StoreSearchHealth.svelte";
  import StoreSearchConfiguration from "./StoreSearchConfiguration.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let active = $state("configuration");
  let { controller }: { controller: StoreController } = $props();
</script>

<Tabs.Root bind:value={active} class="store-settings__tabs">
  <Tabs.List
    aria-label="Search settings sections"
    class="store-settings__tab-list"
  >
    <Tabs.Trigger value="configuration">Configuration</Tabs.Trigger>
    <Tabs.Trigger value="scopes">File scopes</Tabs.Trigger>
    <Tabs.Trigger value="health">Health</Tabs.Trigger>
  </Tabs.List>
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
</Tabs.Root>
