import type { StoreController } from "./contracts.js";
import type { createProductionBranchActions } from "./production-branch-actions.js";
import type { createProductionStoreConflicts } from "./production-store-conflicts.js";
import type { createProductionStoreIndex } from "./production-store-index.js";
import type { createProductionStoreMutations } from "./production-store-mutations.js";
import type { createProductionStoreReader } from "./production-store-reader.svelte.js";
import type { createProductionStoreSearch } from "./production-store-search-controller.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

type History = ReturnType<typeof createStoreRouteHistory>;
type Reader = ReturnType<typeof createProductionStoreReader>;

export function createProductionStoreActions(options: {
  data: ProductionStoreState;
  history: History;
  reader: Reader;
  search: ReturnType<typeof createProductionStoreSearch>;
  mutations: ReturnType<typeof createProductionStoreMutations>;
  conflicts: ReturnType<typeof createProductionStoreConflicts>;
  branches: ReturnType<typeof createProductionBranchActions>;
  selectRevision: StoreController["actions"]["selectRevision"];
  prepareIndex: ReturnType<typeof createProductionStoreIndex>;
  connect: StoreController["actions"]["connect"];
  connectRepository: (
    repository: string,
    token: string | null,
  ) => Promise<void>;
  disconnect: () => void;
  destroy: () => void;
  refresh: () => Promise<void>;
  selectOperation: (id: string) => Promise<void>;
  restoreRoute: () => Promise<void>;
  activeRepository: () => string | undefined;
  invalidate: () => void;
}): StoreController["actions"] {
  return {
    connect: options.connect,
    disconnect: options.disconnect,
    destroy: options.destroy,
    refresh: options.refresh,
    selectOperation: options.selectOperation,
    async selectComparisonParent(id) {
      if (
        !options.data.comparisonParents.includes(id) ||
        !options.history.route.inspectedOperationId
      )
        return;
      options.data.comparisonParents = [];
      await options.reader.loadOperation(
        options.history.route.inspectedOperationId,
        id,
      );
    },
    prepareIndex: options.prepareIndex,
    async loadMore(collection) {
      if (collection === "bookmarks" && options.data.refsCursor)
        await options.reader.loadReferences(true);
      if (collection === "graph" && options.data.graphCursor)
        await options.reader.loadGraph(true);
      if (collection === "files") await options.reader.loadMoreFiles();
      if (collection === "detail") await options.reader.loadMoreDetail();
      if (collection === "search" && options.data.searchCursor)
        await options.search.loadMore();
    },
    search: options.search.run,
    selectSearchResult: options.search.select,
    ...options.mutations.actions,
    uploadConflict: options.conflicts.upload,
    setFileViewMode: options.reader.setFileViewMode,
    toggleDirectory: options.reader.toggleDirectory,
    async openRepository(id) {
      if (id !== options.activeRepository()) {
        if (
          options.data.repositories.some((repository) => repository.id === id)
        )
          await options.connectRepository(id, null);
        return;
      }
      options.history.page("files");
      await options.restoreRoute();
    },
    async selectPage(page) {
      options.history.page(page);
      await options.restoreRoute();
    },
    async navigate(delta) {
      if (options.history.navigate(delta)) await options.restoreRoute();
    },
    ...options.branches,
    selectRevision: options.selectRevision,
    async selectFile(path) {
      options.invalidate();
      options.history.push({ ...options.history.route, path });
      options.data.blob = undefined;
      if (!path) return;
      if (options.history.route.page === "files")
        await options.reader.loadBlob(path);
      else await options.reader.loadPatch(path);
    },
    selectDirectory: options.reader.selectDirectory,
    selectRef: (value) => options.reader.selectRef(value),
    signIn() {},
    signOut: options.disconnect,
  };
}
