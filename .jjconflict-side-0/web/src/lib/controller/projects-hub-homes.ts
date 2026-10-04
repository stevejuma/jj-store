import { readProjectHome } from "./projects-home-read.js";
import type { StoreRoute, StoreRepositorySummary } from "./contracts.js";
import type { ProjectHome } from "./projects-hub-types.js";
export function projectHubHomes(
  route: StoreRoute,
  candidates: readonly StoreRepositorySummary[],
  previous: readonly ProjectHome[],
  cached?: ProjectHome[],
) {
  const repositories = candidates.filter((repository) =>
    route.projectId
      ? repository.id === route.repository
      : !route.workHome || repository.id === route.workHome,
  );
  const homes =
    cached ??
    repositories.map((repository) => ({
      ...(previous.find((home) => home.repository.id === repository.id) ?? {
        projects: [],
        tasks: [],
        snapshot: null,
        cursor: null,
        total: 0,
        loading: false,
        error: "",
      }),
      repository,
    }));
  return { repositories, homes };
}

export function createProjectHomeReader(
  transport: (
    home: string,
  ) => import("./projects-hub-types.js").WorkBrowseTransport,
  signal: () => AbortSignal,
  generation: () => number,
  denied: (id: string) => void,
) {
  return (
    home: ProjectHome,
    query: import("./projects-hub-types.js").WorkBrowseQuery,
    ticket: number,
    more = false,
  ) =>
    readProjectHome(
      {
        transport,
        signal: signal(),
        current: () => ticket === generation(),
        denied,
      },
      home,
      query,
      more,
    );
}
