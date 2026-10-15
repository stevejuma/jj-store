<script lang="ts">
  import type { Snippet } from "svelte";
  import { useAppearance } from "../../appearance/store.svelte.js";
  import CommunityLinkCard from "./CommunityLinkCard.svelte";
  let {
    href,
    label,
    children,
  }: { href: string; label?: string; children?: Snippet } = $props();
  const appearance = useAppearance();
  const layout = $derived(appearance?.live.links ?? "inline");
</script>

{#if layout === "inline"}
  <p>{@render children?.()}</p>
{:else}
  <div class="store-link-preview">
    <CommunityLinkCard {href} {label} {layout} />
  </div>
{/if}

<style>
  .store-link-preview {
    margin-block: var(--store-paragraph-spacing, 0.45rem);
    max-width: 100%;
  }
</style>
