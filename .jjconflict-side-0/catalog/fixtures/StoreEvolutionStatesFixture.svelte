<script lang="ts">
  import { untrack } from "svelte";
  import type {
    StoreController,
    StoreLoadStatus,
  } from "../../web/src/lib/controller/contracts.js";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  import { IDS } from "./repository.js";
  let {
    state = "loading",
  }: { state: "loading" | "empty" | "failed" | "incomplete" } = $props();
  const base = untrack(() => {
    const controller = createInMemoryStoreController("log");
    controller.actions.selectRevision(IDS.merge);
    controller.actions.selectEvolution(IDS.merge);
    return controller;
  });
  const storage = untrack(createMemoryStoryStorage);
  const controller: StoreController = {
    get state() {
      const view = base.state;
      return {
        ...view,
        evolution: {
          ...view.evolution,
          status: state === "incomplete" ? "ready" : (state as StoreLoadStatus),
          graph:
            state === "loading" || state === "empty"
              ? { revisions: [], devices: [] }
              : view.evolution.graph,
          error:
            state === "failed"
              ? "The captured evolution snapshot is unavailable."
              : "",
          predecessorsUnknown: state === "incomplete",
        },
      };
    },
    actions: base.actions,
  };
</script>

<StoreRepositoryApplication
  {controller}
  initiallyCollapsed
  layoutStorage={storage}
/>
