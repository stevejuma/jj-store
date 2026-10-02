<script lang="ts">
  import type { Snippet } from "svelte";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  let {
    taskDetail,
    bounded = false,
    viewportRef = $bindable<HTMLElement | null>(null),
    children,
  }: {
    taskDetail: boolean;
    bounded?: boolean;
    viewportRef?: HTMLElement | null;
    children: Snippet;
  } = $props();
</script>

{#if taskDetail || bounded}
  {@render children()}
{:else}
  <ScrollArea type="hover" class="store-projects__scroll" bind:viewportRef>
    {@render children()}
  </ScrollArea>
{/if}
