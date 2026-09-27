<script lang="ts">
  import "./CommunityNavigation.css";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import { AppShell } from "@lapismd/design-core/shell";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
  import HistoryIcon from "@lucide/svelte/icons/history";
  import FolderGit2Icon from "@lucide/svelte/icons/folder-git-2";
  import { type Component, type Snippet } from "svelte";
  import {
    type CommunityNavigationItemModel,
    type CommunityNavigationModel,
    type CommunityNavigationSectionModel,
    communityNavigationSectionKind,
  } from "./model.js";
  let {
    model,
    activeItemId,
    compact = false,
    sectionOpen = $bindable<Record<string, boolean>>({}),
    onSelect,
    headerStart,
    footer,
  }: {
    model: CommunityNavigationModel;
    activeItemId?: string;
    compact?: boolean;
    sectionOpen?: Record<string, boolean>;
    onSelect: (item: CommunityNavigationItemModel) => void;
    headerStart?: Snippet;
    footer?: Snippet;
  } = $props();
  let itemOpen = $state<Record<string, boolean>>({});
  let compactSectionId = $state("");
  let compactTooltipId = $state("");
  let compactSectionQuery = $state("");
  let compactSectionSearchInput = $state<HTMLInputElement | null>(null);
  let compactSectionTrigger: HTMLButtonElement | undefined;
  let compactSectionTriggerId = "";
  $effect(() => {
    if (compactSectionId && compactSectionSearchInput)
      compactSectionSearchInput.focus();
  });
  function iconFor(item: CommunityNavigationItemModel): Component {
    if (item.kind === "branches") return GitBranchIcon;
    if (item.kind === "log") return GitCommitIcon;
    if (item.kind === "oplog") return HistoryIcon;
    return FolderGit2Icon;
  }
  function iconForSection(
    _section: CommunityNavigationSectionModel,
  ): Component {
    return FolderGit2Icon;
  }
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
      item.children?.some((child) => child.id === activeItemId) ||
      item.initiallyExpanded ||
      itemOpen[item.id] ||
      false
    );
  }

  function setItemOpen(itemId: string, open: boolean): void {
    itemOpen = { ...itemOpen, [itemId]: open };
  }

  function compactItems(
    section: CommunityNavigationSectionModel,
  ): readonly CommunityNavigationItemModel[] {
    return section.items.flatMap((item) => [item, ...(item.children ?? [])]);
  }

  function setCompactSectionOpen(sectionId: string, open: boolean): void {
    if (open) {
      compactSectionId = sectionId;
      compactTooltipId = "";
      compactSectionQuery = "";
    } else if (compactSectionId === sectionId) {
      compactSectionId = "";
      const trigger =
        compactSectionTriggerId === sectionId
          ? compactSectionTrigger
          : undefined;
      queueMicrotask(() => trigger?.focus());
    }
  }

  function selectCompactItem(item: CommunityNavigationItemModel): void {
    compactSectionId = "";
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
  {@const Icon = iconFor(item)}<Icon aria-hidden="true" />
{/snippet}
{#if headerStart}<AppShell.Sidebar.Header
    class="lc-community-navigation__header"
  >
    {@render headerStart()}
  </AppShell.Sidebar.Header>{/if}
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
        {#if compact}
          {@const SectionIcon = iconForSection(section)}

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
                sideOffset={8}
                collisionPadding={8}
                class="lc-community-navigation__section-picker"
              >
                <Popover.Title class="lc-community-navigation__sr-only"
                  >{section.label}</Popover.Title
                >
                <CommandView.Root label={`${section.label} navigation`} loop>
                  <CommandView.Input
                    bind:ref={compactSectionSearchInput}
                    value={compactSectionQuery}
                    placeholder={`Search ${section.label.toLocaleLowerCase()}`}
                    aria-label={`Search ${section.label}`}
                    oninput={(event) => {
                      compactSectionQuery = event.currentTarget.value;
                    }}
                  />
                  <CommandView.List>
                    <CommandView.Empty
                      >No matching {section.label.toLocaleLowerCase()}</CommandView.Empty
                    >
                    {#if section.items.length}
                      <CommandView.Group heading={section.label}>
                        {#each compactItems(section) as item (item.id)}
                          <CommandView.Item
                            value={`${item.label} ${item.id}`}
                            data-active={activeItemId === item.id
                              ? ""
                              : undefined}
                            onSelect={() => selectCompactItem(item)}
                          >
                            <CommandView.ItemIcon>
                              {@render itemIcon(item)}
                            </CommandView.ItemIcon>
                            <CommandView.ItemLabel
                              >{item.label}</CommandView.ItemLabel
                            >
                          </CommandView.Item>
                        {/each}
                      </CommandView.Group>
                    {/if}
                  </CommandView.List>
                </CommandView.Root>
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
