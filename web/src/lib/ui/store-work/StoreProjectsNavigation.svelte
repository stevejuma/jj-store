<script lang="ts">
  import { tick } from "svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as InputGroup from "@lapismd/design-core/shadcn/input-group";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import SearchIcon from "@lucide/svelte/icons/search";
  import XIcon from "@lucide/svelte/icons/x";
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
  let open = $state(false),
    search = $state("");
  let input: HTMLInputElement | null = $state(null);
  let trigger: HTMLButtonElement | null = $state(null);
  $effect(() => {
    scope;
    search = query;
    open = !!query;
  });
  async function show() {
    open = true;
    await tick();
    input?.focus();
  }
  async function close() {
    search = "";
    open = false;
    if (query) await hub.filter({ workQuery: "" });
    await tick();
    trigger?.focus({ preventScroll: true });
  }
</script>

<div class="community-projects__tabs store-projects-navigation">
  {#if tab !== "reviews" && open}
    <form
      class="store-projects-search store-projects__search-field"
      data-filled={search ? "" : undefined}
      onsubmit={(event) => {
        event.preventDefault();
        void hub.filter({ workQuery: search });
      }}
    >
      <InputGroup.Root>
        <InputGroup.Input
          onkeydown={(event) => {
            if (event.key === "Escape" && !event.defaultPrevented) {
              event.preventDefault();
              void close();
            }
          }}
          bind:ref={input}
          bind:value={search}
          maxlength={512}
          aria-label={`Search ${tab}`}
          placeholder={`Search ${tab}`}
        />
        <InputGroup.Addon align="inline-start"
          ><InputGroup.Button
            type="submit"
            size="icon-sm"
            variant="outline"
            aria-label="Search"
            title="Search"><SearchIcon aria-hidden="true" /></InputGroup.Button
          ></InputGroup.Addon
        >
        <InputGroup.Addon align="inline-end"
          ><InputGroup.Button
            class="store-projects-search__close"
            size="icon-sm"
            aria-label="Close search"
            title="Close search"
            onclick={close}><XIcon aria-hidden="true" /></InputGroup.Button
          ></InputGroup.Addon
        >
      </InputGroup.Root>
    </form>
  {:else}
    {#if tab !== "reviews"}<Button
        bind:ref={trigger}
        class="community-projects__tab-back"
        variant="outline"
        size="icon-sm"
        aria-label={`Search ${tab}`}
        title={`Search ${tab}`}
        onclick={show}><SearchIcon aria-hidden="true" /></Button
      >{/if}
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
  .store-projects-search {
    flex: 1;
    min-width: 0;
    padding: 0;
  }
  .store-projects-navigation__filters {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    flex: none;
  }
  .store-projects-search :global(.store-projects-search__close) {
    opacity: 0;
    pointer-events: none;
  }
  .store-projects-search:hover :global(.store-projects-search__close),
  .store-projects-search:focus-within :global(.store-projects-search__close),
  .store-projects-search[data-filled] :global(.store-projects-search__close) {
    opacity: 1;
    pointer-events: auto;
  }
</style>
