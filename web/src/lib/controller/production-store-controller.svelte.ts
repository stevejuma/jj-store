import { ApiError } from "../api/client.js";
import type { StoreController } from "./contracts.js";
import {
  HttpStoreDataSource,
  storeErrorMessage,
} from "./http-store-data-source.js";
import type { StoreDataSource } from "./native-data-source.js";
import { followProductionStoreEvents } from "./production-store-events.js";
import { createProductionStoreReader } from "./production-store-reader.svelte.js";
import {
  clearProductionStoreView,
  createProductionStoreState,
  initialStatus,
} from "./production-store-state.svelte.js";
import { createStoreQueryClient } from "./store-query-client.js";
import { createStoreRouteHistory } from "./store-route-history.svelte.js";
import { productionStoreView } from "./production-store-view.js";
import { createProductionStoreSearch } from "./production-store-search-controller.js";
import { createProductionStoreMutations } from "./production-store-mutations.js";
import { createProductionStoreConflicts } from "./production-store-conflicts.js";
import { createProductionStoreIndex } from "./production-store-index.js";
import {
  createProductionRevisionSelector,
  reportProductionStoreError,
  startProductionStoreLifecycle,
} from "./production-store-lifecycle.svelte.js";
import { createProductionBranchActions } from "./production-branch-actions.js";

export function createProductionStoreController(
  options: { fetcher?: typeof fetch; now?: () => number } = {},
): StoreController {
  const history = createStoreRouteHistory();
  const data = createProductionStoreState();
  let source: StoreDataSource | null = null;
  let session = 0;
  let version = 0,
    feed = new AbortController(),
    lifecycle = new AbortController();
  const operation = () => history.route.snapshotOperationId;
  const activeSource = () => {
    if (!source) throw new ApiError("not_connected");
    return source;
  };
  const current = (ticket: number, active: StoreDataSource) =>
    ticket === version && active === source;
  const reader = createProductionStoreReader({
    data,
    history,
    source: activeSource,
    operation,
    version: () => version,
    current,
    report: (cause) => reportProductionStoreError(data, cause),
  });
  if (history.error)
    Object.assign(data.status, { navigation: "failed", error: history.error });

  async function restoreRoute() {
    if (!source || !operation()) return;
    version += 1;
    clearProductionStoreView(data);
    await reader.restorePage();
  }

  const search = createProductionStoreSearch({
    data,
    history,
    source: activeSource,
    isCurrent: (active) => active === source,
    restoreRoute,
  });

  function disconnect(message = "", denied = false) {
    version += 1;
    feed.abort();
    source?.close();
    source = null;
    Object.assign(data, { heads: [], head: null });
    mutations.configure([]);
    mutations.reset();
    data.conflictResolutionActions = [];
    data.conflictUploadBytes = 0;
    data.searchAvailable = false;
    search.clear();
    clearProductionStoreView(data);
    Object.assign(data.status, initialStatus(), {
      connection: denied ? "denied" : "disconnected",
      error: message,
    });
  }
  async function connect(repository: string, token: string | null) {
    disconnect();
    data.status.connection = "connecting";
    const ticket = ++version;
    let active!: HttpStoreDataSource;
    active = new HttpStoreDataSource(
      repository.trim(),
      token,
      createStoreQueryClient(),
      `session-${++session}`,
      () => {
        if (source === active)
          disconnect(
            data.accessMode === "local_trusted"
              ? "Trusted local access was rejected. Check the server and repository catalog."
              : "Access expired or was revoked. Connect with a current token.",
            true,
          );
      },
      options.fetcher,
    );
    source = active;
    try {
      const capabilities = await active.capabilities();
      const captured = await active.snapshot();
      if (!current(ticket, active)) return;
      data.heads = [...captured.heads];
      data.head = captured;
      mutations.configure(capabilities.typed_mutations);
      data.conflictResolutionActions = capabilities.conflict_resolution_actions;
      data.conflictUploadBytes = capabilities.conflict_upload_bytes;
      const selected = data.heads.length === 1 ? data.heads[0]! : null;
      if (data.accessMode === "authenticated")
        data.repositories = [
          { id: active.repository, name: active.repository },
        ];
      data.searchAvailable = capabilities.search;
      history.connect(active.repository, selected);
      data.status.connection = "connected";
      data.status.repositories = "ready";
      feed = new AbortController();
      void followProductionStoreEvents(
        active,
        () => active === source,
        data,
        feed.signal,
      );
      if (history.route.snapshotOperationId) await reader.restorePage();
    } catch (cause) {
      if (!current(ticket, active)) return;
      const message = storeErrorMessage(cause);
      disconnect(message);
      data.status.connection = "failed";
    }
  }
  async function refresh() {
    const active = activeSource();
    feed.abort();
    const captured = await active.snapshot(true);
    data.heads = [...captured.heads];
    data.head = captured;
    const selected = data.heads.length === 1 ? data.heads[0]! : null;
    version += 1;
    clearProductionStoreView(data);
    search.clear();
    history.refresh(selected);
    data.status.refreshAvailable = false;
    feed = new AbortController();
    void followProductionStoreEvents(
      active,
      () => active === source,
      data,
      feed.signal,
    );
    if (selected) await reader.restorePage();
  }
  const mutations = createProductionStoreMutations({
    state: data.mutation,
    source: activeSource,
    head: () => data.head,
    current: (active) => active === source,
    committed: refresh,
  });
  const conflictUploads = createProductionStoreConflicts({
    source: activeSource,
    head: () => data.head,
    current: (active) => active === source,
    uploadLimit: () => data.conflictUploadBytes,
  });
  async function selectOperation(id: string) {
    if (!data.heads.includes(id)) return;
    version += 1;
    clearProductionStoreView(data);
    search.clear();
    history.selectSnapshot(id);
    await reader.restorePage();
  }
  const prepareIndex = createProductionStoreIndex({
    data,
    source: activeSource,
    version: () => version,
    current,
    restore: reader.restorePage,
  });
  const selectRevision = createProductionRevisionSelector(
    data,
    history,
    reader,
  );
  const branchActions = createProductionBranchActions({
    data,
    history,
    reader,
    invalidate: () => {
      version += 1;
    },
  });

  const actions: StoreController["actions"] = {
    connect: ({ repository, token }) => connect(repository, token),
    disconnect: () => disconnect(),
    destroy() {
      lifecycle.abort();
      disconnect();
      history.destroy();
    },
    refresh,
    selectOperation,
    async selectComparisonParent(id) {
      if (
        !data.comparisonParents.includes(id) ||
        !history.route.inspectedOperationId
      )
        return;
      data.comparisonParents = [];
      await reader.loadOperation(history.route.inspectedOperationId, id);
    },
    prepareIndex,
    async loadMore(collection) {
      if (collection === "bookmarks" && data.refsCursor)
        await reader.loadReferences(true);
      if (collection === "graph" && data.graphCursor)
        await reader.loadGraph(true);
      if (collection === "files") await reader.loadMoreFiles();
      if (collection === "detail") await reader.loadMoreDetail();
      if (collection === "search" && data.searchCursor) await search.loadMore();
    },
    search: search.run,
    selectSearchResult: search.select,
    ...mutations.actions,
    uploadConflict: conflictUploads.upload,
    setFileViewMode: reader.setFileViewMode,
    toggleDirectory: reader.toggleDirectory,
    async openRepository(id) {
      if (id !== source?.repository) {
        if (
          data.accessMode === "local_trusted" &&
          data.repositories.some((repository) => repository.id === id)
        )
          await connect(id, null);
        return;
      }
      history.page("files");
      await restoreRoute();
    },
    async selectPage(page) {
      history.page(page);
      await restoreRoute();
    },
    async navigate(delta) {
      if (history.navigate(delta)) await restoreRoute();
    },
    ...branchActions,
    selectRevision,
    async selectFile(path) {
      version += 1;
      history.push({ ...history.route, path });
      data.blob = undefined;
      if (!path) return;
      if (history.route.page === "files") await reader.loadBlob(path);
      else await reader.loadPatch(path);
    },
    selectDirectory: reader.selectDirectory,
    selectRef: (value) => reader.selectRef(value),
    signIn() {},
    signOut: () => disconnect(),
  };

  startProductionStoreLifecycle({
    data,
    history,
    source: () => source,
    connect,
    disconnect,
    restore: restoreRoute,
    fetcher: options.fetcher,
    signal: lifecycle.signal,
  });

  return {
    get state() {
      return productionStoreView(data, history.route, {
        connected: source !== null,
        repository: source?.repository ?? history.route.repository,
        canBack: history.canBack,
        canForward: history.canForward,
        now: options.now?.() ?? Math.floor(Date.now() / 1000),
      });
    },
    actions,
  };
}
