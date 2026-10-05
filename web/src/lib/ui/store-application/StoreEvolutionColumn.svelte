<script lang="ts">
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  const notifications = ownNotifications();
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsRevisionGraph from "../community-projects/CommunityProjectsRevisionGraph.svelte";
  import CommunityProjectsRepositorySkeleton from "../community-projects/CommunityProjectsRepositorySkeleton.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
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
    <StoreNotice
      noticeKey="evolution-request"
      title="Evolution unavailable"
      message={evolution.error}
    />
    <StoreNotice
      noticeKey="evolution-predecessors"
      severity="warning"
      title="Incomplete predecessor history"
      message={evolution.predecessorsUnknown
        ? "Some native predecessor records are unavailable. Ancestry is not inferred."
        : ""}
    />
    <StoreNotice
      noticeKey="evolution-empty"
      severity="info"
      title="No evolution history"
      message={evolution.status === "empty"
        ? "No recorded predecessors are available for this commit."
        : ""}
    />
    {#if evolution.status === "loading"}
      <CommunityProjectsRepositorySkeleton
        layout="graph"
        label="Loading evolution log"
      />
    {:else}
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

<StoreNotificationHost owner={notifications} />
