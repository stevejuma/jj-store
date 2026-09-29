import type { StoreControllerState, StoreRoute } from "./contracts.js";
import { nextPathPage } from "./production-file-pages.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

export function productionStoreView(
  data: ProductionStoreState,
  route: StoreRoute,
  options: {
    connected: boolean;
    repository: string;
    canBack: boolean;
    canForward: boolean;
    now: number;
  },
): StoreControllerState {
  const repository = options.repository;
  return {
    accessMode: data.accessMode,
    storageReady: data.storageReady,
    backups: data.backups,
    route,
    canNavigateBack: options.canBack,
    canNavigateForward: options.canForward,
    repositories: data.repositories,
    repository: {
      id: repository,
      name:
        data.repositories.find((candidate) => candidate.id === repository)
          ?.name ?? repository,
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
    canLoadMoreFiles: Boolean(
      nextPathPage(data.pathPages, data.fileViewMode, route.directory),
    ),
    canLoadMoreDetail: data.inspectionContinuation !== null,
    canLoadMoreAgents: data.agentCursor !== null,
    canLoadMoreTranscript: data.transcriptCursor !== null,
    canLoadMoreReviews: data.reviewCursor !== null,
    fileViewMode: data.fileViewMode,
    collapsedDirectoryPaths: data.collapsedDirectoryPaths,
    graph: data.graph,
    now: options.now,
    detail: data.detail,
    fileRevision: data.fileRevision,
    bookmarks: data.bookmarks,
    choices: data.choices,
    workingCopies: data.workingCopies,
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
    mutation: data.mutation,
    agentSessions: data.agentSessions,
    agentSession: data.agentSession,
    agentRelations: data.agentRelations,
    agentTranscript: data.agentTranscript,
    reviewThreads: data.reviewThreads,
    reviewThread: data.reviewThread,
    reviewActionMessage: data.reviewActionMessage,
    reviewActionRequestId: data.reviewActionRequestId,
    authenticated: options.connected && data.status.connection === "connected",
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
