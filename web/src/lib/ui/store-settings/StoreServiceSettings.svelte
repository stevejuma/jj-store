<script lang="ts">
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreLocalFolders from "./StoreLocalFolders.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
</script>

<section class="store-settings__section" aria-labelledby="service-status">
  <header><h2 id="service-status">Service status</h2></header>
  <dl class="store-settings__rows">
    <StoreReadOnlyRow
      label="Access mode"
      value={state.accessMode === "local_trusted"
        ? "Local trusted"
        : "Repository token"}
    />
    <StoreReadOnlyRow
      label="Authoritative storage"
      tone={state.storageReady ? "ready" : "error"}
      value={state.storageReady ? "Ready" : "Unavailable"}
    />
    <StoreReadOnlyRow label="HTTP connection" value={state.status.connection} />
    <StoreReadOnlyRow label="Live events" value={state.status.live} />
    <StoreReadOnlyRow
      label="Live updates"
      value={state.status.refreshAvailable ? "Updating…" : "Current"}
    />
  </dl>
</section>

{#if state.accessMode === "local_trusted"}<StoreLocalFolders
    {controller}
  />{/if}
