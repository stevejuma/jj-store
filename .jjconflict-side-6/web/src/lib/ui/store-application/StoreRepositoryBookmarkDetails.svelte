<script lang="ts">
  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import type { Bookmark } from "../../projects/presentation.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";

  let {
    bookmark,
    revisions,
    now,
  }: {
    bookmark: Bookmark;
    revisions: readonly RepositoryLogRevision[];
    now?: number;
  } = $props();

  const status = $derived(
    bookmark.targets.length === 0
      ? "Deleted"
      : bookmark.targets.length > 1
        ? "Conflicted"
        : bookmark.workspace
          ? "Workspace"
          : bookmark.default
            ? "Default"
            : "Local",
  );
  const revision = $derived(
    revisions.find((item) => item.eventId === bookmark.targets[0]),
  );
  const author = $derived(
    revision?.author ?? bookmark.author ?? "Unknown author",
  );
  const authorEmail = $derived(revision?.authorEmail ?? bookmark.authorEmail);
  const updatedAt = $derived(revision?.createdAt ?? bookmark.updatedAt);
</script>

<section aria-label={`Bookmark ${bookmark.name}`}>
  <h3>{bookmark.name}</h3>
  <div class="bookmark-revision" role="group" aria-label="Bookmark revision">
    <CommunityProjectsGitAuthorAvatar
      email={authorEmail}
      name={author}
      size="small"
    />
    <div class="bookmark-summary">
      <p title={revision?.message ?? bookmark.description}>
        {revision?.message ?? bookmark.description ?? "No commit message"}
      </p>
      <div class="bookmark-author">
        <span>{author}</span>
        {#if updatedAt}<CommunityRelativeTime
            timestamp={updatedAt}
            {now}
          />{/if}
      </div>
    </div>
  </div>
  <div class="metadata-row" role="group" aria-label="Bookmark status">
    <span>Status</span>
    <CommunityChip
      size="compact"
      variant="muted"
      tone={status === "Conflicted"
        ? "warning"
        : status === "Workspace"
          ? "success"
          : "neutral"}>{status}</CommunityChip
    >
  </div>
  <div class="targets" role="group" aria-label="Bookmark targets">
    <div class="metadata-row">
      <span>Targets</span>
      <CommunityChip size="compact" variant="muted"
        >{bookmark.targets.length}</CommunityChip
      >
    </div>
    {#if bookmark.targets.length}
      <div class="target-ids">
        {#each bookmark.targets as target (target)}
          <CommunityProjectsRevisionGraphIds
            commitId={target}
            placement="inline"
          />
        {/each}
      </div>
    {/if}
  </div>
</section>

<style>
  section {
    display: grid;
    min-width: 0;
    gap: 0.55rem;
  }

  h3 {
    min-width: 0;
    margin: 0.9rem 0.4rem 0.3rem;
    overflow: hidden;
    font-size: var(--community-text-body);
    font-weight: 700;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .bookmark-revision {
    display: grid;
    min-width: 0;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: start;
    gap: 0.45rem;
    padding-inline: 0.4rem;
  }

  .bookmark-summary {
    display: grid;
    min-width: 0;
    gap: 0.18rem;
  }

  .bookmark-summary p {
    min-width: 0;
    margin: 0;
    overflow: hidden;
    font-size: var(--community-text-secondary);
    font-weight: 550;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .bookmark-author {
    display: flex;
    min-width: 0;
    align-items: center;
    gap: 0.35rem;
    overflow: hidden;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
    white-space: nowrap;
  }

  .bookmark-author > span {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .metadata-row {
    display: flex;
    min-width: 0;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding-inline: 0.4rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  .targets,
  .target-ids {
    display: grid;
    min-width: 0;
    gap: 0.4rem;
  }

  .target-ids :global(.community-revision-graph-ids) {
    padding-inline: 0.4rem;
  }
</style>
