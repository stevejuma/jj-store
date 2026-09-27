import { ApiError } from "../api/client.js";
import type { StorePage } from "../ui/community-navigation/model.js";
import type { StoreController } from "./contracts.js";
import {
  HttpStoreDataSource,
  storeErrorMessage,
} from "./http-store-data-source.js";
import type { StoreDataSource } from "./native-data-source.js";
import { createProductionStoreReader } from "./production-store-reader.svelte.js";
import {
  clearProductionStoreView,
  createProductionStoreState,
  emptyGraph,
  initialStatus,
} from "./production-store-state.svelte.js";
import { createStoreQueryClient } from "./store-query-client.js";
import { createStoreRouteHistory } from "./store-route-history.svelte.js";

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

  async function follow(active: StoreDataSource, signal: AbortSignal) {
    let after: string | undefined,
      delay = 500;
    while (!signal.aborted && active === source) {
      try {
        data.status.live = "live";
        await active.events(after, signal, (event, id) => {
          if (signal.aborted || active !== source) return;
          after = id;
          delay = 500;
          if (event.type === "ready") data.heads = [...event.snapshot.heads];
          else data.status.refreshAvailable = true;
        });
      } catch {
        if (signal.aborted || active !== source) return;
      }
      data.status.live = "reconnecting";
      await new Promise((resolve) => setTimeout(resolve, delay));
      delay = Math.min(delay * 2, 10_000);
    }
  }
  function disconnect(message = "", denied = false) {
    version += 1;
    feed.abort();
    source?.close();
    source = null;
    data.heads = [];
    data.repositories = [];
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
      await active.capabilities();
      const captured = await active.snapshot();
      if (!current(ticket, active)) return;
      data.heads = [...captured.heads];
      const selected = data.heads.length === 1 ? data.heads[0]! : null;
      data.repositories = [{ id: active.repository, name: active.repository }];
      history.connect(active.repository, selected);
      data.status.connection = "connected";
      data.status.repositories = "ready";
      feed = new AbortController();
      void follow(active, feed.signal);
      if (selected) await reader.loadPage();
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
    const selected = data.heads.length === 1 ? data.heads[0]! : null;
    version += 1;
    clearProductionStoreView(data);
    history.connect(active.repository, selected);
    data.status.refreshAvailable = false;
    feed = new AbortController();
    void follow(active, feed.signal);
    if (selected) await reader.loadPage();
  }
  async function selectOperation(id: string) {
    if (!data.heads.includes(id)) return;
    version += 1;
    clearProductionStoreView(data);
    history.connect(activeSource().repository, id);
    await reader.loadPage();
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
      await reader.loadPage();
    }
  }
  async function selectPage(page: StorePage) {
    history.page(page);
    data.graph = emptyGraph();
    data.detail = undefined;
    await reader.loadPage();
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
    data.detail = undefined;
    data.blob = undefined;
    if (!id) return;
    if (page === "oplog") await reader.loadOperation(id);
    else if (page === "files") await reader.loadFiles(id);
    else await reader.loadCommit(id);
  }

  const actions: StoreController["actions"] = {
    connect: ({ repository, token }) => connect(repository, token),
    disconnect: () => disconnect(),
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
      if (collection === "files" && data.pathsCursor && history.route.commitId)
        await reader.loadFiles(history.route.commitId, true);
    },
    async openRepository(id) {
      if (id !== source?.repository) return;
      history.page("files");
      await reader.loadPage();
    },
    selectPage,
    async navigate(delta) {
      history.navigate(delta);
      await reader.loadPage();
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
    selectDirectory(directory) {
      history.push({ ...history.route, directory, path: null });
    },
    selectRef: (value) => reader.selectRef(value),
    signIn() {},
    signOut: () => disconnect(),
  };

  return {
    get state() {
      const repository = source?.repository ?? history.route.repository;
      return {
        route: history.route,
        canNavigateBack: history.canBack,
        canNavigateForward: history.canForward,
        repositories: data.repositories,
        repository: { id: repository, name: repository },
        operationHeads: data.heads,
        comparisonParents: data.comparisonParents,
        conflictCount: data.conflictCount,
        unavailableComparisonCount: data.unavailableComparisonCount,
        canLoadMoreBookmarks: data.refsCursor !== null,
        canLoadMoreGraph: data.graphCursor !== null,
        canLoadMoreFiles: data.pathsCursor !== null,
        graph: data.graph,
        now: options.now?.() ?? Math.floor(Date.now() / 1000),
        detail: data.detail,
        bookmarks: data.bookmarks,
        choices: data.choices,
        targets: data.targets,
        snapshot: data.snapshot,
        blob: data.blob,
        authenticated:
          source !== null && data.status.connection === "connected",
        member: {
          id: repository || "disconnected",
          name: source ? "Repository connected" : "Disconnected",
          handle: source ? repository.slice(0, 12) : "Connect",
          initials: source ? "JJ" : "—",
          presence: source ? ("online" as const) : ("offline" as const),
          status: source
            ? `${data.status.live} · token held in memory`
            : "No repository session",
        },
        status: data.status,
      };
    },
    actions,
  };
}
