import { repositoryReaders } from "./repository-sources.js";
import { createReadScope } from "./read-scope.js";
import { connectRepositorySource } from "./production-repository-connect.js";
import { createEntitySearch } from "./production-entity-search.js";
import { createProductionRefresh } from "./production-store-refresh.js";
import { createThreadsIntegration } from "./production-threads-integration.js";
import { createProfileRouteRestorer } from "./production-profile-route.js";
import { createProductionProjects } from "./production-projects-integration.js";
import { createProductionConnect } from "./production-store-connect.js";
import { startProductionSessionEvents } from "./production-session-events.js";
import { ApplicationFreshness } from "./application-freshness.js";

import { createLocalConfiguration } from "./production-local-configuration.js";
import type { StoreController } from "./contracts.js";

import type { StoreDataSource } from "./native-data-source.js";
import { createProductionStoreReaders } from "./production-store-reader.svelte.js";
import { createProductionStoreState } from "./production-store-state.svelte.js";
import { createStoreRouteHistory } from "./store-route-history.svelte.js";
import { productionControllerSurface } from "./production-store-view.js";
import { createProductionReadServices } from "./production-read-services.js";
import { createProductionStoreMutations } from "./production-store-mutations.js";
import { createProductionStoreConflicts } from "./production-store-conflicts.js";
import {
  createProductionSelections,
  requireProductionSource,
  resetConnectionView,
  destroyProductionStoreLifecycle,
  authorizationReady,
  controllerAbortScopes,
} from "./production-store-lifecycle.svelte.js";
import { createProductionStoreActions } from "./production-store-actions.js";
import { createProductionMutationCommitter } from "./production-mutation-commit.js";

export function createProductionStoreController(
  options: { fetcher?: typeof fetch; now?: () => number } = {},
): StoreController {
  const history = createStoreRouteHistory();
  const data = createProductionStoreState();
  const freshness = new ApplicationFreshness(options.fetcher, () =>
    disconnect("Access denied", true),
  );
  const local = createLocalConfiguration(data, options.fetcher, freshness);
  let source: StoreDataSource | null = null,
    sessionToken: string | null = null;
  let session = 0;
  const { sources, threadNavigation } = repositoryReaders(
    () => data.repositories,
    () => sessionToken,
    options.fetcher,
    () => disconnect("Access denied", true),
  );
  let { feed, discovery, lifecycle } = controllerAbortScopes();
  const { invalidate, signal, version } = createReadScope();
  const follow = (active: StoreDataSource) =>
    automatic.follow(active, (feed = new AbortController()).signal);
  const activeSource = () => requireProductionSource(source);
  const current = (ticket: number, active: StoreDataSource) =>
    ticket === version() && active === source;
  const { reader, historyQuery } = createProductionStoreReaders({
    data,
    history,
    source: activeSource,
    version,
    begin: invalidate,
    signal,
    current,
  });

  const {
    projects,
    clear: clearProjects,
    stopEvents: stopProjectEvents,
    poll: pollProjects,
    discover: discoverProjects,
  } = createProductionProjects(
    data,
    history,
    () => sessionToken,
    () => session,
    () => feed.abort(),
    options.fetcher,
    () => automatic.request(),
    () => controller,
    sources,
    freshness,
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

  const { search, settings } = createProductionReadServices({
    data,
    history,
    source: activeSource,
    version,
    signal,
    current,
    restoreRoute,
    restore: reader.restorePage,
    freshness,
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
    invalidate();
    automatic.reset();
    feed.abort();
    stopProjectEvents();
    discovery.abort();
    if (!preserveSession) {
      freshness.stop();
      sources.clear();
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
        nextVersion: invalidate,
        version,
        nextSession: () => ++session,
        current,
        disconnect,
        configure: mutations.configure,
        follow,
        connected: threadsIntegration.connected,
        projects,
        reader,
        fetcher: options.fetcher,
        sources,
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
    version,
    discovery: (abort) => {
      discovery = abort;
    },
    token: (token) => {
      sessionToken = token;
    },
    fetcher: options.fetcher,
    events: (token) => freshness.configure(freshness.supported, token),
  });
  const { refresh, selectOperation } = createProductionRefresh({
    data,
    history,
    source: activeSource,
    projects,
    threads: threadsIntegration,
    search,
    reader,
    discover: discoverProjects,
    refreshThreadNavigation: () =>
      sources.refreshThreads(threadNavigation, history.route.repository),
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
  const { selectRevision, branchActions } = createProductionSelections({
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
    destroy: () => {
      local.destroy();
      destroyProductionStoreLifecycle(lifecycle, disconnect, history);
    },
    refresh,
    selectOperation,
    restoreRoute,
    activeRepository: () =>
      data.status.connection === "connected" ? source?.repository : undefined,
    invalidate,
  });

  const automatic = startProductionSessionEvents(
    {
      data,
      history,
      source: () => source,
      connect,
      disconnect,
      restore: restoreRoute,
      fetcher: options.fetcher,
      signal: lifecycle.signal,
      automatic: {
        projects,
        threads,
        pollProjects,
        sources,
        active: () => authorizationReady(data, sessionToken),
        refresh: (changes) => refresh(true, changes),
      },
    },
    freshness,
  );

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
      ...local,
      freshness,
      actions: { ...actions, openThreads, openRoute },
    },
    automatic.pause,
  );
  return controller;
}
