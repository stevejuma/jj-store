import { createProductionSessionState } from "./production-session-state.svelte.js";
import { createProductionDisconnect } from "./production-session-reset.js";
import {
  connectionActivity,
  connectionEvents,
} from "./production-connection-activity.js";
import { connectionViews } from "./production-connection-views.js";
import { agentsRoute } from "./agents-route-codec.js";
import { createProductionStacksReaders } from "./production-stacks-history.js";
import { createReadScope } from "./read-scope.js";
import { connectRepositorySource } from "./production-repository-connect.js";
import { createProductionRefresh } from "./production-store-refresh.js";
import { createThreadsIntegration } from "./production-threads-integration.js";
import { createProfileRouteRestorer } from "./production-profile-route.js";
import { createProductionProjects } from "./production-projects-integration.js";
import { createProductionConnect } from "./production-store-connect.js";
import { startConnectionEvents } from "./production-session-events.js";

import type {
  StoreController,
  ProductionStoreControllerOptions,
} from "./contracts.js";

import type { StoreDataSource } from "./native-data-source.js";
import { createProductionStoreState } from "./production-store-state.svelte.js";
import { createStoreRouteHistory } from "./store-route-history.svelte.js";
import { productionControllerSurface } from "./production-store-view.js";
import { createProductionReadServices } from "./production-read-services.js";
import {
  createProductionSelections,
  authorizationReady,
  createProductionConnectionScope,
  authorizedRepositoryReader,
} from "./production-store-lifecycle.svelte.js";
import { createProductionStoreActions } from "./production-store-actions.js";
import { createProductionCommandServices } from "./production-command-services.js";

export function createProductionStoreController(
  options: ProductionStoreControllerOptions = {},
): StoreController {
  const history = createStoreRouteHistory(options.browser);
  const data = createProductionStoreState();
  const sessionState = createProductionSessionState();
  let session = 0;
  const connection = createProductionConnectionScope(
    data,
    options.fetcher,
    () => sessionState.token,
    () => disconnect("Access denied", true),
    options.active,
  );
  const { sources, threadNavigation, local, freshness, lifecycle } = connection;
  let { feed, discovery } = connection;
  const { invalidate, signal, version } = createReadScope();
  const follow = (active: StoreDataSource) => {
    if (options.active?.() ?? true)
      automatic.follow(active, (feed = new AbortController()).signal);
  };
  const current = (ticket: number, active: StoreDataSource) =>
    ticket === version() && active === sessionState.source;
  const reads = { data, history, source: sessionState.requireSource };
  const snapshots = { ...reads, version, signal, current };
  const { reader, historyQuery, stacks } = createProductionStacksReaders({
    ...snapshots,
    localSource: () => sessionState.source,
    begin: invalidate,
    fetcher: options.fetcher,
  });

  const {
    projects,
    clear: clearProjects,
    stopEvents: stopProjectEvents,
    discover: discoverProjects,
  } = createProductionProjects(
    data,
    history,
    () => sessionState.token,
    () => session,
    () => feed.abort(),
    options.fetcher,
    () => automatic.request(),
    () => controller,
    sources,
    freshness,
  );
  const threadsIntegration = createThreadsIntegration({
    profiles: local.directory,
    source: () => sessionState.source,
    data,
    history,
    connectRepository,
    refreshDirectory: (home) => sources.refreshThreads(threadNavigation, home),
  });
  const { threads, openThreads } = threadsIntegration;
  const restoreRoute = createProfileRouteRestorer({
    data,
    route: () => history.route,
    source: () => sessionState.source,
    projects,
    connect: connectRepository,
    token: () => sessionState.token,
    restoreThreads: threadsIntegration.restore,
    invalidate,
    restore: reader.restorePage,
  });

  const { search, settings } = createProductionReadServices({
    ...snapshots,
    restoreRoute,
    restore: restoreRoute,
    freshness,
  });

  const { openRoute, globalSearch, referenceSearch, threadWorkspaces } =
    connectionViews({
      data,
      history,
      token: () => sessionState.token,
      source: () => sessionState.source,
      projects,
      connectRepository,
      restoreRoute,
      threadNavigation,
      sources,
      openThreads,
      fetcher: options.fetcher,
    });
  local.execution.onNavigate = (view, id, relatedId) =>
    void openRoute(agentsRoute(view, id, relatedId));
  const disconnect = createProductionDisconnect(() => ({
    ...connection,
    data,
    invalidate,
    stacks,
    automatic,
    feed,
    stopProjectEvents,
    discovery,
    clearProjects,
    threads,
    dropSource: sessionState.closeSource,
    settings,
    mutations,
    historyQuery,
    search,
    globalSearch,
    referenceSearch,
    authorizationLost: options.authorizationLost,
    clearToken: sessionState.clearToken,
  }));
  async function connectRepository(repository: string, token: string | null) {
    await connectRepositorySource(
      {
        data,
        history,
        token: () => sessionState.token,
        setToken: sessionState.setToken,
        source: () => sessionState.source,
        setSource: sessionState.setSource,
        nextVersion: invalidate,
        version,
        nextSession: () => ++session,
        current,
        disconnect,
        configure: mutations.configure,
        follow,
        connected: async () => {
          await threadsIntegration.connected();
          await stacks.restore(false);
        },
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
    discovery: (abort) => (discovery = abort),
    token: sessionState.setToken,
    fetcher: options.fetcher,
    events: connectionEvents(freshness, options.active),
  });
  const { refresh, selectOperation } = createProductionRefresh({
    ...reads,
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
  const { mutations, conflictUploads } = createProductionCommandServices({
    ...reads,
    current: (active) => active === sessionState.source,
    refresh,
  });
  const { selectRevision, branchActions } = createProductionSelections({
    data,
    history,
    reader,
    invalidate,
  });

  const actions = createProductionStoreActions({
    ...reads,
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
      threadWorkspaces.destroy();
      connection.destroy(disconnect, history);
    },
    refresh,
    selectOperation,
    restoreRoute,
    ensureStacks: () => stacks.ensure(),
    activeRepository: () =>
      data.status.connection === "connected"
        ? sessionState.source?.repository
        : undefined,
    invalidate,
  });

  const automatic = startConnectionEvents(
    {
      data,
      history,
      source: () => sessionState.source,
      connect,
      disconnect,
      restore: restoreRoute,
      fetcher: options.fetcher,
      signal: lifecycle.signal,
      connectionActive: options.active,
      projects,
      threads,
      sources,
      active: () =>
        (options.active?.() ?? true) &&
        authorizationReady(data, sessionState.token),
      refresh: (changes) => refresh(true, changes),
    },
    freshness,
  );

  const controller: StoreController = productionControllerSurface(
    data,
    history,
    () => sessionState.source,
    () => sessionState.token,
    {
      historyQuery,
      stacks,
      threads: threadsIntegration,
      projects,
      threadNavigation,
      threadWorkspaces,
      acquireRepository: authorizedRepositoryReader(
        data,
        sources,
        () => sessionState.token,
      ),
      now: options.now,
      globalSearch,
      referenceSearch,
      ...local,
      freshness,
      actions: {
        ...actions,
        openThreads,
        openRoute,
        setConnectionActive: connectionActivity(
          automatic.pause,
          () => feed,
          stopProjectEvents,
          () => sessionState.source,
          () => history.route.page !== "settings",
          follow,
          freshness,
          () => sessionState.token,
          local.execution,
        ),
      },
    },
    automatic.pause,
  );
  return controller;
}
