import { repositoryReaders } from "./repository-sources.js";
import { ApplicationFreshness } from "./application-freshness.js";
import { createLocalConfiguration } from "./production-local-configuration.js";
import { createProductionBranchActions } from "./production-branch-actions.js";
import { isRepositoryProjects } from "./projects-view-cache.js";
import {
  clearProductionStoreView,
  initialStatus,
} from "./production-store-state.svelte.js";
import { bootstrap as loadBootstrap } from "../api/bootstrap.js";
import { repositories as loadRepositories } from "../api/repositories.js";
import { ApiError } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { createProductionStoreReader } from "./production-store-reader.svelte.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { storeErrorMessage } from "./http-store-data-source.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

type History = ReturnType<typeof createStoreRouteHistory>;
type Reader = ReturnType<typeof createProductionStoreReader>;

export function requireProductionSource(source: StoreDataSource | null) {
  if (!source) throw new ApiError("not_connected");
  return source;
}

export function reportProductionStoreError(
  data: ProductionStoreState,
  cause: unknown,
) {
  if (cause instanceof DOMException && cause.name === "AbortError") return;
  if (cause instanceof ApiError && cause.code === "index_rebuild_required")
    data.neededIndex = cause.snapshot ?? null;
  data.status.error = storeErrorMessage(cause);
}

export function createProductionRevisionSelector(
  data: ProductionStoreState,
  history: History,
  reader: Reader,
) {
  return async (id: string | null) => {
    const page = history.route.page;
    const previous = history.route;
    const next = {
      ...previous,
      inspectedOperationId: page === "oplog" ? id : null,
      commitId: page === "oplog" ? null : id,
      evolutionCommitId: null,
      changeId:
        data.graph.revisions.find((revision) => revision.eventId === id)
          ?.changeId ?? null,
      path: null,
      agentSessionId: null,
      reviewThreadId: null,
    };
    history.push(next);
    if (history.route === previous) return;
    // Retire interrupted restoration without cancelling this graph's completion.
    data.status.navigation = "ready";
    Object.assign(data, {
      detail: undefined,
      blob: undefined,
      inspectionContinuation: null,
    });
    if (!id) return;
    if (page === "oplog") {
      await reader.loadOperation(id);
      await reader.loadOperationContext(id);
    } else if (page === "files") await reader.loadFiles(id);
    else {
      await Promise.all([
        reader.loadCommit(id),
        reader.loadRevisionContext(id),
      ]);
    }
  };
}

export function followProductionExternalRoutes(options: {
  data: ProductionStoreState;
  history: History;
  source: () => StoreDataSource | null;
  connect: (repository: string, token: string | null) => Promise<void>;
  disconnect: (message?: string) => void;
  restore: () => Promise<void>;
}) {
  options.history.onExternalRoute((result) => {
    if (result.kind === "invalid") {
      options.data.status.navigation = "failed";
      options.data.status.error = result.message;
      return;
    }
    if (
      result.kind === "route" &&
      (result.route.globalSettings ||
        result.route.page === "workflows" ||
        result.route.agentsHub ||
        (result.route.threadsHub && !result.route.repository) ||
        (result.route.page === "projects" &&
          !isRepositoryProjects(result.route)))
    ) {
      void options.restore();
      return;
    }
    const source = options.source();
    if (result.kind !== "route") return;
    if (source && result.route.repository === source.repository) {
      void options.restore();
      return;
    }
    if (
      options.data.repositories.some(
        (repository) => repository.id === result.route.repository,
      )
    )
      void options.connect(result.route.repository, null);
    else options.disconnect("Reconnect to open the requested repository.");
  });
}

export async function initializeProductionStore(options: {
  data: ProductionStoreState;
  history: History;
  connect: (repository: string, token: string | null) => Promise<void>;
  fetcher?: typeof fetch;
  signal: AbortSignal;
  events?(supported: boolean, token: string | null | undefined): Promise<void>;
}) {
  options.data.status.connection = "connecting";
  try {
    const bootstrap = await loadBootstrap(options.fetcher, options.signal);
    if (options.signal.aborted) return;
    options.data.accessMode = bootstrap.access_mode;
    options.data.storageReady = bootstrap.storage.ready;
    options.data.repositoryConversion =
      bootstrap.capabilities.local_repository_conversion === true;
    options.data.nativeRepositoryCreation =
      bootstrap.capabilities.local_native_repository_creation === true;
    options.data.directorySupported =
      bootstrap.capabilities.installation_directory === true;
    options.data.settingsSupported =
      bootstrap.capabilities.local_settings_management === true;
    options.data.captureSupported =
      bootstrap.capabilities.local_capture_management === true;
    options.data.executionSupported =
      bootstrap.capabilities.local_agent_execution === true ||
      bootstrap.capabilities.authenticated_agent_execution === true;
    options.data.backups = [...bootstrap.backups];
    await options.events?.(
      bootstrap.capabilities.application_events === true,
      bootstrap.access_mode === "authenticated" ? undefined : null,
    );
    if (options.signal.aborted) return;
    if (!bootstrap.storage.ready) {
      options.data.status.connection = "failed";
      options.data.status.error = "Authoritative storage is not ready.";
      return;
    }
    if (bootstrap.access_mode === "authenticated") {
      options.data.repositories = [];
      options.data.status.repositories = "idle";
      options.data.status.connection = "disconnected";
      return;
    }
    options.data.status.repositories = "loading";
    options.data.repositories = (
      await loadRepositories(null, options.fetcher, options.signal)
    ).map((repository) => ({ ...repository }));
    options.data.status.repositories = options.data.repositories.length
      ? "ready"
      : "empty";
    const requested = options.data.repositories.find(
      (repository) => repository.id === options.history.route.repository,
    );
    const selected =
      requested ??
      (options.data.repositories.length === 1
        ? options.data.repositories[0]
        : undefined);
    if (selected || options.history.route.page === "projects")
      await options.connect(selected?.id ?? "", null);
    else await options.connect("", null);
  } catch (cause) {
    if (cause instanceof DOMException && cause.name === "AbortError") return;
    options.data.status.connection = "failed";
    options.data.status.error = storeErrorMessage(cause);
  }
}

export function startProductionStoreLifecycle(options: {
  data: ProductionStoreState;
  history: History;
  source: () => StoreDataSource | null;
  connect: (repository: string, token: string | null) => Promise<void>;
  disconnect: (message?: string) => void;
  restore: () => Promise<void>;
  fetcher?: typeof fetch;
  signal: AbortSignal;
  events?(supported: boolean, token: string | null | undefined): Promise<void>;
}) {
  if (options.history.error)
    Object.assign(options.data.status, {
      navigation: "failed",
      error: options.history.error,
    });
  followProductionExternalRoutes(options);
  void initializeProductionStore(options);
}

export function resetConnectionView(
  data: ProductionStoreState,
  message: string,
  denied: boolean,
) {
  Object.assign(data, {
    heads: [],
    head: null,
    conflictResolutionActions: [],
    conflictUploadBytes: 0,
    searchAvailable: false,
  });
  clearProductionStoreView(data);
  Object.assign(data.status, initialStatus(), {
    connection: denied ? "denied" : "disconnected",
    error: message,
  });
}

export function destroyProductionStoreLifecycle(
  lifecycle: AbortController,
  disconnect: () => void,
  history: History,
) {
  lifecycle.abort();
  disconnect();
  history.destroy();
}

export function createProductionSelections(
  context: Parameters<typeof createProductionBranchActions>[0],
) {
  return {
    selectRevision: createProductionRevisionSelector(
      context.data,
      context.history,
      context.reader,
    ),
    branchActions: createProductionBranchActions(context),
  };
}

export function authorizationReady(
  data: ProductionStoreState,
  token: string | null,
) {
  return (
    data.storageReady && (data.accessMode === "local_trusted" || token !== null)
  );
}

export function controllerAbortScopes() {
  return {
    feed: new AbortController(),
    discovery: new AbortController(),
    lifecycle: new AbortController(),
  };
}

/** Repository access, invalidation and device-local services share one session owner. */
export function createProductionConnectionScope(
  data: ProductionStoreState,
  fetcher: typeof fetch | undefined,
  token: () => string | null,
  denied: () => void,
  active?: () => boolean,
) {
  const freshness = new ApplicationFreshness(fetcher, denied);
  const local = createLocalConfiguration(
    data,
    fetcher,
    freshness,
    token,
    denied,
    active,
  );
  const scopes = controllerAbortScopes();
  return {
    ...repositoryReaders(() => data.repositories, token, fetcher, denied),
    freshness,
    local,
    ...scopes,
    destroy(disconnect: () => void, history: History) {
      local.destroy();
      destroyProductionStoreLifecycle(scopes.lifecycle, disconnect, history);
    },
  };
}

/** Every acquired repository transport uses the current admitted catalog and session. */
export function authorizedRepositoryReader(
  data: ProductionStoreState,
  sources: import("./repository-sources.js").RepositorySources,
  token: () => string | null,
) {
  return (id: string) => {
    const repository = data.repositories.find(
      (candidate) => candidate.id === id,
    );
    if (!repository) throw new ApiError("access_denied");
    return sources.acquire(repository, token());
  };
}
