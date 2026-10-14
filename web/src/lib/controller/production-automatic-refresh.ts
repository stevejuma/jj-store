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
  refresh(
    changes?: readonly import("../api/application-events.js").FreshnessChange[],
  ): Promise<void>;
  report(cause: unknown): void;
  signal: AbortSignal;
  freshness?: import("./application-freshness.js").ApplicationFreshness;
  sources?: import("./repository-sources.js").RepositorySources;
}) {
  const { data, projects, threads } = options;
  let queued: import("../api/application-events.js").FreshnessChange[] = [];
  const projectReadError = (ids?: ReadonlySet<string>) =>
    projects.error ||
    ((!ids || ids.has(options.route().repository)) &&
      projects.detail?.state.error) ||
    projects.homes.some(
      (home) => (!ids || ids.has(home.repository.id)) && home.error,
    ) ||
    projects.reviews.homes.some(
      (home) => (!ids || ids.has(home.repository.id)) && home.error,
    ) ||
    projects.activity.homes.some(
      (home) => (!ids || ids.has(home.repository.id)) && home.error,
    ) ||
    projects.repositories.entries.some(
      (entry) => (!ids || ids.has(entry.repository.id)) && entry.error,
    );
  const automatic = createAutomaticRefresh({
    ...options,
    refresh: async () => {
      if (
        data.status.connection === "connecting" ||
        projects.loading ||
        projects.detail?.state.writing ||
        threads.busy ||
        data.indexPreparation === "loading" ||
        data.reviewActionRequestId ||
        ["refreshing", "loading"].includes(data.status.reviewAction) ||
        ["previewing", "prepared", "applying", "unknown"].includes(
          data.mutation.status,
        ) ||
        data.status.navigation === "loading"
      )
        return false;
      const batch = queued.length ? [...queued] : undefined;
      await options.refresh(batch);
      const accepted = !(options.route().page === "projects"
        ? projectReadError(
            batch && !batch.some((change) => change.repository === null)
              ? new Set(batch.map((change) => change.repository!))
              : undefined,
          )
        : options.route().page === "threads"
          ? threads.status === "failed"
          : data.status.error);
      if (accepted && batch)
        queued = queued.filter((change) => !batch.includes(change));
      return accepted;
    },
  });
  const unlisten = options.freshness?.listen(({ changes, reset, status }) => {
    data.status.live =
      status === "live" || status === "renewing" ? "live" : "reconnecting";
    for (const change of changes) {
      void options.sources?.invalidate(change.repository ?? undefined, [
        change.domain,
      ]);
    }
    projects.invalidate?.(
      reset
        ? [{ repository: null, domain: "repository", witness: "reset" }]
        : changes,
    );
    if (reset) void options.sources?.invalidate();
    const route = options.route();
    const matching = changes.filter((change) => {
      if (change.repository === null)
        return ["repository", "discovery", "workspaces", "conversion"].includes(
          change.domain,
        );
      if (route.page === "projects") {
        const tab = route.workTab ?? "projects";
        const domains =
          tab === "reviews"
            ? ["reviews", "work"]
            : tab === "agents"
              ? ["agents", "work"]
              : tab === "activity"
                ? ["code", "agents", "work"]
                : tab === "repositories"
                  ? ["code", "agents", "reviews", "work"]
                  : route.taskId
                    ? ["work", "threads"]
                    : ["work"];
        return (
          change.domain === "repository" || domains.includes(change.domain)
        );
      }
      if (change.repository !== route.repository) return false;
      return (
        change.domain === "repository" ||
        change.domain === "code" ||
        (route.page === "threads" && change.domain === "threads") ||
        (route.page === "agents" && change.domain === "agents") ||
        (["reviews", "files", "branches", "log", "oplog"].includes(
          route.page,
        ) &&
          change.domain === "reviews") ||
        (["files", "branches", "log", "oplog"].includes(route.page) &&
          change.domain === "agents")
      );
    });
    automatic.reschedule();
    if (reset || matching.length) {
      if (reset) queued = [];
      else queued.push(...matching);
      if (queued.length > 256) queued = [];
      data.status.refreshAvailable = true;
      automatic.request();
    }
  });
  options.signal.addEventListener("abort", () => unlisten?.(), { once: true });
  return {
    ...automatic,
    reset() {
      queued = [];
      automatic.reset();
    },
    follow(active: StoreDataSource, signal: AbortSignal) {
      if (options.freshness?.supported) return;
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
