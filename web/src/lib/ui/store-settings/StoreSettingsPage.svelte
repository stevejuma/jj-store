<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import SearchIcon from "@lucide/svelte/icons/search";
  import ServerIcon from "@lucide/svelte/icons/server";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import type { StoreSettingsSection } from "./model.js";
  import "./StoreSettings.css";

  let {
    section,
    onClose,
    tabs = [],
    value = $bindable(""),
    children,
  }: {
    section: StoreSettingsSection;
    onClose?: () => void;
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
  const PageIcon = $derived(
    section === "service"
      ? ServerIcon
      : section === "repository"
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
    {#if onClose}<Button
        variant="ghost"
        size="icon"
        class="store-settings__back"
        style="--ui-button-radius: 9999px"
        aria-label="Back to app"
        title="Back to app"
        onclick={onClose}
        ><ArrowLeftIcon data-icon="inline-start" aria-hidden="true" /></Button
      >{/if}
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
    <h1 id="store-settings-title" class="store-settings__page-title">
      <PageIcon aria-hidden="true" />
      <span>{title}</span>
    </h1>
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
    padding: 1rem 1rem 4rem;
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
    margin-inline-start: auto;
    font-size: var(--community-text-body);
    font-weight: 600;
    white-space: nowrap;
  }
  .store-settings__page-title :global(svg) {
    width: 1rem;
    height: 1rem;
  }
  @media (max-width: 40rem) {
    .store-settings {
      padding-block-end: 3rem;
    }
  }
</style>
