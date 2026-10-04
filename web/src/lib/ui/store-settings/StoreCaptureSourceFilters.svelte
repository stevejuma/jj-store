<script lang="ts">
  import * as Select from "@lapismd/design-core/shadcn/select";
  import { captureSourceHarnesses } from "./capture-source-presentation.js";
  import StoreCaptureStatusFilter from "./StoreCaptureStatusFilter.svelte";
  let {
    harness = $bindable("all"),
    statuses = $bindable([]),
  }: { harness?: string; statuses?: string[] } = $props();
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
  <StoreCaptureStatusFilter bind:value={statuses} />
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
