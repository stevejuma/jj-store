import { FreshnessJournal } from "../../src/lib/controller/freshness-journal.js";
import type { ApplicationFreshness } from "../../src/lib/controller/application-freshness.js";
import { afterEach, expect, test, vi } from "vitest";
import type { Model } from "../../src/lib/api/client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProductionStoreSettings } from "../../src/lib/controller/production-store-index.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
const repository = "ab".repeat(16),
  operation = "cd".repeat(64),
  later = "ef".repeat(64);
type Kind = "operations" | "copies";
const task = (
  kind: Kind,
  status: "running" | "ready" | "canceled" = "running",
  windows = 1,
) =>
  ({
    version: 1,
    id: "kind-task",
    kind,
    status,
    snapshot: { repo_id: repository, operation_id: operation },
    progress: {
      complete: status === "ready",
      matching_ids: windows,
      stats: {
        windows,
        scanned: windows * 256,
        descriptor_work: windows * 256,
        record_reads: windows * 1024,
        record_bytes: windows * 1024,
      },
    },
  }) as Model<"KindPreparationTask">;
function fixture() {
  vi.useFakeTimers();
  const data = createProductionStoreState();
  data.capabilities = { index_tasks: true } as typeof data.capabilities;
  data.neededIndex = { operation_id: operation } as NonNullable<
    typeof data.neededIndex
  >;
  const prepareKindIndex = vi.fn(async (_operation: string, kind: Kind) =>
    task(kind),
  );
  const kindIndexTask = vi.fn(async (_id: string, _signal?: AbortSignal) =>
    task("operations", "ready", 2),
  );
  const cancelKindIndexTask = vi.fn(async () =>
    task("operations", "canceled", 2),
  );
  const prepareIndex = vi.fn();
  const source = {
    repository,
    profile: "native",
    prepareKindIndex,
    kindIndexTask,
    cancelKindIndexTask,
    prepareIndex,
  } as unknown as StoreDataSource;
  const abort = new AbortController();
  const restore = vi.fn(async () => {});
  const settings = createProductionStoreSettings({
    data,
    source: () => source,
    operation: () => later,
    version: () => 1,
    current: () => true,
    signal: () => abort.signal,
    restore,
    freshness: {
      journal: new FreshnessJournal(),
      healthy: true,
      listen: () => () => {},
    } as unknown as ApplicationFreshness,
  });
  const prepare = (kind: Kind) =>
    (
      settings as unknown as { prepareKindIndex(kind: Kind): Promise<void> }
    ).prepareKindIndex(kind);
  return {
    data,
    source,
    prepare,
    settings,
    prepareKindIndex,
    kindIndexTask,
    cancelKindIndexTask,
    prepareIndex,
    abort,
    restore,
  };
}
afterEach(() => vi.useRealTimers());
test("native kind preparation retains the original operation and restores only after complete progress", async () => {
  const f = fixture();
  const pending = f.prepare("operations");
  await vi.advanceTimersByTimeAsync(1);
  expect(f.prepareKindIndex).toHaveBeenCalledWith(
    operation,
    "operations",
    expect.any(AbortSignal),
  );
  expect(f.prepareIndex).not.toHaveBeenCalled();
  expect(f.kindIndexTask).not.toHaveBeenCalled();
  await vi.advanceTimersByTimeAsync(500);
  await pending;
  expect(f.kindIndexTask).toHaveBeenCalledWith(
    "kind-task",
    expect.any(AbortSignal),
  );
  expect(f.restore).toHaveBeenCalledOnce();
  expect(f.data.neededIndex).toBeNull();
  expect(f.data.indexPreparation).toBe("ready");
  expect(
    (f.data as unknown as { indexPreparationKind: Kind }).indexPreparationKind,
  ).toBe("operations");
  expect(
    (f.data as unknown as { indexProgress: Model<"KindPreparationProgress"> })
      .indexProgress.matching_ids,
  ).toBe(2);
});

test("copy preparation uses only copy task routes and cannot overlap graph preparation", async () => {
  const f = fixture();
  f.prepareKindIndex.mockImplementationOnce(async () => task("copies"));
  f.kindIndexTask.mockResolvedValueOnce(task("copies", "ready", 2));
  const pending = f.prepare("copies");
  await vi.advanceTimersByTimeAsync(1);
  await f.settings.prepareIndex();
  await f.prepare("operations");
  expect(f.prepareKindIndex).toHaveBeenCalledTimes(1);
  expect(f.prepareIndex).not.toHaveBeenCalled();
  await vi.advanceTimersByTimeAsync(500);
  await pending;
  expect(f.restore).toHaveBeenCalledOnce();
  expect(f.data.indexPreparationKind).toBe("copies");
});
test.each(["id", "operation", "repository", "kind", "epoch"])(
  "kind task rejects changed %s before restoring",
  async (field) => {
    const f = fixture();
    let bad = task("operations", "ready", 2);
    if (field === "id") bad = { ...bad, id: "foreign" };
    if (field === "kind") bad = { ...bad, kind: "copies" };
    if (field === "operation")
      bad = { ...bad, snapshot: { ...bad.snapshot, operation_id: later } };
    if (field === "repository")
      bad = { ...bad, snapshot: { ...bad.snapshot, repo_id: "11".repeat(16) } };
    if (field === "epoch")
      bad = { ...bad, snapshot: { ...bad.snapshot, epoch: "22".repeat(16) } };
    f.kindIndexTask.mockResolvedValueOnce(bad);
    const pending = f.prepare("operations");
    await vi.advanceTimersByTimeAsync(500);
    await pending;
    expect(f.data.indexPreparation).toBe("failed");
    expect(f.restore).not.toHaveBeenCalled();
    expect(f.data.neededIndex).not.toBeNull();
  },
);
test.each(["incomplete", "negative", "missing", "regressing"])(
  "kind task rejects %s progress",
  async (mode) => {
    const f = fixture();
    let bad = task("operations", "ready", 2);
    if (mode === "incomplete") bad.progress.complete = false;
    if (mode === "negative") bad.progress.stats.descriptor_work = -1;
    if (mode === "missing")
      delete (bad.progress.stats as Partial<typeof bad.progress.stats>)
        .record_reads;
    if (mode === "regressing") bad = task("operations", "ready", 0);
    f.kindIndexTask.mockResolvedValueOnce(bad);
    const pending = f.prepare("operations");
    await vi.advanceTimersByTimeAsync(500);
    await pending;
    expect(f.data.indexPreparation).toBe("failed");
    expect(f.restore).not.toHaveBeenCalled();
  },
);
test("kind status observes bounded progress even with healthy settlement events", async () => {
  const f = fixture();
  f.kindIndexTask.mockResolvedValue(task("operations", "running", 2));
  const pending = f.prepare("operations");
  await vi.advanceTimersByTimeAsync(60000);
  expect(f.kindIndexTask.mock.calls.length).toBeLessThanOrEqual(16);
  expect(f.kindIndexTask.mock.calls.length).toBeGreaterThan(1);
  expect(f.data.indexProgress?.matching_ids).toBe(2);
  await f.settings.cancelIndex();
  await vi.advanceTimersByTimeAsync(1);
  await pending;
  expect(f.cancelKindIndexTask).toHaveBeenCalledOnce();
  expect(f.data.indexTaskStatus).toBe("canceled");
  expect(f.restore).not.toHaveBeenCalled();
});
test("kind cancellation remains retryable when confirmation fails", async () => {
  const f = fixture();
  f.cancelKindIndexTask.mockRejectedValueOnce(new Error("network"));
  const pending = f.prepare("operations");
  await vi.advanceTimersByTimeAsync(1);
  await f.settings.cancelIndex();
  await vi.advanceTimersByTimeAsync(1);
  expect(f.data.indexCancellationUnconfirmed).toBe(true);
  expect(f.data.indexTaskStatus).toBe("running");
  expect(f.data.indexPreparationError).toContain("Cancellation is unconfirmed");
  await f.settings.cancelIndex();
  await vi.advanceTimersByTimeAsync(1);
  await pending;
  expect(f.cancelKindIndexTask).toHaveBeenCalledTimes(2);
  expect(f.data.indexCancellationUnconfirmed).toBe(false);
  expect(f.data.indexTaskStatus).toBe("canceled");
  expect(f.restore).not.toHaveBeenCalled();
});
test("one in-flight kind status read suppresses later requests and stale output after navigation", async () => {
  const f = fixture();
  let finish!: (value: Model<"KindPreparationTask">) => void;
  f.kindIndexTask.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        finish = resolve;
      }),
  );
  const pending = f.prepare("operations");
  await vi.advanceTimersByTimeAsync(500);
  await vi.advanceTimersByTimeAsync(60000);
  expect(f.kindIndexTask).toHaveBeenCalledOnce();
  const signal = f.kindIndexTask.mock.calls[0]![1];
  f.abort.abort();
  expect(signal?.aborted).toBe(true);
  finish(task("operations", "ready", 2));
  await pending;
  expect(f.data.indexPreparation).toBe("idle");
  expect(f.data.indexProgress).toBeNull();
  expect(f.restore).not.toHaveBeenCalled();
});
test("native kind actions do no work for local or unsupported sources", async () => {
  const f = fixture();
  Object.assign(f.source, { profile: "git_sha1_jj045" });
  await f.prepare("operations");
  expect(f.prepareKindIndex).not.toHaveBeenCalled();
  expect(f.data.indexPreparation).toBe("idle");
  Object.assign(f.source, { profile: "native", kindIndexTask: undefined });
  await f.prepare("copies");
  expect(f.prepareKindIndex).not.toHaveBeenCalled();
  expect(f.prepareIndex).not.toHaveBeenCalled();
});
