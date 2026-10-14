import { expect, test, vi } from "vitest";
import { createProjectsHub } from "../../src/lib/controller/production-projects-hub.svelte.js";
import { ProjectsDirtiness } from "../../src/lib/controller/projects-dirtiness.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import type { StoreRoute } from "../../src/lib/controller/contracts.js";
import type { WorkBrowseTransport } from "../../src/lib/controller/projects-hub-types.js";
const a = "11".repeat(16),
  b = "22".repeat(16);
const change = (
  repository = a,
  domain: "agents" | "work" | "threads" = "agents",
  witness = "one",
) => ({ repository, domain, witness });
test("task reactions retain home-scoped dirtiness without dirtying code or other project domains", () => {
  const dirty = new ProjectsDirtiness();
  const route = {
    ...initialStoreRoute(),
    page: "projects" as const,
    workTab: "tasks" as const,
    repository: a,
    projectId: "33".repeat(16),
    taskId: "44".repeat(16),
  };
  dirty.record([change(a, "threads")], [a, b]);
  expect([...dirty.capture(route).work]).toEqual([a]);
  expect(dirty.capture({ ...route, repository: b }).work.size).toBe(0);
  expect(
    dirty.capture({ ...route, workTab: "activity", projectId: null }).affected
      .size,
  ).toBe(0);
  const pending = dirty.capture(route);
  pending.accept(() => false);
  expect(dirty.capture(route).work.has(a)).toBe(true);
  pending.accept(() => true);
  expect(dirty.capture(route).work.size).toBe(0);
});
test("hidden Agents projections refresh only their dirty repository on return", async () => {
  let route: StoreRoute = {
    ...initialStoreRoute(),
    page: "projects",
    workTab: "agents",
  };
  const reads = new Map(
    [a, b].map((id) => [
      id,
      vi.fn(async () => ({
        version: 1,
        snapshot: { repo_id: id },
        sessions: [],
        next_cursor: null,
      })),
    ]),
  );
  const hub = createProjectsHub({
    route: () => route,
    push: (next) => {
      route = next;
    },
    changed: () => {},
    repositories: () =>
      [a, b].map((id) => ({ id, name: id, permission: "read" as const })),
    source: (id) =>
      ({
        agents: reads.get(id),
        browse: async () => ({
          version: 1,
          snapshot: { repo_id: id },
          projects: [
            { id: "33".repeat(16), repositories: { [a]: {}, [b]: {} } },
          ],
          tasks: [],
          next_cursor: null,
          total_matches: 0,
        }),
      }) as unknown as WorkBrowseTransport,
  });
  await hub.restore();
  route = { ...route, workTab: "projects" };
  await hub.restore();
  hub.invalidate?.([change()]);
  route = { ...route, workTab: "agents" };
  await hub.restore();
  expect(reads.get(a)).toHaveBeenCalledTimes(2);
  expect(reads.get(b)).toHaveBeenCalledTimes(1);
  await hub.restore();
  expect(reads.get(a)).toHaveBeenCalledTimes(2);
  hub.reset();
});
test("failed and superseded domain reads retain dirtiness independently", () => {
  const dirty = new ProjectsDirtiness();
  const route = {
    ...initialStoreRoute(),
    page: "projects" as const,
    workTab: "agents" as const,
  };
  dirty.record([change()], [a, b]);
  const first = dirty.capture(route);
  first.accept(() => false);
  expect(dirty.capture(route).affected.has(a)).toBe(true);
  dirty.record([change(a, "agents", "two")], [a, b]);
  first.accept(() => true);
  expect(dirty.capture(route).affected.has(a)).toBe(true);
  dirty.capture(route).accept(() => true);
  expect(dirty.capture(route).affected.size).toBe(0);
  expect(dirty.capture({ ...route, workTab: "activity" }).affected.has(a)).toBe(
    true,
  );
});
