<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { createProductionStoreController } from "./lib/controller/production-store-controller.svelte.js";
  import StoreConnection from "./lib/ui/store-application/StoreConnection.svelte";
  import StoreRepositoryApplication from "./lib/ui/store-application/StoreRepositoryApplication.svelte";

  import { ownAppearance } from "./lib/appearance/runtime.svelte.js";
  import type { AppearanceStore } from "./lib/appearance/store.svelte.js";
  let { appearance }: { appearance?: AppearanceStore } = $props();
  ownAppearance(untrack(() => appearance));
  const controller = createProductionStoreController();
  onDestroy(() => controller.actions.destroy());
</script>

{#if controller.state.authenticated || controller.state.installationReady}
  <StoreRepositoryApplication {controller} />
{:else}
  <StoreConnection
    status={controller.state.status}
    accessMode={controller.state.accessMode}
    repositories={controller.state.repositories}
    storageReady={controller.state.storageReady}
    onConnect={controller.actions.connect}
  />
{/if}
