<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
  import XIcon from "@lucide/svelte/icons/x";
  import StoreProjectsFilters from "./StoreProjectsFilters.svelte";
  import StoreActivityFilter from "./StoreActivityFilter.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type { Model } from "../../api/client.js";
  let {
    controller,
    project,
  }: {
    controller: StoreController;
    project?: Model<"ProjectV1"> | null;
  } = $props();
  const hub = $derived(controller.projects!),
    route = $derived(hub.route);
  const tab = $derived(route.workTab ?? (project ? "tasks" : "projects"));
  const title = $derived(`${tab[0]!.toUpperCase() + tab.slice(1)} filters`);
  const filtered = $derived(
    Boolean(
      (tab !== "reviews" && route.workQuery) ||
      (tab !== "repositories" && route.workRepository) ||
      (!project &&
        tab !== "repositories" &&
        tab !== "reviews" &&
        route.workHome) ||
      (tab === "tasks" && (route.workState ?? "active") !== "active"),
    ),
  );
  let open = $state(false),
    search = $state("");
  $effect(() => {
    search = route.workQuery ?? "";
  });
</script>

{#if tab === "activity"}<StoreActivityFilter
    value={route.workActivity ?? "all"}
    onChange={(workActivity) => void hub.filter({ workActivity })}
  />{/if}

<Popover.Root bind:open>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        type="button"
        variant={filtered ? "default" : "ghost"}
        size="icon-sm"
        aria-label={`Filters for ${tab}`}
        title={`Filters for ${tab}`}
        aria-pressed={filtered}
      >
        <SlidersHorizontalIcon aria-hidden="true" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content
    align="end"
    role="dialog"
    aria-label={title}
    style="--ui-popover-width: min(24rem, calc(100vw - 2rem)); max-height: min(80vh, 40rem); overflow-y: auto;"
  >
    <Popover.Header
      style="flex-direction: row; align-items: center; justify-content: space-between;"
    >
      <Popover.Title role="heading" aria-level={2}>{title}</Popover.Title>
      <Popover.Close
        >{#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            size="icon-sm"
            aria-label="Close filters"
          >
            <XIcon aria-hidden="true" />
          </Button>
        {/snippet}</Popover.Close
      >
    </Popover.Header>
    <StoreProjectsFilters {controller} {project} bind:search />
  </Popover.Content>
</Popover.Root>
