import { expect, test, vi } from "vitest";
import type { Model, Task } from "../../src/lib/api/client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProductionStoreSettings } from "../../src/lib/controller/production-store-index.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";

const repository = "ab".repeat(16);
const operation = "cd".repeat(64);
const capabilities = (search: boolean): Model<"Capabilities"> =>
  ({ search, index_tasks: true }) as Model<"Capabilities">;
const status = (): Model<"SearchStatus"> => ({
  version: 1,
  repository,
  epoch: "ef".repeat(16),
  repository_position: 8,
  durable_position: 8,
  indexed_position: 8,
  index_generation: "12".repeat(32),
  is_stale: false,
  degraded: false,
});
const task = (selected: string, state: "running" | "ready") =>
  ({
    version: 1,
    id: "task-1",
    snapshot: { operation_id: selected },
    status: state,
    index: state === "ready" ? {} : null,
  }) as unknown as Task;

test("settings refresh loads advertised search health and clears it on disconnect", async () => {
  const data = createProductionStoreState();
  data.capabilities = capabilities(true);
  const searchStatus = vi.fn(async () => status());
  const source = { searchStatus } as unknown as StoreDataSource;
  const settings = createProductionStoreSettings({
    data,
    source: () => source,
    operation: () => operation,
    version: () => 1,
    current: () => true,
    restore: async () => {},
  });

  await settings.refresh();
  expect(searchStatus).toHaveBeenCalledWith(true);
  expect(data.searchStatusState).toBe("ready");
  expect(data.searchStatus?.indexed_position).toBe(8);

  data.neededIndex = { operation_id: operation } as NonNullable<
    typeof data.neededIndex
  >;
  settings.clear();
  expect(data.capabilities).toBeNull();
  expect(data.neededIndex).toBeNull();
  expect(data.searchStatus).toBeNull();
  expect(data.searchStatusState).toBe("idle");
});

test("settings skip unavailable search and prepare the selected operation", async () => {
  const data = createProductionStoreState();
  const prepareIndex = vi.fn(async (selected: string) =>
    task(selected, "running"),
  );
  const indexTask = vi.fn(async () => task(operation, "ready"));
  const searchStatus = vi.fn(async () => status());
  const restore = vi.fn(async () => {});
  const source = {
    prepareIndex,
    indexTask,
    searchStatus,
  } as unknown as StoreDataSource;
  const settings = createProductionStoreSettings({
    data,
    source: () => source,
    operation: () => operation,
    version: () => 2,
    current: () => true,
    restore,
  });

  data.capabilities = capabilities(false);
  await settings.refresh();
  expect(data.searchStatusState).toBe("unavailable");
  expect(searchStatus).not.toHaveBeenCalled();

  data.capabilities = capabilities(true);
  await settings.prepareIndex();
  expect(prepareIndex).toHaveBeenCalledWith(operation);
  expect(indexTask).toHaveBeenCalledWith("task-1");
  expect(restore).toHaveBeenCalledOnce();
  expect(searchStatus).not.toHaveBeenCalled();
  expect(data.indexPreparation).toBe("ready");
});

test("settings prefer the operation reported by an index-lag response", async () => {
  const lagOperation = "34".repeat(64);
  const data = createProductionStoreState();
  data.capabilities = capabilities(true);
  data.neededIndex = { operation_id: lagOperation } as NonNullable<
    typeof data.neededIndex
  >;
  const prepareIndex = vi.fn(async (selected: string) =>
    task(selected, "ready"),
  );
  const source = {
    prepareIndex,
    searchStatus: vi.fn(async () => status()),
  } as unknown as StoreDataSource;
  const settings = createProductionStoreSettings({
    data,
    source: () => source,
    operation: () => operation,
    version: () => 3,
    current: () => true,
    restore: async () => {},
  });

  await settings.prepareIndex();
  expect(prepareIndex).toHaveBeenCalledWith(lagOperation);
  expect(data.neededIndex).toBeNull();
});

test("settings do not prepare an index without an exact operation", async () => {
  const data = createProductionStoreState();
  data.capabilities = capabilities(true);
  const prepareIndex = vi.fn();
  const source = { prepareIndex } as unknown as StoreDataSource;
  const settings = createProductionStoreSettings({
    data,
    source: () => source,
    operation: () => null,
    version: () => 4,
    current: () => true,
    restore: async () => {},
  });

  await settings.prepareIndex();
  expect(prepareIndex).not.toHaveBeenCalled();
  expect(data.indexPreparation).toBe("idle");
});

test("graph preparation works without text search and preserves search health", async () => {
  const data = createProductionStoreState();
  data.capabilities = capabilities(false);
  const prepareIndex = vi.fn(async (selected: string) =>
    task(selected, "ready"),
  );
  const searchStatus = vi.fn(async () => status());
  const restore = vi.fn(async () => {});
  const source = { prepareIndex, searchStatus } as unknown as StoreDataSource;
  const settings = createProductionStoreSettings({
    data,
    source: () => source,
    operation: () => operation,
    version: () => 5,
    current: () => true,
    restore,
  });

  await settings.prepareIndex();
  expect(prepareIndex).toHaveBeenCalledWith(operation);
  expect(restore).toHaveBeenCalledOnce();
  expect(data.indexPreparation).toBe("ready");
  expect(searchStatus).not.toHaveBeenCalled();

  data.capabilities = capabilities(true);
  data.searchStatus = { ...status(), is_stale: true, degraded: true };
  data.searchStatusState = "ready";
  await settings.prepareIndex();
  expect(searchStatus).not.toHaveBeenCalled();
  expect(data.searchStatus).toMatchObject({ is_stale: true, degraded: true });
});

test("graph preparation requires the native index-task capability", async () => {
  const data = createProductionStoreState();
  data.capabilities = { ...capabilities(true), index_tasks: false };
  const prepareIndex = vi.fn();
  const source = { prepareIndex } as unknown as StoreDataSource;
  const settings = createProductionStoreSettings({
    data,
    source: () => source,
    operation: () => operation,
    version: () => 6,
    current: () => true,
    restore: async () => {},
  });

  await settings.prepareIndex();
  expect(prepareIndex).not.toHaveBeenCalled();
  expect(data.indexPreparation).toBe("idle");
});
