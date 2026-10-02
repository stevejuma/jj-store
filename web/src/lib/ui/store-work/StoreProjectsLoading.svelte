<script lang="ts">
  import StoreCollaborationSkeleton from "../store-collaboration/StoreCollaborationSkeleton.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import type { StoreRoute } from "../../controller/contracts.js";
  let {
    route,
    layout = "list",
  }: { route: StoreRoute; layout?: "list" | "grid" } = $props();
  const agents = $derived(route.workTab === "agents" && !route.taskId);
  const label = $derived(
    route.taskId
      ? "Loading task details"
      : route.projectId
        ? "Loading project details"
        : `Loading ${route.workTab ?? "projects"}`,
  );
</script>

<div
  class="store-projects__loading"
  class:store-projects__loading--agents={agents}
>
  {#if agents}
    <StoreCollaborationSkeleton {label} agents />
  {:else}
    <CommunitySkeleton
      layout={route.projectId
        ? "details"
        : layout === "grid" && route.workTab !== "activity"
          ? "gallery"
          : "timeline"}
      {label}
    />
  {/if}
</div>

<style>
  .store-projects__loading--agents {
    padding: 0;
  }
</style>
