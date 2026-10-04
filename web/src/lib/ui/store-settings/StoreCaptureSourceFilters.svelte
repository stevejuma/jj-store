<script lang="ts">
  import * as Select from "@lapismd/design-core/shadcn/select";
  import {
    captureSourceHarnesses,
    captureSourceStatuses,
  } from "./capture-source-presentation.js";
  let {
    harness = $bindable("all"),
    status = $bindable("all"),
  }: { harness?: string; status?: string } = $props();
</script>

<div class="store-capture-sources__filters" aria-label="Source table filters">
  <Select.Root type="single" bind:value={harness}>
    <Select.Trigger aria-label="Filter by harness"
      >{captureSourceHarnesses.find((h) => h.value === harness)
        ?.label}</Select.Trigger
    >
    <Select.Content
      ><Select.Group
        ><Select.Label>Harness</Select.Label>
        {#each captureSourceHarnesses as h (h.value)}<Select.Item
            value={h.value}
            label={h.label}>{h.label}</Select.Item
          >{/each}
      </Select.Group></Select.Content
    >
  </Select.Root>
  <Select.Root type="single" bind:value={status}>
    <Select.Trigger aria-label="Filter by source status"
      >{captureSourceStatuses.find((s) => s.value === status)
        ?.label}</Select.Trigger
    >
    <Select.Content
      ><Select.Group
        ><Select.Label>Status</Select.Label>
        {#each captureSourceStatuses as s (s.value)}<Select.Item
            value={s.value}
            label={s.label}>{s.label}</Select.Item
          >{/each}
      </Select.Group></Select.Content
    >
  </Select.Root>
</div>

<style>
  .store-capture-sources__filters {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: 0.5rem;
    min-width: 0;
  }
</style>
