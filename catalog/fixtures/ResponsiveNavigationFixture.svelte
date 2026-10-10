<script lang="ts">
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreViewTabs from "../../web/src/lib/ui/store-application/StoreViewTabs.svelte";
  import StoreToolbarSearch from "../../web/src/lib/ui/StoreToolbarSearch.svelte";
  let width = $state(360),
    value = $state("projects"),
    search = $state(""),
    open = $state(false);
  const sections = [
    "activity",
    "projects",
    "tasks",
    "reviews",
    "agents",
    "repositories",
  ].map((value) => ({
    value,
    label: value[0]!.toUpperCase() + value.slice(1),
    disabled: value === "agents",
  }));
</script>

<div style:width={`${width}px`}>
  <Tabs.Root bind:value>
    <StoreViewTabs
      {value}
      {sections}
      inheritContext
      hideTabs={open}
      label="Fixture views"
      onSelect={(next) => (value = next)}
    >
      {#snippet leading()}<StoreToolbarSearch
          label="Search fixture"
          scope="fixture"
          query={search}
          bind:open
          onQuery={(next) => {
            search = next;
          }}
        />{/snippet}
      {#snippet trailing()}<Button size="icon-sm" aria-label="Filter fixture"
          >F</Button
        >{/snippet}
    </StoreViewTabs>
    <Tabs.Content {value}>Selected {value}</Tabs.Content>
  </Tabs.Root>
</div>
<Button onclick={() => (width = 960)}>Widen toolbar</Button>
<Button onclick={() => (width = 224)}>Narrow toolbar</Button>
