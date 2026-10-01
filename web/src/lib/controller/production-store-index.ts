import { createProductionSearchSettings } from "./production-search-settings.js";
import { createProductionSyncSettings } from "./production-sync-settings.js";
import { createProductionWorkspaceLinks } from "./production-workspace-links.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { storeErrorMessage } from "./http-store-data-source.js";

export function createProductionStoreSettings(options: {
  data: ProductionStoreState;
  source: () => StoreDataSource;
  operation: () => string | null;
  version: () => number;
  current: (ticket: number, source: StoreDataSource) => boolean;
  restore: () => Promise<void>;
}) {
  const { data } = options;
  const search = createProductionSearchSettings(options);
  const sync = createProductionSyncSettings(options);
  const workspaces = createProductionWorkspaceLinks(options);

  function clear() {
    workspaces.clear();
    sync.clear();
    search.clear();
    data.capabilities = null;
    data.neededIndex = null;
    data.searchStatus = null;
    data.searchStatusState = "idle";
    data.searchStatusError = "";
    data.indexPreparation = "idle";
    data.indexPreparationError = "";
  }

  async function refreshSearch() {
    if (!data.capabilities?.search) {
      data.searchStatus = null;
      data.searchStatusState = "unavailable";
      data.searchStatusError = "";
      return;
    }
    const source = options.source();
    const ticket = options.version();
    data.searchStatusState = data.searchStatus ? "refreshing" : "loading";
    data.searchStatusError = "";
    try {
      const status = await source.searchStatus(true);
      if (!options.current(ticket, source)) return;
      data.searchStatus = status;
      data.searchStatusState = "ready";
    } catch (cause) {
      if (!options.current(ticket, source)) return;
      data.searchStatusState = "failed";
      data.searchStatusError = storeErrorMessage(cause);
    }
  }

  async function prepareIndex() {
    const { data } = options;
    const operation = data.neededIndex?.operation_id ?? options.operation();
    if (
      !operation ||
      !data.capabilities?.index_tasks ||
      data.indexPreparation === "loading"
    )
      return;
    const source = options.source();
    const ticket = options.version();
    data.indexPreparation = "loading";
    data.indexPreparationError = "";
    try {
      let task = await source.prepareIndex(operation);
      while (
        options.current(ticket, source) &&
        ["running", "canceling"].includes(task.status)
      ) {
        await new Promise((resolve) => setTimeout(resolve, 500));
        task = await source.indexTask(task.id);
      }
      if (!options.current(ticket, source)) return;
      if (task.status !== "ready") {
        data.indexPreparation = "failed";
        data.indexPreparationError =
          task.status === "canceled"
            ? "Graph index preparation was canceled."
            : "Graph index preparation failed.";
        return;
      }
      data.neededIndex = null;
      data.status.error = "";
      data.indexPreparation = "ready";
      await options.restore();
    } catch (cause) {
      if (!options.current(ticket, source)) return;
      data.indexPreparation = "failed";
      data.indexPreparationError = storeErrorMessage(cause);
    }
  }

  async function refresh() {
    await Promise.all([
      refreshSearch(),
      workspaces.refreshWorkspaceLinks(),
      sync.refreshSyncSettings(),
      search.refreshSearchSettings(),
    ]);
  }
  return { ...workspaces, ...sync, ...search, clear, refresh, prepareIndex };
}
