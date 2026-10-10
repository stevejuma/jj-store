import type { StoreDataSource } from "./native-data-source.js";
import type { StoreSearchResult } from "./contracts.js";
import { storeErrorMessage } from "./store-error.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
import {
  assertSearchPage,
  presentSearchResults,
  routeForSearchResult,
} from "./production-store-search.js";

export function createProductionStoreSearch(options: {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source: () => StoreDataSource;
  isCurrent: (source: StoreDataSource) => boolean;
  restoreRoute: () => Promise<void>;
}) {
  let version = 0;
  const { data, history } = options;

  function clear() {
    version += 1;
    Object.assign(data, {
      searchQuery: "",
      searchResults: [],
      searchCursor: null,
      searchStale: false,
      searchDegraded: false,
      searchTruncated: false,
      searchError: "",
    });
    data.status.search = "idle";
  }

  async function run(query: string, more = false) {
    const normalized = query.trim();
    if (!normalized) {
      clear();
      return;
    }
    data.searchQuery = normalized;
    if (!data.searchAvailable) {
      data.status.search = "unavailable";
      data.searchError = "Search is not configured for this repository.";
      return;
    }
    const operation = history.route.snapshotOperationId;
    if (!operation) {
      data.status.search = "unavailable";
      data.searchError = "Choose an operation snapshot before searching.";
      return;
    }
    const source = options.source();
    const cursor = more ? (data.searchCursor ?? undefined) : undefined;
    const ticket = ++version;
    if (!more) {
      data.searchResults = [];
      data.searchCursor = null;
    }
    data.searchError = "";
    data.status.search = more ? "refreshing" : "loading";
    try {
      const page = await source.search({
        query: normalized,
        operation,
        cursor,
      });
      if (ticket !== version || !options.isCurrent(source)) return;
      assertSearchPage(page, source.repository, operation);
      const incoming = presentSearchResults(page.results);
      const results = more ? [...data.searchResults, ...incoming] : incoming;
      data.searchResults = [
        ...new Map(results.map((result) => [result.id, result])).values(),
      ];
      data.searchCursor = page.next_cursor;
      data.searchStale = page.is_stale;
      data.searchDegraded = page.degraded;
      data.searchTruncated = page.truncated;
      data.status.search = data.searchResults.length ? "ready" : "empty";
    } catch (cause) {
      if (ticket !== version || !options.isCurrent(source)) return;
      data.status.search = "failed";
      data.searchError = storeErrorMessage(cause);
    }
  }

  async function select(result: StoreSearchResult) {
    const next = routeForSearchResult(history.route, result);
    clear();
    history.push(next);
    await options.restoreRoute();
  }

  return {
    clear,
    run,
    select,
    loadMore: () => run(data.searchQuery, true),
  };
}
