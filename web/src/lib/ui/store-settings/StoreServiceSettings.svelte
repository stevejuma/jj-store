<script lang="ts">
  import StoreLocalFolders from "./StoreLocalFolders.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
</script>

<section class="store-settings__section" aria-labelledby="service-status">
  <header><h2 id="service-status">Service status</h2></header>
  <dl class="store-settings__rows">
    <div>
      <dt>Access mode</dt>
      <dd>
        {state.accessMode === "local_trusted"
          ? "Local trusted"
          : "Repository token"}
      </dd>
    </div>
    <div>
      <dt>Authoritative storage</dt>
      <dd>
        <span
          class="store-settings__status"
          data-tone={state.storageReady ? "ready" : "error"}
          >{state.storageReady ? "Ready" : "Unavailable"}</span
        >
      </dd>
    </div>
    <div>
      <dt>HTTP connection</dt>
      <dd>{state.status.connection}</dd>
    </div>
    <div>
      <dt>Live events</dt>
      <dd>{state.status.live}</dd>
    </div>
    <div>
      <dt>Live updates</dt>
      <dd>{state.status.refreshAvailable ? "Updating…" : "Current"}</dd>
    </div>
  </dl>
</section>

{#if state.accessMode === "local_trusted"}<StoreLocalFolders
    {controller}
  />{/if}
