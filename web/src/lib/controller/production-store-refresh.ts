import {
  storeLoadedWindows,
  reloadStoreWindows,
} from "./store-refresh-pages.js";
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
  refreshThreadNavigation?: () => Promise<void>;
}) {
  const { data, history, projects, threads, reader, search } = options;
  return {
    async refresh(automatic = false) {
      void options.refreshThreadNavigation?.();
      if (await threads.refresh()) return;
      if (history.route.page === "projects") {
        await projects.refresh();
        data.status.refreshAvailable = false;
        return;
      }
      const active = options.source(),
        route = history.route;
      const oldHead = data.head,
        counts = storeLoadedWindows(data);
      const followedHead =
        oldHead?.heads.length === 1 &&
        route.snapshotOperationId === oldHead.heads[0];
      if (!automatic) options.stopFeed();
      const captured = await active.snapshot(true);
      try {
        if (options.source() !== active || history.route !== route) return;
      } catch {
        return;
      }
      data.heads = [...captured.heads];
      data.head = captured;
      const selected =
        automatic && !followedHead
          ? route.snapshotOperationId
          : data.heads.length === 1
            ? data.heads[0]!
            : automatic
              ? route.snapshotOperationId
              : null;
      const followedRef = data.choices.find(
        (choice) =>
          (route.bookmark
            ? choice.kind === "bookmark" && choice.label === route.bookmark
            : route.changeId
              ? choice.value === `change:${route.changeId}`
              : false) &&
          choice.targets.length === 1 &&
          choice.targets[0] === route.commitId,
      );
      const query = data.searchQuery;
      options.invalidate();
      if (!automatic) {
        clearProductionStoreView(data);
        search.clear();
      } else {
        data.pathPages.clear();
        data.knownDirectories.clear();
        data.fileTerms.clear();
      }
      if (automatic && followedHead && followedRef && route.page === "files")
        history.replace({
          ...route,
          commitId: null,
          changeId: route.bookmark ? null : route.changeId,
        });
      history.refresh(selected);
      data.status.refreshAvailable = false;
      if (!automatic) options.follow(active);
      if (selected) await reader.restorePage();
      if (automatic && selected) {
        if (["branches", "log", "oplog"].includes(history.route.page))
          await reader.refreshCollaborationContext?.();
        const retainedRoute = history.route;
        await reloadStoreWindows(
          data,
          retainedRoute,
          counts,
          reader,
          () => history.route === retainedRoute && options.source() === active,
        );
      }
      if (automatic && query) await search.run(query);
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
