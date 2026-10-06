import { ApiError } from "../api/client.js";
import { repositories as loadRepositories } from "../api/repositories.js";
import { storeErrorMessage } from "./store-error.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { ProjectsHubController } from "./projects-hub-types.js";
import type { StoreRoute } from "./contracts.js";
export function createProductionConnect(context: {
  data: ProductionStoreState;
  route: () => StoreRoute;
  projects: ProjectsHubController;
  connectRepository: (
    repository: string,
    token: string | null,
  ) => Promise<void>;
  disconnect: (message?: string, denied?: boolean) => void;
  version: () => number;
  discovery: (abort: AbortController) => void;
  token: (token: string) => void;
  fetcher?: typeof fetch;
  events?(token: string): Promise<void>;
}) {
  return async (repository: string, token: string | null) => {
    const { data, projects } = context;
    if (data.accessMode === "local_trusted") void projects.navigation.load();
    if (context.route().agentsHub) {
      data.status.connection = "disconnected";
      return;
    }
    if (
      data.accessMode === "local_trusted" &&
      ((context.route().page === "projects" && context.route().projectsHub) ||
        !repository)
    ) {
      if (context.route().page === "projects") await projects.restore();
      else await projects.openHub();
      data.status.connection = "disconnected";
      return;
    }
    if (data.accessMode !== "authenticated" || token === null)
      return context.connectRepository(repository, token);
    context.disconnect();
    const ticket = context.version(),
      discovery = new AbortController();
    context.discovery(discovery);
    data.status.connection = "connecting";
    data.status.repositories = "loading";
    try {
      await context.events?.(token);
      if (ticket !== context.version()) return;
      const repositories = await loadRepositories(
        token,
        context.fetcher,
        discovery.signal,
      );
      if (ticket !== context.version()) return;
      data.repositories = repositories.map((candidate) => ({ ...candidate }));
      data.status.repositories = data.repositories.length ? "ready" : "empty";
      context.token(token);
      void projects.navigation.load();
      const requested = data.repositories.find(
        (candidate) =>
          candidate.id === (repository.trim() || context.route().repository),
      );
      const selected =
        requested ??
        (data.repositories.length === 1 ? data.repositories[0] : undefined);
      if (
        (context.route().page === "projects" && context.route().projectsHub) ||
        !selected
      ) {
        if (context.route().page === "projects") await projects.restore();
        else await projects.openHub();
        data.status.connection = "disconnected";
      } else await context.connectRepository(selected.id, token);
    } catch (cause) {
      context.disconnect(storeErrorMessage(cause), cause instanceof ApiError);
      data.status.connection = "failed";
    }
  };
}
