<script lang="ts">
  import * as ToggleGroup from "@lapismd/design-core/shadcn/toggle-group";
  import LayoutGridIcon from "@lucide/svelte/icons/layout-grid";
  import ListIcon from "@lucide/svelte/icons/list";
  import "./community-work-cards.css";
  let { value = $bindable("list") }: { value?: "grid" | "list" } = $props();
</script>

<ToggleGroup.Root
  type="single"
  bind:value={
    () => value,
    (next) => {
      if (next === "grid" || next === "list") value = next;
    }
  }
  size="sm"
  spacing={1}
  class="community-projects__layout-controls"
  aria-label="Project layout"
>
  <ToggleGroup.Item value="grid" aria-label="Grid layout" title="Grid layout"
    ><LayoutGridIcon aria-hidden="true" /></ToggleGroup.Item
  >
  <ToggleGroup.Item value="list" aria-label="List layout" title="List layout"
    ><ListIcon aria-hidden="true" /></ToggleGroup.Item
  >
</ToggleGroup.Root>
