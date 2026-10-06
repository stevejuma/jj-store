import { expect, test, vi } from "vitest";
import { QueryClient } from "@tanstack/svelte-query";
import { createBlameQuery } from "../../src/lib/controller/blame-query.js";
import type {
  BlameProgress,
  BlameViewState,
} from "../../src/lib/controller/blame-types.js";
const selection = {
  repository: "a",
  operation: "op",
  commit: "34".repeat(20),
  path: "test.ts",
  lineCount: 2,
};
const first: BlameProgress = {
  lineCount: 2,
  ranges: [],
  commits: {},
  complete: false,
  continuation: "next",
};
const complete: BlameProgress = {
  ...first,
  complete: true,
  continuation: null,
};
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((r) => {
    resolve = r;
  });
  return { promise, resolve };
}
test("observers coalesce, release independently, resume progress and reuse completion", async () => {
  const client = new QueryClient();
  const windows = [
    deferred<BlameProgress>(),
    deferred<BlameProgress>(),
    deferred<BlameProgress>(),
  ];
  const signals: AbortSignal[] = [];
  const cursors: (string | null)[] = [];
  const read = vi.fn((_selection, cursor, signal) => {
    signals.push(signal);
    cursors.push(cursor);
    return windows[signals.length - 1]!.promise;
  });
  const provider = createBlameQuery({
    client,
    scope: "session1",
    active: () => true,
    read,
  });
  const a: BlameViewState[] = [],
    b: BlameViewState[] = [];
  const stopA = provider.observe(selection, (s) => a.push(s));
  const stopB = provider.observe(selection, (s) => b.push(s));
  expect(read).toHaveBeenCalledTimes(1);
  stopA();
  expect(signals[0]!.aborted).toBe(false);
  windows[0]!.resolve(first);
  await vi.waitFor(() => expect(read).toHaveBeenCalledTimes(2));
  expect(b.at(-1)?.progress).toEqual(first);
  stopB();
  expect(signals[1]!.aborted).toBe(true);
  const c: BlameViewState[] = [];
  const stopC = provider.observe(selection, (s) => c.push(s));
  expect(c[0]?.progress).toEqual(first);
  await vi.waitFor(() => expect(read).toHaveBeenCalledTimes(3));
  expect(cursors).toEqual([null, "next", "next"]);
  windows[1]!.resolve(complete); // canceled late publication is ignored
  windows[2]!.resolve(complete);
  await vi.waitFor(() => expect(c.at(-1)?.status).toBe("ready"));
  stopC();
  const d: BlameViewState[] = [];
  const stopD = provider.observe(selection, (s) => d.push(s));
  expect(d[0]?.progress?.complete).toBe(true);
  expect(read).toHaveBeenCalledTimes(3);
  expect(a).toHaveLength(1);
  stopD();
  await vi.waitFor(() =>
    expect(client.getQueryCache().getAll()).toHaveLength(1),
  );
  client.clear();
});
test("inactive cached scopes fail closed; invalid progress stops without retry", async () => {
  const client = new QueryClient();
  let active = true;
  const read = vi.fn(async () => complete);
  const provider = createBlameQuery({
    client,
    scope: "s",
    active: () => active,
    read,
  });
  const states: BlameViewState[] = [];
  const stop = provider.observe(selection, (s) => states.push(s));
  await vi.waitFor(() => expect(states.at(-1)?.status).toBe("ready"));
  stop();
  active = false;
  const stop2 = provider.observe(selection, (s) => states.push(s));
  expect(states.at(-1)?.status).toBe("failed");
  expect(states.at(-1)?.progress).toBeUndefined();
  expect(read).toHaveBeenCalledTimes(1);
  stop2();
  const invalid = vi.fn(async () => ({ ...first, lineCount: 999 }));
  const invalidProvider = createBlameQuery({
    client,
    scope: "other",
    active: () => true,
    read: invalid,
  });
  const stop3 = invalidProvider.observe(selection, (s) => states.push(s));
  await vi.waitFor(() =>
    expect(states.at(-1)?.error).toContain("Invalid Blame progress"),
  );
  expect(invalid).toHaveBeenCalledTimes(1);
  stop3();
  client.clear();
});
test("file and operation selections never share cached results", async () => {
  const client = new QueryClient();
  const read = vi.fn(async () => complete);
  const provider = createBlameQuery({
    client,
    scope: "s",
    active: () => true,
    read,
  });
  for (const scope of [
    selection,
    { ...selection, path: "other.ts" },
    { ...selection, operation: "other" },
  ]) {
    let state: BlameViewState | undefined;
    const stop = provider.observe(scope, (s) => {
      state = s;
    });
    await vi.waitFor(() => expect(state?.status).toBe("ready"));
    stop();
  }
  expect(read).toHaveBeenCalledTimes(3);
  client.clear();
});

test("terminal failures preserve proven ranges without background retries", async () => {
  const client = new QueryClient();
  const commitId = "ab".repeat(20);
  const proven: BlameProgress = {
    ...first,
    ranges: [{ start: 1, end: 2, commitId }],
    commits: {
      [commitId]: {
        eventId: commitId,
        changeId: "ab".repeat(16),
        message: "An actual origin",
        author: "A",
        createdAt: 1,
      },
    },
  };
  const read = vi
    .fn()
    .mockResolvedValueOnce(proven)
    .mockRejectedValueOnce(new Error("Read failed"));
  const provider = createBlameQuery({
    client,
    scope: "failure",
    active: () => true,
    read,
  });
  const states: BlameViewState[] = [];
  const stop = provider.observe(selection, (s) => states.push(s));
  await vi.waitFor(() => expect(states.at(-1)?.status).toBe("failed"));
  expect(states.at(-1)?.progress).toEqual(proven);
  expect(read).toHaveBeenCalledTimes(2);
  stop();
  client.clear();
});
