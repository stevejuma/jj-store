<script lang="ts">
  import { onMount, tick } from "svelte";
  import {
    AppShell,
    type AppShellSidebarController,
  } from "@lapismd/design-core/shell";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import { Skeleton } from "@lapismd/design-core/shadcn/skeleton";
  import "../store-work/community-work-layout.css";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import ComparisonIcon from "@lucide/svelte/icons/arrow-left-right";
  import FolderIcon from "@lucide/svelte/icons/folder";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import CommunityProjectsRepositoryFileTree from "../community-projects/CommunityProjectsRepositoryFileTree.svelte";
  import type { WorkspaceBrowser } from "../../conversations/workspace-browser.svelte.js";
  import type { Model } from "../../api/client.js";
  let {
    browser,
    sidebarController,
  }: {
    browser: WorkspaceBrowser;
    sidebarController: AppShellSidebarController;
  } = $props();
  const listing = $derived(browser.listing);
  let body = $state<HTMLDivElement | null>(null);
  $effect(() => {
    const root = body,
      saved = listing;
    if (!root) return;
    let cancelled = false;
    const remember = (event: Event) => {
      if (
        event.target instanceof HTMLElement &&
        event.target.matches('[data-slot="scroll-area-viewport"]')
      )
        saved.scroll = event.target.scrollTop;
    };
    root.addEventListener("scroll", remember, true);
    void tick().then(() => {
      if (!cancelled) {
        const viewport = root.querySelector<HTMLElement>(
          '[data-slot="scroll-area-viewport"]',
        );
        if (viewport) viewport.scrollTop = saved.scroll;
      }
    });
    return () => {
      cancelled = true;
      root.removeEventListener("scroll", remember, true);
    };
  });
  onMount(() => {
    const timer = setInterval(() => {
      if (browser.railOpen) void browser.load();
    }, 15000);
    return () => clearInterval(timer);
  });
  const needsBase = $derived(
    browser.range === "branch" &&
      !browser.base &&
      listing.value?.comparison?.unavailable ===
        "Choose a comparison base. No default branch is configured.",
  );
  const baseLabel = $derived(
    browser.base
      ? (listing.value?.references.find((r) => r.commit === browser.base)
          ?.name ?? browser.base.slice(0, 8))
      : needsBase
        ? "Choose branch"
        : "Configured branch",
  );
  const ranges: { value: Model<"ComparisonRange">; label: string }[] = [
    { value: "branch", label: "Branch" },
    { value: "since_last_commit", label: "Since last commit" },
    { value: "last_turn", label: "Last turn" },
  ];
</script>

<Tabs.Root
  value={browser.tab}
  onValueChange={(value) =>
    void browser.selectTab(value as "files" | "changes")}
>
  {#snippet child({ props })}
    <AppShell.Sidebar
      {...props}
      side="right"
      data-conversation-workspace
      {sidebarController}
      constraintPriority={100}
      label="Workspace files"
      resizeLabel="Resize workspace files"
      closeable
    >
      <AppShell.Sidebar.Header class="community-projects">
        <Tabs.List
          class="community-projects__tab-list"
          aria-label="Workspace contents"
          title={browser.binding?.repositoryName}
        >
          <Tabs.Trigger class="community-projects__tab-trigger" value="files"
            >Files</Tabs.Trigger
          >
          <Tabs.Trigger class="community-projects__tab-trigger" value="changes"
            >Changes</Tabs.Trigger
          >
        </Tabs.List>
        <div class="workspace-files-refresh">
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="Refresh workspace files"
            disabled={listing.loading}
            onclick={() => void browser.load()}
            >{#if listing.loading}<Spinner />{:else}<RefreshIcon />{/if}</Button
          >
        </div>
      </AppShell.Sidebar.Header>
      <AppShell.Sidebar.Body bind:ref={body}>
        <Tabs.Content value={browser.tab} aria-busy={listing.loading}>
          <div class="workspace-files-content">
            {#if browser.tab === "changes"}
              <div class="workspace-files-comparison">
                <Select.Root
                  type="single"
                  value={browser.range}
                  onValueChange={(value) =>
                    void browser.compare(value as Model<"ComparisonRange">)}
                >
                  <Select.Trigger
                    size="sm"
                    aria-label="Comparison range"
                    title={ranges.find((r) => r.value === browser.range)?.label}
                    ><ComparisonIcon aria-hidden="true" /><span class="truncate"
                      >{ranges.find((r) => r.value === browser.range)
                        ?.label}</span
                    ></Select.Trigger
                  >
                  <Select.Content
                    ><Select.Group
                      >{#each ranges as range}<Select.Item value={range.value}
                          >{range.label}</Select.Item
                        >{/each}</Select.Group
                    ></Select.Content
                  >
                </Select.Root>
                {#if browser.range === "branch"}
                  <Select.Root
                    type="single"
                    value={browser.base ?? "default"}
                    onValueChange={(value) =>
                      void browser.compare(
                        "branch",
                        value === "default" ? null : value,
                      )}
                  >
                    <Select.Trigger
                      size="sm"
                      class="workspace-files-base"
                      aria-label="Comparison base"
                      title={baseLabel}
                      ><GitBranchIcon aria-hidden="true" /><span
                        class="truncate">{baseLabel}</span
                      ></Select.Trigger
                    >
                    <Select.Content
                      ><Select.Group
                        ><Select.Item value="default"
                          >Configured branch</Select.Item
                        >{#each Array.from(new Map((listing.value?.references ?? []).map( (r) => [r.commit, r] )).values()) as ref (ref.name)}<Select.Item
                            value={ref.commit}>{ref.name}</Select.Item
                          >{/each}</Select.Group
                      ></Select.Content
                    >
                  </Select.Root>
                {/if}
              </div>
            {/if}
            {#if listing.error}<p class="workspace-files-status" role="status">
                {listing.error}
              </p>{/if}
            {#if listing.value?.comparison?.unavailable && !needsBase}<p
                class="workspace-files-status"
                role="status"
              >
                {listing.value.comparison.unavailable}
              </p>{/if}
            {#if listing.value?.incomplete}<p
                class="workspace-files-status"
                role="status"
              >
                {listing.value.incomplete}
              </p>{/if}
            {#if !listing.loading && !listing.error && !listing.value?.files.length && !listing.value?.comparison?.unavailable}
              <Empty.Root
                ><Empty.Header
                  ><Empty.Media variant="icon"><FolderIcon /></Empty.Media
                  ><Empty.Title
                    >{browser.tab === "files"
                      ? "No workspace files"
                      : "No changes"}</Empty.Title
                  ><Empty.Description
                    >{browser.tab === "files"
                      ? "Tracked and nonignored files appear here."
                      : "No supported paths differ within this comparison."}</Empty.Description
                  ></Empty.Header
                ></Empty.Root
              >
            {:else if !listing.value && listing.loading}
              <div
                class="workspace-files-skeleton"
                aria-label="Loading workspace files"
                role="status"
              >
                {#each [1, 2, 3] as row (row)}<Skeleton
                    style="height: 2rem"
                  />{/each}
              </div>
            {:else if !needsBase}
              <CommunityProjectsRepositoryFileTree
                files={listing.value?.files ?? []}
                mode={browser.mode}
                collapsedDirectoryPaths={listing.collapsed}
                selectedPath={listing.selected}
                hasMore={!!listing.value?.next && !listing.loading}
                onSelectPath={(path) => void browser.select(path)}
                onLoadMore={() => void browser.load(true)}
                onModeChange={(mode) => (browser.mode = mode)}
                onDirectoryToggle={(path) => browser.toggleDirectory(path)}
              />
            {/if}
          </div>
        </Tabs.Content>
      </AppShell.Sidebar.Body>
    </AppShell.Sidebar>
  {/snippet}
</Tabs.Root>

<style>
  .workspace-files-refresh {
    margin-inline-start: auto;
    flex: none;
  }
  .workspace-files-comparison {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.25rem;
  }
  .workspace-files-comparison :global([data-slot="select-trigger"]) {
    min-width: 0;
    max-width: 100%;
  }
  .workspace-files-comparison :global(.workspace-files-base) {
    flex: 1 1 5rem;
    overflow: hidden;
  }
  .workspace-files-content {
    min-width: 0;
  }
  .workspace-files-comparison,
  .workspace-files-status,
  .workspace-files-skeleton {
    padding: 0.5rem;
  }
  .workspace-files-skeleton {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
</style>
