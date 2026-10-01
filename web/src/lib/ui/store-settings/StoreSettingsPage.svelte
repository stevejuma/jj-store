<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import type { StoreSettingsSection } from "./model.js";
  import "./StoreSettings.css";

  let {
    section,
    tabs = [],
    value = $bindable(""),
    children,
  }: {
    section: StoreSettingsSection;
    tabs?: readonly { value: string; label: string; disabled?: boolean }[];
    value?: string;
    children: Snippet;
  } = $props();

  const title = $derived(
    section === "service"
      ? "Local service"
      : section === "repository"
        ? "Repository"
        : section === "sync"
          ? "Sync"
          : "Search",
  );
  const description = $derived(
    section === "service"
      ? "Connection and storage state for this jj-store installation."
      : section === "repository"
        ? "Identity, snapshot, graph preparation and capabilities for the selected repository."
        : section === "sync"
          ? "Local readiness and controls for the configured upstream."
          : "Configuration, file scopes and text-search health for the selected repository.",
  );
</script>

{#snippet content()}
  <div class="store-settings" aria-labelledby="store-settings-title">
    <header class="store-settings__heading">
      <h1 id="store-settings-title">{title}</h1>
      <p>{description}</p>
    </header>
    {@render children()}
  </div>
{/snippet}

{#if tabs.length}
  <Tabs.Root bind:value class="store-settings__tabs">
    <div class="store-settings__tab-bar">
      <ScrollArea
        orientation="horizontal"
        type="hover"
        class="store-settings__tab-scroll"
      >
        <Tabs.List
          aria-label={`${title} settings sections`}
          class="store-settings__tab-list"
        >
          {#each tabs as tab (tab.value)}
            <Tabs.Trigger value={tab.value} disabled={tab.disabled}
              >{tab.label}</Tabs.Trigger
            >
          {/each}
        </Tabs.List>
      </ScrollArea>
    </div>
    {@render content()}
  </Tabs.Root>
{:else}
  {@render content()}
{/if}
