import assert from "node:assert/strict";
import { describe, test } from "node:test";
import { ApiError, type Model } from "../src/lib/api/client.js";
import type { StoreDataSource } from "../src/lib/controller/native-data-source.js";
import { createProductionStoreSearch } from "../src/lib/controller/production-store-search-controller.js";
import {
  assertSearchPage,
  presentSearchResults,
  routeForSearchResult,
} from "../src/lib/controller/production-store-search.js";
import type { ProductionStoreState } from "../src/lib/controller/production-store-state.svelte.js";
import type {
  StoreRoute,
  StoreSearchResult,
} from "../src/lib/controller/contracts.js";

const repository = "01".repeat(16);
const operation = "02".repeat(64);
const commit = "03".repeat(64);
const change = "04".repeat(16);
const file = "05".repeat(64);

function document(key: Model<"SearchDocumentKey">): Model<"SearchDocument"> {
  return {
    key,
    title: "<script>source is text</script>",
    body: "",
    actor: null,
    author: "Maya Chen",
    workspace: "demo",
    operation,
    commit,
    change,
    file,
    parents: [],
    predecessors: [],
    attributes: {},
    language: "rust",
    has_conflict: false,
    timestamp_millis: 1_790_506_800_000,
  };
}

const route: StoreRoute = {
  page: "branches",
  repository,
  bookmark: "",
  snapshotOperationId: operation,
  inspectedOperationId: null,
  commitId: null,
  changeId: null,
  comparisonParentId: null,
  comparisonGroupId: null,
  path: null,
  directory: "",
};

function searchPage(
  title: string,
  nextCursor: string | null = null,
): Model<"SearchPage"> {
  return {
    repository,
    selected_view: { repo_id: repository, operation_id: operation },
    results: [
      {
        ...document({ kind: "commit", id: commit }),
        title,
      },
    ],
    next_cursor: nextCursor,
    is_stale: false,
    degraded: false,
    truncated: false,
  } as Model<"SearchPage">;
}

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (cause: unknown) => void;
  const promise = new Promise<T>((accept, fail) => {
    resolve = accept;
    reject = fail;
  });
  return { promise, resolve, reject };
}

function searchHarness(
  search: StoreDataSource["search"],
  pushed: StoreRoute[] = [],
) {
  const data = {
    status: { search: "idle" },
    searchAvailable: true,
    searchQuery: "",
    searchResults: [],
    searchCursor: null,
    searchStale: false,
    searchDegraded: false,
    searchTruncated: false,
    searchError: "",
  } as unknown as ProductionStoreState;
  const source = { repository, search } as StoreDataSource;
  const history = {
    route,
    push(next: StoreRoute) {
      pushed.push(next);
    },
  };
  const controller = createProductionStoreSearch({
    data,
    history: history as never,
    source: () => source,
    isCurrent: (candidate) => candidate === source,
    restoreRoute: async () => {},
  });
  return { controller, data, pushed };
}

describe("production search presentation", () => {
  test("preserves source strings and full native selection identities", () => {
    const [result] = presentSearchResults([
      document({ kind: "file", scope: "main", path: "src/lib.rs" }),
    ]);
    assert.ok(result);
    assert.deepEqual(
      {
        kind: result.kind,
        title: result.title,
        operationId: result.operationId,
        commitId: result.commitId,
        changeId: result.changeId,
        scope: result.scope,
        path: result.path,
      },
      {
        kind: "file",
        title: "<script>source is text</script>",
        operationId: operation,
        commitId: commit,
        changeId: change,
        scope: "main",
        path: "src/lib.rs",
      },
    );
    assert.deepEqual(routeForSearchResult(route, result), {
      ...route,
      page: "files",
      bookmark: "main",
      commitId: commit,
      changeId: change,
      path: "src/lib.rs",
      directory: "src",
    });
  });

  test("routes commit and operation results without confusing identities", () => {
    const base: StoreSearchResult = {
      id: "result",
      kind: "commit",
      title: "Commit",
      description: "Description",
      author: "Maya Chen",
      timestamp: 0,
      operationId: operation,
      commitId: commit,
      changeId: change,
      scope: null,
      path: null,
      hasConflict: false,
    };
    const commitRoute = routeForSearchResult(route, base);
    assert.equal(commitRoute.page, "log");
    assert.equal(commitRoute.commitId, commit);
    assert.equal(commitRoute.changeId, change);
    assert.equal(commitRoute.inspectedOperationId, null);
    const operationRoute = routeForSearchResult(route, {
      ...base,
      kind: "operation",
      operationId: operation,
      commitId: null,
      changeId: null,
    });
    assert.equal(operationRoute.page, "oplog");
    assert.equal(operationRoute.inspectedOperationId, operation);
    assert.equal(operationRoute.commitId, null);
  });

  test("rejects search pages from another snapshot", () => {
    const page = {
      repository,
      selected_view: { repo_id: repository, operation_id: operation },
    } as Model<"SearchPage">;
    assert.doesNotThrow(() => assertSearchPage(page, repository, operation));
    assert.throws(
      () => assertSearchPage(page, repository, "ff".repeat(64)),
      /snapshot mismatch/u,
    );
  });

  test("ignores a late response from an older query", async () => {
    const oldResponse = deferred<Model<"SearchPage">>();
    const newResponse = deferred<Model<"SearchPage">>();
    const { controller, data } = searchHarness(({ query }) =>
      query === "old" ? oldResponse.promise : newResponse.promise,
    );

    const oldRun = controller.run("old");
    const newRun = controller.run("new");
    newResponse.resolve(searchPage("New result"));
    await newRun;
    oldResponse.resolve(searchPage("Old result"));
    await oldRun;

    assert.equal(data.searchQuery, "new");
    assert.deepEqual(
      data.searchResults.map((result) => result.title),
      ["New result"],
    );
  });

  test("continues with the opaque cursor and deduplicates overlapping pages", async () => {
    const cursors: (string | undefined)[] = [];
    const first = searchPage("First result", "opaque.next.cursor");
    const duplicate = searchPage("First result");
    duplicate.results.push({
      ...document({ kind: "file", scope: "main", path: "src/lib.rs" }),
      title: "src/lib.rs",
    });
    const { controller, data } = searchHarness(async ({ cursor }) => {
      cursors.push(cursor);
      return cursor ? duplicate : first;
    });

    await controller.run("native");
    await controller.loadMore();

    assert.deepEqual(cursors, [undefined, "opaque.next.cursor"]);
    assert.deepEqual(
      data.searchResults.map((result) => result.title),
      ["First result", "src/lib.rs"],
    );
    assert.equal(data.searchCursor, null);
  });

  test("presents cursor reset failures without retaining a refreshing state", async () => {
    let page = 0;
    const { controller, data } = searchHarness(async () => {
      page += 1;
      if (page === 1) return searchPage("First result", "expired.cursor");
      throw new ApiError("search_cursor_expired");
    });

    await controller.run("native");
    await controller.loadMore();

    assert.equal(data.status.search, "failed");
    assert.equal(data.searchError, "search cursor expired");
    assert.deepEqual(
      data.searchResults.map((result) => result.title),
      ["First result"],
    );
  });
});
