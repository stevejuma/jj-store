import { afterEach, expect, test, vi } from "vitest";
import { createProductionStoreSettings } from "../../src/lib/controller/production-store-index.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import type { StoreRoute } from "../../src/lib/controller/contracts.js";
const repository = "ab".repeat(16),
  operation = "cd".repeat(64);
function fixture() {
  const data = createProductionStoreState();
  data.capabilities = {
    index_tasks: true,
    automatic_code_index: true,
  } as typeof data.capabilities;
  data.neededIndex = {
    repo_id: repository,
    operation_id: operation,
    epoch: "ef".repeat(16),
  } as typeof data.neededIndex;
  const task = (status: string) => ({
    version: 1,
    id: "12".repeat(16),
    snapshot: data.neededIndex,
    status,
  });
  const ready = task("ready");
  const source = {
    repository,
    prepareCodeIndex: vi.fn(async () => ready),
    prepareIndex: vi.fn(),
    indexTask: vi.fn(async () => ready),
    cancelIndexTask: vi.fn(),
  } as unknown as StoreDataSource;
  const restore = vi.fn(async () => {});
  let ticket = 1;
  const route = {
    page: "files",
    snapshotOperationId: operation,
    repository,
  } as StoreRoute;
  const settings = createProductionStoreSettings({
    data,
    source: () => source,
    version: () => ticket,
    current: (t) => t === ticket,
    route: () => route,
    restore,
  });
  return {
    data,
    source,
    restore,
    settings,
    route,
    task,
    navigate() {
      ticket++;
    },
  };
}
afterEach(() => vi.useRealTimers());
test("managed history recovery preserves the exact selection and does not prepare advanced inventories", async () => {
  for (const page of ["files", "branches", "log"] as const) {
    const f = fixture();
    f.route.page = page;
    f.settings.recoverCodeIndex();
    await vi.waitFor(() => expect(f.restore).toHaveBeenCalledTimes(1));
    expect(f.source.prepareCodeIndex).toHaveBeenCalledWith(
      operation,
      expect.any(AbortSignal),
    );
    expect(f.source.prepareIndex).not.toHaveBeenCalled();
    expect(f.data.neededIndex).toBeNull();
  }
});
test("failed managed recovery requires a new episode or explicit retry", async () => {
  const f = fixture();
  vi.mocked(f.source.prepareCodeIndex!).mockResolvedValue(
    f.task("failed") as never,
  );
  f.settings.recoverCodeIndex();
  await vi.waitFor(() => expect(f.data.indexPreparation).toBe("failed"));
  f.settings.recoverCodeIndex();
  expect(f.source.prepareCodeIndex).toHaveBeenCalledTimes(1);
  await f.settings.prepareIndex();
  expect(f.source.prepareCodeIndex).toHaveBeenCalledTimes(2);
  expect(f.restore).not.toHaveBeenCalled();
});
test("automatic maintenance defaults off and advanced operation views stay explicit", async () => {
  const f = fixture();
  f.data.capabilities = { index_tasks: true } as typeof f.data.capabilities;
  f.settings.recoverCodeIndex();
  expect(f.source.prepareCodeIndex).not.toHaveBeenCalled();
  f.data.capabilities = {
    index_tasks: true,
    automatic_code_index: true,
  } as typeof f.data.capabilities;
  f.route.page = "oplog";
  f.settings.recoverCodeIndex();
  expect(f.source.prepareCodeIndex).not.toHaveBeenCalled();
});
test("navigation aborts obsolete readiness without restoring another view", async () => {
  vi.useFakeTimers();
  const f = fixture();
  vi.mocked(f.source.prepareCodeIndex!).mockResolvedValue(
    f.task("running") as never,
  );
  f.settings.recoverCodeIndex();
  await vi.advanceTimersByTimeAsync(0);
  f.navigate();
  await vi.advanceTimersByTimeAsync(500);
  expect(f.restore).not.toHaveBeenCalled();
  expect(f.source.indexTask).not.toHaveBeenCalled();
  expect(vi.getTimerCount()).toBe(0);
});
test("explicit advanced preparation retains its existing task route", async () => {
  const f = fixture();
  f.route.page = "oplog";
  vi.mocked(f.source.prepareIndex).mockResolvedValue(f.task("ready") as never);
  await f.settings.prepareIndex();
  expect(f.source.prepareIndex).toHaveBeenCalledWith(
    operation,
    expect.any(AbortSignal),
  );
  expect(f.source.prepareCodeIndex).not.toHaveBeenCalled();
});
