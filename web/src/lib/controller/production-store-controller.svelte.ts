import { createProductionRefresh } from "./production-store-refresh.js";
import { createThreadsIntegration } from "./production-threads-integration.js";
import { createProductionProjects } from "./production-projects-integration.js";
import { createProductionConnect } from "./production-store-connect.js";
import { connectionDeniedMessage, storeErrorMessage } from "./store-error.js";

import { ApiError } from "../api/client.js";
import type { StoreController } from "./contracts.js";
import { HttpStoreDataSource } from "./http-store-data-source.js";
import type { StoreDataSource } from "./native-data-source.js";
import { followProductionStoreEvents } from "./production-store-events.js";
import { createProductionStoreReader } from "./production-store-reader.svelte.js";
import {
  clearProductionStoreView,
  createProductionStoreState,
} from "./production-store-state.svelte.js";
import { createStoreQueryClient } from "./store-query-client.js";
import { createStoreRouteHistory } from "./store-route-history.svelte.js";
import { productionControllerView } from "./production-store-view.js";
import { createProductionStoreSearch } from "./production-store-search-controller.js";
import { createProductionStoreMutations } from "./production-store-mutations.js";
import { createProductionStoreConflicts } from "./production-store-conflicts.js";
import { createProductionStoreSettings } from "./production-store-index.js";
import {
  createProductionRevisionSelector,
  reportProductionStoreError,
  resetConnectionView,
  startProductionStoreLifecycle,
} from "./production-store-lifecycle.svelte.js";
import { createProductionBranchActions } from "./production-branch-actions.js";
import { createProductionStoreActions } from "./production-store-actions.js";
import { createProductionMutationCommitter } from "./production-mutation-commit.js";

export function createProductionStoreController(
  options: { fetcher?: typeof fetch; now?: () => number } = {},
): StoreController {
  const history = createStoreRouteHistory();
  const data = createProductionStoreState();
  let source: StoreDataSource | null = null,
    sessionToken: string | null = null;
  let session = 0;
  let version = 0,
    feed = new AbortController(),
    discovery = new AbortController(),
    lifecycle = new AbortController();
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
    operation: () => history.route.snapshotOperationId,
    version: () => version,
    current,
    report: (cause) => reportProductionStoreError(data, cause),
  });
  if (history.error)
    Object.assign(data.status, { navigation: "failed", error: history.error });

  const {
    projects,
    clear: clearProjects,
    stopEvents: stopProjectEvents,
  } = createProductionProjects(
    data,
    history,
    () => sessionToken,
    () => session,
    () => feed.abort(),
    options.fetcher,
  );
  const threadsIntegration = createThreadsIntegration({
    source: () => source,
    data,
    history,
    connectRepository,
  });
  const { threads, openThreads } = threadsIntegration;
  async function restoreRoute() {
    if (await threadsIntegration.restore()) return;
    if (history.route.page === "projects") {
      version += 1;
      await projects.restore();
      return;
    }
    if (!source || !history.route.snapshotOperationId) return;
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
  const settings = createProductionStoreSettings({
    data,
    source: activeSource,
    operation: () => history.route.snapshotOperationId,
    version: () => version,
    current,
    restore: reader.restorePage,
  });

  function disconnect(message = "", denied = false, preserveSession = false) {
    const repositoryStatus = data.status.repositories;
    version += 1;
    feed.abort();
    stopProjectEvents();
    discovery.abort();
    if (!preserveSession) clearProjects();
    threads.clear(preserveSession);
    source?.close();
    source = null;
    settings.clear();
    mutations.configure([]);
    mutations.reset();
    search.clear();
    resetConnectionView(data, message, denied);
    if (preserveSession) data.status.repositories = repositoryStatus;
    else if (data.accessMode === "authenticated") {
      sessionToken = null;
      data.repositories = [];
    }
  }
  async function connectRepository(repository: string, token: string | null) {
    disconnect("", false, true);
    data.status.connection = "connecting";
    const ticket = ++version,
      requestedRoute = history.route;
    const credential =
      data.accessMode === "authenticated" ? (token ?? sessionToken) : null;
    if (data.accessMode === "authenticated" && !credential)
      throw new ApiError("bearer_required");
    let active!: HttpStoreDataSource;
    active = new HttpStoreDataSource(
      repository.trim(),
      credential,
      createStoreQueryClient(),
      `session-${++session}`,
      () => {
        if (source === active)
          disconnect(connectionDeniedMessage(data.accessMode), true);
      },
      options.fetcher,
    );
    source = active;
    try {
      const capabilities = await active.capabilities();
      const captured = await active.snapshot();
      if (!current(ticket, active) || history.route !== requestedRoute) return;
      data.heads = [...captured.heads];
      data.head = captured;
      data.capabilities = capabilities;
      mutations.configure(capabilities.typed_mutations);
      data.conflictResolutionActions = capabilities.conflict_resolution_actions;
      data.conflictUploadBytes = capabilities.conflict_upload_bytes;
      const selected = data.heads.length === 1 ? data.heads[0]! : null;
      if (data.accessMode === "authenticated") sessionToken = credential;
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
      await threadsIntegration.connected();
      if (history.route.page === "projects") await projects.restore();
      else if (history.route.snapshotOperationId) await reader.restorePage();
    } catch (cause) {
      if (!current(ticket, active)) return;
      const message = storeErrorMessage(cause);
      disconnect(message);
      data.status.connection = "failed";
    }
  }
  const connect = createProductionConnect({
    data,
    route: () => history.route,
    projects,
    connectRepository,
    disconnect,
    version: () => version,
    discovery: (abort) => {
      discovery = abort;
    },
    token: (token) => {
      sessionToken = token;
    },
    fetcher: options.fetcher,
  });
  const { refresh, selectOperation } = createProductionRefresh({
    data,
    history,
    source: activeSource,
    projects,
    threads: threadsIntegration,
    search,
    reader,
    stopFeed: () => feed.abort(),
    invalidate: () => {
      version += 1;
    },
    follow: (active) => {
      feed = new AbortController();
      void followProductionStoreEvents(
        active,
        () => active === source,
        data,
        feed.signal,
      );
    },
  });
  const mutations = createProductionStoreMutations({
    state: data.mutation,
    source: activeSource,
    head: () => data.head,
    current: (active) => active === source,
    committed: createProductionMutationCommitter(history, refresh),
  });
  const conflictUploads = createProductionStoreConflicts({
    source: activeSource,
    head: () => data.head,
    current: (active) => active === source,
    uploadLimit: () => data.conflictUploadBytes,
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

  const actions = createProductionStoreActions({
    data,
    history,
    reader,
    search,
    mutations,
    conflicts: conflictUploads,
    branches: branchActions,
    selectRevision,
    settings,
    connect: ({ repository, token }) => connect(repository, token),
    connectRepository,
    disconnect: () => disconnect(),
    destroy: () => {
      lifecycle.abort();
      disconnect();
      history.destroy();
    },
    refresh,
    selectOperation,
    restoreRoute,
    activeRepository: () =>
      data.status.connection === "connected" ? source?.repository : undefined,
    invalidate: () => {
      version += 1;
    },
  });

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
    threads,
    projects,
    get state() {
      return productionControllerView(
        data,
        history,
        source,
        options.now?.() ?? Math.floor(Date.now() / 1000),
        data.storageReady &&
          (data.accessMode === "local_trusted" || sessionToken !== null),
      );
    },
    actions: { ...actions, openThreads },
  };
}
