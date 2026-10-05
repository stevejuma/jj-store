import { expect, test } from "vitest";
import { createProductionFileReader } from "../../src/lib/controller/production-file-reader.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";

test("quick path metadata does not wait for the optional commit header or landing documents", async () => {
  const repository = "11".repeat(16),
    operation = "22".repeat(64),
    commit = "33".repeat(64);
  const data = createProductionStoreState(),
    history = createStoreRouteHistory(undefined);
  const blocked = Promise.withResolvers<never>();
  // The prioritized quick read can complete before either optional job starts.
  void blocked.promise.catch(() => {});
  const scope = new AbortController();
  const signature = {
    name: "A",
    email: "a@example.test",
    timestamp: { millis: 1, offset_minutes: 0 },
  };
  const source = {
    repository,
    async paths(
      _commit: string,
      _operation: string,
      query: { includeLastChange: boolean },
    ) {
      return {
        snapshot: {
          version: 1,
          repo_id: repository,
          epoch: "44".repeat(16),
          published_seq: 1,
          manifest_revision: 1,
          code_state_version: 1,
          operation_id: operation,
          heads: [operation],
        },
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
                  id: "55".repeat(64),
                  executable: false,
                  copy: { kind: "placeholder" },
                },
              ],
              conflicted: false,
              last_change: query.includeLastChange
                ? {
                    status: "available",
                    attribution: "snapshot",
                    commit,
                    change: "66".repeat(16),
                    description: "Quick details",
                    author: signature,
                    committer: signature,
                  }
                : { status: "unavailable", reason: "not_requested" },
            },
          ],
          previous: null,
          next: null,
          stats: {},
        },
        next_cursor: null,
        previous_cursor: null,
      };
    },
    commit: () => blocked.promise,
    file: () => blocked.promise,
  } as unknown as StoreDataSource;
  const reader = createProductionFileReader({
    data,
    history,
    source: () => source,
    operation: () => operation,
    version: () => 1,
    signal: () => scope.signal,
    current: () => true,
    report: () => {},
  });
  try {
    await reader.loadFiles(commit);
    await expect
      .poll(() => data.snapshot?.pathChanges?.["README.md"]?.message, {
        timeout: 500,
      })
      .toBe("Quick details");
    expect(data.status.files).toBe("ready");
    expect(data.snapshot?.files[0]?.contents).toBeUndefined();
  } finally {
    scope.abort();
    blocked.reject(new Error("Canceled optional fixture read"));
  }
});
