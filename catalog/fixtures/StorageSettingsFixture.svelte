<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { untrack } from "svelte";
  import StoreStorageSettings from "../../web/src/lib/ui/store-settings/StoreStorageSettings.svelte";
  import {
    emptyStorageSettings,
    type StorageSettings,
    type StorageAction,
  } from "../../web/src/lib/controller/storage-settings.js";
  let {
    embedded = false,
    loading = false,
    retainedOperation = false,
    supported = true,
    phase = "prepared",
  }: {
    embedded?: boolean;
    loading?: boolean;
    retainedOperation?: boolean;
    supported?: boolean;
    phase?: "prepared" | "incomplete" | "unknown" | "interrupted";
  } = $props();
  const target = {
    repository: "ab".repeat(16),
    epoch: "cd".repeat(16),
    position: 4,
    revision: 6,
    digest: "ef".repeat(32),
  };
  const limits = { entries: 100, bytes: 1024, timeout_millis: 1000 };
  let settings = $state<StorageSettings>(
    untrack(() => ({
      ...emptyStorageSettings(),
      state: "ready",
      value: {
        version: 1,
        ready: true,
        current: target,
        checkpoint: null,
        latest: null,
        limits,
      },
      operation: {
        version: 1,
        request_id: "12".repeat(16),
        kind:
          phase === "incomplete" || phase === "interrupted"
            ? "check"
            : "checkpoint",
        phase,
        target,
        limits,
        message: null,
        selection: null,
      },
    })),
  );
  const loaded = untrack(() => structuredClone($state.snapshot(settings)));
  if (untrack(() => loading))
    settings = {
      ...emptyStorageSettings(),
      state: "loading",
      operation: untrack(() => retainedOperation) ? loaded.operation : null,
    };
  async function start(action: StorageAction) {
    if (settings.operation)
      settings.operation = {
        ...settings.operation,
        phase: action === "cancel_checkpoint" ? "canceled" : "complete",
        selection: action === "cancel_checkpoint" ? null : "selected",
      };
  }
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

<StoreStorageSettings
  {embedded}
  {settings}
  {supported}
  onStart={start}
  onRefresh={async () => {}}
  onRetry={async () => {}}
/>
