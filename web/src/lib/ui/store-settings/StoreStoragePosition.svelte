<script lang="ts">
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import type { StorageOperation } from "../../controller/storage-settings.js";
  let { position }: { position: StorageOperation["target"] } = $props();
</script>

<dl class="store-settings__rows">
  <StoreReadOnlyRow
    label="Repository"
    value={position.repository}
    copyLabel="Repository"
  />
  <StoreReadOnlyRow
    label="Epoch"
    value={position.epoch}
    copyLabel="Storage epoch"
  />
  <StoreReadOnlyRow
    label="Published position"
    value={position.position.toLocaleString("en-GB")}
  />
  <StoreReadOnlyRow
    label="Manifest revision"
    value={position.revision.toLocaleString("en-GB")}
  />
  <StoreReadOnlyRow
    label="Manifest digest"
    value={position.digest}
    copyLabel="Manifest digest"
  />
</dl>
