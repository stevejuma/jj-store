import { createEntitySearch } from "./production-entity-search.js";
import { createProductionThreadWorkspaces } from "./production-thread-workspaces.js";
import type { StoreController, StoreRoute } from "./contracts.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";
export function connectionViews(options: {
  data: ProductionStoreState;
  history: {
    route: StoreRoute;
    push(route: StoreRoute): void;
    canNavigateTo(route: StoreRoute): boolean;
  };
  token(): string | null;
  source(): StoreDataSource | null;
  projects: { restore(): Promise<void> };
  connectRepository(repository: string, token: string | null): Promise<void>;
  restoreRoute(): Promise<void>;
  threadNavigation: NonNullable<StoreController["threadNavigation"]>;
  sources: import("./repository-sources.js").RepositorySources;
  openThreads(thread: string, home?: string): Promise<void>;
  fetcher?: typeof fetch;
}) {
  const search = createEntitySearch({
    ...options,
    repositories: () => options.data.repositories,
    route: () => options.history.route,
    push: options.history.push,
    canOpen: options.history.canNavigateTo,
    repository: () => options.source()?.repository,
  });
  return {
    ...search,
    threadWorkspaces: createProductionThreadWorkspaces(options),
  };
}
