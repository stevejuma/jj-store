<script lang="ts">
  import "./CommunityNavigation.css";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { AppShell } from "@lapismd/design-core/shell";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import { type Snippet } from "svelte";
  import CommunityNavigationSearch from "./CommunityNavigationSearch.svelte";
  import CommunityNavigationAction from "./CommunityNavigationAction.svelte";
  import CommunityNavigationSectionPicker from "./CommunityNavigationSectionPicker.svelte";
  import {
    type CommunityNavigationItemModel,
    type CommunityNavigationModel,
    type CommunityNavigationSectionModel,
    communityNavigationSectionKind,
  } from "./model.js";
  import { navigationItemIcon, navigationSectionIcon } from "./icons.js";
  let {
    model,
    activeItemId,
    compact = false,
    sectionOpen = $bindable<Record<string, boolean>>({}),
    itemOpen = $bindable<Record<string, boolean>>({}),
    onSelect,
    onSearch,
    footer,
    onCompactSectionChange,
  }: {
    model: CommunityNavigationModel;
    activeItemId?: string;
    compact?: boolean;
    sectionOpen?: Record<string, boolean>;
    itemOpen?: Record<string, boolean>;
    onSelect: (item: CommunityNavigationItemModel) => void;
    onSearch?: (query: string) => void;
    footer?: Snippet;
    onCompactSectionChange?: (id: string) => void;
  } = $props();
  let compactSectionId = $state("");
  let compactTooltipId = $state("");
  let compactSectionTrigger: HTMLButtonElement | undefined;
  let compactSectionTriggerId = "";
  function sectionContainsActive(
    section: CommunityNavigationSectionModel,
  ): boolean {
    return section.items.some(
      (item) =>
        item.id === activeItemId ||
        item.children?.some((child) => child.id === activeItemId),
    );
  }

  function isItemOpen(item: CommunityNavigationItemModel): boolean {
    return (
      itemOpen[item.id] ??
      (item.children?.some((child) => child.id === activeItemId) ||
        item.initiallyExpanded ||
        false)
    );
  }

  function setItemOpen(itemId: string, open: boolean): void {
    itemOpen = { ...itemOpen, [itemId]: open };
  }

  function setCompactSectionOpen(sectionId: string, open: boolean): void {
    if (open) {
      compactSectionId = sectionId;
      onCompactSectionChange?.(sectionId);
      compactTooltipId = "";
    } else if (compactSectionId === sectionId) {
      compactSectionId = "";
      onCompactSectionChange?.("");
      const trigger =
        compactSectionTriggerId === sectionId
          ? compactSectionTrigger
          : undefined;
      queueMicrotask(() => trigger?.focus());
    }
  }

  function selectCompactItem(item: CommunityNavigationItemModel): void {
    compactSectionId = "";
    onCompactSectionChange?.("");
    onSelect(item);
  }

  function sortedItems(section: CommunityNavigationSectionModel) {
    return section.items;
  }

  function isSectionOpen(section: CommunityNavigationSectionModel): boolean {
    return sectionOpen[section.id] ?? !section.initiallyCollapsed;
  }

  function setSectionOpen(sectionId: string, open: boolean): void {
    sectionOpen = { ...sectionOpen, [sectionId]: open };
  }
</script>

{#snippet itemIcon(item: CommunityNavigationItemModel)}
  {@const Icon = navigationItemIcon(item)}<Icon aria-hidden="true" />
{/snippet}
{#if onSearch}<CommunityNavigationSearch {compact} {onSearch} />{/if}
<AppShell.Sidebar.Body class="lc-community-navigation__body">
  <Tooltip.Provider>
    <nav aria-label="Repository navigation" data-compact={compact || undefined}>
      <div class="lc-community-navigation__primary">
        {#each model.primaryItems as item (item.id)}
          {#if compact}
            <Tooltip.Root>
              <Tooltip.Trigger>
                {#snippet child({ props })}
                  <Button
                    {...props}
                    variant="ghost"
                    size="icon-sm"
                    class="lc-community-navigation__item"
                    data-active={activeItemId === item.id ? "" : undefined}
                    aria-current={activeItemId === item.id ? "page" : undefined}
                    aria-label={item.label}
                    onclick={() => onSelect(item)}
                  >
                    {@render itemIcon(item)}
                  </Button>
                {/snippet}
              </Tooltip.Trigger>
              <Tooltip.Content side="right" sideOffset={8}
                >{item.label}</Tooltip.Content
              >
            </Tooltip.Root>
          {:else}
            <Button
              variant="ghost"
              size="default"
              class="lc-community-navigation__item"
              data-active={activeItemId === item.id ? "" : undefined}
              aria-current={activeItemId === item.id ? "page" : undefined}
              aria-label={item.label}
              title={item.label}
              onclick={() => onSelect(item)}
            >
              {@render itemIcon(item)}
              <span>{item.label}</span>
            </Button>
          {/if}
        {/each}
      </div>

      {#each model.sections as section (section.id)}
        {@const actions =
          section.actions ?? (section.action ? [section.action] : [])}
        {#if compact}
          {@const SectionIcon = navigationSectionIcon(section.id)}

          <div class="lc-community-navigation__compact-section">
            <Popover.Root
              open={compactSectionId === section.id}
              onOpenChange={(open) => setCompactSectionOpen(section.id, open)}
            >
              <Tooltip.Root
                open={compactTooltipId === section.id}
                disabled={compactSectionId === section.id}
              >
                <Tooltip.Trigger>
                  {#snippet child({ props: tooltipProps })}
                    <span
                      {...tooltipProps}
                      class="lc-community-navigation__compact-tooltip-anchor"
                      onpointerenter={() => {
                        compactTooltipId = section.id;
                      }}
                      onpointerleave={() => {
                        compactTooltipId = "";
                      }}
                    >
                      <Popover.Trigger>
                        {#snippet child({ props })}
                          <Button
                            {...props}
                            variant="ghost"
                            size="icon-sm"
                            class="lc-community-navigation__compact-section-trigger"
                            data-active={sectionContainsActive(section)
                              ? ""
                              : undefined}
                            data-navigation-section-kind={communityNavigationSectionKind(
                              section,
                            )}
                            aria-current={sectionContainsActive(section)
                              ? "page"
                              : undefined}
                            aria-label={section.label}
                            onfocus={(event) => {
                              compactSectionTrigger = event.currentTarget;
                              compactSectionTriggerId = section.id;
                            }}
                          >
                            <SectionIcon aria-hidden="true" />
                          </Button>
                        {/snippet}
                      </Popover.Trigger>
                    </span>
                  {/snippet}
                </Tooltip.Trigger>
                <Tooltip.Content side="right" sideOffset={8}
                  >{section.label}</Tooltip.Content
                >
              </Tooltip.Root>
              <Popover.Content
                side="right"
                align="start"
                sideOffset={16}
                collisionPadding={8}
                class="lc-community-navigation__section-picker"
              >
                <Popover.Title class="lc-community-navigation__sr-only"
                  >{section.label}</Popover.Title
                >
                <CommunityNavigationSectionPicker
                  {section}
                  {activeItemId}
                  onSelect={selectCompactItem}
                />
              </Popover.Content>
            </Popover.Root>
          </div>
        {:else}
          <Collapsible.Root
            class="lc-community-navigation__section"
            data-navigation-section-kind={communityNavigationSectionKind(
              section,
            )}
            open={isSectionOpen(section)}
            onOpenChange={(open) => setSectionOpen(section.id, open)}
          >
            <div class="lc-community-navigation__section-header">
              <h2>
                <Collapsible.Trigger
                  class="lc-community-navigation__section-trigger"
                  title={`Toggle ${section.label}`}
                >
                  <ChevronRightIcon aria-hidden="true" />
                  <span>{section.label}</span>
                </Collapsible.Trigger>
              </h2>
              {#each actions as action (action.id)}
                <CommunityNavigationAction item={action} {onSelect} />
              {/each}
            </div>
            <Collapsible.Content>
              <div class="lc-community-navigation__items">
                {#each sortedItems(section) as item (item.id)}
                  <Collapsible.Root
                    class="lc-community-navigation__item-tree"
                    open={item.children?.length ? isItemOpen(item) : false}
                    onOpenChange={(open) => setItemOpen(item.id, open)}
                  >
                    <div class="lc-community-navigation__item-row">
                      <Button
                        variant="ghost"
                        size="default"
                        class="lc-community-navigation__item"
                        data-active={activeItemId === item.id ? "" : undefined}
                        aria-current={activeItemId === item.id
                          ? "page"
                          : undefined}
                        aria-label={item.label}
                        title={item.label}
                        onclick={() => onSelect(item)}
                      >
                        {@render itemIcon(item)}
                        <span>{item.label}</span>
                      </Button>

                      {#if item.children?.length}
                        <Collapsible.Trigger
                          class="lc-community-navigation__item-toggle"
                          aria-label={`${isItemOpen(item) ? "Collapse" : "Expand"} ${item.label}`}
                          title={`${isItemOpen(item) ? "Collapse" : "Expand"} ${item.label}`}
                        >
                          <ChevronRightIcon aria-hidden="true" />
                        </Collapsible.Trigger>
                      {/if}
                    </div>
                    {#if item.children?.length}
                      <Collapsible.Content>
                        <div class="lc-community-navigation__item-children">
                          {#each item.children as child (child.id)}
                            <Button
                              variant="ghost"
                              size="default"
                              class="lc-community-navigation__item lc-community-navigation__item--child"
                              data-active={activeItemId === child.id
                                ? ""
                                : undefined}
                              aria-current={activeItemId === child.id
                                ? "page"
                                : undefined}
                              aria-label={child.label}
                              title={child.label}
                              onclick={() => onSelect(child)}
                            >
                              {@render itemIcon(child)}
                              <span>{child.label}</span>
                            </Button>
                          {/each}
                        </div>
                      </Collapsible.Content>
                    {/if}
                  </Collapsible.Root>
                {/each}
              </div>
            </Collapsible.Content>
          </Collapsible.Root>
        {/if}
      {/each}
    </nav>
  </Tooltip.Provider>
</AppShell.Sidebar.Body>

{#if footer}
  <AppShell.Sidebar.Footer>
    {@render footer()}
  </AppShell.Sidebar.Footer>
{/if}
