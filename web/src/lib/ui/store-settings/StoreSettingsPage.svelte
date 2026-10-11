<script lang="ts">
  import type { Component, Snippet } from "svelte";
  import { provideSettingsPresentation } from "./settings-presentation.js";
  import FileTextIcon from "@lucide/svelte/icons/file-text";
  import WrenchIcon from "@lucide/svelte/icons/wrench";
  import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
  import ListFilterIcon from "@lucide/svelte/icons/list-filter";
  import Settings2Icon from "@lucide/svelte/icons/settings-2";
  import ActivityIcon from "@lucide/svelte/icons/activity";
  import ChartNoAxesCombinedIcon from "@lucide/svelte/icons/chart-no-axes-combined";
  import BotIcon from "@lucide/svelte/icons/bot";
  import PaletteIcon from "@lucide/svelte/icons/palette";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import SearchIcon from "@lucide/svelte/icons/search";
  import ServerIcon from "@lucide/svelte/icons/server";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreViewTabs from "../store-application/StoreViewTabs.svelte";
  import type { StoreSettingsPageSection } from "./model.js";
  import "./StoreSettings.css";

  let {
    section,
    tabs = [],
    value = $bindable(""),
    onRefresh,
    refreshLabel = "Refresh settings",
    refreshDisabled = false,
    embedded = false,
    onSelect,
    children,
  }: {
    section: StoreSettingsPageSection;
    tabs?: readonly {
      value: string;
      label: string;
      disabled?: boolean;
      Icon?: Component;
    }[];
    value?: string;
    onRefresh?: () => Promise<void>;
    refreshLabel?: string;
    refreshDisabled?: boolean;
    embedded?: boolean;
    onSelect?: (value: string) => void;
    children: Snippet;
  } = $props();

  provideSettingsPresentation({
    get value() {
      return embedded ? "repository" : "default";
    },
  });
  const tabIcons: Record<string, Component> = {
    overview: FileTextIcon,
    maintenance: WrenchIcon,
    configuration: SlidersHorizontalIcon,
    scopes: ListFilterIcon,
    advanced: Settings2Icon,
    health: ActivityIcon,
    progress: ChartNoAxesCombinedIcon,
  };
  const presentedTabs = $derived(
    embedded
      ? tabs.map((tab) => ({ ...tab, Icon: tab.Icon ?? tabIcons[tab.value] }))
      : tabs,
  );
  const titleId = $props.id();
  const title = $derived(
    section === "appearance"
      ? "Appearance"
      : section === "service"
        ? "Local service"
        : section === "agents"
          ? "Agents"
          : section === "compute"
            ? "Compute"
            : section === "repository"
              ? "General"
              : section === "folders"
                ? "Local folders"
                : section === "advanced"
                  ? "Advanced"
                  : section === "storage"
                    ? "Storage"
                    : section === "sync"
                      ? "Sync"
                      : "Search",
  );
  const PageIcon = $derived(
    section === "appearance"
      ? PaletteIcon
      : section === "service" || section === "compute"
        ? ServerIcon
        : section === "agents"
          ? BotIcon
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
  <div
    class="store-settings"
    class:store-settings--repository={embedded}
    aria-labelledby={titleId}
  >
    {#if embedded && section === "advanced"}<header
        class="store-settings__content-heading"
      >
        <h1 id={titleId}>Repository capabilities</h1>
        <p>
          Technical details about this repository, including indexing, API
          support and operational limits.
        </p>
      </header>{/if}
    {@render children()}
  </div>
{/snippet}

{#snippet body()}
  {#if embedded}
    <ScrollArea
      class="store-settings__scroll"
      aria-label={`${title} settings content`}
    >
      {@render content()}
    </ScrollArea>
  {:else}
    {@render content()}
  {/if}
{/snippet}

{#snippet navigation()}
  <StoreViewTabs
    inheritContext
    {value}
    sections={presentedTabs}
    presentation={embedded ? "line" : "pill"}
    onSelect={(next) => {
      if (onSelect) onSelect(next);
      else value = next;
    }}
    label={`${title} settings sections`}
    navigationLabel="Settings page navigation"
  >
    {#snippet leading()}
      {#if !embedded}<h1 id={titleId} class="store-settings__page-title">
          <PageIcon aria-hidden="true" /><span>{title}</span>
        </h1>{/if}
    {/snippet}
    {#snippet trailing()}
      {#if onRefresh && !embedded}
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
    {/snippet}
  </StoreViewTabs>
{/snippet}

{#if embedded && section !== "advanced"}<h1
    id={titleId}
    class="store-settings__sr-only"
  >
    {title}
  </h1>{/if}
{#if tabs.length}
  <Tabs.Root
    {value}
    onValueChange={(next) => {
      if (onSelect) onSelect(next);
      else value = next;
    }}
    class={embedded
      ? "store-settings__tabs store-settings__tabs--repository"
      : "store-settings__tabs"}
  >
    {@render navigation()}
    {@render body()}
  </Tabs.Root>
{:else}
  {#if !embedded}{@render navigation()}{/if}
  {@render body()}
{/if}

<style>
  .store-settings__sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
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
    margin-inline-end: 1rem;
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
