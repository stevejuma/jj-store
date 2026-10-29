import assert from "node:assert/strict";
import { test } from "vitest";
import { LocalRepositoryRemoval } from "../../src/lib/controller/local-repository-removal.js";
const repository = "ab".repeat(16);
const json = (value: unknown, status = 200) =>
  new Response(JSON.stringify(value), {
    status,
    headers: { "content-type": "application/json" },
  });

test("lost removal acknowledgement resolves an absent binding without another mutation", async () => {
  let registered = true,
    writes = 0;
  const removal = new LocalRepositoryRemoval(repository, async (url, init) => {
    assert.equal(init?.credentials, "omit");
    assert.equal(init?.redirect, "error");
    if (String(url).endsWith("/remove")) {
      writes++;
      assert.deepEqual(JSON.parse(String(init?.body)), {
        version: 1,
        expected_revision: 7,
      });
      registered = false;
      throw new TypeError("Acknowledgement lost");
    }
    return json({
      version: 1,
      revision: registered ? 7 : 8,
      bindings: registered ? [{ repository }] : [],
    });
  });
  await assert.rejects(removal.remove(), /Acknowledgement lost/);
  await removal.remove();
  assert.equal(writes, 1);
  removal.close();
});

test("retry cannot rebase onto a newer registration after an uncertain removal", async () => {
  let revision = 7;
  const requests: unknown[] = [];
  const removal = new LocalRepositoryRemoval(repository, async (url, init) => {
    if (String(url).endsWith("/remove")) {
      requests.push(JSON.parse(String(init?.body)));
      if (revision === 7) {
        revision = 9; // Original removal committed, then the folder was registered again.
        throw new TypeError("Acknowledgement lost");
      }
      return json({ code: "stale_local_revision" }, 409);
    }
    return json({ version: 1, revision, bindings: [{ repository }] });
  });
  await assert.rejects(removal.remove(), /Acknowledgement lost/);
  await assert.rejects(removal.remove(), { code: "stale_local_revision" });
  assert.deepEqual(requests, [
    { version: 1, expected_revision: 7 },
    { version: 1, expected_revision: 7 },
  ]);
  removal.close();
});

test("already absent registrations need no mutation and teardown aborts further work", async () => {
  let calls = 0;
  const removal = new LocalRepositoryRemoval(repository, async (_url, init) => {
    calls++;
    assert.notEqual(init?.method, "POST");
    return json({ version: 1, revision: 7, bindings: [] });
  });
  await removal.remove();
  removal.close();
  await assert.rejects(removal.remove(), { name: "AbortError" });
  assert.equal(calls, 1);
});
