<script lang="ts">
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { untrack } from "svelte";
  import { projectsViewKey } from "../../controller/projects-view-cache.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreProjectsDirectory from "./StoreProjectsDirectory.svelte";
  import StoreProjectsLoading from "./StoreProjectsLoading.svelte";
  import {
    projectsBusy,
    projectsInitialLoading,
  } from "./projects-initial-loading.js";
  import StoreWorkLayoutControls from "./StoreWorkLayoutControls.svelte";
  import "./community-work-rows.css";
  import "./community-work-layout.css";
  import "./store-projects.css";
  let { controller }: { controller: StoreController } = $props();
  let layout = $state<"grid" | "list">("list");
  let element = $state<HTMLElement | null>(null);
  const key = $derived(projectsViewKey(controller.projects!.route));
  $effect(() => {
    const captured = key,
      root = element,
      ready = !controller.projects!.loading;
    if (ready && root)
      untrack(() =>
        root.scrollTo(0, controller.projects!.scroll.get(captured) ?? 0),
      );
  });
  $effect(() => {
    const root = element;
    if (!root) return;
    const save = () => {
      if (!controller.projects!.loading)
        controller.projects!.scroll.set(key, root.scrollTop);
    };
    root.addEventListener("scroll", save);
    return () => root.removeEventListener("scroll", save);
  });
</script>

<section
  class="store-projects community-projects"
  aria-label="Linked repository projects"
  aria-busy={projectsBusy(controller)}
>
  <ScrollArea
    type="hover"
    class="store-projects__scroll"
    bind:viewportRef={element}
  >
    <header class="community-projects__surface-header">
      <div><h2>Projects</h2></div>
      <div class="community-projects__surface-actions">
        <StoreWorkLayoutControls bind:value={layout} />
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label="Refresh projects"
          title="Refresh projects"
          disabled={projectsBusy(controller)}
          onclick={() => controller.actions.refresh()}
          ><RefreshIcon aria-hidden="true" /></Button
        >
      </div>
    </header>
    <StoreNotice
      noticeKey="projects"
      title="Projects request failed"
      message={controller.projects?.error}
      pending={controller.projects?.loading}
    />
    {#if projectsInitialLoading(controller)}<StoreProjectsLoading
        route={controller.projects!.route}
        {layout}
      />
    {:else}<StoreProjectsDirectory {controller} {layout} />{/if}
  </ScrollArea>
</section>
