<script lang="ts">
  import ShieldIcon from "@lucide/svelte/icons/shield";
  import RocketIcon from "@lucide/svelte/icons/arrow-up";
  import GaugeIcon from "@lucide/svelte/icons/gauge";
  import FolderSearchIcon from "@lucide/svelte/icons/folder";
  import DownloadIcon from "@lucide/svelte/icons/download";
  import CpuIcon from "@lucide/svelte/icons/server";
  import NetworkIcon from "@lucide/svelte/icons/network";
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
      return embedded ? "repository" : "global";
    },
  });
  const tabIcons: Record<string, Component> = {
    theme: PaletteIcon,
    preferences: SlidersHorizontalIcon,
    limits: GaugeIcon,
    launch: RocketIcon,
    protection: ShieldIcon,
    storage: DatabaseIcon,
    runtimes: BotIcon,
    capture: ActivityIcon,
    sources: FolderSearchIcon,
    imports: DownloadIcon,
    diagnostics: ActivityIcon,
    models: BotIcon,
    resources: CpuIcon,
    sharing: NetworkIcon,
    overview: FileTextIcon,
    maintenance: WrenchIcon,
    configuration: SlidersHorizontalIcon,
    scopes: ListFilterIcon,
    advanced: Settings2Icon,
    health: ActivityIcon,
    progress: ChartNoAxesCombinedIcon,
  };
  const presentedTabs = $derived(
    tabs.map((tab) => ({ ...tab, Icon: tab.Icon ?? tabIcons[tab.value] })),
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
    class:store-settings--global={!embedded}
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
  <ScrollArea
    class="store-settings__scroll"
    aria-label={`${title} settings content`}
  >
    {@render content()}
  </ScrollArea>
{/snippet}

{#snippet navigation()}
  <StoreViewTabs
    inheritContext
    {value}
    sections={presentedTabs}
    presentation="line"
    onSelect={(next) => {
      if (onSelect) onSelect(next);
      else value = next;
    }}
    label={`${title} settings sections`}
    navigationLabel="Settings page navigation"
  ></StoreViewTabs>
{/snippet}

{#if !embedded}
  <header class="store-settings__global-header">
    <h1 id={titleId} class="store-settings__page-title">
      <PageIcon aria-hidden="true" /><span>{title}</span>
    </h1>
    {#if onRefresh}<Tooltip.Provider
        ><Tooltip.Root
          ><Tooltip.Trigger>
            {#snippet child({ props })}<Button
                {...props}
                variant="ghost"
                size="icon-sm"
                aria-label={refreshLabel}
                disabled={refreshDisabled}
                onclick={() => void onRefresh?.()}
                ><RefreshCwIcon aria-hidden="true" /></Button
              >{/snippet}
          </Tooltip.Trigger><Tooltip.Content>{refreshLabel}</Tooltip.Content
          ></Tooltip.Root
        ></Tooltip.Provider
      >{/if}
  </header>
{/if}
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
    class="store-settings__tabs store-settings__tabs--modern"
  >
    {@render navigation()}
    {@render body()}
  </Tabs.Root>
{:else}
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
  .store-settings
    :global(
      .store-settings__section:first-child:not(.store-settings-save-card)
    ) {
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
  .store-settings__global-header {
    display: flex;
    flex: none;
    align-items: center;
    justify-content: space-between;
    min-height: 3rem;
    padding: 0.5rem 1rem;
    gap: 0.5rem;
    border-bottom: 1px solid var(--border);
  }
  @media (max-width: 40rem) {
    .store-settings {
      padding-block-end: 3rem;
    }
  }
</style>
