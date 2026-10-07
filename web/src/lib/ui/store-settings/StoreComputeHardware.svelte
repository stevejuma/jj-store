<script lang="ts">
  import { object } from "../../controller/agent-execution-types.js";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  let { value }: { value: unknown } = $props();
  const hardware = $derived(object(value));
</script>

<h2>This machine</h2>
<Field.Group class="community-settings__option-group"
  ><StoreWorkField label="Hardware"
    ><span
      >{String(hardware.os ?? "")} · {String(hardware.architecture ?? "")} · {String(
        hardware.cpu_threads ?? "",
      )} CPU threads{hardware.memory_bytes
        ? ` · ${(Number(hardware.memory_bytes) / 1024 ** 3).toFixed(1)} GiB memory`
        : ""}</span
    ></StoreWorkField
  ></Field.Group
>
