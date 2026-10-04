import { expect, test, vi } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { createHistoryQuery } from "../../src/lib/controller/history-query.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";

const repository = "11".repeat(16),
  operation = "22".repeat(64);
function fixture(queryInfo?: StoreDataSource["queryInfo"]) {
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
    queryInfo,
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

test("metadata defaults are loaded before queries and cannot overwrite manual email", async () => {
  const metadata = {
    version: 1,
    jj_version: "0.45.1",
    max_query_bytes: 4096,
    callables: [],
    aliases: [],
    operators: [],
    operation_fields: [],
    unsupported: [],
    default_user_email: "host@example.invalid",
  };
  const source = vi.fn(async () => metadata);
  const f = fixture(source);
  f.query.setDraft("mine()");
  await f.query.run();
  expect(f.query.email).toBe("host@example.invalid");
  expect(f.graph).toHaveBeenCalledWith(
    "log",
    expect.objectContaining({ userEmail: "host@example.invalid" }),
  );
  f.query.setEmail("");
  await f.query.prepare();
  expect(f.query.email).toBe("");
  expect(source).toHaveBeenCalledTimes(1);
  f.query.dispose();
  let resolve!: (value: typeof metadata) => void;
  const delayed = fixture(
    () =>
      new Promise((done) => {
        resolve = done;
      }),
  );
  const pending = delayed.query.prepare();
  delayed.query.setEmail("manual@example.invalid");
  resolve(metadata);
  await pending;
  expect(delayed.query.email).toBe("manual@example.invalid");
  delayed.query.dispose();
});

test("failed or disconnected metadata can retry and disposed metadata cannot apply", async () => {
  const value = {
    version: 1,
    jj_version: "0.45.1",
    max_query_bytes: 4096,
    callables: [],
    aliases: [],
    operators: [],
    operation_fields: [],
    unsupported: [],
    default_user_email: "retry@example.invalid",
  };
  const reader = vi
    .fn()
    .mockRejectedValueOnce(new Error("offline"))
    .mockResolvedValue(value);
  const f = fixture(reader);
  await f.query.prepare();
  await f.query.prepare();
  expect(f.query.email).toBe("retry@example.invalid");
  expect(reader).toHaveBeenCalledTimes(2);
  f.query.dispose();
  let resolve!: (result: typeof value) => void;
  const delayed = fixture(
    () =>
      new Promise((done) => {
        resolve = done;
      }),
  );
  const pending = delayed.query.prepare();
  delayed.query.dispose();
  resolve(value);
  await pending;
  expect(delayed.query.email).toBe("");
});

test("disposing while identity metadata loads prevents the waiting query from running", async () => {
  let resolve!: (
    value: NonNullable<
      Awaited<ReturnType<NonNullable<StoreDataSource["queryInfo"]>>>
    >,
  ) => void;
  const f = fixture(
    () =>
      new Promise((done) => {
        resolve = done;
      }),
  );
  f.query.setDraft("mine()");
  const waiting = f.query.run();
  f.query.dispose();
  resolve({
    version: 1,
    jj_version: "0.45.1",
    max_query_bytes: 4096,
    callables: [],
    aliases: [],
    operators: [],
    operation_fields: [],
    unsupported: [],
    default_user_email: "host@example.invalid",
  });
  await waiting;
  expect(f.graph).not.toHaveBeenCalled();
});
