<script lang="ts">
  import CommunityProjectsRepositoryLogHeader from "../community-projects/CommunityProjectsRepositoryLogHeader.svelte";
  import StoreHistoryQueryToggle from "./StoreHistoryQueryToggle.svelte";
  let {
    title,
    count,
    enabled,
    context,
    controls,
    expanded = $bindable(false),
  }: {
    title: string;
    count: number;
    enabled: boolean;
    context: string;
    controls: string;
    expanded?: boolean;
  } = $props();
  $effect(() => {
    context;
    expanded = false;
  });
</script>

<CommunityProjectsRepositoryLogHeader {title} {count}>
  {#snippet actions()}{#if enabled}<StoreHistoryQueryToggle
        bind:expanded
        {controls}
        label={`Search ${title}`}
      />{/if}{/snippet}
</CommunityProjectsRepositoryLogHeader>
