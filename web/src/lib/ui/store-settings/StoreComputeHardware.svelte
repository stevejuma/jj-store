<script lang="ts">
  import { object } from "../../controller/agent-execution-types.js";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  let { value }: { value: unknown } = $props();
  const hardware = $derived(object(value));
  const id = $props.id();
</script>

<section class="store-settings__section" aria-labelledby={`${id}-machine`}>
  <header class="store-settings__section-heading">
    <h2 id={`${id}-machine`}>This machine</h2>
  </header>
  <dl class="store-settings__rows">
    <StoreReadOnlyRow
      label="System"
      value={`${String(hardware.os ?? "")} · ${String(hardware.architecture ?? "")}`}
    />
    <StoreReadOnlyRow
      label="CPU"
      value={`${String(hardware.cpu_threads ?? "")} threads`}
    />
    <StoreReadOnlyRow
      label="Memory"
      value={hardware.memory_bytes
        ? `${(Number(hardware.memory_bytes) / 1024 ** 3).toFixed(1)} GiB`
        : "Unavailable"}
    />
  </dl>
</section>
