import { test } from "node:test";
import assert from "node:assert/strict";
import { NativeRepositoryManager } from "../src/lib/controller/native-repository-manager.js";
function storage() {
  const values = new Map<string, string>();
  return {
    getItem: (key: string) => values.get(key) ?? null,
    setItem: (key: string, value: string) => {
      values.set(key, value);
    },
    removeItem: (key: string) => {
      values.delete(key);
    },
  };
}
const response = (value: unknown, status = 200) =>
  new Response(JSON.stringify(value), {
    status,
    headers: { "content-type": "application/json" },
  });
test("native creation retains exact identity through unknown outcome, reload and retry", async () => {
  const saved = storage();
  const requests: string[] = [];
  const fetcher: typeof fetch = async (_url, init) => {
    requests.push(String(init?.body));
    throw new Error("response lost");
  };
  const first = new NativeRepositoryManager(fetcher, saved);
  await assert.rejects(first.create("Original"), /response lost/);
  const pending = first.pending!;
  first.close();
  const resumed = new NativeRepositoryManager(async (_url, init) => {
    requests.push(String(init?.body));
    return response({
      status: "committed",
      request_id: pending.request_id,
      repository: "22".repeat(16),
      name: "Original",
    });
  }, saved);
  const outcome = await resumed.create("Changed draft");
  assert.equal(outcome.status, "committed");
  assert.equal(requests[0], requests[1]);
  assert.equal(resumed.pending, null);
  assert.equal(new NativeRepositoryManager(fetcher, saved).pending, null);
});
test("native creation refuses mismatched receipts and keeps unresolved intent", async () => {
  const manager = new NativeRepositoryManager(
    async () =>
      response({
        status: "committed",
        request_id: "00".repeat(16),
        repository: "22".repeat(16),
        name: "Original",
      }),
    storage(),
  );
  await assert.rejects(
    manager.create("Original"),
    /creation identity mismatch/,
  );
  assert.equal(manager.pending?.name, "Original");
});
test("known name rejection permits correction while unknown status retains the original request", async () => {
  const saved = storage();
  let reject = true;
  const manager = new NativeRepositoryManager(async (_url, init) => {
    if (reject) return response({ code: "local_registration_conflict" }, 409);
    const body = JSON.parse(String(init?.body));
    return response({ status: "outcome_unknown", request_id: body.request_id });
  }, saved);
  await assert.rejects(manager.create("Taken"));
  assert.equal(manager.pending, null);
  reject = false;
  const result = await manager.create("Corrected");
  assert.equal(result.status, "outcome_unknown");
  assert.equal(manager.pending?.name, "Corrected");
});

test("already mounted creation entrypoints recover the same installation intent", async () => {
  const saved = storage();
  const requests: string[] = [];
  const fetcher: typeof fetch = async (_url, init) => {
    requests.push(String(init?.body));
    const body = JSON.parse(String(init?.body));
    return response({ status: "outcome_unknown", request_id: body.request_id });
  };
  const sidebar = new NativeRepositoryManager(fetcher, saved);
  const menu = new NativeRepositoryManager(fetcher, saved);
  await sidebar.create("Original");
  await menu.create("Other draft");
  assert.equal(requests[0], requests[1]);
  assert.equal(menu.pending?.name, "Original");
});
