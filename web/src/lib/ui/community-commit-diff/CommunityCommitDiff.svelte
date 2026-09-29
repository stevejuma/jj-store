<script lang="ts">
  import type { ComponentProps } from "svelte";
  import { FileDiff, FileDiffComposer } from "@lapismd/design-core/diff";
  let {
    single = false,
    wrap = false,
    ...diff
  }: ComponentProps<typeof FileDiffComposer> & {
    single?: boolean;
    wrap?: boolean;
  } = $props();
</script>

{#if single && diff.files?.[0]}
  <FileDiff
    path={diff.files[0].path}
    patch={diff.files[0].patch ?? ""}
    viewMode={diff.viewMode ?? "unified"}
    {wrap}
    lineAccessory={diff.lineAccessory}
    lineAnnotation={diff.lineAnnotation}
  />
{:else}<FileDiffComposer {...diff} />{/if}
