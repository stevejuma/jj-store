<script lang="ts">
  import { onDestroy } from "svelte";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import type {
    KindPreparationTask,
    StoreDataSource,
  } from "../../web/src/lib/controller/native-data-source.js";
  import { createProductionStoreState } from "../../web/src/lib/controller/production-store-state.svelte.js";
  import { createIndexPreparation } from "../../web/src/lib/controller/production-index-preparation.js";
  import StoreGraphIndexSettings from "../../web/src/lib/ui/store-settings/StoreGraphIndexSettings.svelte";
  import "../../web/src/lib/ui/store-settings/StoreSettings.css";
  let {
    phase = "ready",
    local = false,
    automatic = false,
  }: {
    phase?: "ready" | "running" | "unknown_cancel";
    local?: boolean;
    automatic?: boolean;
  } = $props();
  const repository = "ab".repeat(16),
    operation = "cd".repeat(64);
  const data = createProductionStoreState();
  data.capabilities = {
    index_tasks: true,
    automatic_code_index: (() => automatic)(),
  } as typeof data.capabilities;
  let kind: "operations" | "copies" = "operations",
    cancelAttempts = 0;
  const report = (
    status: "running" | "ready" | "canceled",
  ): KindPreparationTask =>
    ({
      version: 1,
      id: "fixture-kind",
      kind,
      status,
      snapshot: { repo_id: repository, operation_id: operation },
      progress: {
        complete: status === "ready",
        matching_ids: 2,
        stats: {
          windows: 2,
          scanned: 512,
          descriptor_work: 512,
          record_reads: 1024,
          record_bytes: 2048,
        },
      },
    }) as KindPreparationTask;
  const source = {
    repository,
    profile: "native",
    prepareKindIndex: async (
      _operation: string,
      selected: "operations" | "copies",
    ) => {
      kind = selected;
      return report("running");
    },
    kindIndexTask: async () => report(phase === "ready" ? "ready" : "running"),
    cancelKindIndexTask: async () => {
      if (phase === "unknown_cancel" && cancelAttempts++ === 0)
        throw new Error("confirmation missing");
      return report("canceled");
    },
  } as unknown as StoreDataSource;
  const preparation = createIndexPreparation({
    data,
    source: () => source,
    operation: () => operation,
    version: () => 1,
    current: () => true,
    restore: async () => {},
  });
  const controller = {
    get state() {
      return {
        repository: { profile: local ? "git_sha1_jj045" : "native" },
        settings: {
          capabilities: data.capabilities,
          indexOperation: operation,
          indexPreparation: data.indexPreparation,
          indexPreparationError: data.indexPreparationError,
          indexTaskStatus: data.indexTaskStatus,
          indexPreparationKind: data.indexPreparationKind,
          indexProgress: data.indexProgress,
          indexCancellationUnconfirmed: data.indexCancellationUnconfirmed,
        },
      };
    },
    actions: {
      prepareKindIndex: preparation.prepareKindIndex,
      prepareIndex: async () => {},
      cancelIndex: preparation.cancelIndex,
    },
  } as unknown as StoreController;
  onDestroy(preparation.clear);
</script>

<StoreGraphIndexSettings {controller} />
