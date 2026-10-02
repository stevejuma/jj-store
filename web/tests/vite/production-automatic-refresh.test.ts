import { afterEach, expect, test, vi } from "vitest";
import { createProductionAutomaticRefresh } from "../../src/lib/controller/production-automatic-refresh.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import type { Head } from "../../src/lib/api/client.js";
afterEach(() => vi.useRealTimers());
function fixture() {
  vi.useFakeTimers();
  const abort = new AbortController(),
    data = createProductionStoreState();
  const head: Head = {
    version: 1,
    repo_id: "11".repeat(16),
    epoch: "22".repeat(16),
    published_seq: 1,
    manifest_revision: 1,
    code_state_version: 1,
    heads: ["33".repeat(64)],
  };
  data.head = head;
  data.status.connection = "connected";
  const source = {
    snapshot: vi.fn(async () => ({ ...head, published_seq: 2 })),
  };
  const navigation = vi.fn(async () => {}),
    refresh = vi.fn(async () => {}),
    directories = vi.fn(async () => false);
  const options = {
    data,
    route: () => ({ ...initialStoreRoute(), repository: head.repo_id }),
    source: () => source,
    active: () => true,
    signal: abort.signal,
    refresh,
    pollProjects: directories,
    report: vi.fn(),
    projects: {
      loading: false,
      error: "",
      homes: [],
      reviews: { homes: [] },
      activity: { homes: [] },
      repositories: { entries: [] },
      navigation: { load: navigation },
    },
    threads: { busy: false, status: "ready" },
  } as unknown as Parameters<typeof createProductionAutomaticRefresh>[0];
  return {
    abort,
    data,
    source,
    refresh,
    directories,
    navigation,
    updates: createProductionAutomaticRefresh(options),
  };
}
test("a missed repository publication reloads without an SSE or a manual action", async () => {
  const f = fixture();
  await vi.advanceTimersByTimeAsync(5000);
  expect(f.refresh).toHaveBeenCalledTimes(1);
  f.abort.abort();
});
test("directory changes refresh sidebar metadata even in a code view", async () => {
  const f = fixture();
  f.source.snapshot.mockResolvedValue(f.data.head!);
  f.directories.mockResolvedValue(true);
  await vi.advanceTimersByTimeAsync(5000);
  expect(f.navigation).toHaveBeenCalledWith(true);
  expect(f.refresh).toHaveBeenCalledTimes(1);
  f.abort.abort();
});
test("a prepared code mutation defers updates until its exact preview is released", async () => {
  const f = fixture();
  f.data.mutation.status = "prepared";
  f.updates.request();
  await vi.advanceTimersByTimeAsync(150);
  expect(f.refresh).not.toHaveBeenCalled();
  f.data.mutation.status = "idle";
  await vi.advanceTimersByTimeAsync(5000);
  expect(f.refresh).toHaveBeenCalledTimes(1);
  f.abort.abort();
});
