import {
  clearProductionStoreView,
  type ProductionStoreState,
} from "./production-store-state.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
import type { createProductionProjects } from "./production-projects-integration.js";
import type { createThreadsIntegration } from "./production-threads-integration.js";
import type { createProductionStoreReader } from "./production-store-reader.svelte.js";
import type { createProductionStoreSearch } from "./production-store-search-controller.js";
export function createProductionRefresh(options: {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source: () => StoreDataSource;
  projects: ReturnType<typeof createProductionProjects>["projects"];
  threads: ReturnType<typeof createThreadsIntegration>;
  reader: ReturnType<typeof createProductionStoreReader>;
  search: ReturnType<typeof createProductionStoreSearch>;
  stopFeed: () => void;
  follow: (source: StoreDataSource) => void;
  invalidate: () => void;
}) {
  const { data, history, projects, threads, reader, search } = options;
  return {
    async refresh() {
      if (await threads.refresh()) return;
      if (history.route.page === "projects") {
        await projects.refresh();
        data.status.refreshAvailable = false;
        return;
      }
      const active = options.source();
      options.stopFeed();
      const captured = await active.snapshot(true);
      try {
        if (options.source() !== active) return;
      } catch {
        return;
      }
      data.heads = [...captured.heads];
      data.head = captured;
      const selected = data.heads.length === 1 ? data.heads[0]! : null;
      options.invalidate();
      clearProductionStoreView(data);
      search.clear();
      history.refresh(selected);
      data.status.refreshAvailable = false;
      options.follow(active);
      if (selected) await reader.restorePage();
    },
    async selectOperation(id: string) {
      if (!data.heads.includes(id)) return;
      options.invalidate();
      clearProductionStoreView(data);
      search.clear();
      history.selectSnapshot(id);
      await reader.restorePage();
    },
  };
}
