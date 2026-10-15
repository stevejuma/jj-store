<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import { globalSettingsSkeletonCards } from "./global-settings-skeleton.js";
  import {
    globalSettingsTabs,
    type GlobalSettingsSection,
  } from "../../controller/global-settings-navigation.js";
  let {
    section = "service",
    tab,
    onSelect,
  }: {
    section?: GlobalSettingsSection;
    tab?: string;
    onSelect?: (tab: string) => void;
  } = $props();
  const selected = $derived(tab ?? globalSettingsTabs[section][0]);
  const labels: Record<string, string> = {
    theme: "Theme",
    preferences: "Preferences",
    overview: "Overview",
    limits: "Limits",
    launch: "Launch",
    protection: "Protection",
    storage: "Storage",
    workflows: "Workflows",
    runtimes: "Runtimes",
    capture: "Capture",
    configuration: "Configuration",
    sources: "Sources",
    imports: "Imports",
    diagnostics: "Diagnostics",
    models: "Models",
    resources: "Resources",
    sharing: "Sharing",
  };
</script>

<AppShell.Body layout="regions" label="Store settings">
  <div class="global-settings-loading">
    <StoreSettingsPage
      {section}
      tabs={globalSettingsTabs[section].map((value) => ({
        value,
        label: labels[value],
      }))}
      value={selected}
      {onSelect}
    >
      <StoreSettingsSkeleton
        heading={false}
        tab={selected}
        cards={globalSettingsSkeletonCards(section, selected)}
        label={`Loading ${section} settings`}
      />
    </StoreSettingsPage>
  </div>
</AppShell.Body>

<style>
  .global-settings-loading {
    display: flex;
    flex: 1;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
  }
</style>
