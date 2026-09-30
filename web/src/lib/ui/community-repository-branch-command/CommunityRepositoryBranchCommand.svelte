<script lang="ts">
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { Bookmark } from "../../projects/presentation.js";
  import type { RepositoryWorkingCopy } from "../../projects/repository-log.js";
  import CommunityRepositoryBranchEmpty from "./CommunityRepositoryBranchEmpty.svelte";
  let {
    bookmarks,
    workingCopies = [],
    selected = "",
    selectedCommit = null,
    now,
    hasMore = false,
    loading = false,
    onSelect,
    onSelectWorkingCopy = () => {},
    onLoadMore,
  }: {
    bookmarks: readonly Bookmark[];
    workingCopies?: readonly RepositoryWorkingCopy[];
    selected?: string;
    selectedCommit?: string | null;
    now?: number;
    hasMore?: boolean;
    loading?: boolean;
    onSelect: (name: string) => void;
    onSelectWorkingCopy?: (commit: string) => void;
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
>
  {#each workingCopies as workingCopy (workingCopy.workspace)}
    <button
      type="button"
      class="community-working-copy-card"
      data-current={workingCopy.commitId === selectedCommit || undefined}
      aria-label={`Working copy · ${workingCopy.workspace}`}
      onclick={() => onSelectWorkingCopy(workingCopy.commitId)}
    >
      <span class="community-bookmark-card__heading">
        <GitCommitIcon aria-hidden="true" />
        <strong>Working copy</strong>
        <CommunityChip size="compact" tone="success"
          >{workingCopy.workspace}</CommunityChip
        >
      </span>
      <span class="community-working-copy-card__description"
        >{workingCopy.message}</span
      >
      <span class="community-bookmark-card__footer">
        <CommunityProjectsGitAuthorAvatar
          name={workingCopy.author}
          email={workingCopy.authorEmail}
          size="xsmall"
        />
        <span>workspace</span><span class="community-bookmark-card__author"
          ><CommunityRelativeTime
            timestamp={workingCopy.createdAt}
            {now}
          />{" by "}{workingCopy.author}</span
        >
      </span>
    </button>
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
  <CommandView.List>
    {#if visible.length === 0}
      <CommunityRepositoryBranchEmpty {filter} {query} />
    {:else}
      <CommandView.Group heading="Bookmarks">
        <CommunityProjectsVirtualRows
          rows={visible}
          row={bookmarkRow}
          getKey={(bookmark) => bookmark.name}
          estimateSize={() => 132}
          {hasMore}
          {loading}
          loadingLabel="Loading more bookmarks"
          {onLoadMore}
        />
      </CommandView.Group>
    {/if}
  </CommandView.List>
</CommandView.Root>

<style>
  :global(.community-bookmark-cards) {
    --ui-command-view-list-max-height: max(12rem, calc(100dvh - 16rem));
    --ui-command-view-list-padding: 0;
    --ui-command-view-item-radius: 0px;
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
  .community-working-copy-card {
    position: relative;
    display: flex;
    width: 100%;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0.9rem;
    border: 0;
    border-bottom: 1px solid var(--border);
    border-radius: 0;
    color: var(--foreground);
    background: var(--background);
    font: inherit;
    text-align: start;
    cursor: pointer;
  }
  .community-working-copy-card:hover,
  .community-working-copy-card:focus-visible,
  .community-working-copy-card[data-current="true"] {
    background: var(--accent);
  }
  .community-working-copy-card:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: -2px;
  }
  .community-working-copy-card__description {
    color: var(--muted-foreground);
    font-size: var(--community-text-body);
  }
  .community-working-copy-card .community-bookmark-card__heading > strong {
    font-size: var(--community-text-body);
    font-weight: 650;
  }
  :global(.community-bookmark-card[data-ui-part="item"]) {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 0.5rem;
    padding: 0.9rem;
    border-radius: 0;
    border-bottom: 1px solid var(--border);
    white-space: normal;
  }
  :global(.community-bookmark-card[data-current="true"]) {
    background: var(--accent);
  }
  :global(.community-bookmark-card[data-current="true"])::before {
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
    flex-wrap: wrap;
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
