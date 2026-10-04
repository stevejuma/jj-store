<script lang="ts">
  import type { ComponentProps } from "svelte";
  import { FileDiff, FileDiffComposer } from "@lapismd/design-core/diff";
  import type { ScrollAreaStrategy } from "@lapismd/design-core/shadcn/scroll-area";
  let {
    single = false,
    label,
    scrollStrategy = "auto",
    wrap = false,
    ...diff
  }: ComponentProps<typeof FileDiffComposer> & {
    single?: boolean;
    label?: string;
    scrollStrategy?: ScrollAreaStrategy;
    wrap?: boolean;
  } = $props();
</script>

{#if single && diff.files?.[0]}
  <FileDiff
    path={diff.files[0].path}
    patch={diff.files[0].patch ?? ""}
    viewMode={diff.viewMode ?? "unified"}
    {wrap}
    {scrollStrategy}
    {label}
    lineAccessory={diff.lineAccessory}
    lineAnnotation={diff.lineAnnotation}
  />
{:else}<FileDiffComposer {...diff} />{/if}
