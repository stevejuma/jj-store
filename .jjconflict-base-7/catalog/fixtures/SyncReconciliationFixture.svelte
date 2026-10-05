<script lang="ts">
  import { untrack } from "svelte";
  import StoreSyncReconciliation from "../../web/src/lib/ui/store-settings/StoreSyncReconciliation.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { emptySyncSettings } from "../../web/src/lib/controller/sync-settings.js";
  import { syncWorker } from "./sync-settings.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  let { outcome = "committed" }: { outcome?: "committed" | "unknown" } =
    $props();
  const base = untrack(() =>
    createInMemoryStoreController("branches", { mutationOutcome: outcome }),
  );
  let captures = $state(0);
  const controller: StoreController = {
    ...base,
    get state(): StoreController["state"] {
      return {
        ...base.state,
        settings: {
          ...base.state.settings,
          syncSettings: {
            ...emptySyncSettings(),
            state: "ready",
            value: { version: 1, configured: true, worker: syncWorker() },
          },
        },
      };
    },
    actions: {
      ...base.actions,
      async captureReconciliation() {
        captures++;
        return {
          version: 1,
          repo_id: "ab".repeat(16),
          epoch: "cd".repeat(16),
          published_seq: 10,
          manifest_revision: 12,
          code_state_version: 3,
          heads: ["12".repeat(64), "34".repeat(64)],
        };
      },
    },
  };
</script>

<StoreSyncReconciliation {controller} />
<p data-testid="captures">{captures}</p>
