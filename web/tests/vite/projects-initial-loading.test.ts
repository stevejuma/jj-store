import { expect, test } from "vitest";
import { projectsInitialLoading } from "../../src/lib/ui/store-work/projects-initial-loading.js";
import type { StoreController } from "../../src/lib/controller/contracts.js";

test("captured project repository views survive summary invalidation while initial and task views still wait for their owners", () => {
  const projectId = "11".repeat(16);
  const hub = {
    loading: true,
    error: "",
    route: {
      projectId,
      workTab: "repositories",
      taskId: null as string | null,
    },
    detail: {
      state: {
        loading: true,
        error: "",
        project: null as { id: string } | null,
        task: null,
      },
    },
    repositories: { entries: [] },
  };
  const controller = {
    projects: hub,
    state: { status: { connection: "connected", repositories: "ready" } },
  } as unknown as StoreController;
  expect(projectsInitialLoading(controller)).toBe(true);
  hub.detail.state.project = { id: projectId };
  expect(projectsInitialLoading(controller)).toBe(false);
  hub.route.taskId = "22".repeat(16);
  expect(projectsInitialLoading(controller)).toBe(true);
  hub.route.taskId = null;
  hub.route.projectId = "";
  expect(projectsInitialLoading(controller)).toBe(true);
});
