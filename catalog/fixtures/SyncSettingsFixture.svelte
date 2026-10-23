<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { untrack } from "svelte";
  import StoreSyncSettings from "../../web/src/lib/ui/store-settings/StoreSyncSettings.svelte";
  import { syncWorker } from "./sync-settings.js";
  import { emptySyncSettings } from "../../web/src/lib/controller/sync-settings.js";
  import type { SyncSettings } from "../../web/src/lib/controller/sync-settings.js";
  let {
    embedded = false,
    loading = false,
    supported = true,
    configured = true,
    failed = false,
  } = $props<{
    embedded?: boolean;
    loading?: boolean;
    supported?: boolean;
    configured?: boolean;
    failed?: boolean;
  }>();
  let settings = $state<SyncSettings>(
    untrack(() => ({
      ...emptySyncSettings(),
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
  const loaded = untrack(() => structuredClone($state.snapshot(settings)));
  if (untrack(() => loading))
    settings = { ...emptySyncSettings(), state: "loading" };
</script>

{#if loading}
  <FixtureControls floating={false}
    ><button onclick={() => (settings = structuredClone(loaded))}
      >Finish loading</button
    ></FixtureControls
  >
  <FixtureControls floating={false}
    ><button onclick={() => (settings.state = "refreshing")}
      >Start background refresh</button
    ></FixtureControls
  >
{/if}

<div>
  <StoreSyncSettings
    {embedded}
    {settings}
    {supported}
    onRefresh={async () => {
      settings.state = "ready";
      settings.error = "";
    }}
    onWait={async (capture) => {
      if (capture)
        settings.wait.barrier = {
          version: 1,
          repository: "ab".repeat(16),
          local_replica: "01".repeat(16),
          peer_replica: "02".repeat(16),
          push_epoch: "03".repeat(16),
          pull_epoch: "04".repeat(16),
          push_through: 9,
          pull_through: 12,
        };
      settings.wait.state = capture ? "pending" : "reached";
    }}
    onStopWait={() => {
      settings.wait.state = "stopped";
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
