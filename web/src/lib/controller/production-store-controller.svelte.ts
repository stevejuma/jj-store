import { ApiError } from "../api/client.js";
import { repositories as loadRepositories } from "../api/repositories.js";
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
import { createProductionStoreSettings } from "./production-store-index.js";
import {
  createProductionRevisionSelector,
  reportProductionStoreError,
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

  async function restoreRoute() {
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
    discovery.abort();
    source?.close();
    source = null;
    Object.assign(data, { heads: [], head: null });
    settings.clear();
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
    if (preserveSession) data.status.repositories = repositoryStatus;
    else if (data.accessMode === "authenticated") {
      sessionToken = null;
      data.repositories = [];
    }
  }
  async function connectRepository(repository: string, token: string | null) {
    disconnect("", false, true);
    data.status.connection = "connecting";
    const ticket = ++version;
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
      if (history.route.snapshotOperationId) await reader.restorePage();
    } catch (cause) {
      if (!current(ticket, active)) return;
      const message = storeErrorMessage(cause);
      disconnect(message);
      data.status.connection = "failed";
    }
  }
  async function connect(repository: string, token: string | null) {
    if (data.accessMode !== "authenticated" || token === null)
      return connectRepository(repository, token);
    disconnect();
    const ticket = version;
    discovery = new AbortController();
    data.status.connection = "connecting";
    data.status.repositories = "loading";
    try {
      const repositories = await loadRepositories(
        token,
        options.fetcher,
        discovery.signal,
      );
      if (ticket !== version) return;
      data.repositories = repositories.map((candidate) => ({ ...candidate }));
      data.status.repositories = data.repositories.length ? "ready" : "empty";
      sessionToken = token;
      const requested = data.repositories.find(
        (candidate) =>
          candidate.id === (repository.trim() || history.route.repository),
      );
      const selected =
        requested ??
        (data.repositories.length === 1 ? data.repositories[0] : undefined);
      if (selected) await connectRepository(selected.id, token);
      else data.status.connection = "disconnected";
    } catch (cause) {
      const message = storeErrorMessage(cause);
      disconnect(message, cause instanceof ApiError);
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
    committed: createProductionMutationCommitter(history, refresh),
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
    activeRepository: () => source?.repository,
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
