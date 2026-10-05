import type { StoreRoute } from "../../controller/contracts.js";
import { isRepositoryProjects } from "../../controller/projects-view-cache.js";
import { projectSidebarId } from "../store-work/projects-sidebar-model.js";
import type { StoreController } from "../../controller/contracts.js";
import type {
  CommunityNavigationItemModel,
  StorePage,
} from "../community-navigation/model.js";
export function selectStoreNavigation(
  controller: StoreController,
  item: CommunityNavigationItemModel,
) {
  if (item.kind === "threads")
    void controller.actions.openThreads?.(null, item.repository);
  else if (item.kind === "channel")
    void controller.actions.openThreads?.(item.thread, item.repository);
  else if (item.kind === "more-threads" && item.repository)
    void controller.threadNavigation?.more(item.repository);
  else if (item.kind === "retry-threads" && item.repository)
    void controller.threadNavigation?.read(item.repository);
  else if (item.kind === "repository")
    controller.actions.openRepository(item.id);
  else if (
    item.kind === "project" &&
    controller.projects &&
    item.home &&
    item.project
  )
    void controller.projects.openProject(item.home, item.project);
  else if (item.kind === "more-projects" && controller.projects && item.home)
    void controller.projects.navigation.more(item.home);
  else if (item.kind === "retry-projects" && controller.projects)
    void controller.projects.refresh();
  else if (item.kind === "projects" && controller.projects)
    void controller.projects.openHub();
  else if (
    [
      "stacks",
      "branches",
      "log",
      "oplog",
      "files",
      "agents",
      "reviews",
      "projects",
    ].includes(item.kind)
  )
    controller.actions.selectPage(item.kind as StorePage);
}

export function activeStoreNavigation(route: StoreRoute) {
  if (route.page === "threads")
    return route.threadId
      ? `thread:${route.repository}:${route.threadId}`
      : route.repository;
  if (route.page === "projects" && !isRepositoryProjects(route))
    return route.projectId
      ? projectSidebarId(route.repository, route.projectId)
      : "projects";
  return route.repository;
}
