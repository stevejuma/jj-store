import { createGlobalSearch } from "./global-search.svelte.js";
import type { StoreRoute, StoreRepositorySummary } from "./contracts.js";
export function createEntitySearch(options: {
  repositories(): readonly StoreRepositorySummary[];
  token(): string | null;
  route(): StoreRoute;
  push(route: StoreRoute): void;
  repository(): string | undefined;
  projects: { restore(): Promise<void> };
  connectRepository(repository: string, token: string | null): Promise<void>;
  restoreRoute(): Promise<void>;
  fetcher?: typeof fetch;
}) {
  async function openRoute(route: StoreRoute) {
    options.push(route);
    if (route.page === "projects") await options.projects.restore();
    else if (options.repository() !== route.repository)
      await options.connectRepository(route.repository, options.token());
    else await options.restoreRoute();
  }
  const settings = { ...options, open: openRoute };
  return {
    openRoute,
    globalSearch: createGlobalSearch(settings),
    referenceSearch: createGlobalSearch(settings),
  };
}
