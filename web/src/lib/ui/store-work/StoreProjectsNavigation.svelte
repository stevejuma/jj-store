<script lang="ts">
  import StoreViewTabs from "../store-application/StoreViewTabs.svelte";
  import StoreToolbarSearch from "../StoreToolbarSearch.svelte";
  import StoreProjectsFilterControls from "./StoreProjectsFilterControls.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type { ProjectsTab } from "../../controller/projects-hub-types.js";
  import type { Model } from "../../api/client.js";
  let {
    controller,
    project,
    tabs,
    tab,
  }: {
    controller: StoreController;
    project?: Model<"ProjectV1"> | null;
    tabs: ProjectsTab[];
    tab: ProjectsTab;
  } = $props();
  const hub = $derived(controller.projects!);
  const query = $derived(hub.route.workQuery ?? "");
  const scope = $derived(
    `${hub.route.repository}:${hub.route.projectId}:${tab}`,
  );
  let open = $state(false);
</script>

<div class="community-projects__tabs store-projects-navigation">
  <StoreViewTabs
    inheritContext
    value={tab}
    sections={tabs.map((value) => ({
      value,
      label: value[0]!.toUpperCase() + value.slice(1),
    }))}
    onSelect={(value) => void hub.filter({ workTab: value as ProjectsTab })}
    label={project ? "Project views" : "Projects views"}
    hideTabs={tab !== "reviews" && open}
  >
    {#snippet leading()}
      {#if tab !== "reviews"}<StoreToolbarSearch
          label={`Search ${tab}`}
          {scope}
          {query}
          bind:open
          onQuery={(value) => hub.filter({ workQuery: value })}
        />{/if}
    {/snippet}
    {#snippet trailing()}
      {#if tab !== "repositories"}<StoreProjectsFilterControls
          {controller}
          {project}
        />{/if}
    {/snippet}
  </StoreViewTabs>
</div>

<style>
  .store-projects-navigation {
    padding: 0;
    min-width: 0;
  }
</style>
