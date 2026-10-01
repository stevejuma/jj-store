<script lang="ts">
  import { AppShell } from "@lapismd/design-core/shell";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreRepositorySettings from "./StoreRepositorySettings.svelte";
  import StoreSearchSettings from "./StoreSearchSettings.svelte";
  import StoreServiceSettings from "./StoreServiceSettings.svelte";
  import type { StoreSettingsSection } from "./model.js";
  import "./StoreSettings.css";

  let {
    controller,
    section,
  }: { controller: StoreController; section: StoreSettingsSection } = $props();

  const title = $derived(
    section === "service"
      ? "Local service"
      : section === "repository"
        ? "Repository"
        : "Search",
  );
  const description = $derived(
    section === "service"
      ? "Connection and storage state for this jj-store installation."
      : section === "repository"
        ? "Identity, snapshot, graph preparation and capabilities for the selected repository."
        : "Text-search projection health for the selected repository.",
  );
</script>

<AppShell.Body layout="regions" label="Store settings">
  <AppShell.Body.Content>
    <div class="store-settings" aria-labelledby="store-settings-title">
      <header class="store-settings__heading">
        <h1 id="store-settings-title">{title}</h1>
        <p>{description}</p>
      </header>
      {#if section === "service"}
        <StoreServiceSettings {controller} />
      {:else if section === "repository"}
        <StoreRepositorySettings {controller} />
      {:else}
        <StoreSearchSettings {controller} />
      {/if}
    </div>
  </AppShell.Body.Content>
</AppShell.Body>
