<script lang="ts">
  import type { Snippet } from "svelte";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  let {
    taskDetail,
    viewportRef = $bindable<HTMLElement | null>(null),
    children,
  }: {
    taskDetail: boolean;
    viewportRef?: HTMLElement | null;
    children: Snippet;
  } = $props();
</script>

{#if taskDetail}
  {@render children()}
{:else}
  <ScrollArea type="hover" class="store-projects__scroll" bind:viewportRef>
    {@render children()}
  </ScrollArea>
{/if}
