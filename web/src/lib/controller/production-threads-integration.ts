import { ApiError } from "../api/client.js";
import { createThreadsController } from "./production-threads.svelte.js";
import { initialStoreRoute } from "./store-route-history.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";
export function createThreadsIntegration(options: {
  source: () => Pick<StoreDataSource, "repository" | "threads"> | null;
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  connectRepository: (
    repository: string,
    token: string | null,
  ) => Promise<void>;
}) {
  const { data, history } = options;
  const threads = createThreadsController({
    source: () => {
      const t = options.source()?.threads;
      if (!t) throw new ApiError("threads_unavailable");
      return t;
    },
    repository: () => options.source()?.repository ?? "",
    principal: () =>
      data.repositories.find((r) => r.id === options.source()?.repository)
        ?.principal ?? "",
    route: () => history.route,
    selectReply: (replyId) => history.push({ ...history.route, replyId }),
    select: (threadId) =>
      history.push({
        ...initialStoreRoute(),
        repository: options.source()?.repository ?? history.route.repository,
        snapshotOperationId: history.route.snapshotOperationId,
        page: "threads",
        threadId,
      }),
  });
  return {
    threads,
    async restore() {
      if (history.route.page !== "threads" || !options.source()) return false;
      await threads.restore();
      return true;
    },
    async refresh() {
      if (history.route.page !== "threads") return false;
      await threads.refresh();
      data.status.refreshAvailable = false;
      return true;
    },
    async connected() {
      if (history.route.page === "threads") await threads.restore();
      else await threads.list();
    },
    async openThreads(id: string | null = null) {
      if (!options.source()) {
        const repo = data.repositories[0];
        if (!repo) return;
        history.push({
          ...initialStoreRoute(),
          repository: repo.id,
          page: "threads",
          threadId: id,
        });
        await options.connectRepository(repo.id, null);
      }
      await threads.open(id);
    },
  };
}
