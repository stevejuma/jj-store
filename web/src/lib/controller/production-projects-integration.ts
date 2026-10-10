import { repositories as loadRepositories } from "../api/repositories.js";
import { followProductionStoreEvents } from "./production-store-events.js";
import { ApiError } from "../api/client.js";
import {
  repositorySource,
  captureRepositoryActor,
} from "./repository-source.js";
import type { StoreDataSource } from "./native-data-source.js";
import { createProjectsHub } from "./production-projects-hub.svelte.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreController } from "./contracts.js";
import { createProjectAgentResponse } from "./project-agent-response.svelte.js";
export function createProductionProjects(
  data: ProductionStoreState,
  history: ReturnType<
    typeof import("./store-route-history.svelte.js").createStoreRouteHistory
  >,
  token: () => string | null,
  session: () => number,
  pauseCodeEvents: () => void,
  fetcher?: typeof fetch,
  changed: () => void = () => {},
  base?: () => StoreController,
  shared?: import("./repository-sources.js").RepositorySources,
  freshness?: import("./application-freshness.js").ApplicationFreshness,
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
  let selectedHome: string | null = null;
  let feed = new AbortController(),
    discovery = new AbortController();
  async function discover() {
    discovery.abort();
    discovery = new AbortController();
    const ticket = context.session(),
      credential = context.token();
    const repositories = await loadRepositories(
      credential,
      context.fetcher,
      discovery.signal,
    );
    if (ticket !== context.session() || credential !== context.token()) return;
    context.data.repositories = repositories.map((r) => ({ ...r }));
    for (const [id, source] of sources) {
      if (!repositories.some((r) => r.id === id)) {
        source.close();
        sources.delete(id);
      }
    }
  }
  const projects = createProjectsHub({
    agentResponse: (home) => {
      projects.transport(home);
      if (!base) throw new Error("Agent response unavailable");
      return createProjectAgentResponse(() => {
        projects.transport(home);
        return sources.get(home)!;
      }, base);
    },
    repositories: () => context.data.repositories,
    discover,
    route: context.route,
    push: context.push,
    changed: () => {
      context.data.status.refreshAvailable = true;
      changed();
    },
    source: (home) => {
      if (!context.data.repositories.some((r) => r.id === home))
        throw new ApiError("access_denied");
      let source = sources.get(home);
      if (!source) {
        const candidate = context.data.repositories.find((r) => r.id === home)!;
        source = shared
          ? shared.acquire(candidate, context.token())
          : repositorySource(
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
      if (freshness?.supported) return;
      context.pauseCodeEvents();
      if (selectedHome === home && !feed.signal.aborted) return;
      selectedHome = home;
      feed.abort();
      feed = new AbortController();
      const active = home ? sources.get(home) : undefined;
      if (active)
        void followProductionStoreEvents(
          active,
          () =>
            context.route().page === "projects" &&
            context.route().repository === active.repository &&
            sources.get(active.repository) === active,
          context.data,
          feed.signal,
          changed,
        );
    },
  });
  return {
    projects,
    discover,
    stopEvents() {
      selectedHome = null;
      feed.abort();
    },
    clear() {
      feed.abort();
      discovery.abort();
      projects.reset();
      for (const source of sources.values()) source.close();
      sources.clear();
      selectedHome = null;
    },
  };
}
