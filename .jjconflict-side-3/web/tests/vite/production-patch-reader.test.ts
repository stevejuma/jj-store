import { expect, test, vi } from "vitest";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProductionStoreReader } from "../../src/lib/controller/production-store-reader.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";

const repository = "ab".repeat(16),
  operation = "cd".repeat(64);
const before = "12".repeat(64),
  after = "34".repeat(64);
const snapshot = {
  version: 1 as const,
  repo_id: repository,
  epoch: "epoch",
  published_seq: 1,
  manifest_revision: 1,
  code_state_version: 1,
  operation_id: operation,
  heads: [operation],
};

function harness() {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  data.detail = { files: [], patches: {} };
  data.comparisons.set("long.c", { from: before, to: after });
  let resolve!: (value: Awaited<ReturnType<StoreDataSource["diff"]>>) => void;
  let reject!: (reason: Error) => void;
  const diff = vi.fn<StoreDataSource["diff"]>().mockImplementation(
    () =>
      new Promise((ok, fail) => {
        resolve = ok;
        reject = fail;
      }),
  );
  const source = { repository, diff } as unknown as StoreDataSource;
  let version = 1;
  const report = vi.fn();
  const reader = createProductionStoreReader({
    data,
    history,
    report,
    source: () => source,
    operation: () => operation,
    version: () => version,
    current: (ticket) => ticket === version,
  });
  return {
    data,
    reader,
    report,
    diff,
    resolve: () =>
      resolve({
        snapshot,
        data: {
          version: 1,
          from: before,
          to: after,
          path: "long.c",
          stats: {
            visited_objects: 1,
            visited_bytes: 1,
            elapsed_millis: 1,
            cache_hits: 0,
          },
          changes: [
            {
              path: "long.c",
              before: [],
              after: [],
              sources: [],
              content: { status: "unavailable", reason: "complexity" },
            },
          ],
        },
      }),
    reject: () => reject(new Error("request failed")),
    invalidate: () => {
      version += 1;
    },
  };
}

test("selected patch stays pending until its typed unavailable result arrives", async () => {
  const h = harness();
  h.data.detail = { files: [], patches: { "long.c": "obsolete patch" } };
  const loading = h.reader.loadPatch("long.c");
  expect(h.data.detail?.pendingPatch).toBe("long.c");
  h.resolve();
  await loading;
  expect(h.data.detail?.pendingPatch).toBeUndefined();
  expect(h.data.detail?.patchUnavailableReasons?.["long.c"]).toBe("complexity");
  expect(h.data.detail?.patches["long.c"]).toBeUndefined();
  expect(h.report).not.toHaveBeenCalled();
});

test("obsolete patch responses cannot replace a new selection", async () => {
  const h = harness();
  const loading = h.reader.loadPatch("long.c");
  h.invalidate();
  h.data.detail = { files: [], patches: {} };
  h.resolve();
  await loading;
  expect(h.data.detail).toEqual({ files: [], patches: {} });
});

test("failed patch reads clear pending state and report through the request service", async () => {
  const h = harness();
  const loading = h.reader.loadPatch("long.c");
  h.reject();
  await loading;
  expect(h.data.detail?.pendingPatch).toBeUndefined();
  expect(h.report).toHaveBeenCalledOnce();
});
