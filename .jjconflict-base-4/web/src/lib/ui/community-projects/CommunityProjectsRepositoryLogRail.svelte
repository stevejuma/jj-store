<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import HistoryIcon from "@lucide/svelte/icons/history";
  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import CommunityProjectsGitAuthorAvatar from "./CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraphIds from "./CommunityProjectsRevisionGraphIds.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";

  let {
    operation,
    now,
    onSelect,
  }: {
    operation: RepositoryLogRevision;
    now?: number | undefined;
    onSelect: (id: string) => void;
  } = $props();
</script>

<section
  class="community-projects-repository-log-rail"
  aria-label="Selected operation metadata"
>
  <div class="community-projects-repository-log-rail__meta">
    <span>Author:</span>
    <CommunityProjectsGitAuthorAvatar
      email={operation.authorEmail}
      name={operation.author}
      size="small"
    />
    <span>{operation.author}</span>
    <span aria-hidden="true">•</span>
    <CommunityRelativeTime timestamp={operation.createdAt} {now} />
    <span aria-hidden="true">•</span>
    <CommunityProjectsRevisionGraphIds
      commitId={operation.eventId}
      commitLabel="operation ID"
      placement="inline"
    />
  </div>
  <nav
    class="community-projects-repository-log-rail__parents"
    aria-label="Operation parents"
  >
    {#each operation.parents as parent (parent)}<Button
        type="button"
        variant="ghost"
        class="community-projects-repository-log-rail__pill"
        onclick={() => onSelect(parent)}
        ><HistoryIcon data-icon="inline-start" aria-hidden="true" /><span
          >Parent {parent.slice(0, 8)}…{parent.slice(-4)}</span
        ></Button
      >{:else}<p>Root operation</p>{/each}
  </nav>
</section>

<style>
  .community-projects-repository-log-rail {
    display: grid;
    gap: 0.55rem;
    padding: 0.85rem 1rem;
    background: var(--background);
  }

  .community-projects-repository-log-rail__meta {
    display: flex;
    min-width: 0;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.45rem;
    color: var(--muted-foreground);
    font-size: var(--community-text-body);
  }

  .community-projects-repository-log-rail__meta
    :global(.community-revision-graph-ids__button) {
    height: auto;
    min-height: 1.65rem;
    padding-inline: 0.45rem;
    border-color: color-mix(in srgb, var(--foreground) 10%, transparent);
    background: color-mix(in srgb, var(--foreground) 7%, transparent);
    color: inherit;
    text-decoration: none;
  }

  .community-projects-repository-log-rail__meta
    :global(.community-revision-graph-ids__button:hover),
  .community-projects-repository-log-rail__meta
    :global(.community-revision-graph-ids__button:focus-visible) {
    border-color: color-mix(in srgb, var(--foreground) 18%, transparent);
    background: color-mix(in srgb, var(--foreground) 11%, transparent);
  }

  .community-projects-repository-log-rail__parents {
    display: flex;
    min-width: 0;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.35rem;
  }

  :global(.community-projects-repository-log-rail__pill) {
    display: inline-flex;
    min-width: 0;
    max-width: 100%;
    height: auto;
    min-height: 1.65rem;
    gap: 0.35rem;
    padding-inline: 0.45rem;
    border: 1px solid color-mix(in srgb, var(--foreground) 10%, transparent);
    border-radius: 0.4rem;
    color: var(--muted-foreground);
    background: color-mix(in srgb, var(--foreground) 7%, transparent);
    font-size: var(--community-text-secondary);
    font-weight: 500;
  }

  @media (hover: hover) {
    :global(.community-projects-repository-log-rail__pill:hover) {
      border-color: color-mix(in srgb, var(--foreground) 18%, transparent);
      background: color-mix(in srgb, var(--foreground) 11%, transparent);
      color: var(--foreground);
    }
  }

  :global(.community-projects-repository-log-rail__pill:focus-visible) {
    border-color: color-mix(in srgb, var(--foreground) 24%, transparent);
    background: color-mix(in srgb, var(--foreground) 11%, transparent);
    color: var(--foreground);
  }

  .community-projects-repository-log-rail p {
    margin: 0;
    overflow-wrap: anywhere;
    color: var(--muted-foreground);
  }
</style>
