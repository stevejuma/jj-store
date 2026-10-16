<script lang="ts">
  import * as Select from "@lapismd/design-core/shadcn/select";
  import {
    captureSourceHarnesses,
    type CaptureSourceFilterOption,
  } from "./capture-source-presentation.js";
  import StoreCaptureStatusFilter from "./StoreCaptureStatusFilter.svelte";
  import StoreCaptureCommandFilter from "./StoreCaptureCommandFilter.svelte";
  let {
    harness = $bindable("all"),
    statuses = $bindable([]),
    repositories = $bindable([]),
    repositoryOptions,
  }: {
    harness?: string;
    statuses?: string[];
    repositories?: string[];
    repositoryOptions: CaptureSourceFilterOption[];
  } = $props();
</script>

<div class="store-capture-sources__filters" aria-label="Source table filters">
  <Select.Root type="single" bind:value={harness}>
    <Select.Trigger aria-label="Filter by harness"
      >{captureSourceHarnesses.find((h) => h.value === harness)
        ?.label}</Select.Trigger
    >
    <Select.Content aria-label="Capture source harnesses"
      ><Select.Group
        ><Select.Label>Harness</Select.Label>
        {#each captureSourceHarnesses as h (h.value)}<Select.Item
            value={h.value}
            label={h.label}>{h.label}</Select.Item
          >{/each}
      </Select.Group></Select.Content
    >
  </Select.Root>
  <StoreCaptureCommandFilter
    bind:value={repositories}
    singular="repository"
    plural="repositories"
    options={repositoryOptions}
  />
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
