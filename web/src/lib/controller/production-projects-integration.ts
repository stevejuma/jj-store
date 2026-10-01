import { repositories as loadRepositories } from "../api/repositories.js";
import { followProductionStoreEvents } from "./production-store-events.js";
import { ApiError } from "../api/client.js";
import {
  repositorySource,
  captureRepositoryActor,
} from "./repository-source.js";
import type { StoreDataSource } from "./native-data-source.js";
import { createStoreQueryClient } from "./store-query-client.js";
import { createProjectsHub } from "./production-projects-hub.svelte.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
export function createProductionProjects(
  data: ProductionStoreState,
  history: ReturnType<
    typeof import("./store-route-history.svelte.js").createStoreRouteHistory
  >,
  token: () => string | null,
  session: () => number,
  pauseCodeEvents: () => void,
  fetcher?: typeof fetch,
) {
  const context = {
    data,
    route: () => history.route,
    push: history.push,
    token,
    session,
    pauseCodeEvents,
    fetcher,
  };
  const sources = new Map<string, StoreDataSource>();
  let feed = new AbortController(),
    discovery = new AbortController();
  const projects = createProjectsHub({
    repositories: () => context.data.repositories,
    discover: async () => {
      discovery.abort();
      discovery = new AbortController();
      const ticket = context.session(),
        credential = context.token();
      const repositories = await loadRepositories(
        credential,
        context.fetcher,
        discovery.signal,
      );
      if (ticket !== context.session() || credential !== context.token())
        return;
      context.data.repositories = repositories.map((r) => ({ ...r }));
      for (const [id, source] of sources) {
        if (!repositories.some((r) => r.id === id)) {
          source.close();
          sources.delete(id);
        }
      }
    },
    route: context.route,
    push: context.push,
    changed: () => {
      context.data.status.refreshAvailable = true;
    },
    source: (home) => {
      if (!context.data.repositories.some((r) => r.id === home))
        throw new ApiError("access_denied");
      let source = sources.get(home);
      if (!source) {
        const candidate = context.data.repositories.find((r) => r.id === home)!;
        source = repositorySource(
          candidate,
          context.token(),
          `projects-${context.session()}`,
          () => {},
          context.fetcher,
        );
        sources.set(home, source);
      }
      if (!source.work) throw new ApiError("work_unavailable");
      const active = source;
      if (active.profile === "git_sha1_jj045")
        void active
          .snapshot()
          .then((head) => {
            if (sources.get(home) === active)
              captureRepositoryActor(context.data, home, head);
          })
          .catch(() => {});
      return source.work;
    },
    activate: (home) => {
      context.pauseCodeEvents();
      feed.abort();
      feed = new AbortController();
      const active = home ? sources.get(home) : undefined;
      if (active)
        void followProductionStoreEvents(
          active,
          () =>
            context.route().page === "projects" &&
            context.route().repository === active.repository,
          context.data,
          feed.signal,
        );
    },
  });
  return {
    projects,
    stopEvents() {
      feed.abort();
    },
    clear() {
      feed.abort();
      discovery.abort();
      projects.reset();
      for (const source of sources.values()) source.close();
      sources.clear();
    },
  };
}
