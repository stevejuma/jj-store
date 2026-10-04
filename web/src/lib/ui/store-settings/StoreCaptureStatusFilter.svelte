<script lang="ts">
  import StoreCaptureCommandFilter from "./StoreCaptureCommandFilter.svelte";
  import { captureSourceStatuses } from "./capture-source-presentation.js";
  let { value = $bindable([]) }: { value?: string[] } = $props();
</script>

<StoreCaptureCommandFilter
  bind:value
  singular="status"
  plural="statuses"
  options={captureSourceStatuses}
/>
