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
  expect(prepareIndex).toHaveBeenCalledWith(operation, expect.any(AbortSignal));
  expect(indexTask).toHaveBeenCalledWith("task-1", expect.any(AbortSignal));
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
  expect(prepareIndex).toHaveBeenCalledWith(
    lagOperation,
    expect.any(AbortSignal),
  );
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
  expect(prepareIndex).toHaveBeenCalledWith(operation, expect.any(AbortSignal));
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

test("preparation stops before polling after navigation during its delay", async () => {
  vi.useFakeTimers();
  try {
    const data = createProductionStoreState();
    data.capabilities = capabilities(false);
    let current = true;
    const indexTask = vi.fn(async () => task(operation, "ready"));
    const source = {
      prepareIndex: vi.fn(async () => task(operation, "running")),
      indexTask,
    } as unknown as StoreDataSource;
    const restore = vi.fn(async () => {});
    const settings = createProductionStoreSettings({
      data,
      source: () => source,
      operation: () => operation,
      version: () => 1,
      current: () => current,
      restore,
    });
    const pending = settings.prepareIndex();
    await vi.advanceTimersByTimeAsync(1);
    current = false;
    await vi.advanceTimersByTimeAsync(5000);
    await pending;
    expect(indexTask).not.toHaveBeenCalled();
    expect(restore).not.toHaveBeenCalled();
    expect(data.indexPreparation).toBe("idle");
  } finally {
    vi.useRealTimers();
  }
});

test("preparation cancels only its original task and waits for terminal status", async () => {
  vi.useFakeTimers();
  try {
    const data = createProductionStoreState();
    data.capabilities = capabilities(false);
    let selected = operation;
    const cancelIndexTask = vi.fn(async (_id: string) => ({
      ...task(operation, "running"),
      status: "canceling",
    }));
    const indexTask = vi.fn(async () => ({
      ...task(operation, "running"),
      status: "canceled",
    }));
    const prepareIndex = vi.fn(async () => task(operation, "running"));
    const source = {
      prepareIndex,
      indexTask,
      cancelIndexTask,
    } as unknown as StoreDataSource;
    const restore = vi.fn(async () => {});
    const settings = createProductionStoreSettings({
      data,
      source: () => source,
      operation: () => selected,
      version: () => 1,
      current: () => true,
      restore,
    });
    const pending = settings.prepareIndex();
    await vi.advanceTimersByTimeAsync(1);
    selected = "ef".repeat(64);
    await settings.cancelIndex();
    await settings.cancelIndex();
    await settings.prepareIndex();
    await vi.advanceTimersByTimeAsync(1);
    expect(cancelIndexTask).toHaveBeenCalledTimes(1);
    expect(cancelIndexTask.mock.calls[0]?.[0]).toBe("task-1");
    expect(data.indexPreparation).toBe("loading");
    expect(data.indexTaskStatus).toBe("canceling");
    await vi.advanceTimersByTimeAsync(5000);
    await pending;
    expect(data.indexTaskStatus).toBe("canceled");
    expect(data.indexPreparation).toBe("idle");
    expect(data.indexPreparationError).toBe("");
    expect(prepareIndex).toHaveBeenCalledTimes(1);
    expect(restore).not.toHaveBeenCalled();
  } finally {
    vi.useRealTimers();
  }
});

test("clearing preparation aborts a pending observation and releases its timer", async () => {
  vi.useFakeTimers();
  try {
    const data = createProductionStoreState();
    data.capabilities = capabilities(false);
    let pendingSignal: AbortSignal | undefined;
    const indexTask = vi.fn(
      (_id: string, signal: AbortSignal) =>
        new Promise<Task>((_resolve, reject) => {
          pendingSignal = signal;
          signal.addEventListener(
            "abort",
            () => reject(new DOMException("Aborted", "AbortError")),
            { once: true },
          );
        }),
    );
    const source = {
      prepareIndex: async () => task(operation, "running"),
      indexTask,
    } as unknown as StoreDataSource;
    const settings = createProductionStoreSettings({
      data,
      source: () => source,
      operation: () => operation,
      version: () => 1,
      current: () => true,
      restore: async () => {},
    });
    const pending = settings.prepareIndex();
    await vi.advanceTimersByTimeAsync(500);
    expect(indexTask).toHaveBeenCalledTimes(1);
    settings.clear();
    await pending;
    expect(pendingSignal?.aborted).toBe(true);
    expect(vi.getTimerCount()).toBe(0);
    expect(data.indexPreparation).toBe("idle");
    expect(data.indexTaskStatus).toBeNull();
  } finally {
    vi.useRealTimers();
  }
});

test("cancel transport failure remains unconfirmed and polls without replaying cancel", async () => {
  vi.useFakeTimers();
  try {
    const data = createProductionStoreState();
    data.capabilities = capabilities(false);
    const cancelIndexTask = vi.fn(async () => {
      throw new Error("offline");
    });
    const indexTask = vi.fn(async () => task(operation, "running"));
    const source = {
      prepareIndex: async () => task(operation, "running"),
      indexTask,
      cancelIndexTask,
    } as unknown as StoreDataSource;
    const settings = createProductionStoreSettings({
      data,
      source: () => source,
      operation: () => operation,
      version: () => 1,
      current: () => true,
      restore: async () => {},
    });
    const pending = settings.prepareIndex();
    await vi.advanceTimersByTimeAsync(1);
    await settings.cancelIndex();
    await vi.advanceTimersByTimeAsync(1);
    expect(data.indexPreparation).toBe("loading");
    expect(data.indexTaskStatus).toBe("running");
    expect(data.indexPreparationError).toContain("Cancellation is unconfirmed");
    await vi.advanceTimersByTimeAsync(27500);
    expect(indexTask).toHaveBeenCalledTimes(8);
    expect(cancelIndexTask).toHaveBeenCalledTimes(1);
    await settings.cancelIndex();
    await vi.advanceTimersByTimeAsync(1);
    expect(cancelIndexTask).toHaveBeenCalledTimes(2);
    settings.clear();
    await pending;
    expect(vi.getTimerCount()).toBe(0);
  } finally {
    vi.useRealTimers();
  }
});

test("native task cancellation is bodyless and carries the observation signal", async () => {
  const { Client } = await import("../../src/lib/api/client.js");
  const { HttpStoreWriteDataSource } =
    await import("../../src/lib/controller/http-store-write-data-source.js");
  const abort = new AbortController();
  const fetcher = vi.fn(
    async (_url: string | URL | Request, init?: RequestInit) => {
      expect(init?.method).toBe("POST");
      expect(init?.body).toBeUndefined();
      expect(init?.signal).toBeInstanceOf(AbortSignal);
      return new Response(JSON.stringify(task(operation, "ready")), {
        headers: { "content-type": "application/json" },
      });
    },
  );
  const source = new HttpStoreWriteDataSource(
    new Client(repository, null, () => {}, fetcher),
  );
  await source.cancelIndexTask("task-1", abort.signal);
  expect(fetcher.mock.calls[0]?.[0]).toBe(
    `/v1/repos/${repository}/index/tasks/task-1/cancel`,
  );
});
