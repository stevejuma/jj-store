import type { Model } from "../api/client.js";
import type { StoreRepositorySummary, StoreRoute } from "./contracts.js";
import type { ProjectsHubController } from "./projects-hub-types.js";
export async function loadProjectCollections(context: {
  route: StoreRoute;
  project: Model<"ProjectV1"> | null;
  repositories: readonly StoreRepositorySummary[];
  directory: ProjectsHubController["repositories"];
  reviews: ProjectsHubController["reviews"];
  refresh: boolean;
  affected?: ReadonlySet<string>;
  report(message: string): void;
}) {
  const { route, project, repositories, directory, reviews, refresh } = context;
  if (route.workTab !== "repositories" && route.workTab !== "reviews")
    return false;
  if (route.projectId && !project) {
    (route.workTab === "repositories" ? directory : reviews).reset();
    context.report("Project unavailable. Return to Projects.");
  } else if (route.workTab === "repositories")
    await directory.load(repositories, refresh, context.affected);
  else
    await reviews.load(
      route,
      repositories.filter(
        (repo) => !project || repo.id in project.repositories,
      ),
      refresh,
      context.affected,
    );
  return true;
}
