import { expect, test } from "vitest";
import { ApiError } from "../../src/lib/api/api-error.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProductionFileReader } from "../../src/lib/controller/production-file-reader.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";

const repository = "11".repeat(16);
const operation = "12".repeat(64);
const commit = "13".repeat(64);
const change = "14".repeat(16);
const snapshot = {
  version: 1,
  repo_id: repository,
  epoch: "15".repeat(16),
  published_seq: 2,
  manifest_revision: 3,
  code_state_version: 4,
  operation_id: operation,
  heads: [operation],
};
const stats = {
  visited_objects: 1,
  visited_bytes: 1,
  elapsed_millis: 1,
  cache_hits: 0,
};

test("files load the selected commit metadata independently of graph history", async () => {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  history.push({
    ...history.route,
    page: "files",
    commitId: commit,
    changeId: change,
  });
  let commitReads = 0;
  const source = {
    repository,
    async paths() {
      return {
        snapshot,
        data: {
          version: 1,
          commit,
          path: "",
          recursive: true,
          entries: [],
          previous: null,
          next: null,
          stats,
        },
        previous_cursor: null,
        next_cursor: null,
      };
    },
    async commit() {
      commitReads++;
      return {
        snapshot,
        data: {
          object: { kind: "commit", id: commit },
          commit: {
            id: commit,
            change_id: change,
            parents: ["16".repeat(64)],
            description: "Expose exact file metadata",
            author: {
              name: "Maya Chen",
              email: "maya@example.test",
              timestamp: { millis: 1_796_000_000_000, offset_minutes: 0 },
            },
            committer: {
              name: "Maya Chen",
              email: "maya@example.test",
              timestamp: { millis: 1_796_000_000_000, offset_minutes: 0 },
            },
            tree_terms: [],
            conflicted: false,
            bookmarks: [],
            workspaces: [],
            visible: true,
            divergent: false,
          },
        },
        next_cursor: null,
      };
    },
  } as unknown as StoreDataSource;
  const reader = createProductionFileReader({
    data,
    history,
    source: () => source,
    operation: () => operation,
    version: () => 1,
    current: () => true,
    report: (cause) => {
      throw cause;
    },
  });

  await reader.loadFiles(commit);

  expect(commitReads).toBe(1);
  await expect
    .poll(() => data.fileRevision)
    .toMatchObject({
      eventId: commit,
      changeId: change,
      message: "Expose exact file metadata",
      author: "Maya Chen",
    });
});

// File rows must be usable while optional commit and document work is pending.
test("file rows become ready before commit metadata and landing documents", async () => {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  history.push({ ...history.route, page: "files", commitId: commit });
  const metadata = Promise.withResolvers<never>();
  const document = Promise.withResolvers<{ text: string; bytes: number }>();
  const failures: unknown[] = [];
  const source = {
    repository,
    paths: async () => ({
      snapshot,
      data: {
        version: 1,
        commit,
        path: "",
        recursive: false,
        entries: [
          {
            path: "README.md",
            terms: [
              {
                type: "file",
                id: "31".repeat(64),
                executable: false,
                copy: { kind: "placeholder" },
              },
            ],
            conflicted: false,
            last_change: { status: "unavailable", reason: "budget" },
          },
        ],
        previous: null,
        next: null,
        stats,
      },
      previous_cursor: null,
      next_cursor: null,
    }),
    commit: () => metadata.promise,
    file: () => document.promise,
  } as unknown as StoreDataSource;
  const reader = createProductionFileReader({
    data,
    history,
    source: () => source,
    operation: () => operation,
    version: () => 1,
    current: () => true,
    report: (cause) => {
      failures.push(cause);
    },
  });
  const loading = reader.loadFiles(commit);
  try {
    await expect.poll(() => data.status.files, { timeout: 300 }).toBe("ready");
    expect(data.snapshot?.files[0]?.path).toBe("README.md");
    metadata.reject(new Error("metadata unavailable"));
    document.resolve({ text: "# Ready", bytes: 7 });
    await loading;
    await expect.poll(() => data.snapshot?.files[0]?.contents).toBe("# Ready");
    expect(data.status.files).toBe("ready");
  } finally {
    metadata.reject(new Error("metadata unavailable"));
    document.resolve({ text: "# Ready", bytes: 7 });
    await loading;
  }
});

test("obsolete metadata is canceled and cannot replace a newer file selection", async () => {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  const old = Promise.withResolvers<never>();
  let metadataSignal: AbortSignal | undefined;
  let version = 1;
  let scope = new AbortController();
  const newer = "23".repeat(64);
  const source = {
    repository,
    paths: async (id: string) => ({
      snapshot,
      data: {
        version: 1,
        commit: id,
        path: "",
        recursive: false,
        entries: [
          {
            path: `${id.slice(0, 2)}.txt`,
            terms: [
              {
                type: "file",
                id: "31".repeat(64),
                executable: false,
                copy: { kind: "placeholder" },
              },
            ],
            conflicted: false,
            last_change: { status: "unavailable", reason: "not_requested" },
          },
        ],
        previous: null,
        next: null,
        stats,
      },
      next_cursor: null,
      previous_cursor: null,
    }),
    commitSummary: (_id: string, _operation: string, signal: AbortSignal) => {
      metadataSignal = signal;
      return old.promise;
    },
  } as unknown as StoreDataSource;
  const reader = createProductionFileReader({
    data,
    history,
    source: () => source,
    operation: () => operation,
    version: () => version,
    signal: () => scope.signal,
    current: (ticket) => ticket === version,
    report: (cause) => {
      throw cause;
    },
  });
  await reader.loadFiles(commit);
  expect(data.status.files).toBe("ready");
  expect(metadataSignal?.aborted).toBe(false);
  scope.abort();
  expect(metadataSignal?.aborted).toBe(true);
  expect(data.fileRevisionStatus).toBe("unavailable");
  scope = new AbortController();
  version++;
  await reader.loadFiles(newer);
  old.reject(new Error("aborted"));
  await expect.poll(() => data.fileRevisionStatus).toBe("unavailable");
  expect(data.snapshot?.id).toBe(newer);
  expect(data.snapshot?.files.map((file) => file.path)).toEqual(["23.txt"]);
  expect(data.fileRevision).toBeUndefined();
  expect(data.status.files).toBe("ready");
});

test("attribution failures preserve rows and report unavailable without inventing server budget results", async () => {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  const requests: unknown[] = [];
  const source = {
    repository,
    paths: async (
      _commit: string,
      _operation: string,
      query: { includeLastChange: boolean },
    ) => {
      requests.push(query);
      if (query.includeLastChange) throw new Error("network unavailable");
      return {
        snapshot,
        data: {
          version: 1,
          commit,
          path: "",
          recursive: false,
          entries: [
            {
              path: "a.txt",
              terms: [
                {
                  type: "file",
                  id: "31".repeat(64),
                  executable: false,
                  copy: { kind: "placeholder" },
                },
              ],
              conflicted: false,
              last_change: { status: "unavailable", reason: "not_requested" },
            },
          ],
          previous: null,
          next: null,
          stats,
        },
        next_cursor: null,
        previous_cursor: null,
      };
    },
    commit: async () => {
      throw new Error("metadata unavailable");
    },
  } as unknown as StoreDataSource;
  const reader = createProductionFileReader({
    data,
    history,
    source: () => source,
    operation: () => operation,
    version: () => 1,
    current: () => true,
    report: (cause) => {
      throw cause;
    },
  });
  await reader.loadFiles(commit);
  await expect
    .poll(() => Object.hasOwn(data.snapshot?.pathChanges ?? {}, "a.txt"))
    .toBe(true);
  expect(data.snapshot?.pathChanges?.["a.txt"]).toBeUndefined();
  expect(data.pathPages.get("directory:")?.entries[0]?.last_change).toEqual({
    status: "unavailable",
    reason: "not_requested",
  });
  expect(data.status.files).toBe("ready");
  expect(requests).toHaveLength(2);
});

for (const outcome of ["complete", "failure", "busy", "eviction"]) {
  test(`pending row attribution ${outcome} retains usable names and resolved rows`, async () => {
    const data = createProductionStoreState();
    const history = createStoreRouteHistory(undefined);
    history.connect(repository, operation);
    history.push({ ...history.route, page: "files", commitId: commit });
    const finish = Promise.withResolvers<void>();
    let batches = 0;
    const row = (path: string, ready: boolean) => ({
      path,
      terms: [
        {
          type: "file",
          id: "31".repeat(64),
          executable: false,
          copy: { kind: "placeholder" },
        },
      ],
      conflicted: false,
      last_change: ready
        ? {
            status: "available",
            commit,
            change,
            description: "Original change",
            author: {
              name: "A",
              email: "a@example.test",
              timestamp: { millis: 1, offset_minutes: 0 },
            },
            committer: {
              name: "A",
              email: "a@example.test",
              timestamp: { millis: 1, offset_minutes: 0 },
            },
          }
        : { status: "unavailable", reason: "pending" },
    });
    const source = {
      repository,
      paths: async (
        _commit: string,
        _operation: string,
        query: { includeLastChange: boolean },
      ) => {
        if (query.includeLastChange && ++batches === 2) {
          await finish.promise;
          if (outcome === "busy") throw new ApiError("busy");
        }
        return {
          snapshot,
          data: {
            version: 1,
            commit,
            path: "",
            recursive: false,
            entries: [
              row(
                "recent.txt",
                query.includeLastChange &&
                  !(outcome === "eviction" && batches === 2),
              ),
              row("old.txt", batches >= 2),
            ],
            previous: null,
            next: null,
            stats,
          },
          next_cursor: null,
          previous_cursor: null,
        };
      },
      commit: async () => {
        throw new Error("optional header unavailable");
      },
    } as unknown as StoreDataSource;
    const scope = new AbortController();
    const reader = createProductionFileReader({
      data,
      history,
      source: () => source,
      operation: () => operation,
      version: () => 1,
      signal: () => scope.signal,
      current: () => true,
      report: (cause) => {
        throw cause;
      },
    });
    try {
      await reader.loadFiles(commit);
      await expect.poll(() => batches).toBe(2);
      expect(data.status.files).toBe("ready");
      expect(data.snapshot?.files).toHaveLength(2);
      expect(data.snapshot?.pathChanges?.["recent.txt"]?.message).toBe(
        "Original change",
      );
      expect(Object.hasOwn(data.snapshot?.pathChanges ?? {}, "old.txt")).toBe(
        false,
      );
      if (outcome !== "failure") {
        finish.resolve();
        await expect
          .poll(() => data.snapshot?.pathChanges?.["old.txt"]?.message)
          .toBe("Original change");
      } else {
        finish.reject(new Error("later batch failed"));
        await expect
          .poll(() =>
            Object.hasOwn(data.snapshot?.pathChanges ?? {}, "old.txt"),
          )
          .toBe(true);
        expect(data.snapshot?.pathChanges?.["old.txt"]).toBeUndefined();
      }
      expect(data.snapshot?.pathChanges?.["recent.txt"]?.message).toBe(
        "Original change",
      );
      await expect
        .poll(() => batches)
        .toBe(["busy", "eviction"].includes(outcome) ? 3 : 2);
    } finally {
      scope.abort();
      finish.resolve();
    }
  });
}

test("directory navigation cancels pending attribution for the previous folder", async () => {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  history.push({ ...history.route, page: "files", commitId: commit });
  let pendingSignal: AbortSignal | undefined;
  let oldRequests = 0;
  const source = {
    repository,
    paths: async (
      _commit: string,
      _operation: string,
      query: { includeLastChange: boolean; path: string; signal: AbortSignal },
    ) => {
      if (query.includeLastChange && query.path === "old") {
        oldRequests++;
        pendingSignal = query.signal;
        return new Promise((_, reject) =>
          query.signal.addEventListener(
            "abort",
            () => reject(query.signal.reason),
            { once: true },
          ),
        );
      }
      return {
        snapshot,
        data: {
          version: 1,
          commit,
          path: query.path,
          recursive: false,
          entries: [
            {
              path: `${query.path}/file.txt`,
              terms: [
                {
                  type: "file",
                  id: "31".repeat(64),
                  executable: false,
                  copy: { kind: "placeholder" },
                },
              ],
              conflicted: false,
              last_change: {
                status: "unavailable",
                reason: query.includeLastChange ? "not_found" : "not_requested",
              },
            },
          ],
          previous: null,
          next: null,
          stats,
        },
        next_cursor: null,
        previous_cursor: null,
      };
    },
    commit: async () => {
      throw new Error("optional header unavailable");
    },
  } as unknown as StoreDataSource;
  const scope = new AbortController();
  const reader = createProductionFileReader({
    data,
    history,
    source: () => source,
    operation: () => operation,
    version: () => 1,
    signal: () => scope.signal,
    current: () => true,
    report: (cause) => {
      throw cause;
    },
  });
  try {
    await reader.loadFiles(commit, { path: "old", recursive: false });
    await expect.poll(() => pendingSignal).toBeDefined();
    await reader.loadFiles(commit, { path: "new", recursive: false });
    expect(pendingSignal?.aborted).toBe(true);
    await expect
      .poll(() => data.pathPages.get("directory:new")?.entries[0]?.last_change)
      .toEqual({ status: "unavailable", reason: "not_found" });
    expect(oldRequests).toBe(1);
    expect(data.status.files).toBe("ready");
  } finally {
    scope.abort();
  }
});
