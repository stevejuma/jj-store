<script lang="ts">
  // CommunityProjectsPullRequestCommits (Apache-2.0), exact native commit navigation.
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
  import type { NativeActivityCommit } from "../../controller/native-activity-types.js";
  import { initialStoreRoute } from "../../controller/store-route-history.svelte.js";
  import { storeRouteToHash } from "../../controller/store-route-codec.js";
  let { commits }: { commits: readonly NativeActivityCommit[] } = $props();
</script>

{#if commits.length}<div
    class="community-projects__review-commit-list"
    role="list"
    aria-label="Review commits"
  >
    {#each commits as row (row.revision.eventId)}<div role="listitem">
        <a
          class="community-projects__review-commit-row"
          href={storeRouteToHash({
            ...initialStoreRoute(),
            page: "log",
            repository: row.repository,
            snapshotOperationId: row.operation,
            commitId: row.revision.eventId,
            changeId: row.revision.changeId,
          }) ?? "#"}
          ><GitCommitIcon aria-hidden="true" /><strong
            >{row.revision.message.split("\n")[0]}</strong
          ><code title={row.revision.eventId}
            >{row.revision.eventId.slice(0, 7)}</code
          ></a
        >
      </div>{/each}
  </div>{:else}<StoreWorkEmpty
    kind="activity"
    title="No code commits recorded"
    description="This review anchor has no recorded code commits."
  />{/if}

<style>
  .community-projects__review-commit-list {
    display: grid;
    padding: 0.1rem 1.5rem 0.85rem;
  }
  .community-projects__review-commit-row {
    display: grid;
    width: 100%;
    box-sizing: border-box;
    min-height: 2.35rem;
    grid-template-columns: 1rem minmax(0, 1fr) 5rem;
    align-items: center;
    gap: 0.55rem;
    border: 0;
    padding: 0.35rem 0.5rem;
    border-radius: 0.4rem;
    color: inherit;
    background: transparent;
    font: inherit;
    text-align: left;
    text-decoration: none;
  }
  .community-projects__review-commit-row:hover {
    background: color-mix(in srgb, var(--muted) 25%, transparent);
  }
  .community-projects__review-commit-row :global(svg) {
    width: 0.85rem;
    height: 0.85rem;
    color: var(--muted-foreground);
  }
  strong {
    overflow: hidden;
    font-size: var(--community-text-secondary);
    font-weight: 500;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  code {
    color: var(--muted-foreground);
    font-family: var(--font-mono, monospace);
    font-size: var(--community-text-secondary);
  }
</style>
