import { ApiError } from "../src/lib/api/client.js";
import { test } from "node:test";
import assert from "node:assert/strict";
import {
  RepositoryConversionManager,
  type ConversionIntent,
} from "../src/lib/controller/repository-conversion-manager.js";
const intent: ConversionIntent = {
  version: 1,
  source: "11".repeat(16),
  source_profile: "local_jj_git",
  operation: "22".repeat(64),
  destination_profile: "native",
  name: "Server copy",
  path: null,
};
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
    values,
  };
}
const response = (value: unknown, status = 200) =>
  new Response(JSON.stringify(value), {
    status,
    headers: { "content-type": "application/json" },
  });
function committed(request: any) {
  return {
    status: "committed",
    report: {
      version: 1,
      request_id: request.request_id,
      source: request.intent.source,
      destination: "33".repeat(16),
      destination_profile: request.intent.destination_profile,
      name: request.intent.name,
      operation: "44".repeat(64),
      archive_digest: request.prepared_source,
      code_mappings: {
        operations: { [request.intent.operation]: "44".repeat(64) },
        views: {},
        commits: {},
        trees: {},
        files: {},
        symlinks: {},
      },
      collaboration: { selections: [], objects: {} },
    },
  };
}
test("conversion preserves exact source and destination intent through lost response, reload and retry", async () => {
  const saved = storage();
  const requests: string[] = [];
  const first = new RepositoryConversionManager(async (_url, init) => {
    requests.push(String(init?.body));
    assert.equal(init?.credentials, "omit");
    throw new Error("response lost");
  }, saved);
  await assert.rejects(
    first.start(new Proxy(intent, {}), "55".repeat(32)),
    /response lost/,
  );
  const pending = first.pending!;
  first.close();
  const resumed = new RepositoryConversionManager(async (_url, init) => {
    requests.push(String(init?.body));
    return response(committed(pending));
  }, saved);
  const outcome = await resumed.start(
    { ...intent, name: "Changed", source: "66".repeat(16) },
    "77".repeat(32),
  );
  assert.equal(outcome.status, "committed");
  assert.equal(requests[0], requests[1]);
  assert.equal(resumed.pending, null);
  const reopened = new RepositoryConversionManager(async (url) => {
    assert.ok(String(url).endsWith(pending.request_id));
    return response(committed(pending));
  }, saved);
  assert.ok(reopened.hasReport);
  assert.equal((await reopened.status())?.status, "committed");
  assert.equal(saved.values.size, 1);
});
test("conversion refuses a foreign or altered receipt and retains unknown requests", async () => {
  const saved = storage();
  let original: any;
  const manager = new RepositoryConversionManager(async (_url, init) => {
    original = JSON.parse(String(init?.body));
    const result = committed(original);
    result.report.archive_digest = "99".repeat(32);
    return response(result);
  }, saved);
  await assert.rejects(
    manager.start(intent, "55".repeat(32)),
    (error: unknown) =>
      error instanceof ApiError &&
      error.code === "conversion_identity_mismatch",
  );
  assert.ok(manager.pending);
  const resumed = new RepositoryConversionManager(
    async () =>
      response({ status: "not_known", request_id: original.request_id }),
    saved,
  );
  assert.equal((await resumed.status())?.status, "not_known");
  assert.deepEqual(resumed.pending, original);
});
test("preacceptance stale source allows a fresh check while publication conflicts keep original identity", async () => {
  const saved = storage();
  const stale = new RepositoryConversionManager(
    async () => response({ code: "stale_local_revision" }, 409),
    saved,
  );
  await assert.rejects(
    stale.start(intent, "55".repeat(32)),
    (error: unknown) =>
      error instanceof ApiError && error.code === "stale_local_revision",
  );
  assert.equal(stale.pending, null);
  const conflict = new RepositoryConversionManager(
    async () => response({ code: "local_registration_conflict" }, 409),
    saved,
  );
  await assert.rejects(
    conflict.start(intent, "55".repeat(32)),
    (error: unknown) =>
      error instanceof ApiError && error.code === "local_registration_conflict",
  );
  assert.ok(conflict.pending);
});
test("blocked preflight does not allocate or persist a conversion request", async () => {
  const saved = storage();
  const manager = new RepositoryConversionManager(async (_url, init) => {
    assert.deepEqual(JSON.parse(String(init?.body)), intent);
    return response({
      version: 1,
      prepared_source: null,
      source_bytes: 0,
      history_selections: 0,
      blocker: "Unqualified code evidence",
    });
  }, saved);
  assert.equal((await manager.preview(intent)).prepared_source, null);
  assert.equal(manager.pending, null);
  assert.equal(saved.values.size, 0);
});
