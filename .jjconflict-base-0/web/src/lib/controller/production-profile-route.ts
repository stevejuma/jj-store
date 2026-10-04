import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreRoute } from "./contracts.js";
import { validateRepositoryRoute } from "./repository-source.js";
import { clearProductionStoreView } from "./production-store-state.svelte.js";
export function createProfileRouteRestorer(context: {
  data: ProductionStoreState;
  route(): StoreRoute;
  source(): StoreDataSource | null;
  projects: { restore(): Promise<void> };
  connect(repository: string, token: string | null): Promise<void>;
  token(): string | null;
  invalidate(): void;
  restoreThreads?(): Promise<boolean>;
  restore(): Promise<void>;
}) {
  return async () => {
    const route = context.route(),
      source = context.source();
    context.invalidate();
    if (route.page === "projects") {
      await context.projects.restore();
      return;
    }
    if (source && route.repository !== source.repository) {
      await context.connect(route.repository, context.token());
      return;
    }
    if (await context.restoreThreads?.()) return;
    if (
      !source ||
      !route.snapshotOperationId ||
      !validateRepositoryRoute(context.data, route, source)
    )
      return;
    clearProductionStoreView(context.data);
    await context.restore();
  };
}
