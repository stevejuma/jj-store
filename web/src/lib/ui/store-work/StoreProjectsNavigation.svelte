<script lang="ts">
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
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
  {#if tab !== "reviews"}<StoreToolbarSearch
      label={`Search ${tab}`}
      {scope}
      {query}
      bind:open
      onQuery={(value) => hub.filter({ workQuery: value })}
    />{/if}
  {#if tab === "reviews" || !open}
    <ScrollArea
      type="hover"
      orientation="horizontal"
      class="community-projects__tabs-scroll-area"
    >
      <Tabs.List
        class="community-projects__tab-list"
        aria-label={project ? "Project views" : "Projects views"}
      >
        {#each tabs as value}<Tabs.Trigger
            class="community-projects__tab-trigger"
            {value}>{value[0]!.toUpperCase() + value.slice(1)}</Tabs.Trigger
          >{/each}
      </Tabs.List>
    </ScrollArea>
  {/if}
  {#if tab !== "repositories"}<div class="store-projects-navigation__filters">
      <StoreProjectsFilterControls {controller} {project} />
    </div>{/if}
</div>

<style>
  .store-projects-navigation {
    height: 3.25rem;
  }
  .store-projects-navigation__filters {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    flex: none;
  }
</style>
