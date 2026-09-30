<script lang="ts">
  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";

  let {
    target,
    revision,
    now,
  }: {
    target: string;
    revision?: RepositoryLogRevision;
    now?: number;
  } = $props();
</script>

<div
  class="store-mutation-commit-target"
  role="group"
  aria-label="Selected commit"
>
  <CommunityProjectsGitAuthorAvatar
    name={revision?.author ?? "Unknown author"}
    email={revision?.authorEmail}
    size="small"
  />
  <span class="store-mutation-commit-target__message" title={revision?.message}
    >{revision?.message ?? "Selected commit"}</span
  >
  <CommunityProjectsRevisionGraphIds
    changeId={revision?.changeId}
    commitId={target}
    placement="inline"
  />
  {#if revision}<CommunityRelativeTime
      timestamp={revision.createdAt}
      {now}
    />{/if}
</div>

<style>
  .store-mutation-commit-target {
    display: grid;
    min-width: 0;
    grid-template-columns: auto minmax(0, 1fr) auto auto;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm, 0.375rem);
    background: var(--background);
    font-size: var(--community-text-secondary);
  }

  .store-mutation-commit-target__message {
    min-width: 0;
    overflow: hidden;
    font-size: var(--community-text-body);
    font-weight: 550;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .store-mutation-commit-target :global(.community-revision-graph-ids) {
    min-width: 0;
  }

  .store-mutation-commit-target :global(time) {
    color: var(--muted-foreground);
    white-space: nowrap;
  }
</style>
