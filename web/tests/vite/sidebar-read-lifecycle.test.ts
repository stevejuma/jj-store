import { expect, test, vi } from "vitest";
import { createProjectsNavigation } from "../../src/lib/controller/projects-navigation.svelte.js";
import { createRepositoryThreadNavigation } from "../../src/lib/controller/repository-thread-navigation.svelte.js";

test("removed repository dirtiness cannot keep project navigation refreshing", async () => {
  let repositories = [
    { id: "home", name: "Home", permission: "read" as const },
  ];
  const browse = vi.fn();
  const nav = createProjectsNavigation({
    repositories: () => repositories,
    transport: () => ({ browse }) as never,
    denied: () => {},
  });
  nav.invalidate();
  repositories = [];
  await nav.setVisible(true);
  expect(nav.dirty).toBe(false);
  expect(browse).not.toHaveBeenCalled();
  nav.reset();
});
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";

test("closed project navigation retains event dirtiness and shares its initial read", async () => {
  const browse = vi.fn(async () => ({
    projects: [],
    tasks: [],
    total_matches: 0,
    next_cursor: null,
    snapshot: {},
  }));
  const nav = createProjectsNavigation({
    repositories: () => [{ id: "home", name: "Home", permission: "admin" }],
    transport: () => ({ browse }) as never,
    denied: () => {},
  });
  nav.invalidate();
  expect(browse).not.toHaveBeenCalled();
  await Promise.all([nav.setVisible(true), nav.setVisible(true)]);
  expect(browse).toHaveBeenCalledTimes(1);
  await nav.setVisible(false);
  nav.invalidate(new Set(["home"]));
  expect(browse).toHaveBeenCalledTimes(1);
  await nav.setVisible(true);
  expect(browse).toHaveBeenCalledTimes(2);
  await nav.setVisible(false);
  await nav.setVisible(true);
  expect(browse).toHaveBeenCalledTimes(2);
  nav.reset();
});

test("repository thread children only read visible owners and retain hidden invalidations", async () => {
  const fixture = createThreadTransport(),
    page = await fixture.transport.list();
  const other = "99".repeat(16);
  const list = vi.fn(async (repo: string) => ({
    ...page,
    snapshot: { ...page.snapshot, repo_id: repo },
  }));
  const nav = createRepositoryThreadNavigation({
    repositories: () =>
      [THREAD_IDS.repo, other].map((id) => ({
        id,
        name: id,
        permission: "read",
      })),
    credential: () => null,
    source: (repo) => ({ list: () => list(repo), close: () => {} }),
  });
  nav.invalidate();
  expect(list).not.toHaveBeenCalled();
  await nav.setVisible(new Set([THREAD_IDS.repo]));
  expect(list.mock.calls.map((c) => c[0])).toEqual([THREAD_IDS.repo]);
  nav.invalidate(new Set([other]));
  await nav.setVisible(new Set([THREAD_IDS.repo]));
  expect(list).toHaveBeenCalledTimes(1);
  await nav.setVisible(new Set([other]));
  expect(list).toHaveBeenCalledTimes(2);
  await nav.setVisible(new Set());
  nav.invalidate(new Set([THREAD_IDS.repo]));
  expect(list).toHaveBeenCalledTimes(2);
  await nav.setVisible(new Set([THREAD_IDS.repo]));
  expect(list).toHaveBeenCalledTimes(3);
  nav.clear();
});
