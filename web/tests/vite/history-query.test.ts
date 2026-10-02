import { expect, test, vi } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { createHistoryQuery } from "../../src/lib/controller/history-query.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";

const repository = "11".repeat(16),
  operation = "22".repeat(64);
function fixture() {
  const data = createProductionStoreState();
  data.status.connection = "connected";
  data.graphAtTimeMillis = 12345;
  const history = createStoreRouteHistory();
  history.connect(repository, operation);
  history.page("log");
  let version = 0;
  const snapshot = {
    version: 1,
    repo_id: repository,
    operation_id: operation,
    epoch: "33".repeat(16),
    published_seq: 1,
    manifest_revision: 1,
    code_state_version: 1,
    heads: [operation],
  };
  const result = (): Awaited<ReturnType<StoreDataSource["graph"]>> => ({
    snapshot,
    data: {
      version: 1,
      operation,
      data: { graph: "log", nodes: [] },
      next: null,
      stats: {
        object_reads: 0,
        object_bytes: 0,
        work: 0,
        metadata_nodes: 0,
        predicate_file_bytes: 0,
        predicate_lines: 0,
      },
      cache: {
        state: "built",
        evicted: 0,
        retained_entries: 0,
        retained_bytes: 0,
      },
    },
    next_cursor: null,
    previous_cursor: null,
  });
  const graph = vi.fn<StoreDataSource["graph"]>(async () => result());
  const source = {
    repository,
    profile: "native",
    graph,
  } as unknown as StoreDataSource;
  const query = createHistoryQuery({
    data,
    history,
    source: () => source,
    version: () => version,
    begin: () => ++version,
    current: (ticket) => ticket === version,
  });
  return { data, history, query, graph, result, invalidate: () => ++version };
}

test("history query freezes pin/time and updates only successful text", async () => {
  const f = fixture();
  f.query.setDraft('description("fix")');
  await f.query.run();
  expect(f.graph).toHaveBeenCalledWith(
    "log",
    expect.objectContaining({
      operation,
      revset: 'description("fix")',
      atTimeMillis: 12345,
    }),
  );
  expect(f.history.route.historyQuery).toBe('description("fix")');
  const previous = f.data.graph;
  f.graph.mockRejectedValueOnce(
    new ApiError("invalid_query", undefined, undefined, {
      kind: "syntax",
      message: "Close the quote.",
    }),
  );
  f.query.setDraft('description("');
  await f.query.run();
  expect(f.query.draft).toBe('description("');
  expect(f.query.error).toContain("Showing previous results. Close the quote.");
  expect(f.data.graph).toEqual(previous);
  expect(f.history.route.historyQuery).toBe('description("fix")');
  f.query.dispose();
});

test("superseded queries, navigation and disconnect suppress late results", async () => {
  const f = fixture();
  let resolve!: (value: ReturnType<typeof f.result>) => void;
  f.graph.mockImplementationOnce(
    () =>
      new Promise((done) => {
        resolve = done;
      }),
  );
  f.query.setDraft("all()");
  const first = f.query.run();
  const signal = f.graph.mock.calls[0]?.[1]?.signal;
  f.query.setDraft("none()");
  await f.query.run();
  resolve(f.result());
  await first;
  expect(signal?.aborted).toBe(true);
  expect(f.history.route.historyQuery).toBe("none()");
  f.graph.mockImplementationOnce(
    () =>
      new Promise((done) => {
        resolve = done;
      }),
  );
  f.query.setDraft("all()");
  const late = f.query.run();
  f.history.page("oplog");
  resolve(f.result());
  await late;
  expect(f.history.route.historyQuery).toBeUndefined();
  f.query.dispose();
  f.data.status.connection = "disconnected";
  f.query.setDraft("description:x");
  await f.query.run();
  expect(f.graph).toHaveBeenCalledTimes(3);
  f.query.dispose();
});

test("operation text uses operation metadata transport and email stays out of routes", async () => {
  const f = fixture();
  f.history.page("oplog");
  f.query.setEmail("author@example.invalid");
  f.query.setDraft("from:@-- user:steve");
  await f.query.run();
  expect(f.graph).toHaveBeenCalledWith(
    "operations",
    expect.objectContaining({
      operationQuery: "from:@-- user:steve",
      revset: undefined,
      workspace: undefined,
    }),
  );
  expect(JSON.stringify(f.history.route)).not.toContain(
    "author@example.invalid",
  );
  f.query.dispose();
});
