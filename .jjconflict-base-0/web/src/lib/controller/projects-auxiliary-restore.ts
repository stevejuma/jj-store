import { loadProjectAgents } from "./projects-agent-scope.js";
import { loadProjectCollections } from "./projects-related-views.js";
import { projectsActivityScope } from "./projects-activity-scope.js";
import type { ProjectsHubController } from "./projects-hub-types.js";
import type { StoreRepositorySummary, StoreRoute } from "./contracts.js";
import type { WorkController } from "./work-types.js";
export async function restoreProjectAuxiliary(options: {
  route: StoreRoute;
  repositories: readonly StoreRepositorySummary[];
  detail: WorkController["state"] | null;
  activity: ProjectsHubController["activity"];
  agents: ProjectsHubController["agents"];
  navigation: ProjectsHubController["navigation"];
  directory: ProjectsHubController["repositories"];
  reviews: ProjectsHubController["reviews"];
  refresh: boolean;
  affected?: ReadonlySet<string>;
  current(): boolean;
  report(message: string): void;
}) {
  const { route, detail, refresh, affected } = options;
  if (route.workTab === "activity" || route.taskId) {
    try {
      await options.activity.load(
        projectsActivityScope(
          options.repositories,
          route,
          detail?.project,
          detail?.task,
        ),
        refresh,
        route.snapshotOperationId
          ? { [route.repository]: route.snapshotOperationId }
          : {},
        affected,
      );
    } catch (cause) {
      options.activity.reset();
      options.report(
        cause instanceof Error ? cause.message : "Activity unavailable",
      );
      return true;
    }
    if (!options.current() || route.workTab === "activity") return true;
  }
  if (route.workTab === "agents") {
    await loadProjectAgents({
      ...options,
      project: detail?.project ?? null,
      error: detail?.error ?? "",
    });
    if (options.current())
      await options.agents.restoreSelection(
        refresh &&
          (!affected ||
            affected.has(route.workAgentRepository ?? route.repository)),
      );
    return true;
  }
  options.agents.clearSelection();
  return loadProjectCollections({
    ...options,
    project: detail?.project ?? null,
  });
}
