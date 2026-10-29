import assert from "node:assert/strict";
import { test } from "vitest";
import { NativeRepositoryDeletion } from "../../src/lib/controller/native-repository-deletion.js";
const repository = "ab".repeat(16),
  epoch = "cd".repeat(16);
const json = (value: unknown, status = 200) =>
  new Response(JSON.stringify(value), {
    status,
    headers: { "content-type": "application/json" },
  });
function storage() {
  const entries = new Map<string, string>();
  return {
    getItem: (k: string) => entries.get(k) ?? null,
    setItem: (k: string, v: string) => {
      entries.set(k, v);
    },
    removeItem: (k: string) => {
      entries.delete(k);
    },
  };
}

test("permanent deletion resumes the original public intent after a lost acknowledgement and reload", async () => {
  const saved = storage();
  const requests: any[] = [];
  let fail = true;
  const fetcher: typeof fetch = async (url, init) => {
    if (String(url).endsWith("bootstrap"))
      return json({
        installation_id: "11".repeat(16),
        capabilities: { local_native_repository_deletion: true },
      });
    requests.push(JSON.parse(String(init?.body)));
    assert.equal(init?.credentials, "omit");
    assert.equal(init?.redirect, "error");
    if (fail) {
      fail = false;
      throw new TypeError("Acknowledgement lost");
    }
    return json({
      version: 1,
      repository,
      request_id: requests[0].request_id,
      state: requests.length < 3 ? "pending" : "deleted",
    });
  };
  const first = new NativeRepositoryDeletion(saved, fetcher, {
    repository,
    epoch,
    name: "Review repo",
  });
  await assert.rejects(first.remove(), /Acknowledgement lost/);
  first.close();
  const original = first.pending;
  assert.ok(original);
  const reloaded = new NativeRepositoryDeletion(saved, fetcher);
  assert.deepEqual(reloaded.pending, original);
  await reloaded.remove();
  assert.equal(reloaded.pending, null);
  assert.deepEqual(requests, [
    original.request,
    original.request,
    original.request,
  ]);
  assert.equal(new NativeRepositoryDeletion(saved, fetcher).pending, null);
  reloaded.close();
});

test("foreign deletion outcomes preserve the exact pending request", async () => {
  const saved = storage();
  const manager = new NativeRepositoryDeletion(
    saved,
    async (url, init) => {
      if (String(url).endsWith("bootstrap"))
        return json({
          installation_id: "11".repeat(16),
          capabilities: { local_native_repository_deletion: true },
        });
      const request = JSON.parse(String(init?.body));
      return json({
        version: 1,
        repository: "ef".repeat(16),
        request_id: request.request_id,
        state: "deleted",
      });
    },
    { repository, epoch, name: "Review repo" },
  );
  await assert.rejects(manager.remove(), {
    code: "deletion_identity_mismatch",
  });
  assert.ok(manager.pending);
  manager.close();
});

test("proven preflight refusal clears intent while an unknown failure retains it", async () => {
  for (const code of [
    "repository_deletion_unsupported",
    "repository_deletion_snapshot_changed",
    "repository_deletion_unavailable",
    "repository_deletion_restart_required",
  ]) {
    const saved = storage();
    const manager = new NativeRepositoryDeletion(
      saved,
      async (url) => {
        if (String(url).endsWith("bootstrap"))
          return json({
            installation_id: "11".repeat(16),
            capabilities: { local_native_repository_deletion: true },
          });
        return json(
          { code },
          [
            "repository_deletion_unavailable",
            "repository_deletion_restart_required",
          ].includes(code)
            ? 503
            : 400,
        );
      },
      { repository, epoch, name: "Review repo" },
    );
    await assert.rejects(manager.remove(), { code });
    assert.equal(
      Boolean(manager.pending),
      [
        "repository_deletion_unavailable",
        "repository_deletion_restart_required",
      ].includes(code),
    );
    manager.close();
  }
});
