import type { Model } from "../api/client.js";
import type { StoreRepositorySummary, StoreRoute } from "./contracts.js";
export function projectsActivityScope(
  repositories: readonly StoreRepositorySummary[],
  route: StoreRoute,
  project?: Model<"ProjectV1"> | null,
  task?: Model<"TaskV1"> | null,
) {
  if (route.projectId && !project)
    throw new Error("Project unavailable. Refresh or return to Projects.");
  return repositories.filter(
    (repo) =>
      (!route.workRepository || repo.id === route.workRepository) &&
      (!route.workHome || !!project || repo.id === route.workHome) &&
      (task?.repository
        ? repo.id === task.repository
        : !project || repo.id in project.repositories),
  );
}
