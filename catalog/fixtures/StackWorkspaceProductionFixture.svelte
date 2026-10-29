<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import StoreRepositoryApplication from "./ApplicationStory.svelte";
  import { createProductionStoreController } from "../../web/src/lib/controller/production-store-controller.svelte.js";
  import { productionStacksHttp } from "./stack-workspace/production-http.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  let {
    cold = false,
    dependent = false,
    editing = false,
    dragRead = "unique",
    clearTransientFeedback = false,
  }: {
    cold?: boolean;
    dependent?: boolean;
    editing?: boolean;
    dragRead?: "unique" | "ambiguous" | "failure";
    clearTransientFeedback?: boolean;
  } = $props();
  const controller = untrack(() =>
    createProductionStoreController(
      productionStacksHttp(cold, dependent, editing, dragRead),
    ),
  );
  const layoutStorage = untrack(createMemoryStoryStorage);
  onDestroy(() => controller.actions.destroy());
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

<div style="height: 100vh">
  <StoreRepositoryApplication
    {controller}
    stackWorkspace={controller.stackWorkspace}
    {layoutStorage}
    displayMode="auto"
    initiallyCollapsed={false}
  />
</div>
