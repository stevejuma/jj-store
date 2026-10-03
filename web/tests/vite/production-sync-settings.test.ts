import { test, expect, vi } from "vitest";
import type { Model } from "../../src/lib/api/client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createProductionSyncSettings } from "../../src/lib/controller/production-sync-settings.js";
import { HttpSyncSettings } from "../../src/lib/controller/sync-settings.js";
import { Client } from "../../src/lib/api/client.js";
function fixture() {
  const data = createProductionStoreState();
  data.capabilities = { local_sync_management: true } as Model<"Capabilities">;
  const status = vi.fn(async () => ({
    version: 1,
    configured: true,
    worker: null,
  }));
  const control = vi.fn(async () => ({
    version: 1,
    configured: true,
    worker: null,
  }));
  const source = { sync: { status, control } } as unknown as StoreDataSource;
  let current = true;
  const settings = createProductionSyncSettings({
    data,
    source: () => source,
    version: () => 1,
    current: () => current,
  });
  return {
    data,
    source,
    status,
    control,
    settings,
    disconnect() {
      current = false;
      settings.clear();
    },
  };
}
test("sync controls require advertised configured service and explicit refresh after failure", async () => {
  const f = fixture();
  await f.settings.controlSync("pause");
  expect(f.control).not.toHaveBeenCalled();
  await f.settings.refreshSyncSettings();
  await f.settings.controlSync("pause");
  expect(f.control).toHaveBeenCalledWith("pause");
  f.control.mockRejectedValueOnce(new Error("unavailable"));
  await f.settings.controlSync("resume");
  expect(f.data.syncSettings.error).toContain(
    "Status will retry automatically",
  );
  await f.settings.controlSync("resume");
  expect(f.control).toHaveBeenCalledTimes(2);
  await f.settings.refreshSyncSettings();
  await f.settings.controlSync("resume");
  expect(f.control).toHaveBeenCalledTimes(3);
  f.data.capabilities = {} as Model<"Capabilities">;
  f.status.mockClear();
  await f.settings.refreshSyncSettings();
  expect(f.status).not.toHaveBeenCalled();
});
test("sync is single flight and ignores late replies after repository change", async () => {
  const f = fixture();
  await f.settings.refreshSyncSettings();
  let deliver!: (value: {
    version: number;
    configured: boolean;
    worker: null;
  }) => void;
  f.control.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        deliver = resolve;
      }),
  );
  const pending = f.settings.controlSync("pause");
  await f.settings.controlSync("pause");
  expect(f.control).toHaveBeenCalledOnce();
  f.disconnect();
  deliver({ version: 1, configured: true, worker: null });
  await pending;
  expect(f.data.syncSettings.value).toBeNull();
  expect(f.data.syncSettings.changing).toBe(false);
});
test("sync transport sends typed intent over protected POST", async () => {
  const fetcher = vi.fn<typeof fetch>(
    async () =>
      new Response('{"version":1,"configured":false,"worker":null}', {
        headers: { "content-type": "application/json" },
      }),
  );
  const transport = new HttpSyncSettings(
    new Client("ab".repeat(16), null, () => {}, fetcher),
  );
  await transport.status();
  await transport.control("pause");
  expect(fetcher.mock.calls[1]?.[0]).toContain("/local/sync/control");
  const init = fetcher.mock.calls[1]?.[1];
  expect(init).toMatchObject({
    method: "POST",
    credentials: "omit",
    headers: { "x-jj-store-protocol": "1" },
  });
  expect(JSON.parse(init!.body as string)).toEqual({
    version: 1,
    action: "pause",
  });
});

test("sync waits retain a fixed barrier across timeout and ignore a stopped response", async () => {
  const f = fixture();
  await f.settings.refreshSyncSettings();
  const barrier = {
    version: 1,
    repository: "ab".repeat(16),
    local_replica: "01".repeat(16),
    peer_replica: "02".repeat(16),
    push_epoch: "03".repeat(16),
    pull_epoch: "04".repeat(16),
    push_through: 9,
    pull_through: 12,
  };
  const capture = vi.fn(async () => barrier);
  const wait = vi.fn(async () => ({
    version: 1,
    barrier,
    reached: false,
    status: null,
  }));
  Object.assign(f.source.sync!, { capture, wait });
  await f.settings.waitForSync(true);
  expect(f.data.syncSettings.wait.barrier).toEqual(barrier);
  expect(f.data.syncSettings.wait.state).toBe("pending");
  await f.settings.waitForSync(false);
  expect(capture).toHaveBeenCalledOnce();
  expect(wait.mock.calls[1]![0]).toEqual(barrier);
  let deliver!: (value: unknown) => void;
  wait.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        deliver = resolve;
      }),
  );
  const pending = f.settings.waitForSync(false);
  f.settings.stopSyncWait();
  deliver({ version: 1, barrier, reached: true, status: null });
  await pending;
  expect(f.data.syncSettings.wait.state).toBe("stopped");
  expect(f.data.syncSettings.wait.barrier).toEqual(barrier);
  f.disconnect();
  expect(f.data.syncSettings.wait.barrier).toBeNull();
});
