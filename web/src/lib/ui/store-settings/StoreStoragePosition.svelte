<script lang="ts">
  import BoxIcon from "@lucide/svelte/icons/box";
  import ClockIcon from "@lucide/svelte/icons/clock";
  import ListOrderedIcon from "@lucide/svelte/icons/list-ordered";
  import HashIcon from "@lucide/svelte/icons/hash";
  import FileTextIcon from "@lucide/svelte/icons/file-text";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import type { StorageOperation } from "../../controller/storage-settings.js";
  let { position }: { position: StorageOperation["target"] } = $props();
</script>

<dl class="store-settings__rows">
  <StoreReadOnlyRow
    copyable
    label="Repository"
    Icon={BoxIcon}
    value={position.repository}
    copyLabel="Repository"
  />
  <StoreReadOnlyRow
    copyable
    label="Epoch"
    Icon={ClockIcon}
    value={position.epoch}
    copyLabel="Storage epoch"
  />
  <StoreReadOnlyRow
    label="Published position"
    Icon={ListOrderedIcon}
    value={position.position.toLocaleString("en-GB")}
  />
  <StoreReadOnlyRow
    label="Manifest revision"
    Icon={HashIcon}
    value={position.revision.toLocaleString("en-GB")}
  />
  <StoreReadOnlyRow
    copyable
    label="Manifest digest"
    Icon={FileTextIcon}
    value={position.digest}
    copyLabel="Manifest digest"
  />
</dl>
