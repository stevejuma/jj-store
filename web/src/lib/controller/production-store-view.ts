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
    route,
    canNavigateBack: options.canBack,
    canNavigateForward: options.canForward,
    repositories: data.repositories,
    repository: { id: repository, name: repository },
    operationHeads: data.heads,
    comparisonParents: data.comparisonParents,
    conflictCount: data.conflictCount,
    unavailableComparisonCount: data.unavailableComparisonCount,
    canLoadMoreBookmarks: data.refsCursor !== null,
    canLoadMoreGraph: data.graphCursor !== null,
    canLoadMoreFiles: Boolean(
      nextPathPage(data.pathPages, data.fileViewMode, route.directory),
    ),
    canLoadMoreDetail: data.inspectionContinuation !== null,
    fileViewMode: data.fileViewMode,
    collapsedDirectoryPaths: data.collapsedDirectoryPaths,
    graph: data.graph,
    now: options.now,
    detail: data.detail,
    bookmarks: data.bookmarks,
    choices: data.choices,
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
    authenticated: options.connected && data.status.connection === "connected",
    member: {
      id: repository || "disconnected",
      name: options.connected ? "Repository connected" : "Disconnected",
      handle: options.connected ? repository.slice(0, 12) : "Connect",
      initials: options.connected ? "JJ" : "—",
      presence: options.connected ? "online" : "offline",
      status: options.connected
        ? `${data.status.live} · token held in memory`
        : "No repository session",
    },
    status: data.status,
  };
}
