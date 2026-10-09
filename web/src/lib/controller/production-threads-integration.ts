import { ApiError } from "../api/client.js";
import { supportsRepositoryThreads } from "./repository-thread-navigation.svelte.js";
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
  const threadHome = () => data.repositories.find(supportsRepositoryThreads);
  return {
    threads,
    available() {
      const source = options.source();
      return source ? !!source.threads : !!threadHome();
    },
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
      if (!options.source()?.threads) {
        threads.clear(true);
        if (history.route.page === "threads")
          throw new ApiError("threads_unavailable");
        return;
      }
      if (history.route.page === "threads") await threads.restore();
      else await threads.list();
    },
    async openThreads(id: string | null = null, repository?: string) {
      const active = options.source();
      const target =
        repository ?? (active?.threads ? active.repository : threadHome()?.id);
      const repo = data.repositories.find(
        (candidate) =>
          candidate.id === target && supportsRepositoryThreads(candidate),
      );
      if (!repo) throw new ApiError("threads_unavailable");
      if (active?.repository !== repo.id || !active.threads) {
        history.push({
          ...initialStoreRoute(),
          repository: repo.id,
          page: "threads",
          threadId: id,
        });
        await options.connectRepository(repo.id, null);
        if (
          history.route.repository !== repo.id ||
          history.route.page !== "threads" ||
          history.route.threadId !== id
        )
          return;
      }
      if (options.source()?.repository !== repo.id) return;
      if (!options.source()?.threads) throw new ApiError("threads_unavailable");
      await threads.open(id);
    },
  };
}
