import { createProductionAutomaticRefresh } from "./production-automatic-refresh.js";
import { connectRepositorySource } from "./production-repository-connect.js";
import { createEntitySearch } from "./production-entity-search.js";
import { createProductionRefresh } from "./production-store-refresh.js";
import { createThreadsIntegration } from "./production-threads-integration.js";
import { createProfileRouteRestorer } from "./production-profile-route.js";
import { createProductionProjects } from "./production-projects-integration.js";
import { createProductionConnect } from "./production-store-connect.js";

import { createProductionRepositoryThreadNavigation } from "./repository-thread-navigation.svelte.js";
import type { StoreController } from "./contracts.js";

import type { StoreDataSource } from "./native-data-source.js";
import { createProductionStoreReaders } from "./production-store-reader.svelte.js";
import { createProductionStoreState } from "./production-store-state.svelte.js";
import { createStoreRouteHistory } from "./store-route-history.svelte.js";
import { productionControllerSurface } from "./production-store-view.js";
import { createProductionStoreSearch } from "./production-store-search-controller.js";
import { createProductionStoreMutations } from "./production-store-mutations.js";
import { createProductionStoreConflicts } from "./production-store-conflicts.js";
import { createProductionStoreSettings } from "./production-store-index.js";
import {
  createProductionRevisionSelector,
  requireProductionSource,
  reportProductionStoreError,
  resetConnectionView,
  destroyProductionStoreLifecycle,
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
  const threadNavigation = createProductionRepositoryThreadNavigation(
    () => data.repositories,
    () => sessionToken,
    options.fetcher,
  );
  let version = 0,
    feed = new AbortController(),
    discovery = new AbortController(),
    lifecycle = new AbortController();
  const invalidate = () => ++version;
  function follow(active: StoreDataSource) {
    feed = new AbortController();
    automatic.follow(active, feed.signal);
  }
  const activeSource = () => requireProductionSource(source);
  const current = (ticket: number, active: StoreDataSource) =>
    ticket === version && active === source;
  const { reader, historyQuery } = createProductionStoreReaders({
    data,
    history,
    source: activeSource,
    version: () => version,
    begin: () => ++version,
    current,
    report: (cause) => reportProductionStoreError(data, cause),
  });
  if (history.error)
    Object.assign(data.status, { navigation: "failed", error: history.error });

  const {
    projects,
    clear: clearProjects,
    stopEvents: stopProjectEvents,
    poll: pollProjects,
  } = createProductionProjects(
    data,
    history,
    () => sessionToken,
    () => session,
    () => feed.abort(),
    options.fetcher,
    () => automatic.request(),
    () => controller,
  );
  const threadsIntegration = createThreadsIntegration({
    source: () => source,
    data,
    history,
    connectRepository,
  });
  const { threads, openThreads } = threadsIntegration;
  const restoreRoute = createProfileRouteRestorer({
    data,
    route: () => history.route,
    source: () => source,
    projects,
    connect: connectRepository,
    token: () => sessionToken,
    restoreThreads: threadsIntegration.restore,
    invalidate,
    restore: reader.restorePage,
  });

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

  const { openRoute, globalSearch, referenceSearch } = createEntitySearch({
    repositories: () => data.repositories,
    token: () => sessionToken,
    route: () => history.route,
    push: history.push,
    repository: () => source?.repository,
    projects,
    connectRepository,
    restoreRoute,
    fetcher: options.fetcher,
  });
  function disconnect(message = "", denied = false, preserveSession = false) {
    const repositoryStatus = data.status.repositories;
    version += 1;
    automatic.reset();
    feed.abort();
    stopProjectEvents();
    discovery.abort();
    if (!preserveSession) {
      clearProjects();
      threadNavigation.clear();
    }
    threads.clear(preserveSession);
    source?.close();
    source = null;
    settings.clear();
    mutations.configure([]);
    mutations.reset();
    historyQuery.dispose();
    search.clear();
    globalSearch.clear();
    referenceSearch.clear();
    resetConnectionView(data, message, denied);
    if (preserveSession) data.status.repositories = repositoryStatus;
    else if (data.accessMode === "authenticated") {
      sessionToken = null;
      data.repositories = [];
    }
  }
  async function connectRepository(repository: string, token: string | null) {
    await connectRepositorySource(
      {
        data,
        history,
        token: () => sessionToken,
        setToken: (value) => {
          sessionToken = value;
        },
        source: () => source,
        setSource: (value) => {
          source = value;
        },
        nextVersion: () => ++version,
        version: () => version,
        nextSession: () => ++session,
        current,
        disconnect,
        configure: mutations.configure,
        follow,
        connected: threadsIntegration.connected,
        projects,
        reader,
        fetcher: options.fetcher,
      },
      repository,
      token,
    );
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
    refreshThreadNavigation: () => threadNavigation.load(true),
    stopFeed: () => feed.abort(),
    invalidate,
    follow,
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
    invalidate,
  });

  const actions = createProductionStoreActions({
    data,
    source: activeSource,
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
    destroy: () =>
      destroyProductionStoreLifecycle(lifecycle, disconnect, history),
    refresh,
    selectOperation,
    restoreRoute,
    activeRepository: () =>
      data.status.connection === "connected" ? source?.repository : undefined,
    invalidate,
  });

  const automatic = createProductionAutomaticRefresh({
    data,
    projects,
    threads,
    pollProjects,
    route: () => history.route,
    source: () => source,
    signal: lifecycle.signal,
    active: () =>
      data.storageReady &&
      (data.accessMode === "local_trusted" || sessionToken !== null),
    refresh: () => refresh(true),
    report: (cause) => reportProductionStoreError(data, cause),
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

  const controller: StoreController = productionControllerSurface(
    data,
    history,
    () => source,
    () => sessionToken,
    {
      historyQuery,
      threads: threadsIntegration,
      projects,
      threadNavigation,
      now: options.now,
      globalSearch,
      referenceSearch,
      actions: { ...actions, openThreads, openRoute },
    },
    automatic.pause,
  );
  return controller;
}
