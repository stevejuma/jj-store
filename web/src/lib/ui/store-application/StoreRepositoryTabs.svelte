<script lang="ts">
  import type { StorePage } from "../community-navigation/model.js";
  import { repositorySections } from "./repository-sections.js";
  import StoreViewTabs from "./StoreViewTabs.svelte";
  let {
    page,
    onSelect,
    local = false,
    stacks = false,
  }: {
    page: StorePage;
    onSelect(page: StorePage): void;
    local?: boolean;
    stacks?: boolean;
  } = $props();
  const sections = $derived(
    repositorySections
      .filter(
        (item) =>
          (item.page !== "stacks" || stacks) &&
          (!local || item.page !== "reviews"),
      )
      .map((item) => ({ value: item.page, label: item.label })),
  );
</script>

<StoreViewTabs
  value={page}
  {sections}
  onSelect={(value) => onSelect(value as StorePage)}
/>
