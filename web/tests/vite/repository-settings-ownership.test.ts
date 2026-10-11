import { expect, test, vi } from "vitest";
import { createProductionReadServices } from "../../src/lib/controller/production-read-services.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";

// Automatic graph recovery is outside this transport-ownership regression.
vi.mock("../../src/lib/controller/managed-index-recovery.svelte.js", () => ({
  watchCodeIndexRecovery() {},
}));

function fixture() {
  const data = createProductionStoreState();
  data.capabilities = {
    local_sync_management: true,
    index_tasks: true,
  } as NonNullable<typeof data.capabilities>;
  const statusValue = { version: 1, configured: true, worker: null };
  let deliver!: (value: typeof statusValue) => void;
  const control = vi.fn(
    () =>
      new Promise<typeof statusValue>((resolve) => {
        deliver = resolve;
      }),
  );
  const source = {
    sync: { status: async () => statusValue, control },
  } as unknown as StoreDataSource;
  let navigation = new AbortController();
  let active = source,
    version = 1;
  const { settings } = createProductionReadServices({
    data,
    source: () => active,
    history: {
      route: {
        repository: "ab".repeat(16),
        page: "settings",
        snapshotOperationId: "cd".repeat(64),
      },
    } as never,
    version: () => version,
    signal: () => navigation.signal,
    current: (ticket, candidate) => ticket === version && candidate === active,
    restoreRoute: async () => {},
    restore: async () => {},
  });
  return {
    data,
    settings,
    control,
    source,
    deliver() {
      deliver(statusValue);
    },
    navigate() {
      navigation.abort();
      navigation = new AbortController();
      version++;
    },
    changeRepository() {
      active = {} as StoreDataSource;
      version++;
      settings.clear();
    },
  };
}

test("pending Settings commands settle across category history and repository return", async () => {
  const f = fixture();
  await f.settings.refreshSyncSettings();
  const pending = f.settings.controlSync("pause");
  f.navigate(); // Browser category Back/Forward advances the code read scope.
  f.navigate(); // Back to the repository advances it again.
  expect(f.data.syncSettings.changing).toBe(true);
  f.deliver();
  await pending;
  expect(f.data.syncSettings.changing).toBe(false);
  expect(f.data.syncSettings.state).toBe("ready");
  expect(f.control).toHaveBeenCalledOnce();
});

test("a late Settings response remains rejected after repository owner replacement", async () => {
  const f = fixture();
  await f.settings.refreshSyncSettings();
  const pending = f.settings.controlSync("pause");
  f.changeRepository();
  f.deliver();
  await pending;
  expect(f.data.syncSettings.value).toBeNull();
  expect(f.data.syncSettings.changing).toBe(false);
});

test("exact index preparation retains its task across code-read cancellation", async () => {
  const f = fixture();
  let deliver!: (
    value: Awaited<ReturnType<StoreDataSource["prepareIndex"]>>,
  ) => void;
  let signal!: AbortSignal;
  f.source.prepareIndex = vi.fn((_operation, abort) => {
    signal = abort!;
    return new Promise((resolve) => {
      deliver = resolve;
    });
  });
  const pending = f.settings.prepareIndex();
  expect(f.data.indexPreparation).toBe("loading");
  f.navigate();
  expect(signal.aborted).toBe(false);
  deliver({
    id: "retained-task",
    status: "ready",
    snapshot: { operation: "cd".repeat(64) },
  } as Awaited<ReturnType<StoreDataSource["prepareIndex"]>>);
  await pending;
  expect(f.data.indexPreparation).toBe("ready");
  expect(f.source.prepareIndex).toHaveBeenCalledOnce();
});
