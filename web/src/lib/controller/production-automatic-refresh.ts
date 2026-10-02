import { pretty } from "../api/client.js";
import { createAutomaticRefresh } from "./automatic-refresh.js";
import { followProductionStoreEvents } from "./production-store-events.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { StoreRoute } from "./contracts.js";
import type { ProjectsHubController } from "./projects-hub-types.js";
import type { createThreadsController } from "./production-threads.svelte.js";
export function createProductionAutomaticRefresh(options: {
  data: ProductionStoreState;
  route(): StoreRoute;
  source(): StoreDataSource | null;
  active(): boolean;
  projects: ProjectsHubController;
  threads: ReturnType<typeof createThreadsController>;
  pollProjects(): Promise<boolean>;
  refresh(): Promise<void>;
  report(cause: unknown): void;
  signal: AbortSignal;
}) {
  const { data, projects, threads } = options;
  const projectReadError = () =>
    projects.error ||
    projects.detail?.state.error ||
    projects.homes.some((home) => home.error) ||
    projects.reviews.homes.some((home) => home.error) ||
    projects.activity.homes.some((home) => home.error) ||
    projects.repositories.entries.some((entry) => entry.error);
  const automatic = createAutomaticRefresh({
    ...options,
    poll: async () => {
      const directoryChanged = await options.pollProjects();
      if (options.route().page === "projects")
        return directoryChanged || Boolean(projectReadError());
      if (directoryChanged) await projects.navigation.load(true);
      const active = options.source();
      if (!active || data.status.connection !== "connected") return false;
      const head = await active.snapshot(true);
      return (
        options.source() === active &&
        (directoryChanged ||
          pretty(head) !== pretty(data.head) ||
          Boolean(data.status.error) ||
          (options.route().page === "threads" && threads.status === "failed"))
      );
    },
    refresh: async () => {
      if (
        data.status.connection === "connecting" ||
        projects.loading ||
        projects.detail?.state.writing ||
        threads.busy ||
        data.reviewActionRequestId ||
        ["previewing", "prepared", "applying", "unknown"].includes(
          data.mutation.status,
        ) ||
        data.status.navigation === "loading"
      )
        return false;
      await options.refresh();
      return !(options.route().page === "projects"
        ? projectReadError()
        : options.route().page === "threads"
          ? threads.status === "failed"
          : data.status.error);
    },
  });
  return {
    ...automatic,
    follow(active: StoreDataSource, signal: AbortSignal) {
      void followProductionStoreEvents(
        active,
        () => active === options.source(),
        data,
        signal,
        automatic.request,
      );
    },
  };
}
