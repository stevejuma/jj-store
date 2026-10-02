import { ApiError } from "../api/client.js";
import { createWorkController } from "./production-work.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
export function createWorkReader(
  source: () => StoreDataSource,
  history: ReturnType<typeof createStoreRouteHistory>,
  data: ProductionStoreState,
  restoreCode: () => Promise<void>,
) {
  const work = createWorkController({
    source: () => {
      const transport = source().work;
      if (!transport) throw new ApiError("work_unavailable");
      return transport;
    },
    route: () => history.route,
    select: (projectId, taskId) =>
      history.push({ ...history.route, page: "projects", projectId, taskId }),
    changed: () => {
      data.status.refreshAvailable = false;
    },
  });
  return {
    work,
    restorePage: () =>
      history.route.page === "projects" ? work.restore() : restoreCode(),
  };
}
export const hasBrowseContext = (route: import("./contracts.js").StoreRoute) =>
  Boolean(route.snapshotOperationId || route.page === "projects");
