import { expect, test } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { createProjectsNavigation } from "../../src/lib/controller/projects-navigation.svelte.js";
import { createProjectsReviews } from "../../src/lib/controller/projects-reviews.svelte.js";
import { projectsSidebarModel } from "../../src/lib/ui/store-work/projects-sidebar-model.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import {
  fixtureTransport,
  homeA,
  homeB,
} from "../../../catalog/fixtures/projects-hub-fixture.js";
const repositories = [
  { id: homeA, name: "Home", permission: "write" as const },
  { id: homeB, name: "Code", permission: "read" as const },
];
test("sidebar uses independent unfiltered snapshots and active repository children", async () => {
  const source = fixtureTransport(homeA),
    queries: unknown[] = [],
    original = source.browse;
  source.browse = async (query) => {
    queries.push(query);
    return original(query);
  };
  const navigation = createProjectsNavigation({
    repositories: () => repositories,
    transport: () => source,
    denied: () => {},
  });
  await navigation.load();
  expect(queries).toEqual([
    { kind: "projects", limit: 50 },
    { kind: "projects", limit: 50 },
  ]);
  const model = projectsSidebarModel(navigation.homes, repositories, true);
  expect(model.primaryItems[0]?.label).toBe("Projects");
  expect(model.sections[0]?.items[0]?.children?.map((item) => item.id)).toEqual(
    [homeA],
  );
  await navigation.load();
  expect(queries).toHaveLength(2);
  source.browse = async () => {
    throw new ApiError("access_denied");
  };
  await navigation.load(true);
  expect(
    navigation.homes.every((home) => !home.projects.length && !home.snapshot),
  ).toBe(true);
});
test("sidebar continuation rejects changed roots", async () => {
  const source = fixtureTransport(homeA),
    original = source.browse;
  source.browse = async (query) => ({
    ...(await original(query)),
    next_cursor: "next",
  });
  const navigation = createProjectsNavigation({
    repositories: () => repositories.slice(0, 1),
    transport: () => source,
    denied: () => {},
  });
  await navigation.load();
  source.browse = async (query) => {
    const page = await original(query);
    return { ...page, snapshot: { ...page.snapshot, manifest_revision: 2 } };
  };
  await navigation.more(homeA);
  expect(navigation.homes[0]?.error).toContain("snapshot mismatch");
  expect(navigation.homes[0]?.snapshot?.manifest_revision).toBe(1);
});
test("review continuations stay pinned and permission removal drops cached repositories", async () => {
  const source = fixtureTransport(homeA),
    original = source.reviews!;
  source.reviews = async () => ({ ...(await original()), next_cursor: "next" });
  const denied: string[] = [];
  const reviews = createProjectsReviews({
    source: () => source,
    denied: (home) => denied.push(home),
  });
  const route = { ...initialStoreRoute(), workTab: "reviews" as const };
  await reviews.load(route, repositories);
  source.reviews = async () => {
    const page = await original();
    return { ...page, snapshot: { ...page.snapshot, manifest_revision: 2 } };
  };
  await reviews.more(route, homeA);
  expect(reviews.homes[0]?.error).toContain("snapshot mismatch");
  expect(reviews.homes[0]?.threads).toHaveLength(1);
  await reviews.load(route, repositories.slice(1));
  expect(reviews.homes.map((home) => home.repository.id)).toEqual([homeB]);
  source.reviews = async () => {
    throw new ApiError("access_denied");
  };
  await reviews.load(route, repositories.slice(1), true);
  expect(reviews.homes[0]?.snapshot).toBeNull();
  expect(reviews.homes[0]?.threads).toEqual([]);
  expect(denied).toEqual([homeB]);
});
