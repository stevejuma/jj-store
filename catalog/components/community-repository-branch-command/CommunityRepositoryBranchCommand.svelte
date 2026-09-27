<script lang="ts">
  import * as CommandView from "@lapismd/design-core/shadcn/command-view";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { Bookmark } from "../../projects/presentation.js";
  let {
    bookmarks,
    selected = "",
    now,
    onSelect,
  }: {
    bookmarks: readonly Bookmark[];
    selected?: string;
    now?: number;
    onSelect: (name: string) => void;
  } = $props();
  let query = $state(""),
    filter = $state("all");
  const visible = $derived(
    bookmarks.filter(
      (bookmark) =>
        filter === "all" ||
        (filter === "workspace"
          ? bookmark.workspace
          : bookmark.targets.length > 1),
    ),
  );
</script>

<CommandView.Root label="Bookmarks" class="community-bookmark-cards">
  <CommandView.Filters
    bind:value={filter}
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
    <CommandView.Empty>No matching bookmarks</CommandView.Empty>
    <CommandView.Group heading="Bookmarks">
      {#each visible as bookmark (bookmark.name)}
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
            >{bookmark.description ??
              "No description"}</CommandView.ItemDescription
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
      {/each}
    </CommandView.Group>
  </CommandView.List>
</CommandView.Root>

<style>
  :global(.community-bookmark-cards) {
    --ui-command-view-list-max-height: max(12rem, calc(100dvh - 16rem));
    --ui-command-view-list-padding: 0;
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
    background: var(--secondary);
    border-bottom: 1px solid var(--border);
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
  /* Command items have individual wrappers; ignore empty filtered wrappers. */
  :global(
    .community-bookmark-cards
      [data-item-wrapper]:has(.community-bookmark-card):not(
        :has(~ [data-item-wrapper] .community-bookmark-card)
      )
      > .community-bookmark-card
  ) {
    border-bottom: 0;
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
