<script lang="ts">
  import StoreCollaborationSkeleton from "../store-collaboration/StoreCollaborationSkeleton.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import type { StoreRoute } from "../../controller/contracts.js";
  import StoreWorkRowsSkeleton from "./StoreWorkRowsSkeleton.svelte";
  import StoreWorkTaskSkeleton from "./StoreWorkTaskSkeleton.svelte";
  let {
    route,
    layout = "list",
  }: { route: StoreRoute; layout?: "list" | "grid" } = $props();
  const tab = $derived(
    route.workTab ?? (route.projectId ? "tasks" : "projects"),
  );
  const agents = $derived(tab === "agents" && !route.taskId);
  const label = $derived(
    route.taskId
      ? "Loading task details"
      : route.projectId
        ? "Loading project details"
        : `Loading ${tab}`,
  );
</script>

<div
  class="store-projects__loading"
  class:store-projects__loading--agents={agents}
  class:store-projects__loading--task={!!route.taskId}
>
  {#if route.taskId}<StoreWorkTaskSkeleton />
  {:else if agents}
    <StoreCollaborationSkeleton {label} agents />
  {:else if tab === "activity"}
    <CommunitySkeleton layout="timeline" {label} />
  {:else}<StoreWorkRowsSkeleton
      {label}
      {layout}
      kind={tab === "tasks" || tab === "reviews" || tab === "repositories"
        ? tab
        : "projects"}
    />
  {/if}
</div>

<style>
  .store-projects__loading--agents {
    padding: 0;
  }
  .store-projects__loading--task {
    padding: 0;
    height: 100%;
  }
</style>
