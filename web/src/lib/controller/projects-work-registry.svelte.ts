import { SvelteMap } from "svelte/reactivity";
import { ApiError } from "../api/client.js";
import { createWorkController } from "./production-work.svelte.js";
import type {
  ProjectsHubContext,
  WorkBrowseTransport,
} from "./projects-hub-types.js";
import type { WorkController } from "./work-types.js";
export function createProjectsWorkRegistry(context: ProjectsHubContext) {
  const sources = new Map<string, WorkBrowseTransport>();
  const works = new SvelteMap<string, WorkController>();
  const selections = new Map<
    string,
    { projectId: string | null; taskId: string | null }
  >();
  function transport(home: string) {
    if (!context.repositories().some((r) => r.id === home))
      throw new ApiError("access_denied");
    let source = sources.get(home);
    if (!source) {
      source = context.source(home);
      sources.set(home, source);
    }
    return source;
  }
  function work(home: string) {
    let controller = works.get(home);
    if (!controller) {
      controller = createWorkController({
        source: () => transport(home),
        route: () => selections.get(home) ?? {},
        select: (projectId, taskId) => {
          selections.set(home, { projectId, taskId });
          context.push({
            ...context.route(),
            page: "projects",
            repository: home,
            projectId,
            taskId,
            projectsHub: true,
          });
        },
        changed: context.changed,
      });
      works.set(home, controller);
    }
    return controller;
  }
  return { sources, works, selections, transport, work };
}
