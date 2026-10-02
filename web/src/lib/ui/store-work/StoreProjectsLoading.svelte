<script lang="ts">
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import type { StoreRoute } from "../../controller/contracts.js";
  let {
    route,
    layout = "list",
  }: { route: StoreRoute; layout?: "list" | "grid" } = $props();
  const label = $derived(
    route.taskId
      ? "Loading task details"
      : route.projectId
        ? "Loading project details"
        : `Loading ${route.workTab ?? "projects"}`,
  );
</script>

<div class="store-projects__loading">
  <CommunitySkeleton
    layout={route.projectId
      ? "details"
      : layout === "grid" && route.workTab !== "activity"
        ? "gallery"
        : "timeline"}
    {label}
  />
</div>
