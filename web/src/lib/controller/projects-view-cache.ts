import type { StoreRoute } from "./contracts.js";
import type { ProjectHome, WorkBrowseQuery } from "./projects-hub-types.js";
export const projectsViewKey = (r: StoreRoute) =>
  JSON.stringify([
    r.projectId ? r.repository : "",
    r.projectId ?? null,
    r.taskId ?? null,
    r.workTab ?? (r.projectId ? "tasks" : "projects"),
    r.workQuery ?? "",
    r.workState ?? "active",
    r.workRepository ?? "",
    r.workHome ?? "",
  ]);

export function createProjectsViewCache() {
  const views = new Map<string, ProjectHome[]>();
  const copy = (homes: readonly ProjectHome[]) =>
    homes.map((home) => ({ ...home }));
  return {
    get(route: StoreRoute) {
      const homes = views.get(projectsViewKey(route));
      return homes ? copy(homes) : undefined;
    },
    save(route: StoreRoute, homes: readonly ProjectHome[]) {
      if (homes.some((home) => home.error || home.loading)) return;
      views.set(projectsViewKey(route), copy(homes));
      if (views.size > 16) views.delete(views.keys().next().value!);
    },
    clear() {
      views.clear();
    },
  };
}

export function projectsBrowseQuery(route: StoreRoute): WorkBrowseQuery {
  const kind =
    route.projectId || route.workTab === "tasks" ? "tasks" : "projects";
  return {
    kind,
    ...(route.projectId ? { project: route.projectId } : {}),
    ...(route.workRepository ? { repository: route.workRepository } : {}),
    ...(route.workQuery ? { q: route.workQuery } : {}),
    ...(kind === "tasks" && route.workState !== "all"
      ? { state: route.workState ?? "active" }
      : {}),
  };
}

export const validWorkSearch = (text?: string) =>
  !text || new TextEncoder().encode(text).length <= 512;
