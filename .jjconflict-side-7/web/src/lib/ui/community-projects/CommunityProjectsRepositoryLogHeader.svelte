<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";

  import type { Snippet } from "svelte";
  let {
    title,
    count,
    actions,
  }: { title: string; count: number; actions?: Snippet } = $props();
</script>

<ColumnCanvas.Header>
  <div data-ui-component="column-canvas" data-ui-part="column-header-main">
    <ColumnCanvas.Title>{title}</ColumnCanvas.Title>
    <ColumnCanvas.Count>{count}</ColumnCanvas.Count>
  </div>

  <div class="community-projects-log-header__actions">
    {#if actions}{@render actions()}{/if}
    <ColumnCanvas.Toggle columnTitle={title} />
  </div>
</ColumnCanvas.Header>

<style>
  .community-projects-log-header__actions {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }
</style>
