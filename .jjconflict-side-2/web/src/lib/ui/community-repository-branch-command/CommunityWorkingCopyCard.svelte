<script lang="ts">
  import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { RepositoryWorkingCopy } from "../../projects/repository-log.js";
  let {
    copy,
    selectedCommit,
    selectedWorkspace,
    now,
    onSelect,
  }: {
    copy: RepositoryWorkingCopy;
    selectedCommit: string | null;
    selectedWorkspace: string | null;
    now?: number;
    onSelect: (commit: string, workspace?: string) => void;
  } = $props();
</script>

<button
  type="button"
  class="community-working-copy-card"
  data-current={(selectedWorkspace
    ? selectedWorkspace === copy.workspace
    : copy.commitId === selectedCommit) || undefined}
  aria-label={`Working copy · ${copy.workspace}`}
  onclick={() => onSelect(copy.commitId, copy.workspace)}
>
  <span class="community-bookmark-card__heading">
    <GitCommitIcon aria-hidden="true" />
    <strong>Working copy</strong>
    <CommunityChip size="compact" tone="success">{copy.workspace}</CommunityChip
    >
  </span>
  <span class="community-working-copy-card__description">{copy.message}</span>
  <span class="community-bookmark-card__footer">
    <CommunityProjectsGitAuthorAvatar
      name={copy.author}
      email={copy.authorEmail}
      size="xsmall"
    />
    <span>workspace</span><span class="community-bookmark-card__author"
      ><CommunityRelativeTime
        timestamp={copy.createdAt}
        {now}
      />{" by "}{copy.author}</span
    >
  </span>
</button>

<style>
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
    background: var(--jj-store-repository-row-background);
    font: inherit;
    text-align: start;
    cursor: pointer;
  }
  .community-working-copy-card[data-current="true"] {
    background: var(--jj-store-repository-row-selected-background);
  }
  .community-working-copy-card:hover,
  .community-working-copy-card:focus-visible {
    background: var(--jj-store-repository-row-hover-background);
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
