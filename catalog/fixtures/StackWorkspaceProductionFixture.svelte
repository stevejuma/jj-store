<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createProductionStoreController } from "../../web/src/lib/controller/production-store-controller.svelte.js";
  import { productionStacksHttp } from "./stack-workspace/production-http.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    cold = false,
    dependent = false,
    clearTransientFeedback = false,
  }: {
    cold?: boolean;
    dependent?: boolean;
    clearTransientFeedback?: boolean;
  } = $props();
  const controller = untrack(() =>
    createProductionStoreController(productionStacksHttp(cold, dependent)),
  );
  const layoutStorage = untrack(createMemoryStoryStorage);
  onDestroy(() => controller.actions.destroy());
  $effect(() => {
    if (controller.stackWorkspace) controller.stackWorkspace.page = "branches";
  });
  $effect(() => {
    if (
      clearTransientFeedback &&
      controller.state.status.error.includes("index rebuild required")
    )
      untrack(() => {
        // Simulate navigation clearing transient feedback without recovery.
        const status = controller.state.status as { error: string };
        status.error = "";
      });
  });
</script>

<div style="height: 100vh; min-height: 36rem">
  <StoreRepositoryApplication
    {controller}
    stackWorkspace={controller.stackWorkspace}
    {layoutStorage}
    displayMode="desktop"
    initiallyCollapsed={false}
  />
</div>
