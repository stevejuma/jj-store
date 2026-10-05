import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { ApiError } from "../api/client.js";
import { storeErrorMessage } from "./http-store-data-source.js";
import {
  emptyStorageSettings,
  type StorageAction,
  type StorageCommand,
} from "./storage-settings.js";
export function createProductionStorageSettings(options: {
  data: ProductionStoreState;
  source: () => StoreDataSource;
  version: () => number;
  hasRepositoryContext?: () => boolean;
  current: (ticket: number, source: StoreDataSource) => boolean;
}) {
  const { data } = options;
  let generation = 0;
  const supported = () =>
    Boolean(data.capabilities?.local_storage_management) &&
    options.hasRepositoryContext?.() !== false;
  function clear() {
    generation++;
    data.storageSettings = emptyStorageSettings();
  }
  async function refreshStorage() {
    const state = data.storageSettings;
    if (!supported()) {
      state.state = "unavailable";
      return;
    }
    if (state.changing) return;
    const source = options.source(),
      ticket = options.version(),
      token = ++generation;
    state.state = state.value ? "refreshing" : "loading";
    try {
      if (!source.storage) throw new Error("Storage maintenance unavailable");
      const value = await source.storage.overview();
      if (!options.current(ticket, source) || token !== generation) return;
      state.value = value;
      if (!state.pending) state.operation = value.latest;
      else {
        // A missing response is never permission to replace the original intent.
        const report = await source.storage.operation(state.pending.request_id);
        if (!options.current(ticket, source) || token !== generation) return;
        state.operation = report;
        if (
          state.pending.action !== "publish_checkpoint" ||
          report.phase !== "prepared"
        )
          state.pending = null;
      }
      state.state = "ready";
      state.error = "";
    } catch (cause) {
      if (!options.current(ticket, source) || token !== generation) return;
      state.state = "failed";
      state.error = storeErrorMessage(cause);
    }
  }
  async function submit(command: StorageCommand) {
    const state = data.storageSettings;
    if (!supported() || state.changing) return;
    const source = options.source(),
      ticket = options.version(),
      token = ++generation;
    state.pending = command;
    state.changing = true;
    state.error = "";
    try {
      if (!source.storage) throw new Error("Storage maintenance unavailable");
      const report = await source.storage.command(command);
      if (!options.current(ticket, source) || token !== generation) return;
      state.operation = report;
      state.pending = null;
      state.state = "ready";
    } catch (cause) {
      if (!options.current(ticket, source) || token !== generation) return;
      if (
        cause instanceof ApiError &&
        [
          "invalid_storage_command",
          "storage_intent_conflict",
          "storage_operation_pending",
          "storage_operation_not_found",
          "storage_journal_full",
          "forbidden",
        ].includes(cause.code)
      ) {
        state.pending = null;
        state.error = storeErrorMessage(cause);
      } else
        state.error = `${storeErrorMessage(cause)}. The original request is retained; check status or retry it.`;
    } finally {
      if (options.current(ticket, source) && token === generation)
        state.changing = false;
    }
  }
  async function startStorage(action: StorageAction) {
    const state = data.storageSettings;
    if (!supported() || state.pending || state.changing) return;
    const existing =
      action === "publish_checkpoint" || action === "cancel_checkpoint";
    if (existing) {
      if (
        state.operation?.kind !== "checkpoint" ||
        !["prepared", "unknown"].includes(state.operation.phase)
      )
        return;
    } else if (
      state.operation &&
      ["prepared", "running", "unknown"].includes(state.operation.phase)
    )
      return;
    await submit({
      version: 1,
      request_id: existing
        ? state.operation!.request_id
        : crypto.randomUUID().replaceAll("-", ""),
      action,
    });
  }
  return {
    clear,
    refreshStorage,
    startStorage,
    async retryStorage() {
      const command = data.storageSettings.pending;
      if (command) await submit(command);
    },
  };
}
