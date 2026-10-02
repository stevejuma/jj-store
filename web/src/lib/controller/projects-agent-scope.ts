import type { Model } from "../api/client.js";
import type { StoreRepositorySummary, StoreRoute } from "./contracts.js";
import type { ProjectsHubController } from "./projects-hub-types.js";
/** Drain bounded project pages, preserving the sidebar's independently pinned roots. */
export async function projectsAgentScope(
  route: StoreRoute,
  repositories: readonly StoreRepositorySummary[],
  navigation: ProjectsHubController["navigation"],
  project: Model<"ProjectV1"> | null,
  current: () => boolean,
) {
  if (route.projectId && !project)
    throw new Error("Project unavailable. Return to Projects.");
  if (!project) {
    await navigation.load();
    let index = 0;
    const homes = navigation.homes.filter(
      (home) => !route.workHome || home.repository.id === route.workHome,
    );
    await Promise.all(
      Array.from({ length: Math.min(2, homes.length) }, async () => {
        while (index < homes.length && current()) {
          const home = homes[index++]!,
            seen = new Set<string>();
          for (
            let page = 0;
            page < 80 && home.cursor && !home.error && current();
            page++
          ) {
            if (seen.has(home.cursor))
              throw new Error("Repeated project cursor.");
            seen.add(home.cursor);
            await navigation.more(home.repository.id);
          }
        }
      }),
    );
  }
  const projects = project
    ? [project]
    : navigation.homes
        .filter(
          (home) => !route.workHome || home.repository.id === route.workHome,
        )
        .flatMap((home) => home.projects);
  const ids = new Set(
    projects.flatMap((project) => Object.keys(project.repositories)),
  );
  return repositories.filter(
    (repo) =>
      ids.has(repo.id) &&
      (!route.workRepository || route.workRepository === repo.id),
  );
}

export async function loadProjectAgents(context: {
  route: StoreRoute;
  repositories: readonly StoreRepositorySummary[];
  navigation: ProjectsHubController["navigation"];
  agents: ProjectsHubController["agents"];
  project: Model<"ProjectV1"> | null;
  error: string;
  refresh: boolean;
  current(): boolean;
  report(message: string): void;
}) {
  try {
    if (context.route.projectId && context.error)
      throw new Error("Project unavailable. Return to Projects.");
    const scope = await projectsAgentScope(
      context.route,
      context.repositories,
      context.navigation,
      context.project,
      context.current,
    );
    if (context.current()) await context.agents.load(scope, context.refresh);
  } catch (cause) {
    if (context.current()) {
      context.agents.reset();
      context.report(
        cause instanceof Error ? cause.message : "Project agents unavailable",
      );
    }
  }
}
