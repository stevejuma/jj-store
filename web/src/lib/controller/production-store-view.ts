import { snapshotOperation } from "./repository-profile.js";
import type {
  StoreController,
  StoreControllerState,
  StoreRoute,
} from "./contracts.js";
import { nextPathPage } from "./production-file-pages.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

export function productionStoreView(
  data: ProductionStoreState,
  route: StoreRoute,
  options: {
    connected: boolean;
    installationReady?: boolean;
    repository: string;
    canBack: boolean;
    canForward: boolean;
    now: number;
  },
): StoreControllerState {
  const repository = options.repository;
  return {
    installationReady: options.installationReady ?? false,
    nativeRepositoryCreation: data.nativeRepositoryCreation,
    nativeRepositoryDeletion: data.nativeRepositoryDeletion,
    repositoryConversion: data.repositoryConversion,
    accessMode: data.accessMode,
    storageReady: data.storageReady,
    backups: data.backups,
    route,
    canNavigateBack: options.canBack,
    canNavigateForward: options.canForward,
    repositories: data.repositories,
    repository: {
      ...data.repositories.find((candidate) => candidate.id === repository),
      id: repository,
      name:
        data.repositories.find((candidate) => candidate.id === repository)
          ?.name ?? repository,
      permission:
        data.repositories.find((candidate) => candidate.id === repository)
          ?.permission ?? null,
    },
    operationHeads: data.heads,
    comparisonParents: data.comparisonParents,
    conflictCount: data.conflictCount,
    conflicts: data.conflicts,
    conflictResolutionActions: data.conflictResolutionActions,
    conflictUploadBytes: data.conflictUploadBytes,
    unavailableComparisonCount: data.unavailableComparisonCount,
    canLoadMoreBookmarks: data.refsCursor !== null,
    canLoadMoreGraph: data.graphCursor !== null,
    canLoadMoreFiles:
      data.status.files !== "failed" &&
      Boolean(nextPathPage(data.pathPages, data.fileViewMode, route.directory)),
    canLoadMoreDetail: data.inspectionContinuation !== null,
    canLoadMoreAgents: data.agentCursor !== null,
    canLoadMoreTranscript: data.transcriptCursor !== null,
    canLoadMoreReviews: data.reviewCursor !== null,
    fileViewMode: data.fileViewMode,
    collapsedDirectoryPaths: data.collapsedDirectoryPaths,
    graph: data.graph,
    evolution: data.evolution,
    now: options.now,
    detail: data.detail,
    fileRevision: data.fileRevision,
    fileRevisionStatus: data.fileRevisionStatus,
    bookmarks: data.bookmarks,
    choices: data.choices,
    workingCopies: data.workingCopies,
    workingCopiesStatus: data.workingCopiesStatus,
    canLoadMoreWorkingCopies: data.workingCopiesCursor !== null,
    targets: data.targets,
    snapshot: data.snapshot,
    blob: data.blob,
    search: {
      available: data.searchAvailable,
      query: data.searchQuery,
      results: data.searchResults,
      status: data.status.search,
      error: data.searchError,
      canLoadMore: data.searchCursor !== null,
      isStale: data.searchStale,
      degraded: data.searchDegraded,
      truncated: data.searchTruncated,
    },
    settings: {
      searchSettings: data.searchSettings,
      storageSettings: data.storageSettings,
      syncSettings: data.syncSettings,
      workspaceLinks: data.workspaceLinks,
      workspaceLinksState: data.workspaceLinksState,
      workspaceLinksError: data.workspaceLinksError,
      workspaceValidation: data.workspaceValidation,
      workspacePath: data.workspacePath,
      workspaceLinking: data.workspaceLinking,
      workspacePicking: data.workspacePicking,
      workspacePickerError: data.workspacePickerError,
      capabilities: data.capabilities,
      snapshot: data.head,
      searchStatus: data.searchStatus,
      searchStatusState: data.searchStatusState,
      searchStatusError: data.searchStatusError,
      indexPreparation: data.indexPreparation,
      indexPreparationKind: data.indexPreparationKind,
      indexProgress: data.indexProgress,
      indexCancellationUnconfirmed: data.indexCancellationUnconfirmed,
      indexPreparationError: data.indexPreparationError,
      indexTaskStatus: data.indexTaskStatus,
      indexRequired: data.neededIndex !== null,
      indexOperation:
        (data.neededIndex ? snapshotOperation(data.neededIndex) : null) ??
        route.snapshotOperationId,
    },
    mutation: data.mutation,
    agentSessions: data.agentSessions,
    agentSession: data.agentSession,
    agentRelations: data.agentRelations,
    agentTranscript: data.agentTranscript,
    reviewThreads: data.reviewThreads,
    reviewThread: data.reviewThread,
    reviewPatch: data.reviewPatch,
    reviewContext: data.reviewContext,
    reviewActionMessage: data.reviewActionMessage,
    reviewActionRequestId: data.reviewActionRequestId,
    committedReviewRequestId: data.committedReviewRequestId,
    authenticated:
      options.connected &&
      (data.status.connection === "connected" ||
        data.status.repositories === "ready" ||
        data.status.repositories === "empty"),
    member: {
      id:
        data.accessMode === "local_trusted"
          ? "local"
          : repository || "disconnected",
      name: options.connected
        ? data.accessMode === "local_trusted"
          ? "Local"
          : "Repository connected"
        : "Disconnected",
      handle: options.connected
        ? data.accessMode === "local_trusted"
          ? "Local installation"
          : repository.slice(0, 12)
        : "Connect",
      initials: options.connected
        ? data.accessMode === "local_trusted"
          ? "L"
          : "JJ"
        : "—",
      presence: options.connected ? "online" : "offline",
      status: options.connected
        ? data.accessMode === "local_trusted"
          ? `${data.status.live} · trusted local access`
          : `${data.status.live} · token held in memory`
        : "No repository session",
    },
    status: data.status,
  };
}

export function productionControllerView(
  data: ProductionStoreState,
  history: ReturnType<
    typeof import("./store-route-history.svelte.js").createStoreRouteHistory
  >,
  source: import("./native-data-source.js").StoreDataSource | null,
  now: number,
  installationReady = false,
) {
  return productionStoreView(data, history.route, {
    connected: source !== null || installationReady,
    installationReady,
    repository: source?.repository ?? history.route.repository,
    canBack: history.canBack,
    canForward: history.canForward,
    now,
  });
}

export function productionSessionView(
  data: ProductionStoreState,
  history: Parameters<typeof productionControllerView>[1],
  source: Parameters<typeof productionControllerView>[2],
  token: string | null,
  now?: () => number,
) {
  return productionControllerView(
    data,
    history,
    source,
    now?.() ?? Math.floor(Date.now() / 1000),
    data.storageReady &&
      (data.accessMode === "local_trusted" || token !== null),
  );
}

export function productionControllerSurface(
  data: ProductionStoreState,
  history: Parameters<typeof productionControllerView>[1],
  source: () => Parameters<typeof productionControllerView>[2],
  token: () => string | null,
  options: {
    stacks?: {
      readonly controller: StoreController["stackWorkspace"];
      readonly loading: boolean;
    };
    threads: Pick<
      ReturnType<
        typeof import("./production-threads-integration.js").createThreadsIntegration
      >,
      "available" | "threads"
    >;
    projects: StoreController["projects"];
    threadNavigation?: StoreController["threadNavigation"];
    threadWorkspaces?: StoreController["threadWorkspaces"];
    acquireRepository?: StoreController["acquireRepository"];
    now?: () => number;
    actions: StoreController["actions"];
    globalSearch?: StoreController["globalSearch"];
    referenceSearch?: StoreController["referenceSearch"];
    historyQuery?: StoreController["historyQuery"];
    capture?: StoreController["capture"];
    execution?: StoreController["execution"];
    reload?: StoreController["reload"];
    agentAdministration?: StoreController["agentAdministration"];
    configuration?: StoreController["configuration"];
    directory?: StoreController["directory"];
    integrations?: StoreController["integrations"];
    freshness?: StoreController["freshness"];
  },
  pause?: StoreController["actions"]["setRefreshPaused"],
): StoreController {
  return {
    get blame() {
      return source()?.blame;
    },
    get stackWorkspaceLoading() {
      return options.stacks?.loading ?? false;
    },
    get stackWorkspace() {
      return options.stacks?.controller;
    },
    capture: options.capture,
    execution: options.execution,
    reload: options.reload,
    agentAdministration: options.agentAdministration,
    configuration: options.configuration,
    directory: options.directory,
    integrations: options.integrations,
    freshness: options.freshness,
    get threads() {
      return options.threads.available() ? options.threads.threads : undefined;
    },
    projects: options.projects,
    threadNavigation: options.threadNavigation,
    threadWorkspaces: options.threadWorkspaces,
    acquireRepository: options.acquireRepository,
    globalSearch: options.globalSearch,
    referenceSearch: options.referenceSearch,
    historyQuery: options.historyQuery,
    get state() {
      const view = productionSessionView(
        data,
        history,
        source(),
        token(),
        options.now,
      );
      const principal = options.directory?.state.profile?.principal;
      return principal
        ? { ...view, member: options.directory!.member(principal, view.member) }
        : view;
    },
    actions: { ...options.actions, setRefreshPaused: pause },
  };
}
