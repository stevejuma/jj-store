<script lang="ts">
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import { globalSettingsSkeletonCards } from "./global-settings-skeleton.js";
  let { label, tab = "capture" }: { label: string; tab?: string } = $props();
  const cards = $derived(globalSettingsSkeletonCards("agents", tab));
</script>

<StoreSettingsSkeleton
  {label}
  {tab}
  heading={false}
  cards={tab === "capture" ? cards.slice(1) : cards}
/>
