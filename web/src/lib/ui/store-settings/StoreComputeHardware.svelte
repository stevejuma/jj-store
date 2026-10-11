<script lang="ts">
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import SectionIcon from "@lucide/svelte/icons/server";
  import { object } from "../../controller/agent-execution-types.js";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  let { value }: { value: unknown } = $props();
  const hardware = $derived(object(value));
  const id = $props.id();
</script>

<StoreSettingsCard id={`${id}-machine`} title="This machine" Icon={SectionIcon}>
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
</StoreSettingsCard>
