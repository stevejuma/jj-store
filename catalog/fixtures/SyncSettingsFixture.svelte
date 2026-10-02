<script lang="ts">
  import { untrack } from "svelte";
  import StoreSyncSettings from "../../web/src/lib/ui/store-settings/StoreSyncSettings.svelte";
  import { syncWorker } from "./sync-settings.js";
  import type { SyncSettings } from "../../web/src/lib/controller/sync-settings.js";
  let {
    supported = true,
    configured = true,
    failed = false,
  } = $props<{ supported?: boolean; configured?: boolean; failed?: boolean }>();
  let settings = $state<SyncSettings>(
    untrack(() => ({
      value: {
        version: 1,
        configured,
        worker: configured ? syncWorker() : null,
      },
      state: "ready",
      error: "",
      changing: false,
    })),
  );
</script>

<div>
  <StoreSyncSettings
    {settings}
    {supported}
    onRefresh={async () => {
      settings.state = "ready";
      settings.error = "";
    }}
    onControl={async (action) => {
      if (failed) {
        settings.state = "failed";
        settings.error =
          "Unavailable. Sync status will be retried automatically.";
        return;
      }
      if (settings.value?.worker)
        settings.value.worker.phase = action === "pause" ? "paused" : "running";
    }}
  />
</div>
