import { expect, test, vi } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { createRepositoryThreadNavigation } from "../../src/lib/controller/repository-thread-navigation.svelte.js";
import {
  createThreadTransport,
  THREAD_IDS,
} from "../../../catalog/fixtures/thread-transport.js";
import type { StoreRepositorySummary } from "../../src/lib/controller/contracts.js";

test("independent repository pages retain their owner and loaded window through refresh", async () => {
  const fixture = createThreadTransport();
  const first = await fixture.transport.list();
  if (first.items.type !== "threads") throw new Error("Expected threads");
  const other = "99".repeat(16),
    later = "aa".repeat(16);
  const repositories: StoreRepositorySummary[] = [
    { id: THREAD_IDS.repo, name: "Home", permission: "read" },
    { id: other, name: "Other", permission: "admin" },
    {
      id: "88".repeat(16),
      name: "Local",
      profile: "git_sha1_jj045",
      permission: "admin",
    },
  ];
  const close = vi.fn();
  const list = vi.fn(async (repo: string, cursor?: string) => ({
    ...first,
    snapshot: { ...first.snapshot, repo_id: repo },
    items: {
      type: "threads" as const,
      threads: cursor
        ? [
            {
              ...first.items.threads[0]!,
              value: {
                ...first.items.threads[0]!.value,
                thread_id: later,
                title: "Later",
              },
            },
          ]
        : first.items.threads,
    },
    next_cursor: cursor ? null : "next",
  }));
  const navigation = createRepositoryThreadNavigation({
    repositories: () => repositories,
    credential: () => null,
    source: (repo) => ({ list: (cursor) => list(repo, cursor), close }),
  });
  await navigation.load();
  expect(list.mock.calls.map((args) => args[0]).sort()).toEqual(
    [THREAD_IDS.repo, other].sort(),
  );
  await navigation.more(THREAD_IDS.repo);
  expect(
    navigation.entries[THREAD_IDS.repo]!.threads.map(
      (row) => row.value.thread_id,
    ),
  ).toEqual([THREAD_IDS.thread, later]);
  await navigation.load(true);
  expect(navigation.entries[THREAD_IDS.repo]!.threads).toHaveLength(2);
  expect(navigation.entries[other]!.threads).toHaveLength(1);
  expect(close).toHaveBeenCalledTimes(list.mock.calls.length - 1);
});

test("late reads cannot restore sidebar data after credential replacement or teardown", async () => {
  const fixture = createThreadTransport();
  let credential = "one";
  let resolve!: (
    value: Awaited<ReturnType<typeof fixture.transport.list>>,
  ) => void;
  const close = vi.fn();
  const navigation = createRepositoryThreadNavigation({
    repositories: () => [
      { id: THREAD_IDS.repo, name: "Home", permission: "read" },
    ],
    credential: () => credential,
    source: () => ({
      list: () =>
        new Promise((done) => {
          resolve = done;
        }),
      close,
    }),
  });
  const reading = navigation.load();
  await vi.waitFor(() => expect(resolve).toBeTypeOf("function"));
  credential = "two";
  navigation.clear();
  resolve(await fixture.transport.list());
  await reading;
  expect(navigation.entries).toEqual({});
  expect(close).toHaveBeenCalled();
});

test("foreign repository pages and access-denied refreshes fail closed", async () => {
  const fixture = createThreadTransport();
  let denied = false;
  const repo = "99".repeat(16);
  const navigation = createRepositoryThreadNavigation({
    repositories: () => [{ id: repo, name: "Other", permission: "read" }],
    credential: () => null,
    source: () => ({
      list: () =>
        denied
          ? Promise.reject(new ApiError("access_denied"))
          : fixture.transport.list(),
      close: () => {},
    }),
  });
  await navigation.load();
  expect(navigation.entries[repo]!.threads).toHaveLength(0);
  expect(navigation.entries[repo]!.error).toBe("invalid thread page");
  denied = true;
  await navigation.load(true);
  expect(navigation.entries[repo]!.error).toBe("access denied");
});
