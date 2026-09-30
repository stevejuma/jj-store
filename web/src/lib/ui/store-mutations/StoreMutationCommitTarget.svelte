<script lang="ts">
  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";

  let {
    target,
    revision,
    now,
    copyableIds = true,
    framed = true,
    ariaLabel = "Selected commit",
  }: {
    target: string;
    revision?: RepositoryLogRevision;
    now?: number;
    copyableIds?: boolean;
    framed?: boolean;
    ariaLabel?: string;
  } = $props();

  const bookmarks = $derived(
    revision?.bookmarks.filter(
      (bookmark) => bookmark.added && !bookmark.removed,
    ) ?? [],
  );

  function bookmarkLabel(bookmark: RepositoryLogRevision["bookmarks"][number]) {
    return bookmark.remote
      ? `${bookmark.name}@${bookmark.remote}`
      : bookmark.name;
  }
</script>

<div
  class="store-mutation-commit-target"
  data-framed={framed || undefined}
  role={ariaLabel ? "group" : undefined}
  aria-label={ariaLabel || undefined}
>
  <CommunityProjectsGitAuthorAvatar
    name={revision?.author ?? "Unknown author"}
    email={revision?.authorEmail}
    size="small"
  />
  <span class="store-mutation-commit-target__body">
    <span class="store-mutation-commit-target__primary">
      <span
        class="store-mutation-commit-target__message"
        title={revision?.message}>{revision?.message ?? "Selected commit"}</span
      >
      {#if copyableIds}
        <CommunityProjectsRevisionGraphIds
          changeId={revision?.changeId}
          commitId={target}
          placement="inline"
        />
      {:else}
        <span
          class="store-mutation-commit-target__ids"
          aria-label="Revision identifiers"
        >
          {#if target}<span
              data-id-kind="git"
              title={`Git commit SHA ${target}`}
            >
              <CommunityChip
                size="indicator"
                shape="pill"
                variant="muted"
                ariaHidden>GIT</CommunityChip
              ><code>{target.slice(0, 5)}</code>
            </span>{/if}
          {#if revision?.changeId}<span
              data-id-kind="jj"
              title={`JJ Change ID ${revision.changeId}`}
            >
              <CommunityChip
                size="indicator"
                shape="pill"
                variant="muted"
                ariaHidden>JJ</CommunityChip
              ><code>{revision.changeId.slice(0, 5)}</code>
            </span>{/if}
        </span>
      {/if}
    </span>
    {#if revision}
      <span class="store-mutation-commit-target__secondary">
        {#if bookmarks.length || revision.workspaces.length}
          <span class="store-mutation-commit-target__bookmarks">
            {#each bookmarks as bookmark (`${bookmark.name}:${bookmark.remote ?? "local"}`)}
              <CommunityChip size="compact" variant="muted"
                >{bookmarkLabel(bookmark)}</CommunityChip
              >
            {/each}
            {#each revision.workspaces as workspace (workspace)}
              <CommunityChip size="compact" tone="success"
                >{workspace}</CommunityChip
              >
            {/each}
          </span>
        {/if}
        <CommunityRelativeTime timestamp={revision.createdAt} {now} />
      </span>
    {/if}
  </span>
</div>

<style>
  .store-mutation-commit-target {
    display: grid;
    width: 100%;
    min-width: 0;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: center;
    gap: 0.5rem;
    box-sizing: border-box;
    color: inherit;
    text-align: start;
  }

  .store-mutation-commit-target[data-framed="true"] {
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm, 0.375rem);
    background: var(--background);
  }

  .store-mutation-commit-target__body {
    display: grid;
    min-width: 0;
    gap: 0.4rem;
  }

  .store-mutation-commit-target__primary {
    display: grid;
    min-width: 0;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: 0.5rem;
  }

  .store-mutation-commit-target__message {
    min-width: 0;
    overflow: hidden;
    font-size: var(--community-text-body);
    font-weight: 550;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .store-mutation-commit-target__ids > span,
  .store-mutation-commit-target__bookmarks {
    display: flex;
    min-width: 0;
    align-items: center;
  }

  .store-mutation-commit-target__ids {
    display: grid;
    grid-template-columns: 4.5rem 4rem;
    align-items: center;
    gap: 0.35rem;
  }

  .store-mutation-commit-target__ids > [data-id-kind="git"] {
    grid-column: 1;
  }

  .store-mutation-commit-target__ids > [data-id-kind="jj"] {
    grid-column: 2;
  }

  .store-mutation-commit-target__ids :global(.community-chip) {
    --community-text-secondary: 0.625rem;
  }

  .store-mutation-commit-target__ids > span {
    gap: 0.2rem;
  }

  .store-mutation-commit-target__ids code {
    font-size: var(--community-text-secondary);
  }

  .store-mutation-commit-target__bookmarks {
    flex-wrap: wrap;
    gap: 0.3rem;
  }

  .store-mutation-commit-target__secondary {
    display: grid;
    min-width: 0;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: end;
    gap: 0.5rem;
  }

  .store-mutation-commit-target :global(.community-revision-graph-ids) {
    min-width: 0;
  }

  .store-mutation-commit-target__secondary :global(time) {
    grid-column: 2;
    justify-self: end;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
    white-space: nowrap;
  }
</style>
