<script lang="ts">
  import {
    TagList,
    type TagListItem,
  } from "@lapismd/design-core/shadcn/tag-list";
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
  const tags = $derived.by((): TagListItem[] => {
    const revision = row.revision;
    return [
      ...(revision.reason === "merge"
        ? [{ id: "merge", label: "Merge", variant: "secondary" as const }]
        : []),
      ...(revision.divergent
        ? [{ id: "divergent", label: "Divergent", tone: "warning" as const }]
        : []),
      ...(revision.conflicted
        ? [{ id: "conflicted", label: "Conflicted", tone: "danger" as const }]
        : []),
      ...revision.bookmarks.map((bookmark) => ({
        id: `bookmark:${JSON.stringify([bookmark.name, bookmark.remote, bookmark.added, bookmark.removed])}`,
        label: `${bookmark.name}${bookmark.remote ? `@${bookmark.remote}` : ""}`,
        tone: bookmark.conflicted
          ? ("danger" as const)
          : bookmark.remote
            ? ("neutral" as const)
            : ("accent" as const),
      })),
      ...revision.workspaces.map((workspace) => ({
        id: `workspace:${workspace}`,
        label:
          workspace === "default"
            ? "Working copy"
            : `Working copy · ${workspace}`,
        tone: "success" as const,
      })),
      ...(revision.operationSnapshot
        ? [{ id: "snapshot", label: "Snapshot", variant: "secondary" as const }]
        : []),
      ...row.deviceAliases.map((alias) => ({
        id: `device:${alias}`,
        label: alias,
        tone: "accent" as const,
      })),
    ];
  });
</script>

<div class="community-revision-graph-copy">
  <CommunityProjectsGitAuthorAvatar
    {members}
    email={row.revision.authorEmail}
    name={row.revision.author}
    size={avatarSize}
  />
  <div class="community-revision-graph-copy__body">
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
    <TagList
      {tags}
      label="Revision tags"
      onclick={(event) => event.stopPropagation()}
    />
  </div>
  {#if activity}<CommunityProjectsRevisionCollaboration {activity} />{/if}
</div>

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
</style>
