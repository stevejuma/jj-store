<script lang="ts">
  import { onDestroy } from "svelte";
  import { createProductionStoreController } from "./lib/controller/production-store-controller.svelte.js";
  import StoreConnection from "./lib/ui/store-application/StoreConnection.svelte";
  import StoreRepositoryApplication from "./lib/ui/store-application/StoreRepositoryApplication.svelte";

  const controller = createProductionStoreController();
  onDestroy(() => controller.actions.destroy());
</script>

{#if controller.state.authenticated}
  <StoreRepositoryApplication {controller} />
{:else}
  <StoreConnection
    status={controller.state.status}
    initialRepository={controller.state.route.repository}
    onConnect={controller.actions.connect}
  />
{/if}
