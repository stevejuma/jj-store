import { createIndexPreparation } from "./production-index-preparation.js";
import { createProductionSearchSettings } from "./production-search-settings.js";
import { createProductionStorageSettings } from "./production-storage-settings.js";
import { createProductionSyncSettings } from "./production-sync-settings.js";
import { createProductionWorkspaceLinks } from "./production-workspace-links.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { storeErrorMessage } from "./http-store-data-source.js";
import type { StoreRoute } from "./contracts.js";

export function createProductionStoreSettings(input: {
  data: ProductionStoreState;
  source: () => StoreDataSource;
  operation?: () => string | null;
  route?: () => StoreRoute;
  version: () => number;
  signal?: () => AbortSignal;
  current: (ticket: number, source: StoreDataSource) => boolean;
  restore: () => Promise<void>;
  freshness?: import("./application-freshness.js").ApplicationFreshness;
}) {
  const options = {
    ...input,
    operation:
      input.operation ?? (() => input.route?.().snapshotOperationId ?? null),
  };
  const { data } = options;
  const index = createIndexPreparation(options);
  const search = createProductionSearchSettings(options);
  const storage = createProductionStorageSettings({
    ...options,
    hasRepositoryContext: () => {
      const route = options.route?.();
      return (
        !route ||
        (!route.projectsHub && route.repository === options.source().repository)
      );
    },
  });
  const sync = createProductionSyncSettings(options);
  const workspaces = createProductionWorkspaceLinks(options);

  function clear() {
    index.clear();
    workspaces.clear();
    storage.clear();
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

  async function refresh(
    section?: import("./settings-refresh.js").SettingsRefreshScope,
  ) {
    if (section === "repository") return workspaces.refreshWorkspaceLinks();
    if (section === "storage") return storage.refreshStorage();
    if (section === "sync") return sync.refreshSyncSettings();
    if (section === "search") return search.refreshSearchSettings();
    if (section === "search_health") return refreshSearch();
    await Promise.all([
      refreshSearch(),
      workspaces.refreshWorkspaceLinks(),
      storage.refreshStorage(),
      sync.refreshSyncSettings(),
      search.refreshSearchSettings(),
    ]);
  }
  return {
    ...workspaces,
    ...storage,
    ...sync,
    ...search,
    clear,
    refresh,
    prepareIndex: index.prepareIndex,
    cancelIndex: index.cancelIndex,
  };
}
