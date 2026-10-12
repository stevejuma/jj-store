import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { resetConnectionView } from "./production-store-lifecycle.svelte.js";

interface ResetScope {
  data: ProductionStoreState;
  invalidate: () => unknown;
  stacks: { clear(): void };
  automatic: { reset(): void };
  feed: AbortController;
  stopProjectEvents: () => void;
  discovery: AbortController;
  freshness: { stop(): void };
  sources: { clear(): void };
  clearProjects: () => void;
  threadNavigation: { clear(): void };
  threads: { clear(preserveSession: boolean): void };
  dropSource: () => void;
  settings: { clear(): void };
  mutations: { configure(actions: []): void; reset(): void };
  historyQuery: { dispose(): void };
  search: { clear(): void };
  globalSearch: { clear(): void };
  referenceSearch: { clear(): void };
  authorizationLost?: () => void;
  clearToken: () => void;
}
export function resetProductionSession(
  scope: ResetScope,
  message: string,
  denied: boolean,
  preserveSession: boolean,
) {
  const repositoryStatus = scope.data.status.repositories;
  scope.invalidate();
  scope.stacks.clear();
  scope.automatic.reset();
  scope.feed.abort();
  scope.stopProjectEvents();
  scope.discovery.abort();
  if (!preserveSession) {
    scope.freshness.stop();
    scope.sources.clear();
    scope.clearProjects();
    scope.threadNavigation.clear();
  }
  scope.threads.clear(preserveSession);
  scope.dropSource();
  scope.settings.clear();
  scope.mutations.configure([]);
  scope.mutations.reset();
  scope.historyQuery.dispose();
  scope.search.clear();
  scope.globalSearch.clear();
  scope.referenceSearch.clear();
  resetConnectionView(scope.data, message, denied);
  if (denied) scope.authorizationLost?.();
  if (preserveSession) scope.data.status.repositories = repositoryStatus;
  else if (scope.data.accessMode === "authenticated") {
    scope.clearToken();
    scope.data.repositories = [];
  }
}

export function createProductionDisconnect(scope: () => ResetScope) {
  return (message = "", denied = false, preserveSession = false) =>
    resetProductionSession(scope(), message, denied, preserveSession);
}
