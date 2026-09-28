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

export function createProductionStoreController(
  options: { fetcher?: typeof fetch; now?: () => number } = {},
): StoreController {
  const history = createStoreRouteHistory();
  const data = createProductionStoreState();
  let source: StoreDataSource | null = null;
  let session = 0,
    version = 0,
    feed = new AbortController();
  const operation = () => history.route.snapshotOperationId;
  const activeSource = () => {
    if (!source) throw new ApiError("not_connected");
    return source;
  };
  const current = (ticket: number, active: StoreDataSource) =>
    ticket === version && active === source;
  function report(cause: unknown) {
    if (cause instanceof DOMException && cause.name === "AbortError") return;
    if (cause instanceof ApiError && cause.code === "index_rebuild_required")
      data.neededIndex = cause.snapshot ?? null;
    data.status.error = storeErrorMessage(cause);
  }
  const reader = createProductionStoreReader({
    data,
    history,
    source: activeSource,
    operation,
    version: () => version,
    current,
    report,
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
    Object.assign(data, { heads: [], repositories: [], head: null });
    mutations.configure([]);
    mutations.reset();
    data.searchAvailable = false;
    search.clear();
    clearProductionStoreView(data);
    Object.assign(data.status, initialStatus(), {
      connection: denied ? "denied" : "disconnected",
      error: message,
    });
  }
  async function connect(repository: string, token: string) {
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
            "Access expired or was revoked. Connect with a current token.",
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
      const selected = data.heads.length === 1 ? data.heads[0]! : null;
      data.repositories = [{ id: active.repository, name: active.repository }];
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
  async function selectOperation(id: string) {
    if (!data.heads.includes(id)) return;
    version += 1;
    clearProductionStoreView(data);
    search.clear();
    history.selectSnapshot(id);
    await reader.restorePage();
  }
  async function prepareIndex() {
    if (!data.neededIndex) return;
    const active = activeSource(),
      ticket = version;
    let task = await active.prepareIndex(data.neededIndex.operation_id);
    while (
      current(ticket, active) &&
      ["running", "canceling"].includes(task.status)
    ) {
      await new Promise((resolve) => setTimeout(resolve, 500));
      task = await active.indexTask(task.id);
    }
    if (current(ticket, active) && task.status === "ready") {
      data.neededIndex = null;
      data.status.error = "";
      await reader.restorePage();
    }
  }
  async function selectRevision(id: string | null) {
    const page = history.route.page;
    history.push({
      ...history.route,
      inspectedOperationId: page === "oplog" ? id : null,
      commitId: page === "oplog" ? null : id,
      changeId:
        data.graph.revisions.find((revision) => revision.eventId === id)
          ?.changeId ?? null,
      path: null,
    });
    Object.assign(data, {
      detail: undefined,
      blob: undefined,
      inspectionContinuation: null,
    });
    if (!id) return;
    if (page === "oplog") await reader.loadOperation(id);
    else if (page === "files") await reader.loadFiles(id);
    else await reader.loadCommit(id);
  }

  const actions: StoreController["actions"] = {
    connect: ({ repository, token }) => connect(repository, token),
    disconnect: () => disconnect(),
    destroy() {
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
    setFileViewMode: reader.setFileViewMode,
    toggleDirectory: reader.toggleDirectory,
    async openRepository(id) {
      if (id !== source?.repository) return;
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
    selectBookmark(name) {
      const bookmark = data.bookmarks.find((item) => item.name === name);
      if (!bookmark) return;
      data.targets = [...bookmark.targets];
      history.push({
        ...history.route,
        bookmark: name,
        commitId: null,
        changeId: null,
        path: null,
      });
    },
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

  history.onExternalRoute((result) => {
    if (result.kind === "invalid") {
      data.status.navigation = "failed";
      data.status.error = result.message;
    } else if (result.kind === "route" && source) {
      if (result.route.repository !== source.repository)
        disconnect("Reconnect to open the requested repository.");
      else void restoreRoute();
    }
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
