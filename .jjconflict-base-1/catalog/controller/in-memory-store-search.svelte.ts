import type {
  StoreRoute,
  StoreSearchResult,
  StoreSearchState,
} from "../../web/src/lib/controller/contracts.js";
import { routeForSearchResult } from "../../web/src/lib/controller/production-store-search.js";
import { searchResults as fixtureResults } from "../fixtures/search.js";

export function createInMemoryStoreSearch(
  route: () => StoreRoute,
  push: (route: StoreRoute) => void,
) {
  let query = $state("");
  let results = $state<readonly StoreSearchResult[]>([]);

  async function search(value: string) {
    query = value.trim();
    const term = query.toLocaleLowerCase();
    results = term
      ? fixtureResults.filter((result) =>
          `${result.title} ${result.description}`
            .toLocaleLowerCase()
            .includes(term),
        )
      : [];
  }

  async function select(result: StoreSearchResult) {
    await search("");
    push(routeForSearchResult(route(), result));
  }

  return {
    get state(): StoreSearchState {
      return {
        available: true,
        query,
        results,
        status: query ? (results.length ? "ready" : "empty") : "idle",
        error: "",
        canLoadMore: false,
        isStale: false,
        degraded: false,
        truncated: false,
      };
    },
    search,
    select,
  };
}
