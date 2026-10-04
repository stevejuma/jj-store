import type { StoreController } from "../../controller/contracts.js";
import type { GlobalSearchScope } from "../../controller/global-search-types.js";
export function currentSearchScope(
  controller: StoreController,
): GlobalSearchScope {
  const route = controller.state.route;
  if (route.page === "threads")
    return {
      label: controller.threads?.detail
        ? `#${controller.threads.detail.thread.value.title}`
        : "Threads",
      repository: route.repository,
      section: "threads",
      thread: route.threadId ?? undefined,
    };
  if (route.page === "projects")
    return {
      label: route.taskId
        ? "this task"
        : route.projectId
          ? "this project"
          : "Projects",
      section: "projects",
      repository: route.projectId ? route.repository : undefined,
      project: route.projectId ?? undefined,
      task: route.taskId ?? undefined,
    };
  return {
    label: controller.state.repository.name || "this repository",
    repository: route.repository,
    section: "all",
  };
}
