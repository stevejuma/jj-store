<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createProductionStoreController } from "../../web/src/lib/controller/production-store-controller.svelte.js";
  import { productionStacksHttp } from "./stack-workspace/production-http.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let { cold = false }: { cold?: boolean } = $props();
  const controller = untrack(() =>
    createProductionStoreController(productionStacksHttp(cold)),
  );
  const layoutStorage = untrack(createMemoryStoryStorage);
  onDestroy(() => controller.actions.destroy());
  $effect(() => {
    if (controller.stackWorkspace) controller.stackWorkspace.page = "branches";
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
