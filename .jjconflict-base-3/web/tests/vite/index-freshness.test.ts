import { afterEach, expect, test, vi } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { FreshnessJournal } from "../../src/lib/controller/freshness-journal.js";
import type {
  ApplicationFreshness,
  FreshnessNotice,
} from "../../src/lib/controller/application-freshness.js";
import { createIndexPreparation } from "../../src/lib/controller/production-index-preparation.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
const repository = "ab".repeat(16),
  operation = "cd".repeat(64);
function fixture(early = false) {
  vi.useFakeTimers();
  const journal = new FreshnessJournal(),
    listeners = new Set<(n: FreshnessNotice) => void>();
  let healthy = true;
  const notice = (id = "task") => {
    const n: FreshnessNotice = {
      status: healthy ? "live" : "offline",
      reset: false,
      changes: [
        { repository, domain: "index_tasks", witness: "settled", identity: id },
      ],
    };
    journal.record(n);
    for (const listener of listeners) listener(n);
  };
  const freshness = {
    journal,
    get healthy() {
      return healthy;
    },
    listen(fn: (n: FreshnessNotice) => void) {
      listeners.add(fn);
      return () => {
        listeners.delete(fn);
      };
    },
  } as unknown as ApplicationFreshness;
  const task = (status: string) => ({
    id: "task",
    snapshot: { operation_id: operation },
    status,
  });
  const indexTask = vi.fn(async () => task("ready"));
  const cancelIndexTask = vi.fn(async () => task("canceling"));
  const source = {
    repository,
    prepareIndex: vi.fn(async () => {
      if (early) notice();
      return task("running");
    }),
    indexTask,
    cancelIndexTask,
  } as unknown as StoreDataSource;
  const data = createProductionStoreState();
  data.capabilities = { index_tasks: true } as typeof data.capabilities;
  const restore = vi.fn(async () => {});
  const abort = new AbortController();
  const prepare = createIndexPreparation({
    data,
    source: () => source,
    operation: () => operation,
    version: () => 1,
    current: () => true,
    signal: () => abort.signal,
    restore,
    freshness,
  });
  return {
    data,
    prepare,
    indexTask,
    cancelIndexTask,
    restore,
    notice,
    abort,
    health(value: boolean) {
      healthy = value;
      for (const listener of listeners)
        listener({
          changes: [],
          reset: false,
          status: value ? "live" : "offline",
        });
    },
  };
}
afterEach(() => vi.useRealTimers());
test("completion before start response is observed at the exact retained operation", async () => {
  const f = fixture(true);
  await f.prepare.prepareIndex();
  expect(f.indexTask).toHaveBeenCalledOnce();
  expect(f.restore).toHaveBeenCalledOnce();
  expect(f.data.indexPreparation).toBe("ready");
});
test("healthy preparation ignores foreign tasks and performs no periodic status reads", async () => {
  const f = fixture();
  const pending = f.prepare.prepareIndex();
  await vi.advanceTimersByTimeAsync(1);
  f.notice("foreign");
  await vi.advanceTimersByTimeAsync(60000);
  expect(f.indexTask).not.toHaveBeenCalled();
  f.notice();
  await pending;
  expect(f.indexTask).toHaveBeenCalledOnce();
});
test("cancel wakes the exact task and expiry is an explicit failed observation", async () => {
  const f = fixture();
  const pending = f.prepare.prepareIndex();
  await vi.advanceTimersByTimeAsync(1);
  await f.prepare.cancelIndex();
  await vi.advanceTimersByTimeAsync(1);
  expect(f.cancelIndexTask).toHaveBeenCalledWith(
    "task",
    expect.any(AbortSignal),
  );
  f.indexTask.mockRejectedValueOnce(new ApiError("index_task_expired"));
  f.notice();
  await pending;
  expect(f.data.indexPreparation).toBe("failed");
  expect(f.data.indexPreparationError).toContain("expired");
  expect(f.restore).not.toHaveBeenCalled();
});
test("outage enables status fallback and recovery cancels it until a settlement event", async () => {
  const f = fixture();
  f.indexTask.mockResolvedValueOnce({
    id: "task",
    snapshot: { operation_id: operation },
    status: "running",
  });
  const pending = f.prepare.prepareIndex();
  await vi.advanceTimersByTimeAsync(1);
  f.health(false);
  await vi.advanceTimersByTimeAsync(500);
  expect(f.indexTask).toHaveBeenCalledOnce();
  f.health(true);
  await vi.advanceTimersByTimeAsync(60000);
  expect(f.indexTask).toHaveBeenCalledOnce();
  f.notice();
  await pending;
  expect(f.indexTask).toHaveBeenCalledTimes(2);
});
