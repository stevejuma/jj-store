import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { storeErrorMessage } from "./http-store-data-source.js";
import { emptySyncSettings, type SyncAction } from "./sync-settings.js";
export function createProductionSyncSettings(options: {
  data: ProductionStoreState;
  source: () => StoreDataSource;
  version: () => number;
  current: (ticket: number, source: StoreDataSource) => boolean;
}) {
  const { data } = options;
  let generation = 0;
  function clear() {
    generation++;
    data.syncSettings = emptySyncSettings();
  }
  async function request(action?: SyncAction) {
    if (data.syncSettings.changing) return;
    if (!data.capabilities?.local_sync_management) {
      data.syncSettings.state = "unavailable";
      return;
    }
    if (
      action &&
      (!data.syncSettings.value?.configured ||
        data.syncSettings.state !== "ready")
    )
      return;
    const source = options.source(),
      ticket = options.version(),
      token = ++generation;
    const state = data.syncSettings;
    state.changing = action !== undefined;
    state.state = state.value ? "refreshing" : "loading";
    state.error = "";
    try {
      if (!source.sync) throw new Error("Sync management unavailable");
      const result = action
        ? await source.sync.control(action)
        : await source.sync.status();
      if (!options.current(ticket, source) || token !== generation) return;
      state.value = result;
      state.state = "ready";
    } catch (cause) {
      if (!options.current(ticket, source) || token !== generation) return;
      state.state = "failed";
      state.error = `${storeErrorMessage(cause)}. Refresh sync status before retrying.`;
    } finally {
      if (options.current(ticket, source) && token === generation)
        state.changing = false;
    }
  }
  return {
    clear,
    refreshSyncSettings: () => request(),
    controlSync: (action: SyncAction) => request(action),
  };
}
