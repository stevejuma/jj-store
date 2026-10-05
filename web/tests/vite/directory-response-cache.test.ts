import { expect, test } from "vitest";
import { createProductionFileReader } from "../../src/lib/controller/production-file-reader.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";
import type {
  PathQuery,
  StoreDataSource,
} from "../../src/lib/controller/native-data-source.js";

const repository = "11".repeat(16),
  operation = "22".repeat(64),
  commit = "33".repeat(64);
const signature = {
  name: "A",
  email: "a@example.test",
  timestamp: { millis: 1, offset_minutes: 0 },
};
const snapshot = {
  version: 1,
  repo_id: repository,
  epoch: "44".repeat(16),
  published_seq: 1,
  manifest_revision: 1,
  code_state_version: 1,
  operation_id: operation,
  heads: [operation],
};
function fixture() {
  const data = createProductionStoreState(),
    history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  history.push({ ...history.route, page: "files", commitId: commit });
  let version = 1,
    selectedOperation = operation;
  const scope = new AbortController();
  const calls: PathQuery[] = [];
  const errors: unknown[] = [];
  let wait: ((query: PathQuery) => Promise<void>) | undefined;
  let paginated = false;
  let source = {
    repository,
    async paths(id: string, op: string, query: PathQuery) {
      calls.push(query);
      await wait?.(query);
      const last = query.cursor || query.anchor;
      const path = `${query.path ? query.path + "/" : ""}${last ? "b" : "a"}.txt`;
      return {
        snapshot: { ...snapshot, operation_id: op },
        data: {
          version: 1,
          commit: id,
          path: query.path ?? "",
          recursive: query.recursive ?? false,
          entries: [
            {
              path,
              terms: [
                {
                  type: "file",
                  id: "55".repeat(64),
                  executable: false,
                  copy: { kind: "placeholder" },
                },
              ],
              conflicted: false,
              last_change: query.includeLastChange
                ? {
                    status: "available",
                    attribution:
                      query.lastChangeMode === "history"
                        ? "path_change"
                        : "snapshot",
                    commit: id,
                    change: "66".repeat(16),
                    description:
                      query.lastChangeMode === "history"
                        ? "Actual edit"
                        : "Snapshot",
                    author: signature,
                    committer: signature,
                  }
                : { status: "unavailable", reason: "not_requested" },
            },
          ],
          previous: null,
          next: paginated && !last ? path : null,
          stats: {},
        },
        next_cursor: paginated && !last ? "next" : null,
        previous_cursor: null,
      };
    },
    async commit() {
      throw new Error("No optional header in this fixture");
    },
  } as unknown as StoreDataSource;
  const reader = createProductionFileReader({
    data,
    history,
    source: () => source,
    operation: () => selectedOperation,
    version: () => version,
    signal: () => scope.signal,
    current: (ticket, active) => ticket === version && active === source,
    report: (cause) => errors.push(cause),
  });
  return {
    data,
    history,
    reader,
    calls,
    errors,
    scope,
    pause: (value: typeof wait) => {
      wait = value;
    },
    paginate: () => {
      paginated = true;
    },
    invalidate: () => {
      version++;
    },
    operation: () => {
      selectedOperation = "77".repeat(64);
    },
    source: () => {
      source = { ...source };
    },
  };
}
const completed = async (f: ReturnType<typeof fixture>, path = "a.txt") => {
  await expect
    .poll(() => f.data.snapshot?.pathChanges?.[path]?.message)
    .toBe("Actual edit");
};

test("revisiting a directory retains names and completed attribution without another request", async () => {
  const f = fixture();
  try {
    await f.reader.loadFiles(commit);
    await completed(f);
    await f.reader.selectDirectory("arch");
    await completed(f, "arch/a.txt");
    const before = f.calls.length;
    await f.reader.selectDirectory("");
    await f.reader.selectDirectory("arch");
    await new Promise((resolve) => setTimeout(resolve, 200));
    expect(f.calls).toHaveLength(before);
    expect(f.data.status.files).toBe("ready");
    expect(f.data.snapshot?.pathChanges?.["a.txt"]?.message).toBe(
      "Actual edit",
    );
    expect(f.errors).toEqual([]);
  } finally {
    f.scope.abort();
  }
});

test("concurrent directory selections share the pending names read", async () => {
  const f = fixture(),
    pending = Promise.withResolvers<void>();
  f.pause((query) =>
    query.includeLastChange ? Promise.resolve() : pending.promise,
  );
  try {
    const first = f.reader.selectDirectory("arch"),
      second = f.reader.selectDirectory("arch");
    expect(f.data.status.files).toBe("loading");
    expect(f.calls.filter((q) => !q.includeLastChange)).toHaveLength(1);
    pending.resolve();
    await Promise.all([first, second]);
    await completed(f, "arch/a.txt");
  } finally {
    pending.resolve();
    f.scope.abort();
  }
});

for (const change of ["invalidate", "operation", "source"] as const)
  test(`directory cache is discarded on ${change}`, async () => {
    const f = fixture();
    try {
      await f.reader.loadFiles(commit);
      await completed(f);
      f[change]();
      await f.reader.loadFiles(commit);
      expect(f.calls.filter((q) => !q.includeLastChange)).toHaveLength(2);
    } finally {
      f.scope.abort();
    }
  });

test("returning after canceled history resumes history without repeating quick attribution", async () => {
  const f = fixture();
  let canceled: AbortSignal | undefined;
  f.pause(async (query) => {
    if (!query.path && query.lastChangeMode === "history") {
      canceled = query.signal;
      await new Promise<void>((_resolve, reject) =>
        query.signal?.addEventListener(
          "abort",
          () => reject(query.signal?.reason),
          { once: true },
        ),
      );
    }
  });
  try {
    await f.reader.loadFiles(commit);
    await expect.poll(() => canceled).toBeDefined();
    await f.reader.selectDirectory("arch");
    expect(canceled?.aborted).toBe(true);
    await completed(f, "arch/a.txt");
    f.pause(undefined);
    await f.reader.selectDirectory("");
    await completed(f);
    expect(f.calls.filter((q) => !q.path && !q.includeLastChange)).toHaveLength(
      1,
    );
    expect(
      f.calls.filter(
        (q) => !q.path && q.includeLastChange && !q.lastChangeMode,
      ),
    ).toHaveLength(1);
  } finally {
    f.scope.abort();
  }
});

test("cached paginated directories retain every loaded window and its attribution", async () => {
  const f = fixture();
  f.paginate();
  try {
    await f.reader.loadFiles(commit);
    await completed(f);
    await f.reader.loadMoreFiles();
    await completed(f, "b.txt");
    await f.reader.selectDirectory("arch");
    await completed(f, "arch/a.txt");
    const before = f.calls.length;
    await f.reader.selectDirectory("");
    expect(f.calls).toHaveLength(before);
    expect(
      f.data.pathPages.get("directory:")?.entries.map((entry) => entry.path),
    ).toEqual(["a.txt", "b.txt"]);
    expect(f.data.pathPages.get("directory:")?.cursor).toBeNull();
  } finally {
    f.scope.abort();
  }
});

test("returning after canceled quick attribution restores only the metadata skeleton", async () => {
  const f = fixture();
  let started = false;
  f.pause(async (query) => {
    if (!query.path && query.includeLastChange) {
      started = true;
      await new Promise<void>((_resolve, reject) =>
        query.signal?.addEventListener(
          "abort",
          () => reject(query.signal?.reason),
          { once: true },
        ),
      );
    }
  });
  try {
    await f.reader.loadFiles(commit);
    await expect.poll(() => started).toBe(true);
    await f.reader.selectDirectory("arch");
    await completed(f, "arch/a.txt");
    await f.reader.selectDirectory("");
    expect(f.data.status.files).toBe("ready");
    expect(
      f.calls.filter((query) => !query.path && !query.includeLastChange),
    ).toHaveLength(1);
    expect(Object.hasOwn(f.data.snapshot?.pathChanges ?? {}, "a.txt")).toBe(
      false,
    );
  } finally {
    f.scope.abort();
  }
});
