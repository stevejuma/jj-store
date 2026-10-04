<script lang="ts">
  import { untrack } from "svelte";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import StoreRepositoryApplication from "../../web/src/lib/ui/store-application/StoreRepositoryApplication.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { createMemoryStoryStorage } from "./memory-storage.js";
  const base = untrack(() => createInMemoryStoreController("log"));
  const storage = untrack(createMemoryStoryStorage);
  let phase = $state<"running" | "ready" | "unknown_cancel" | "canceled">(
    "running",
  );
  const controller: StoreController = {
    get state(): StoreController["state"] {
      const view = base.state;
      return {
        ...view,
        graph: phase === "ready" ? view.graph : { revisions: [], devices: [] },
        status: {
          ...view.status,
          graph: phase === "ready" ? "ready" : "failed",
        },
        settings: {
          ...view.settings,
          indexPreparationKind: "graph",
          indexPreparation:
            phase === "ready"
              ? "ready"
              : phase === "canceled"
                ? "idle"
                : "loading",
          indexTaskStatus:
            phase === "canceled"
              ? "canceled"
              : phase === "ready"
                ? "ready"
                : "running",
          indexCancellationUnconfirmed: phase === "unknown_cancel",
          indexPreparationError:
            phase === "unknown_cancel"
              ? "Cancellation is unconfirmed: confirmation missing"
              : "",
        },
      };
    },
    actions: {
      ...base.actions,
      cancelIndex: async () => {
        phase = phase === "unknown_cancel" ? "canceled" : "unknown_cancel";
      },
    },
  };
</script>

<StoreRepositoryApplication {controller} layoutStorage={storage} />
<button onclick={() => (phase = "ready")}>Finish preparation</button>
