<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsRevisionGraph from "../community-projects/CommunityProjectsRevisionGraph.svelte";
  import CommunityProjectsRepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  let { controller }: { controller: StoreController } = $props();
  const view = $derived(controller.state);
  const evolution = $derived(view.evolution);
</script>

<ColumnCanvas.Column id="evolution">
  <ColumnCanvas.Header>
    <ColumnCanvas.Title>Evo Log</ColumnCanvas.Title>
    <ColumnCanvas.Count>{evolution.graph.revisions.length}</ColumnCanvas.Count>
    <ColumnCanvas.Close
      aria-label="Close Evo Log column"
      title="Close Evo Log"
      onclick={() => void controller.actions.selectEvolution(null)}
    />
  </ColumnCanvas.Header>
  <ColumnCanvas.Body orientation="both">
    {#if evolution.error}
      <CommunityProjectsColumnAlert
        title="Evolution unavailable"
        description={evolution.error}
        tone="destructive"
      />
    {/if}
    {#if evolution.status === "loading"}
      <CommunityProjectsRepositorySkeleton
        layout="graph"
        label="Loading evolution log"
      />
    {:else if evolution.status === "empty"}
      <CommunityProjectsColumnAlert
        title="No evolution history"
        description="No recorded predecessors are available for this commit."
      />
    {:else}
      {#if evolution.predecessorsUnknown}
        <CommunityProjectsColumnAlert
          title="Incomplete predecessor history"
          description="Some native predecessor records are unavailable. Ancestry is not inferred."
        />
      {/if}
      <CommunityProjectsRevisionGraph
        revisions={evolution.graph.revisions}
        selectedId={view.route.evolutionCommitId}
        now={view.now}
        graphLabel="Evolution log"
        onSelect={(revision) =>
          void controller.actions.selectEvolution(revision.eventId)}
        hasMore={evolution.canLoadMore}
        loading={evolution.status === "refreshing"}
        onLoadMore={() => void controller.actions.loadMore("evolution")}
      />
    {/if}
  </ColumnCanvas.Body>
</ColumnCanvas.Column>
