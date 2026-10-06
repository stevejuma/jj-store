import { test } from "vitest";
import assert from "node:assert/strict";
import { ApiError } from "../../src/lib/api/client.js";
import {
  needsAgentRevisionIndex,
  rebuildAgentRevisionIndex,
} from "../../src/lib/controller/agent-revision-index.js";
import type { AgentTarget } from "../../src/lib/controller/agent-execution-types.js";
const target: AgentTarget = {
  id: "folder",
  repository: "11".repeat(16),
  operation: "22".repeat(64),
  commit: "33".repeat(20),
  name: "Repository",
  folder: "/device/repo",
  profile: "local_jj_git",
};
const json = (body: unknown) =>
  new Response(JSON.stringify(body), {
    headers: { "content-type": "application/json" },
  });
const snapshot = {
  version: 1,
  repository: target.repository,
  generation: 1,
  profile: "git_sha1_jj045",
  operation: target.operation,
};

test("revision recovery is offered only for the selected index-required pin", () => {
  assert.equal(
    needsAgentRevisionIndex(new ApiError("index_rebuild_required"), target),
    true,
  );
  assert.equal(
    needsAgentRevisionIndex(new ApiError("access_denied"), target),
    false,
  );
  assert.equal(
    needsAgentRevisionIndex(new Error("index rebuild required"), target),
    false,
  );
  assert.equal(
    needsAgentRevisionIndex(
      new ApiError("index_rebuild_required", undefined, undefined, undefined, {
        ...snapshot,
        repository: "44".repeat(16),
      } as never),
      target,
    ),
    false,
  );
});

test("explicit local index recovery prepares and polls the exact repository and operation", async () => {
  const paths: string[] = [];
  const id = "55".repeat(16);
  await rebuildAgentRevisionIndex(
    target,
    async (url, init) => {
      paths.push(String(url));
      if (String(url).endsWith("/snapshot"))
        return json({
          ...snapshot,
          heads: [target.operation],
          metadata_commit: null,
          actor: null,
        });
      if (String(url).endsWith("/index/tasks")) {
        assert.equal(init?.method, "POST");
        assert.deepEqual(JSON.parse(String(init?.body)), {
          operation: target.operation,
        });
        return json({ version: 1, id, snapshot, status: "running" });
      }
      assert.equal(
        String(url),
        `/v1/local/repos/${target.repository}/index/tasks/${id}`,
      );
      return json({ version: 1, id, snapshot, status: "ready" });
    },
    new AbortController().signal,
  );
  assert.equal(paths.length, 3);
});

test("native recovery refuses a changed task pin and cancellation dispatches nothing", async () => {
  const native = { ...target, profile: "native" as const };
  const id = "66".repeat(16),
    selected = { repo_id: target.repository, operation_id: target.operation };
  await assert.rejects(
    rebuildAgentRevisionIndex(
      native,
      async (url, init) => {
        if (String(url).endsWith("/index/tasks")) {
          assert.equal(
            String(url),
            `/v1/repos/${target.repository}/index/tasks`,
          );
          assert.deepEqual(JSON.parse(String(init?.body)), {
            operation: target.operation,
          });
          return json({
            version: 1,
            id,
            snapshot: selected,
            status: "running",
          });
        }
        return json({
          version: 1,
          id,
          snapshot: { ...selected, operation_id: "77".repeat(64) },
          status: "ready",
        });
      },
      new AbortController().signal,
    ),
    { code: "snapshot_mismatch" },
  );
  const abort = new AbortController();
  abort.abort();
  let calls = 0;
  await assert.rejects(
    rebuildAgentRevisionIndex(
      native,
      async () => {
        calls++;
        return json({});
      },
      abort.signal,
    ),
    { name: "AbortError" },
  );
  assert.equal(calls, 0);
});
