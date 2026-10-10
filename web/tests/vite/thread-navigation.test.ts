import { createRepositoryThreadNavigation } from "../../src/lib/controller/repository-thread-navigation.svelte.js";
import { expect, test, vi } from "vitest";
import { createInMemoryStoreController } from "../../../catalog/controller/in-memory-store-controller.svelte.js";
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";
import { createThreadsController } from "../../src/lib/controller/production-threads.svelte.js";
import { projectsSidebarModel } from "../../src/lib/ui/store-work/projects-sidebar-model.js";
import { withThreadNavigation } from "../../src/lib/ui/store-threads/thread-navigation.js";
import {
  activeStoreNavigation,
  selectStoreNavigation,
} from "../../src/lib/ui/store-application/store-navigation.js";
import type { StoreController } from "../../src/lib/controller/contracts.js";

async function fixture() {
  const base = createInMemoryStoreController();
  const repo = {
    id: THREAD_IDS.repo,
    name: "Home",
    permission: "admin" as const,
  };
  const route = {
    ...base.state.route,
    repository: repo.id,
    page: "threads" as const,
    threadId: null,
  };
  const transport = createThreadTransport();
  const threads = createThreadsController({
    source: () => transport.transport,
    repository: () => repo.id,
    principal: () => THREAD_IDS.actor,
    route: () => route,
    select: () => {},
  });
  await threads.list();
  const openThreads = vi.fn(async () => {});
  const threadNavigation = createRepositoryThreadNavigation({
    repositories: () => [repo],
    credential: () => null,
    source: () => ({ list: transport.transport.list, close: () => {} }),
  });
  await threadNavigation.load();
  const controller: StoreController = {
    threadNavigation,
    ...base,
    threads,
    state: { ...base.state, route, repository: repo, repositories: [repo] },
    actions: { ...base.actions, openThreads },
  };
  return { controller, repo, openThreads };
}

test("repository aliases retain their canonical owner and omit Browse controls", async () => {
  const { controller, repo } = await fixture();
  const model = withThreadNavigation(
    controller,
    projectsSidebarModel([], [repo], false),
  );
  expect(model.sections.map((s) => s.id)).toEqual(["repositories"]);
  const section = model.sections[0]!;
  expect(section.action).toBeUndefined();
  expect(section.items[0]!.children?.map((row) => row.label)).toEqual([
    "general",
  ]);
  expect(section.items[0]!.children).toContainEqual(
    expect.objectContaining({
      id: `thread:${repo.id}:${THREAD_IDS.thread}`,
      kind: "channel",
      repository: repo.id,
      thread: THREAD_IDS.thread,
    }),
  );
  expect(activeStoreNavigation(controller.state.route)).toBe(repo.id);
});

test("thread selection forwards its repository instead of reusing the active connection", async () => {
  const { controller, openThreads } = await fixture();
  const other = "99".repeat(16);
  selectStoreNavigation(controller, {
    id: `thread:${other}:${THREAD_IDS.thread}`,
    label: "Other thread",
    kind: "channel",
    repository: other,
    thread: THREAD_IDS.thread,
  });
  expect(openThreads).toHaveBeenCalledWith(THREAD_IDS.thread, other);
});
