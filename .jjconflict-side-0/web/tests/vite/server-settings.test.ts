import { expect, test, vi } from "vitest";
import { createServerSettings } from "../../src/lib/controller/server-settings.svelte.js";
import {
  validateConfiguration,
  copy,
  type ServerSettings,
  type SaveServerSettings,
} from "../../src/lib/controller/server-configuration.js";
import { serverSettingsValue } from "../../../catalog/fixtures/server-settings.js";
import { ApiError } from "../../src/lib/api/client.js";
function fixture() {
  let value = serverSettingsValue(),
    available = true;
  const request = vi.fn(async (_path: string, command?: unknown) => {
    if (command) {
      const intent = command as SaveServerSettings;
      value = {
        ...copy(value),
        revision: "cd".repeat(32),
        desired: {
          ...copy(value.desired),
          [intent.update.section]: copy(intent.update.configuration),
        },
        restart_required: true,
      };
    }
    return copy(value);
  });
  const manager = createServerSettings({
    available: () => available,
    client: { request: request as never, close: vi.fn() },
  });
  return {
    manager,
    request,
    get value() {
      return value;
    },
    set value(v: ServerSettings) {
      value = v;
    },
    set available(v: boolean) {
      available = v;
    },
  };
}
test("typed configuration saves only its section and retains independent drafts", async () => {
  const f = fixture();
  await f.manager.observe();
  f.manager.configure("service", {
    ...f.value.desired.service,
    concurrent_requests: 3,
  });
  f.manager.configure("protection", { bookmarks: ["release"], commits: [] });
  await f.manager.save("service");
  expect(f.request.mock.calls[1][1]).toMatchObject({
    expected_revision: "ab".repeat(32),
    update: { section: "service", configuration: { concurrent_requests: 3 } },
  });
  expect(f.manager.state.value?.effective.service.concurrent_requests).toBe(8);
  expect(f.manager.state.drafts.protection.dirty).toBe(true);
  expect(f.manager.state.drafts.protection.conflict).toBe(true);
  expect(f.manager.state.drafts.service.dirty).toBe(false);
});
test("refresh never rebases a dirty section after external or Search edits", async () => {
  const f = fixture();
  await f.manager.observe();
  f.manager.configure("service", {
    ...f.value.desired.service,
    request_seconds: 18,
  });
  f.value = { ...f.value, revision: "de".repeat(32) };
  await f.manager.observe();
  expect(f.manager.state.drafts.service.baseRevision).toBe("ab".repeat(32));
  expect(f.manager.state.drafts.service.conflict).toBe(true);
  await f.manager.save("service");
  expect(f.request).toHaveBeenCalledTimes(2);
  f.manager.cancel("service");
  expect(f.manager.state.drafts.service.baseRevision).toBe("de".repeat(32));
});
test("unknown save retains original intent until fresh matching observation", async () => {
  const f = fixture();
  await f.manager.observe();
  f.manager.configure("service", {
    ...f.value.desired.service,
    request_seconds: 18,
  });
  f.request.mockRejectedValueOnce(new Error("lost reply"));
  await f.manager.save("service");
  const unknown = f.manager.state.drafts.service.unknown!;
  expect(unknown.expected_revision).toBe("ab".repeat(32));
  f.manager.cancel("service");
  expect(f.manager.state.drafts.service.unknown).not.toBeNull();
  f.value = {
    ...f.value,
    revision: "cd".repeat(32),
    desired: {
      ...f.value.desired,
      service: { ...f.value.desired.service, request_seconds: 18 },
    },
  };
  await f.manager.observe();
  expect(f.manager.state.drafts.service.unknown).toBeNull();
  expect(f.manager.state.drafts.service.dirty).toBe(false);
});
test("mismatching uncertain draft may be discarded only after observation", async () => {
  const f = fixture();
  await f.manager.observe();
  f.manager.configure("service", {
    ...f.value.desired.service,
    request_seconds: 18,
  });
  f.request.mockRejectedValueOnce(
    new ApiError("server_settings_outcome_unknown"),
  );
  await f.manager.save("service");
  await f.manager.observe();
  expect(f.manager.state.drafts.service.unknown).not.toBeNull();
  f.manager.cancel("service");
  expect(f.manager.state.drafts.service.unknown).toBeNull();
});
test("old GET cannot replace a save acknowledgement and observations coalesce", async () => {
  const f = fixture();
  await f.manager.observe();
  let resolve!: (v: ServerSettings) => void;
  f.request.mockImplementationOnce(
    () => new Promise<ServerSettings>((r) => (resolve = r)),
  );
  const old = f.manager.observe();
  const coalesced = f.manager.observe();
  f.manager.configure("service", {
    ...f.value.desired.service,
    request_seconds: 12,
  });
  await f.manager.save("service");
  resolve(serverSettingsValue());
  await Promise.all([old, coalesced]);
  expect(f.manager.state.value?.desired.service.request_seconds).toBe(12);
  expect(f.request).toHaveBeenCalledTimes(3);
});
test("unavailable connections perform no configuration I/O", async () => {
  const f = fixture();
  f.available = false;
  await f.manager.observe();
  await f.manager.save("service");
  expect(f.request).not.toHaveBeenCalled();
});
test("validation prevents credential values, unsafe upstreams and unbounded budgets", () => {
  const f = serverSettingsValue();
  expect(() =>
    validateConfiguration({
      ...f.desired,
      service: { ...f.desired.service, concurrent_requests: 0 },
    }),
  ).toThrow(/Concurrent/);
  const entry = {
    repository: "01".repeat(16),
    upstream: "https://upstream.example/",
    token_environment: "JJ_STORE_REPLICATION_TOKEN_DEMO",
    insecure_loopback: false,
    ca_certificate: null,
    limits: f.sync_defaults,
  };
  validateConfiguration({ ...f.desired, sync: [entry] });
  for (const patch of [
    { upstream: "https://user:secret@upstream.example/" },
    { token_environment: "actual-secret" },
    { ca_certificate: "relative.pem" },
    {
      limits: {
        ...entry.limits,
        retry_initial_millis: 60000,
        retry_max_seconds: 1,
      },
    },
  ])
    expect(() =>
      validateConfiguration({ ...f.desired, sync: [{ ...entry, ...patch }] }),
    ).toThrow();
});
