import { test, expect, vi } from "vitest";
import { ApiError, type Model } from "../../src/lib/api/client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createProductionStorageSettings } from "../../src/lib/controller/production-storage-settings.js";
const target = {
  repository: "ab".repeat(16),
  epoch: "cd".repeat(16),
  position: 4,
  revision: 6,
  digest: "ef".repeat(32),
};
const operation = {
  version: 1,
  request_id: "12".repeat(16),
  kind: "checkpoint",
  phase: "prepared",
  target,
  limits: { entries: 100, bytes: 1000, timeout_millis: 1000 },
  message: null,
  selection: null,
};
function fixture(hasRepositoryContext = () => true) {
  const data = createProductionStoreState();
  data.capabilities = {
    local_storage_management: true,
  } as Model<"Capabilities">;
  const overview = vi.fn(async () => ({
    version: 1,
    ready: true,
    current: target,
    checkpoint: null,
    latest: null,
    limits: operation.limits,
  }));
  const command = vi.fn(async (request) => ({
    ...operation,
    request_id: request.request_id,
  }));
  const status = vi.fn(async () => operation);
  const source = {
    storage: { overview, command, operation: status },
  } as unknown as StoreDataSource;
  let current = true;
  const settings = createProductionStorageSettings({
    data,
    source: () => source,
    version: () => 1,
    current: () => current,
    hasRepositoryContext,
  });
  return {
    data,
    command,
    overview,
    status,
    settings,
    disconnect() {
      current = false;
      settings.clear();
    },
  };
}
test("storage retains an uncertain command identity and never recaptures its checkpoint", async () => {
  const f = fixture();
  await f.settings.refreshStorage();
  f.command.mockRejectedValueOnce(new Error("response lost"));
  await f.settings.startStorage("prepare_checkpoint");
  const original = f.command.mock.calls[0][0];
  expect(f.data.storageSettings.pending).toEqual(original);
  await f.settings.retryStorage();
  expect(f.command.mock.calls[1][0]).toEqual(original);
  expect(f.data.storageSettings.operation?.target).toEqual(target);
  await f.settings.startStorage("publish_checkpoint");
  expect(f.command.mock.calls[2][0].request_id).toBe(original.request_id);
});
test("storage recovers persisted reports and ignores late responses after leaving a repository", async () => {
  const f = fixture();
  f.overview.mockResolvedValueOnce({
    ...(await f.overview()),
    latest: operation,
  } as any);
  await f.settings.refreshStorage();
  expect(f.data.storageSettings.operation?.phase).toBe("prepared");
  let resolve!: (value: any) => void;
  f.command.mockImplementationOnce(
    () =>
      new Promise((r) => {
        resolve = r;
      }),
  );
  const request = f.settings.startStorage("publish_checkpoint");
  f.disconnect();
  resolve(operation);
  await request;
  expect(f.data.storageSettings.operation).toBeNull();
});
test("storage requires an advertised capability and keeps incomplete checks distinct", async () => {
  const f = fixture();
  f.data.capabilities = {} as Model<"Capabilities">;
  await f.settings.startStorage("check");
  await f.settings.refreshStorage();
  expect(f.command).not.toHaveBeenCalled();
  expect(f.overview).not.toHaveBeenCalled();
  f.data.capabilities = {
    local_storage_management: true,
  } as Model<"Capabilities">;
  f.overview.mockResolvedValueOnce({
    ...(await f.overview()),
    latest: { ...operation, kind: "check", phase: "incomplete" },
  } as any);
  await f.settings.refreshStorage();
  expect(f.data.storageSettings.operation?.phase).toBe("incomplete");
});
test("a definite maintenance rejection clears only the unaccepted command", async () => {
  const f = fixture();
  await f.settings.refreshStorage();
  f.command.mockRejectedValueOnce(new ApiError("storage_operation_pending"));
  await f.settings.startStorage("check");
  expect(f.data.storageSettings.pending).toBeNull();
  expect(f.data.storageSettings.error).toContain("storage operation pending");
});

test("Storage cannot act on a previously selected repository from the Projects hub", async () => {
  const f = fixture(() => false);
  await f.settings.startStorage("check");
  await f.settings.refreshStorage();
  expect(f.command).not.toHaveBeenCalled();
  expect(f.overview).not.toHaveBeenCalled();
});
