import test from "node:test";
import assert from "node:assert/strict";
import { agentRevisions } from "../src/lib/controller/agent-revisions.js";
import type { AgentTarget } from "../src/lib/controller/agent-execution-types.js";
const target: AgentTarget = {
  id: "folder",
  repository: "11".repeat(16),
  name: "Repository",
  folder: "/device/repo",
  profile: "local_jj_git",
  operation: "22".repeat(64),
  commit: "33".repeat(20),
};
const response = (
  repository = target.repository,
  operation = target.operation,
) => ({
  snapshot: { repository, operation, profile: "git_sha1_jj045" },
  data: {
    kind: "references",
    data: {
      operation,
      references: [
        {
          kind: "bookmark",
          name: "main",
          targets: [{ commit: target.commit, description: "Main" }],
        },
      ],
    },
  },
  next_cursor: null,
});
const json = (body: unknown) =>
  new Response(JSON.stringify(body), {
    headers: { "content-type": "application/json" },
  });
test("agent revision choices read only the selected repository and exact operation", async () => {
  let calls = 0;
  const fetcher: typeof fetch = async (url, init) => {
    calls++;
    assert.equal(String(url), `/v1/local/repos/${target.repository}/query`);
    const body = JSON.parse(String(init?.body));
    assert.equal(body.operation, target.operation);
    assert.equal(body.request.query.operation, target.operation);
    assert.equal(body.request.query.kind, "all");
    assert.equal(body.request.query.query, "main");
    assert.equal(body.request.query.limit, 50);
    assert.equal(body.cursor, "retained-cursor");
    return json(response());
  };
  const page = await agentRevisions(
    target,
    fetcher,
    "main",
    new AbortController().signal,
    "retained-cursor",
  );
  assert.deepEqual(page.items, [
    {
      id: target.commit,
      name: "main",
      description: `Main · ${target.commit.slice(0, 12)}`,
    },
  ]);
  assert.equal(calls, 1);
});
test("agent revision choices reject cross-repository pins and cancelled reads", async () => {
  await assert.rejects(
    agentRevisions(
      target,
      async () => json(response("44".repeat(16))),
      "",
      new AbortController().signal,
    ),
    { code: "snapshot_mismatch" },
  );
  const abort = new AbortController();
  await assert.rejects(
    agentRevisions(
      target,
      async () => {
        abort.abort();
        return json(response());
      },
      "",
      abort.signal,
    ),
    { name: "AbortError" },
  );
});
test("native revision choices retain native identities and reject repeated cursors", async () => {
  const native = {
    ...target,
    profile: "native" as const,
    commit: "55".repeat(64),
  };
  const fetcher: typeof fetch = async (url) => {
    const parsed = new URL(String(url), "http://fixture");
    assert.equal(parsed.pathname, `/v1/repos/${target.repository}/references`);
    assert.equal(parsed.searchParams.get("at_op"), target.operation);
    return json({
      snapshot: { repo_id: target.repository, operation_id: target.operation },
      data: {
        operation: target.operation,
        references: [
          {
            kind: "change",
            targets: [
              { commit: native.commit, description: "Native revision" },
            ],
          },
        ],
      },
      next_cursor: "same",
    });
  };
  const page = await agentRevisions(
    native,
    fetcher,
    "",
    new AbortController().signal,
  );
  assert.equal(page.items[0].id, native.commit);
  await assert.rejects(
    agentRevisions(native, fetcher, "", new AbortController().signal, "same"),
    { code: "invalid_cursor" },
  );
});
