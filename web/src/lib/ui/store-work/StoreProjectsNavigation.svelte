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
  let hovered = $state(false);
  // Only our Clear action retains the open bar after a submitted query changes.
  let keepOpenForScope: string | undefined;
  $effect(() => {
    const currentScope = scope;
    search = query;
    open = !!query || keepOpenForScope === currentScope;
    keepOpenForScope = undefined;
  });
  async function show() {
    open = true;
    await tick();
    input?.focus();
  }
  async function close() {
    keepOpenForScope = undefined;
    search = "";
    open = false;
    if (query) await hub.filter({ workQuery: "" });
    await tick();
    trigger?.focus({ preventScroll: true });
  }
  async function clear() {
    search = "";
    if (query) {
      keepOpenForScope = scope;
      await hub.filter({ workQuery: "" });
    }
    await tick();
    input?.focus({ preventScroll: true });
  }
</script>

<div class="community-projects__tabs store-projects-navigation">
  {#if tab !== "reviews"}<Button
      bind:ref={trigger}
      class="community-projects__tab-back store-projects-search__action store-projects-search__close"
      data-open={open ? "" : undefined}
      data-hovered={hovered ? "" : undefined}
      onpointerenter={() => (hovered = true)}
      onpointerleave={() => (hovered = false)}
      size="icon-sm"
      variant="outline"
      aria-label={open ? "Close search" : `Search ${tab}`}
      title={open ? "Close search" : `Search ${tab}`}
      onclick={open ? close : show}
      ><span class="store-projects-search__search-icon" aria-hidden="true"
        ><SearchIcon /></span
      ><span class="store-projects-search__close-icon" aria-hidden="true"
        ><XIcon /></span
      ></Button
    >{/if}
  {#if tab !== "reviews" && open}
    <form
      class="store-projects-search store-projects__search-field"
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
        {#if search.length > 0}<InputGroup.Addon align="inline-end"
            ><InputGroup.Button
              class="store-projects-search__action"
              size="icon-xs"
              variant="secondary"
              aria-label="Clear search"
              title="Clear search"
              onclick={clear}><XIcon aria-hidden="true" /></InputGroup.Button
            ></InputGroup.Addon
          >{/if}
      </InputGroup.Root>
    </form>
  {:else}
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
  .store-projects-navigation :global(.store-projects-search__action) {
    border-radius: 50%;
  }
  .store-projects-search__search-icon {
    display: flex;
  }
  .store-projects-search__close-icon {
    display: none;
  }
  :global(.store-projects-search__close[data-open][data-hovered])
    .store-projects-search__search-icon,
  :global(.store-projects-search__close[data-open]:focus-visible)
    .store-projects-search__search-icon {
    display: none;
  }
  :global(.store-projects-search__close[data-open][data-hovered])
    .store-projects-search__close-icon,
  :global(.store-projects-search__close[data-open]:focus-visible)
    .store-projects-search__close-icon {
    display: flex;
  }
</style>
