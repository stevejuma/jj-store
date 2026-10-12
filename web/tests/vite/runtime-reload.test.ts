import { expect, test } from "vitest";
import { RuntimeReload } from "../../src/lib/controller/runtime-reload.svelte.js";
import { connectionBlocked } from "../../src/lib/connections/management.js";
import type { StoreController } from "../../src/lib/controller/contracts.js";
const revision = "aa".repeat(32);
const status = {
  saved_revision: revision,
  supported: ["service"],
  pending: ["service"],
  generation: 0,
  original: null,
  outcome: null,
};
test("lost reload acknowledgement checks original intent and never reposts", async () => {
  let submissions = 0;
  let original: unknown;
  let applied = 0;
  const manager = new RuntimeReload(
    () => true,
    () => null,
    async (path, init) => {
      const body = init?.body ? JSON.parse(String(init.body)) : null;
      if (String(path) === "/v1/configuration/reload" && body) {
        submissions++;
        original = body;
        throw Error("lost acknowledgement");
      }
      if (String(path).endsWith("/outcome")) {
        expect(body).toEqual(original);
        return Response.json({
          request_id: body.request_id,
          state: "applied",
          generation: 1,
          results: [{ domain: "service", state: "applied", reason: null }],
        });
      }
      return Response.json(status);
    },
    async () => {
      applied++;
    },
  );
  await manager.observe();
  await manager.run();
  expect(manager.pending).not.toBeNull();
  expect(submissions).toBe(1);
  expect(connectionBlocked({ reload: manager } as StoreController)).toBe(true);
  await manager.check();
  expect(submissions).toBe(1);
  expect(applied).toBe(1);
  expect(manager.pending).toBeNull();
  manager.destroy();
});
test("server discovery restores original pending identity after browser reload", async () => {
  const command = {
    version: 1,
    request_id: "11".repeat(16),
    expected_revision: revision,
    domains: ["service"],
  };
  const manager = new RuntimeReload(
    () => true,
    () => null,
    async () =>
      Response.json({
        ...status,
        original: command,
        outcome: {
          request_id: command.request_id,
          state: "unknown",
          generation: 1,
          results: [{ domain: "service", state: "unknown", reason: null }],
        },
      }),
  );
  await manager.observe();
  expect(manager.pending).toEqual(command);
  expect(manager.outcome?.state).toBe("unknown");
  manager.destroy();
});
test("stale owners cannot replace the selected connection reload state", async () => {
  let active = true;
  let release!: (r: Response) => void;
  const manager = new RuntimeReload(
    () => active,
    () => null,
    () =>
      new Promise((r) => {
        release = r;
      }),
  );
  const observed = manager.observe();
  await Promise.resolve();
  active = false;
  release(Response.json(status));
  await observed;
  expect(manager.value).toBeNull();
  manager.destroy();
});

test("malformed original outcomes retain pending identity and prevent another reload", async () => {
  let command: unknown;
  let submissions = 0;
  const manager = new RuntimeReload(
    () => true,
    () => null,
    async (path, init) => {
      if (init?.body) {
        submissions++;
        command = JSON.parse(String(init.body));
        return Response.json({
          request_id: "ff".repeat(16),
          generation: 1,
          state: "applied",
          results: [],
        });
      }
      return Response.json(status);
    },
  );
  await manager.observe();
  await manager.run();
  expect(manager.pending).toEqual(command);
  await manager.run();
  expect(submissions).toBe(1);
  expect(manager.error).toContain("unknown");
  manager.destroy();
});
test("accepted reload followed by denied observation retains original command", async () => {
  let command: unknown;
  let submissions = 0;
  const manager = new RuntimeReload(
    () => true,
    () => null,
    async (path, init) => {
      if (String(path).endsWith("/outcome"))
        return Response.json(
          { code: "configuration_reload_denied" },
          { status: 403 },
        );
      if (init?.body) {
        submissions++;
        command = JSON.parse(String(init.body));
        const c = command as { request_id: string };
        return Response.json({
          request_id: c.request_id,
          generation: 1,
          state: "running",
          results: [{ domain: "service", state: "running", reason: null }],
        });
      }
      return Response.json(status);
    },
  );
  await manager.observe();
  await manager.run();
  expect(manager.pending).toEqual(command);
  await manager.run();
  expect(submissions).toBe(1);
  manager.destroy();
});
