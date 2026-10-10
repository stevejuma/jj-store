import { test, expect, vi } from "vitest";
import { ThreadWorkspaces } from "../../src/lib/controller/thread-workspaces.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../../src/lib/controller/store-route-codec.js";
import { threadsRoute } from "../../src/lib/controller/threads-route-codec.js";
import { threadInboxRows } from "../../src/lib/ui/store-threads/thread-inbox.js";
import { createInMemoryStoreController } from "../../../catalog/controller/in-memory-store-controller.svelte.js";
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";
import { createRepositoryThreadNavigation } from "../../src/lib/controller/repository-thread-navigation.svelte.js";
import { createThreadConversations } from "../../src/lib/controller/thread-conversations.svelte.js";
import { ApiError } from "../../src/lib/api/api-error.js";

test("Thread inbox routes retain canonical metadata home, repository filter and reply identity", () => {
  const route = {
    ...threadsRoute("aa".repeat(16), "bb".repeat(16), "cc".repeat(16)),
    replyId: "dd".repeat(16),
  };
  const hash = storeRouteToHash(route)!;
  expect(storeRouteFromHash(hash)).toEqual({ kind: "route", route });
  expect(storeRouteFromHash("#/threads?repository=foreign").kind).toBe(
    "invalid",
  );
  expect(storeRouteToHash(threadsRoute())).toBe("#/threads");
});
test("unknown Thread publication retains the exact request, busy refusal releases it for editing", async () => {
  let fail = true,
    busy = false;
  const calls: unknown[] = [];
  const outcome = {
    home: "aa".repeat(16),
    thread: "bb".repeat(16),
    revision: { kind: "thread_version", id: "cc".repeat(32) },
  };
  const fetcher = vi.fn(async (_url, init) => {
    calls.push(JSON.parse(String(init?.body)));
    if (busy)
      return Response.json({ code: "agent_execution_busy" }, { status: 409 });
    if (fail) throw new TypeError("Connection lost");
    return Response.json(outcome);
  });
  const published = vi.fn(async () => {});
  const manager = new ThreadWorkspaces(new LocalClient("", fetcher), published);
  await expect(
    manager.submit({ type: "create", title: "Planning", repositories: [] }),
  ).rejects.toThrow();
  const pending = manager.pending;
  fail = false;
  await manager.retry();
  expect(calls[1]).toEqual(calls[0]);
  expect(pending?.request_id).toBeTruthy();
  expect(manager.pending).toBeNull();
  expect(published).toHaveBeenCalledTimes(1);
  busy = true;
  await expect(
    manager.submit({ type: "create", title: "Planning", repositories: [] }),
  ).rejects.toThrow();
  expect(manager.pending).toBeNull();
  manager.destroy();
});
test("two authorized repository aliases and unbound Threads retain one owner; internal records stay hidden", async () => {
  const transport = createThreadTransport();
  const row = transport.threads.get(THREAD_IDS.thread)!;
  const other = "ee".repeat(16),
    empty = "ff".repeat(16),
    internal = "ab".repeat(16);
  transport.threads.set(THREAD_IDS.thread, {
    ...row,
    value: { ...row.value, codebase: [THREAD_IDS.repo, other] },
  });
  transport.threads.set(empty, {
    ...row,
    value: { ...row.value, thread_id: empty, title: "Unbound", codebase: [] },
  });
  transport.threads.set(internal, {
    ...row,
    value: {
      ...row.value,
      thread_id: internal,
      purpose: { type: "annotation" },
    },
  });
  const repositories = [
    { id: THREAD_IDS.repo, name: "First", permission: "admin" as const },
    { id: other, name: "Second", permission: "read" as const },
  ];
  const nav = createRepositoryThreadNavigation({
    repositories: () => repositories,
    credential: () => null,
    source: () => ({ list: transport.transport.list, close: () => {} }),
  });
  await nav.read(THREAD_IDS.repo);
  const base = createInMemoryStoreController();
  const controller = {
    ...base,
    threadNavigation: nav,
    state: { ...base.state, repositories },
  };
  const rows = threadInboxRows(controller);
  expect(rows.filter((r) => r.context === "general")).toHaveLength(2);
  expect(
    new Set(rows.filter((r) => r.context === "general").map((r) => r.targetId))
      .size,
  ).toBe(1);
  expect(rows.find((r) => r.context === "Unbound")?.repository).toBeUndefined();
  expect(rows).toHaveLength(3);
});

test("recorded comment containers are internal and repeated preparation does not recreate them", async () => {
  const fixture = createThreadTransport();
  const read = fixture.transport.thread;
  fixture.transport.thread = async (id) => {
    if (!fixture.threads.has(id)) throw new ApiError("not_found");
    return read(id);
  };
  const execute = vi.fn((request) =>
    fixture.transport.command("threads", request),
  );
  const host = createThreadConversations(
    {
      source: () => fixture.transport,
      repository: () => THREAD_IDS.repo,
      principal: () => THREAD_IDS.actor,
    },
    execute,
  );
  const id = "bc".repeat(16),
    request = "cd".repeat(16);
  const first = await host.ensureConversation(id, request);
  const second = await host.ensureConversation(id, request);
  expect(second.thread.reference).toEqual(first.thread.reference);
  expect(first.thread.value.purpose).toEqual({ type: "annotation" });
  expect(execute).toHaveBeenCalledTimes(1);
  expect(
    [...fixture.threads.values()].filter((r) => !r.value.purpose),
  ).toHaveLength(1);
});
