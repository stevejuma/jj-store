import test from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { seedThreads } from "./dev-demo-threads.mjs";
test("explicit demo Threads cover empty and multiple codebases without replaying after startup", async (t) => {
  const runtimeRoot = await mkdtemp(join(tmpdir(), "intentional-threads-"));
  t.after(() => rm(runtimeRoot, { recursive: true, force: true }));
  const requests = [];
  const fetcher = async (url, init) => {
    const headers = new Headers(init.headers);
    assert.equal(headers.get("origin"), "http://127.0.0.1:7460");
    assert.equal(headers.get("sec-fetch-site"), "same-origin");
    assert.equal(headers.get("sec-fetch-mode"), "cors");
    assert.equal(headers.get("sec-fetch-dest"), "empty");
    assert.equal(headers.get("x-jj-store-protocol"), "1");
    assert.equal(init.redirect, "error");
    const request = JSON.parse(init.body);
    requests.push(request);
    return Response.json({
      home: "aa".repeat(16),
      thread: request.request_id,
      revision: { kind: "thread_version", id: "bb".repeat(32) },
    });
  };
  const options = {
    origin: "http://127.0.0.1:7460/",
    runtimeRoot,
    repositories: ["11".repeat(16), "22".repeat(16)],
    fetcher,
  };
  const first = await seedThreads(options);
  assert.deepEqual(
    requests.map((r) => r.command.repositories.length),
    [0, 1, 2],
  );
  assert.deepEqual(await seedThreads(options), first);
  assert.equal(requests.length, 3);
});
