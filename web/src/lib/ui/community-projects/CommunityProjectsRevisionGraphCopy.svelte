<script lang="ts">
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import type { RepositoryLogGraphRow } from "../../projects/repository-log-graph.js";
  import CommunityProjectsGitAuthorAvatar from "./CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraphMessage from "./CommunityProjectsRevisionGraphMessage.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  import type { StoreControllerState } from "../../controller/contracts.js";
  import { revisionCollaboration } from "../../collaboration/revision-collaboration.js";
  import CommunityProjectsRevisionCollaboration from "./CommunityProjectsRevisionCollaboration.svelte";

  let {
    row,
    members = [],
    avatarSize = "small",
    selected = false,
    now,
    href,
    onSelect,
    collaborationContext,
  }: {
    row: RepositoryLogGraphRow;
    members?: readonly CommunityMemberModel[];
    avatarSize?: "small" | "xsmall";
    selected?: boolean;
    now?: number;
    href?: string | undefined;
    onSelect?: (() => void) | undefined;
    collaborationContext?: StoreControllerState | undefined;
  } = $props();
  const activity = $derived(
    collaborationContext
      ? revisionCollaboration(row.revision, collaborationContext)
      : undefined,
  );
</script>

<span class="community-revision-graph-copy">
  <CommunityProjectsGitAuthorAvatar
    {members}
    email={row.revision.authorEmail}
    name={row.revision.author}
    size={avatarSize}
  />
  <span class="community-revision-graph-copy__body">
    <span class="community-revision-graph-copy__message">
      {#if selected}<CommunityChip size="compact" variant="muted"
          >Selected</CommunityChip
        >{/if}
      <CommunityProjectsRevisionGraphMessage
        message={row.revision.message}
        {href}
        {onSelect}
      /></span
    >
    <span class="community-revision-graph-copy__meta">
      <CommunityRelativeTime timestamp={row.revision.createdAt} {now} />
    </span>
    {#if row.revision.reason === "merge" || row.revision.divergent || row.revision.conflicted || row.revision.bookmarks.length > 0 || row.revision.workspaces.length > 0 || row.revision.operationSnapshot || row.deviceAliases.length > 0}
      <span class="community-revision-graph-copy__chips">
        {#if row.revision.reason === "merge"}
          <CommunityChip size="compact" variant="muted" ariaHidden
            >Merge</CommunityChip
          >
        {/if}
        {#if row.revision.divergent}
          <CommunityChip size="compact" tone="warning">Divergent</CommunityChip>
        {/if}
        {#if row.revision.conflicted}
          <CommunityChip size="compact" tone="danger">Conflicted</CommunityChip>
        {/if}
        {#each row.revision.bookmarks as bookmark (`${bookmark.name}:${bookmark.remote ?? "local"}:${bookmark.added}:${bookmark.removed}`)}
          <CommunityChip
            size="compact"
            tone={bookmark.conflicted
              ? "danger"
              : bookmark.remote
                ? "neutral"
                : "accent"}
            title={bookmark.removed && !bookmark.added
              ? `Removed bookmark target · ${bookmark.name}${bookmark.remote ? `@${bookmark.remote}` : ""}`
              : bookmark.remote
                ? `Remote bookmark · ${bookmark.name}@${bookmark.remote}`
                : `Local bookmark · ${bookmark.name}`}
            ><span class="community-revision-graph-copy__chip-label"
              >{bookmark.name}{bookmark.remote
                ? `@${bookmark.remote}`
                : ""}</span
            ></CommunityChip
          >
        {/each}
        {#each row.revision.workspaces as workspace (workspace)}
          <CommunityChip
            size="compact"
            tone="success"
            title={`Workspace ${workspace}`}
            ><span class="community-revision-graph-copy__chip-label"
              >{workspace === "default"
                ? "Working copy"
                : `Working copy · ${workspace}`}</span
            ></CommunityChip
          >
        {/each}
        {#if row.revision.operationSnapshot}
          <CommunityChip size="compact" variant="muted">Snapshot</CommunityChip>
        {/if}
        {#each row.deviceAliases as alias (alias)}
          <CommunityChip size="compact" tone="accent" title={alias}
            ><span class="community-revision-graph-copy__chip-label"
              >{alias}</span
            ></CommunityChip
          >
        {/each}
      </span>
    {/if}
  </span>
  {#if activity}<CommunityProjectsRevisionCollaboration {activity} />{/if}
</span>

<style>
  .community-revision-graph-copy {
    display: flex;
    min-width: 0;
    flex: 1 1 auto;
    align-items: center;
    gap: 0.55rem;
  }

  .community-revision-graph-copy__message {
    display: flex;
    align-items: center;
    min-width: 0;
    gap: 0.35rem;
  }

  .community-revision-graph-copy__body {
    display: flex;
    min-width: 0;
    flex: 1 1 auto;
    flex-direction: column;
    gap: 0.15rem;
  }

  .community-revision-graph-copy__body
    :global(.community-revision-graph-message) {
    width: 100%;
  }

  .community-revision-graph-copy__meta {
    display: flex;
    min-width: 0;
    flex-wrap: wrap;
    align-items: center;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }

  .community-revision-graph-copy__chips {
    display: flex;
    min-width: 0;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.3rem;
  }
  .community-revision-graph-copy__chips :global(.community-chip) {
    max-inline-size: min(16rem, 100%);
    min-inline-size: 0;
  }
  .community-revision-graph-copy__chip-label {
    min-inline-size: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
