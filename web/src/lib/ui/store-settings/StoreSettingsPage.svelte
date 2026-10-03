<script lang="ts">
  import type { Snippet } from "svelte";
  import PaletteIcon from "@lucide/svelte/icons/palette";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import SearchIcon from "@lucide/svelte/icons/search";
  import ServerIcon from "@lucide/svelte/icons/server";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import type { StoreSettingsSection } from "./model.js";
  import "./StoreSettings.css";

  let {
    section,
    tabs = [],
    value = $bindable(""),
    onRefresh,
    refreshLabel = "Refresh settings",
    refreshDisabled = false,
    children,
  }: {
    section: StoreSettingsSection;
    tabs?: readonly { value: string; label: string; disabled?: boolean }[];
    value?: string;
    onRefresh?: () => Promise<void>;
    refreshLabel?: string;
    refreshDisabled?: boolean;
    children: Snippet;
  } = $props();

  const title = $derived(
    section === "appearance"
      ? "Appearance"
      : section === "service"
        ? "Local service"
        : section === "repository"
          ? "Repository"
          : section === "storage"
            ? "Storage"
            : section === "sync"
              ? "Sync"
              : "Search",
  );
  const PageIcon = $derived(
    section === "appearance"
      ? PaletteIcon
      : section === "service"
        ? ServerIcon
        : section === "repository"
          ? DatabaseIcon
          : section === "storage"
            ? DatabaseIcon
            : section === "sync"
              ? RefreshCwIcon
              : SearchIcon,
  );
</script>

{#snippet content()}
  <div class="store-settings" aria-labelledby="store-settings-title">
    {@render children()}
  </div>
{/snippet}

{#snippet navigation()}
  <nav class="store-settings__tab-bar" aria-label="Settings page navigation">
    <h1 id="store-settings-title" class="store-settings__page-title">
      <PageIcon aria-hidden="true" />
      <span>{title}</span>
    </h1>
    {#if tabs.length}
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
    {/if}
    {#if onRefresh}
      <div class="store-settings__nav-actions">
        <Tooltip.Provider>
          <Tooltip.Root>
            <Tooltip.Trigger>
              {#snippet child({ props })}
                <Button
                  {...props}
                  variant="ghost"
                  size="icon-sm"
                  aria-label={refreshLabel}
                  disabled={refreshDisabled}
                  onclick={() => void onRefresh?.()}
                >
                  <RefreshCwIcon aria-hidden="true" />
                </Button>
              {/snippet}
            </Tooltip.Trigger>
            <Tooltip.Content>{refreshLabel}</Tooltip.Content>
          </Tooltip.Root>
        </Tooltip.Provider>
      </div>
    {/if}
  </nav>
{/snippet}

{#if tabs.length}
  <Tabs.Root bind:value class="store-settings__tabs">
    {@render navigation()}
    {@render content()}
  </Tabs.Root>
{:else}
  {@render navigation()}
  {@render content()}
{/if}

<style>
  .store-settings {
    width: 100%;
    min-width: 0;
    padding: 0 1rem 4rem;
    box-sizing: border-box;
  }
  .store-settings :global(.store-settings__section:first-child) {
    margin-block-start: 0;
  }
  .store-settings__page-title {
    display: flex;
    flex: none;
    align-items: center;
    gap: 0.4rem;
    margin: 0;
    font-size: var(--community-text-body);
    font-weight: 600;
    white-space: nowrap;
  }
  .store-settings__page-title :global(svg) {
    width: 1rem;
    height: 1rem;
  }
  .store-settings__nav-actions {
    flex: none;
    margin-inline-start: auto;
  }
  @media (max-width: 40rem) {
    .store-settings {
      padding-block-end: 3rem;
    }
  }
</style>
