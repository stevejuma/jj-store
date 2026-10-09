import { expect, test, vi } from "vitest";
import { createThreadsIntegration } from "../../src/lib/controller/production-threads-integration.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import {
  createStoreRouteHistory,
  initialStoreRoute,
} from "../../src/lib/controller/store-route-history.svelte.js";
import {
  createThreadTransport,
  THREAD_IDS as ids,
} from "../../../catalog/fixtures/thread-transport.js";
import type { ThreadTransport } from "../../src/lib/controller/thread-types.js";
const other = "99".repeat(16),
  local = "88".repeat(16);
function fixture() {
  const data = createProductionStoreState(),
    history = createStoreRouteHistory();
  data.repositories = [
    { id: ids.repo, name: "Home", permission: "admin", principal: ids.actor },
    { id: other, name: "Other", permission: "read", principal: ids.actor },
    {
      id: local,
      name: "Local",
      permission: "admin",
      profile: "git_sha1_jj045",
    },
  ];
  const transport = createThreadTransport();
  let source: { repository: string; threads?: ThreadTransport } | null = {
    repository: ids.repo,
    threads: transport.transport,
  };
  const connectRepository = vi.fn(async (id: string) => {
    source = { repository: id, threads: transport.transport };
    history.connect(id, null);
  });
  const integration = createThreadsIntegration({
    data,
    history,
    source: () => source,
    connectRepository,
  });
  return {
    data,
    history,
    integration,
    connectRepository,
    setSource: (value: typeof source) => {
      source = value;
    },
  };
}
test("selecting a repository-owned child binds its owner before opening its exact thread", async () => {
  const f = fixture();
  await f.integration.openThreads(ids.thread, other);
  expect(f.connectRepository).toHaveBeenCalledExactlyOnceWith(other, null);
  expect(f.history.route).toMatchObject({
    repository: other,
    page: "threads",
    threadId: ids.thread,
  });
  expect(f.history.route.replyId ?? null).toBeNull();
  expect(f.integration.threads.detail?.thread.value.thread_id).toBe(ids.thread);
});
test("Browse threads from a local folder chooses an authorized native repository", async () => {
  const f = fixture();
  f.data.repositories[0] = { ...f.data.repositories[0]!, permission: null };
  f.setSource({ repository: local });
  await f.integration.openThreads();
  expect(f.connectRepository).toHaveBeenCalledExactlyOnceWith(other, null);
  expect(f.history.route).toMatchObject({
    repository: other,
    page: "threads",
    threadId: null,
  });
});
test("unsupported and unauthorized owners never connect or open a thread", async () => {
  const f = fixture();
  f.data.repositories[1] = { ...f.data.repositories[1]!, permission: null };
  f.data.repositories[2] = {
    ...f.data.repositories[2]!,
    availability: "unavailable",
  };
  for (const id of [local, other, "missing"])
    await expect(f.integration.openThreads(ids.thread, id)).rejects.toThrow();
  expect(f.connectRepository).not.toHaveBeenCalled();
  expect(f.integration.threads.detail).toBeUndefined();
});
test("a late repository connection cannot open a child after navigation changes", async () => {
  const f = fixture();
  let finish!: () => void;
  f.connectRepository.mockImplementation(
    () =>
      new Promise((resolve) => {
        finish = resolve;
      }),
  );
  const open = vi.spyOn(f.integration.threads, "open");
  const selecting = f.integration.openThreads(ids.thread, other);
  f.history.push({
    ...initialStoreRoute(),
    page: "projects",
    projectsHub: true,
  });
  finish();
  await selecting;
  expect(open).not.toHaveBeenCalled();
  expect(f.history.route.page).toBe("projects");
});

test("local thread navigation binds its own authorized repository without falling back to native", async () => {
  const f = fixture();
  await f.integration.openThreads(ids.thread, local);
  expect(f.connectRepository).toHaveBeenCalledExactlyOnceWith(local, null);
  expect(f.history.route).toMatchObject({
    repository: local,
    page: "threads",
    threadId: ids.thread,
  });
  expect(f.integration.threads.detail?.thread.value.thread_id).toBe(ids.thread);
});
