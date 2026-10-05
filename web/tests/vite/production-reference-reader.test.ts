import { expect, test, vi } from "vitest";
import { createProductionReferenceReader } from "../../src/lib/controller/production-reference-reader.js";
import { restoreProductionStoreRoute } from "../../src/lib/controller/production-route-restorer.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";
import type {
  ReferenceQuery,
  StoreDataSource,
} from "../../src/lib/controller/native-data-source.js";

const repository = "ab".repeat(16),
  operation = "cd".repeat(64),
  commit = "12".repeat(64);
const snapshot = {
  version: 1,
  repo_id: repository,
  epoch: "epoch",
  published_seq: 1,
  manifest_revision: 1,
  operation_id: operation,
  heads: [operation],
};
const signature = {
  name: "Maya",
  email: "maya@example.test",
  timestamp: { millis: 1, tz_offset: 0 },
};
const target = {
  commit,
  change: "34".repeat(16),
  description: "selected",
  author: signature,
  committer: signature,
  workspaces: [],
};
function page(name: string, kind = "bookmark", cursor: string | null = null) {
  return {
    snapshot,
    data: {
      references: [
        {
          kind,
          name,
          terms: [commit],
          conflicted: false,
          workspaces: [],
          targets: [
            { ...target, workspaces: kind === "workspace" ? [name] : [] },
          ],
        },
      ],
    },
    next_cursor: cursor,
  };
}
function fixture() {
  const data = createProductionStoreState(),
    history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  const references = vi.fn(
    async (_operation: string, _cursor?: string, query?: ReferenceQuery) =>
      page(
        query?.exact ?? "first",
        query?.kind === "workspaces" ? "workspace" : "bookmark",
        query?.exact ? null : "more",
      ),
  );
  const source = { repository, references } as unknown as StoreDataSource;
  let version = 0;
  const report = vi.fn();
  const reader = createProductionReferenceReader({
    data,
    history,
    source: () => source,
    operation: () => history.route.snapshotOperationId,
    version: () => version,
    current: (ticket) => ticket === version,
    report,
  });
  const reads = {
    ...reader,
    loadFiles: vi.fn(async () => true),
    loadBlob: vi.fn(),
    loadCommit: vi.fn(),
    loadOperation: vi.fn(),
    loadPatch: vi.fn(),
    loadGraph: vi.fn(),
    loadEvolution: vi.fn(),
    loadAgents: vi.fn(),
    loadAgent: vi.fn(),
    loadReviews: vi.fn(),
    loadReview: vi.fn(),
  };
  return {
    data,
    history,
    reader,
    reads,
    references,
    report,
    invalidate: () => version++,
  };
}

test.each(["files", "agents", "reviews"] as const)(
  "%s primary content precedes unresolved reference pickers",
  async (pageName) => {
    const f = fixture();
    f.references.mockImplementation(() => new Promise(() => {}));
    f.history.replace({
      ...f.history.route,
      page: pageName,
      commitId: commit,
      ...(pageName === "files"
        ? {}
        : { bookmark: "inherited", workspace: "inherited" }),
    });
    await restoreProductionStoreRoute(
      f.history.route,
      f.data,
      f.history,
      f.reads,
    );
    expect(
      pageName === "files"
        ? f.reads.loadFiles
        : pageName === "agents"
          ? f.reads.loadAgents
          : f.reads.loadReviews,
    ).toHaveBeenCalled();
    expect(f.data.status.navigation).toBe("ready");
    expect(f.references).toHaveBeenCalledTimes(1);
    expect(f.references.mock.calls[0]?.[2]).toMatchObject({
      kind: "bookmarks",
    });
    f.history.destroy();
  },
);

test("exact bookmark outside the first window stays selected when pickers arrive", async () => {
  const f = fixture();
  f.history.replace({
    ...f.history.route,
    page: "files",
    bookmark: "bookmark-9999",
  });
  await restoreProductionStoreRoute(
    f.history.route,
    f.data,
    f.history,
    f.reads,
  );
  await f.reader.loadReferencePickers();
  expect(f.references.mock.calls[0]?.[2]).toMatchObject({
    kind: "bookmarks",
    exact: "bookmark-9999",
  });
  expect(f.reads.loadFiles).toHaveBeenCalledWith(commit);
  expect(f.data.bookmarks.map((item) => item.name)).toEqual([
    "bookmark-9999",
    "first",
  ]);
  expect(f.data.workingCopies.map((item) => item.workspace)).toEqual(["first"]);
  expect(f.references).toHaveBeenCalledTimes(3);
  f.history.destroy();
});

test("workspace continuations coalesce and cannot overwrite a newer navigation", async () => {
  const f = fixture();
  await f.reader.loadWorkingCopies();
  let resolve!: (value: ReturnType<typeof page>) => void;
  f.references.mockImplementation(
    () =>
      new Promise((done) => {
        resolve = done;
      }),
  );
  const first = f.reader.loadWorkingCopies(true),
    duplicate = f.reader.loadWorkingCopies(true);
  expect(f.references).toHaveBeenCalledTimes(2);
  f.invalidate();
  resolve(page("obsolete", "workspace"));
  await Promise.all([first, duplicate]);
  expect(f.data.workingCopies.map((item) => item.workspace)).toEqual(["first"]);
  f.history.destroy();
});

test("optional picker failure retains usable content and selected references", async () => {
  const f = fixture();
  f.references.mockRejectedValue(new Error("temporarily unavailable"));
  f.history.replace({ ...f.history.route, page: "files", commitId: commit });
  await restoreProductionStoreRoute(
    f.history.route,
    f.data,
    f.history,
    f.reads,
  );
  await f.reader.loadReferencePickers();
  expect(f.data.status.navigation).toBe("ready");
  expect(f.data.status.bookmarks).toBe("failed");
  expect(f.data.workingCopiesStatus).toBe("failed");
  expect(f.report).not.toHaveBeenCalled();
  f.history.destroy();
});

test("a missing exact bookmark does not fall back to a loaded bookmark", async () => {
  const f = fixture();
  await f.reader.loadReferences();
  f.references.mockResolvedValue({
    ...page("unused"),
    data: { references: [] },
  });
  f.history.replace({ ...f.history.route, page: "files", bookmark: "missing" });
  await restoreProductionStoreRoute(
    f.history.route,
    f.data,
    f.history,
    f.reads,
  );
  expect(f.reads.loadFiles).not.toHaveBeenCalled();
  expect(f.data.status.files).toBe("unavailable");
  f.history.destroy();
});

test("selected commit and diff do not resolve the unused change picker", async () => {
  for (const pageName of ["log", "branches"] as const) {
    const f = fixture();
    f.references.mockImplementation(async (_operation, _cursor, query) => {
      if (query?.kind === "changes") throw new Error("index rebuild required");
      return page("first");
    });
    f.history.replace({
      ...f.history.route,
      page: pageName,
      commitId: commit,
      changeId: target.change,
      path: "src/main.rs",
    });
    await restoreProductionStoreRoute(
      f.history.route,
      f.data,
      f.history,
      f.reads,
    );
    await f.reader.loadReferencePickers();
    expect(f.reads.loadGraph).toHaveBeenCalledTimes(1);
    expect(f.reads.loadCommit).toHaveBeenCalledWith(commit);
    expect(f.reads.loadPatch).toHaveBeenCalledWith("src/main.rs");
    expect(f.data.status.navigation).toBe("ready");
    expect(f.report).not.toHaveBeenCalled();
    expect(
      f.references.mock.calls.every((call) => call[2]?.kind !== "changes"),
    ).toBe(true);
    expect(f.history.route).toMatchObject({
      snapshotOperationId: operation,
      commitId: commit,
      changeId: target.change,
    });
    f.history.destroy();
  }
});
