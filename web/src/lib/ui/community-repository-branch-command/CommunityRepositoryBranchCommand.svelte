<script lang="ts">
  import "../community-projects/repository-row-tokens.css";
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import CommunityWorkingCopyCard from "./CommunityWorkingCopyCard.svelte";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { Bookmark } from "../../projects/presentation.js";
  import type { RepositoryWorkingCopy } from "../../projects/repository-log.js";
  import CommunityRepositoryBranchEmpty from "./CommunityRepositoryBranchEmpty.svelte";
  import type { ScrollAreaStrategy } from "@lapismd/design-core/shadcn/scroll-area";
  let {
    bookmarks,
    targetBookmark,
    groupWorkspaceBookmarks = false,
    scrollStrategy = "auto",
    workingCopies = [],
    selected = "",
    selectedCommit = null,
    selectedWorkspace = null,
    now,
    hasMore = false,
    loading = false,
    onSelect,
    onSelectWorkingCopy = () => {},
    onLoadMore,
  }: {
    bookmarks: readonly Bookmark[];
    /** Optional workspace target card, independent of the loaded bookmark filters. */
    targetBookmark?: Bookmark;
    /** Group the loaded applied bookmarks without changing repository picker defaults. */
    groupWorkspaceBookmarks?: boolean;
    scrollStrategy?: ScrollAreaStrategy;
    workingCopies?: readonly RepositoryWorkingCopy[];
    selected?: string;
    selectedCommit?: string | null;
    selectedWorkspace?: string | null;
    now?: number;
    hasMore?: boolean;
    loading?: boolean;
    onSelect: (name: string) => void;
    onSelectWorkingCopy?: (commit: string, workspace?: string) => void;
    onLoadMore?: () => void;
  } = $props();
  let query = $state(""),
    filter = $state("all"),
    commandValue = $state("");
  const visible = $derived.by(() => {
    const search = query.trim().toLocaleLowerCase();
    return bookmarks.filter((bookmark) => {
      const inFilter =
        filter === "all" ||
        (filter === "workspace"
          ? bookmark.workspace
          : bookmark.targets.length > 1);
      const searchable = [bookmark.name, bookmark.description, bookmark.author]
        .filter(Boolean)
        .join(" ")
        .toLocaleLowerCase();
      return inFilter && (!search || searchable.includes(search));
    });
  });
  const groups = $derived(
    groupWorkspaceBookmarks
      ? [
          {
            heading: "Applied",
            bookmarks: visible.filter((item) => item.workspace),
          },
          {
            heading: "Local branches",
            bookmarks: visible.filter((item) => !item.workspace),
          },
        ].filter((group) => group.bookmarks.length)
      : [{ heading: "Bookmarks", bookmarks: visible }],
  );
</script>

{#snippet bookmarkRow(bookmark: Bookmark)}
  <CommandView.Item
    value={bookmark.name}
    aria-label={bookmark.name}
    class="community-bookmark-card"
    data-current={bookmark.name === selected || undefined}
    onSelect={() => onSelect(bookmark.name)}
  >
    <span class="community-bookmark-card__heading"
      ><GitBranchIcon aria-hidden="true" /><CommandView.ItemLabel
        >{bookmark.name}</CommandView.ItemLabel
      >
      {#if bookmark.workspace}<CommunityChip size="compact" tone="success"
          >Workspace</CommunityChip
        >{:else if bookmark.default}<CommunityChip
          size="compact"
          variant="muted">default</CommunityChip
        >{/if}
    </span>
    <CommandView.ItemDescription
      >{bookmark.description ?? "No description"}</CommandView.ItemDescription
    >
    {#if bookmark.targets.length > 1}<span
        ><CommunityChip tone="warning" size="compact"
          >Conflicted · {bookmark.targets.length} targets</CommunityChip
        ></span
      >{:else if bookmark.targets.length === 0}<span
        ><CommunityChip size="compact">deleted</CommunityChip></span
      >{/if}
    <span class="community-bookmark-card__footer">
      <CommunityProjectsGitAuthorAvatar
        name={bookmark.author ?? "Unknown author"}
        email={bookmark.authorEmail}
        size="xsmall"
      />
      <span>local</span><span class="community-bookmark-card__author"
        >{#if bookmark.updatedAt}<CommunityRelativeTime
            timestamp={bookmark.updatedAt}
            {now}
          />{" by "}
        {/if}{bookmark.author ?? "Unknown author"}</span
      >
    </span>
  </CommandView.Item>
{/snippet}

<CommandView.Root
  bind:value={commandValue}
  label="Bookmarks"
  class="community-bookmark-cards"
  shouldFilter={false}
  disablePointerSelection
>
  {#if targetBookmark}
    <CommandView.List {scrollStrategy} aria-label="Workspace target branch">
      <CommandView.Group heading="Target branch">
        {@render bookmarkRow(targetBookmark)}
      </CommandView.Group>
    </CommandView.List>
  {/if}
  {#each workingCopies as workingCopy (workingCopy.workspace)}
    <CommunityWorkingCopyCard
      copy={workingCopy}
      {selectedCommit}
      {selectedWorkspace}
      {now}
      onSelect={onSelectWorkingCopy}
    />
  {/each}
  <CommandView.Filters
    bind:value={filter}
    onValueChange={() => (commandValue = "")}
    label="Bookmark filters"
    tabs={[
      { id: "all", label: "All" },
      { id: "workspace", label: "Workspace" },
      { id: "conflicted", label: "Conflicted" },
    ]}
  />
  <CommandView.Input
    bind:value={query}
    placeholder="Search branches"
    aria-label="Search branches"
  />
  <CommandView.List {scrollStrategy}>
    {#if visible.length === 0}
      <CommunityRepositoryBranchEmpty {filter} {query} />
    {:else}
      {#each groups as group, index (group.heading)}
        <CommandView.Group heading={group.heading}>
          <CommunityProjectsVirtualRows
            rows={group.bookmarks}
            row={bookmarkRow}
            getKey={(bookmark) => bookmark.name}
            estimateSize={() => 132}
            hasMore={index === groups.length - 1 && hasMore}
            {loading}
            loadingLabel="Loading more bookmarks"
            {onLoadMore}
          />
        </CommandView.Group>
      {/each}
    {/if}
  </CommandView.List>
</CommandView.Root>

<style>
  :global(.community-bookmark-cards) {
    --ui-command-view-background: var(--jj-store-repository-row-background);
    --ui-command-view-foreground: var(--foreground);
    --ui-command-view-list-max-height: max(12rem, calc(100dvh - 16rem));
    --ui-command-view-list-padding: 0;
    --ui-command-view-item-radius: 0px;
    --ui-command-view-item-selected-background: var(
      --jj-store-repository-row-hover-background
    );
    --ui-command-view-radius: 0px;
  }
  :global(.community-bookmark-cards [data-ui-part="command-group"]) {
    padding: 0;
    overflow: visible;
  }
  :global(.community-bookmark-cards [data-ui-part="command-group-heading"]) {
    position: sticky;
    top: 0;
    z-index: 2;
    padding: 0.5rem 0.9rem;
    background: var(--jj-store-row-background);
    border-bottom: 1px solid var(--border);
  }
  :global(
    .community-bookmark-cards
      [data-ui-component="command-view"][data-ui-part="item"].community-bookmark-card
  ) {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 0.5rem;
    padding: 0.9rem;
    border-radius: 0;
    border-bottom: 1px solid var(--border);
    white-space: normal;
    background: var(--jj-store-repository-row-background);
  }
  :global(
    [data-ui-component="command-view"][data-ui-part="item"].community-bookmark-card[data-current="true"]
  ) {
    background: var(--jj-store-repository-row-selected-background);
  }
  :global(
    .community-bookmark-cards
      [data-ui-component="command-view"][data-ui-part="item"].community-bookmark-card:is(
        :hover,
        :focus-visible
      ),
    .community-bookmark-cards:has(input:focus-visible)
      [data-ui-component="command-view"][data-ui-part="item"].community-bookmark-card[data-selected]
  ) {
    background: var(--jj-store-repository-row-hover-background);
  }
  :global(
    [data-ui-component="command-view"][data-ui-part="item"].community-bookmark-card[data-current="true"]
  )::before {
    content: "";
    position: absolute;
    inset-block: 1rem;
    inset-inline-start: 0;
    width: 3px;
    background: var(--primary);
    border-radius: 0 3px 3px 0;
  }
  .community-bookmark-card__heading {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }
  .community-bookmark-card__heading :global(svg) {
    width: 1rem;
    height: 1rem;
    flex: none;
    color: var(--muted-foreground);
  }
  .community-bookmark-card__heading :global([data-ui-part="item-label"]) {
    flex: 1;
    min-width: 0;
    font-size: var(--community-text-body);
    font-weight: 650;
    overflow-wrap: anywhere;
  }
  .community-bookmark-card__footer {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.4rem;
    padding-top: 0.5rem;
    border-top: 1px solid var(--border);
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
  .community-bookmark-card__author {
    margin-inline-start: auto;
  }
</style>
