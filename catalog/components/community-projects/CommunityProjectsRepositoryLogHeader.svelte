<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";

  let { title, count }: { title: string; count: number } = $props();
</script>

<ColumnCanvas.Header>
  <div data-ui-component="column-canvas" data-ui-part="column-header-main">
    <ColumnCanvas.Title>{title}</ColumnCanvas.Title>
    <ColumnCanvas.Count>{count}</ColumnCanvas.Count>
  </div>

  <ColumnCanvas.Toggle columnTitle={title} />
</ColumnCanvas.Header>
