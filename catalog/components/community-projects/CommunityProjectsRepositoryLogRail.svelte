<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import HistoryIcon from "@lucide/svelte/icons/history";
  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import CommunityProjectsRevisionGraphIds from "./CommunityProjectsRevisionGraphIds.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  let {
    operation,
    now,
    onSelect,
  }: {
    operation: RepositoryLogRevision;
    now: number;
    onSelect: (id: string) => void;
  } = $props();
</script>

<section
  class="community-projects-repository-log-rail"
  aria-label="Selected operation"
>
  <p>{operation.message}</p>
  <CommunityProjectsRevisionGraphIds
    commitId={operation.eventId}
    commitLabel="operation ID"
    placement="inline"
  />
  <p>
    {operation.author} · <CommunityRelativeTime
      timestamp={operation.createdAt}
      {now}
    />
  </p>
  <nav aria-label="Operation parents">
    {#each operation.parents as parent (parent)}<Button
        type="button"
        variant="ghost"
        class="community-application__project-overview-item"
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
    gap: 0.15rem;
    padding: 0.75rem;
  }
  .community-projects-repository-log-rail p {
    overflow-wrap: anywhere;
  }
</style>
