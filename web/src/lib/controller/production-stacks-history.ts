import type { StoreDataSource } from "./native-data-source.js";
import { createProductionStacks } from "./production-stacks.svelte.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
import {
  createProductionStoreReaders,
  type ReaderContext,
  type createProductionStoreReader,
} from "./production-store-reader.svelte.js";

/** Accepted local operations feed focused history without disposing the canvas. */
export function stacksHistory(
  data: ProductionStoreState,
  history: ReturnType<typeof createStoreRouteHistory>,
  reader: ReturnType<typeof createProductionStoreReader>,
  invalidate: () => unknown,
) {
  return (repository: string, operation: string) => {
    history.stacks.acceptOperation(repository, operation);
    const route = history.route;
    if (
      route.repository === repository &&
      route.context === "stacks" &&
      route.stackHistory !== "pinned" &&
      route.snapshotOperationId !== operation &&
      data.status.navigation !== "loading"
    ) {
      history.refresh(operation);
      invalidate();
      void reader.restorePage();
    }
  };
}

export function createProductionStacksReaders(
  options: Omit<ReaderContext, "operation" | "report"> & {
    begin(): number;
    fetcher?: typeof fetch;
    localSource(): StoreDataSource | null;
  },
) {
  const { reader, historyQuery } = createProductionStoreReaders({
    ...options,
    restoreStacks: (contextOnly) => stacks.restoreRoute(contextOnly),
  });
  const stacks = createProductionStacks({
    data: options.data,
    source: options.localSource,
    route: () => options.history.route,
    fetcher: options.fetcher,
    released: (repository) => options.history.stacks.clear(repository),
    acceptedOperation: stacksHistory(
      options.data,
      options.history,
      reader,
      options.begin,
    ),
  });
  return { reader, historyQuery, stacks };
}
