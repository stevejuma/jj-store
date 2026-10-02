import { expect, test, vi } from "vitest";
import { createProductionRefresh } from "../../src/lib/controller/production-store-refresh.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";
import type { Head } from "../../src/lib/api/client.js";
const old = "11".repeat(64),
  next = "22".repeat(64),
  historical = "33".repeat(64);
const head: Head = {
  version: 1,
  repo_id: "44".repeat(16),
  epoch: "55".repeat(16),
  published_seq: 1,
  manifest_revision: 1,
  code_state_version: 1,
  heads: [old],
};
function fixture() {
  const data = createProductionStoreState(),
    history = createStoreRouteHistory();
  data.head = head;
  history.connect(head.repo_id, old);
  const source = {
    snapshot: vi.fn(async () => ({ ...head, heads: [next], published_seq: 2 })),
  };
  const restore = vi.fn(async () => {}),
    query = vi.fn(async () => {});
  const refresh = createProductionRefresh({
    data,
    history,
    source: () => source,
    threads: { refresh: async () => false },
    projects: {},
    reader: { restorePage: restore },
    search: { clear: vi.fn(), run: query },
    stopFeed: vi.fn(),
    follow: vi.fn(),
    invalidate: vi.fn(),
  } as unknown as Parameters<typeof createProductionRefresh>[0]);
  return { data, history, source, restore, query, refresh };
}
test("current-head views advance automatically and retain commit selections and search", async () => {
  const f = fixture();
  f.history.replace({ ...f.history.route, commitId: historical });
  f.data.searchQuery = "retained query";
  await f.refresh.refresh(true);
  expect(f.history.route.snapshotOperationId).toBe(next);
  expect(f.history.route.commitId).toBe(historical);
  expect(f.query).toHaveBeenCalledWith("retained query");
  expect(f.restore).toHaveBeenCalledTimes(1);
});
test("historical code pins stay fixed while fresh collaboration roots reload", async () => {
  const f = fixture();
  f.history.replace({
    ...f.history.route,
    snapshotOperationId: historical,
    page: "reviews",
  });
  await f.refresh.refresh(true);
  expect(f.history.route.snapshotOperationId).toBe(historical);
  expect(f.data.head?.published_seq).toBe(2);
  expect(f.restore).toHaveBeenCalledTimes(1);
});
test("a late head cannot replace a newer page selection", async () => {
  const f = fixture();
  let resolve!: (value: Head) => void;
  f.source.snapshot.mockImplementation(
    () =>
      new Promise((done) => {
        resolve = done;
      }),
  );
  const updating = f.refresh.refresh(true);
  await Promise.resolve();
  f.history.page("files");
  resolve({ ...head, heads: [next], published_seq: 2 });
  await updating;
  expect(f.history.route.page).toBe("files");
  expect(f.history.route.snapshotOperationId).toBe(old);
  expect(f.restore).not.toHaveBeenCalled();
});
