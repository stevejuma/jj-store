import type {
  ProjectsHubContext,
  ProjectsHubController,
} from "./projects-hub-types.js";
import { initialStoreRoute } from "./store-route-history.svelte.js";
import { validWorkSearch } from "./projects-view-cache.js";
export function projectsRouteActions(
  context: ProjectsHubContext,
  restore: () => Promise<void>,
  reportError: (message: string) => void,
): Pick<ProjectsHubController, "openHub" | "openProject" | "filter"> {
  return {
    async openHub(tab = "projects") {
      context.push({
        ...initialStoreRoute(),
        page: "projects",
        projectsHub: true,
        workTab: tab,
      });
      await restore();
    },
    async openProject(home, project, task = null) {
      context.push({
        ...(context.route().repository === home &&
        context.route().projectId === project
          ? context.route()
          : initialStoreRoute()),
        page: "projects",
        repository: home,
        snapshotOperationId:
          context.route().repository === home
            ? context.route().snapshotOperationId
            : null,
        projectId: project,
        taskId: task,
        projectsHub: true,
        workTab: "tasks",
      });
      await restore();
    },
    async filter(fields) {
      if (!validWorkSearch(fields.workQuery)) {
        reportError("Search is limited to 512 UTF-8 bytes.");
        return;
      }
      context.push({
        ...context.route(),
        ...fields,
        ...(fields.workTab && fields.workTab !== "agents"
          ? { workAgentRepository: null, agentSessionId: null }
          : {}),
        ...(fields.workTab ||
        fields.workState ||
        fields.workQuery !== undefined ||
        fields.workRepository !== undefined
          ? { taskId: null }
          : {}),
      });
      await restore();
    },
  };
}
