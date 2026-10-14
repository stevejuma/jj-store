import assert from "node:assert/strict";
import { mkdtemp, mkdir, rm, writeFile, symlink } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { developmentAgentEnvironment } from "./dev-agent-runtime.mjs";

async function executable(path, versions = "1|2|3", context = true) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(
    path,
    `#!/bin/sh\nif [ "$1" = "--help" ]; then\nprintf '%s\\n' '--control-version <${versions}>'\nelse\ncat >/dev/null\nprintf '%s\\n' '${JSON.stringify({ version: 3, type: "error", requestId: "preflight-text-context", code: context ? "session_not_open" : "invalid_request" })}' '${JSON.stringify({ version: 3, type: "error", requestId: "preflight-context-only", code: context ? "session_not_open" : "invalid_request" })}'\nfi\n`,
    { mode: 0o755 },
  );
}
async function fixture(t) {
  const root = await mkdtemp(join(tmpdir(), "jj-store-agent-startup-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  const acp = join(root, "desktop/resources/jj-acp");
  const jj = join(root, "target/debug/jj-store-jj");
  await executable(acp);
  await executable(jj);
  return { root, acp, jj };
}

test("development restart selects bundled ACP without inherited shell configuration", async (t) => {
  const { root, acp, jj } = await fixture(t);
  for (let restart = 0; restart < 2; restart++)
    assert.deepEqual(await developmentAgentEnvironment(root, {}), {
      JJ_STORE_ACP_EXECUTABLE: acp,
      JJ_STORE_AGENT_JJ_EXECUTABLE: jj,
    });
});

test("explicit trusted resources override stale bundled ACP and pair an alternate server", async (t) => {
  const { root, acp } = await fixture(t);
  await executable(acp, "1|2");
  const custom = join(root, "custom/acp");
  const jj = join(root, "custom/jj-store-jj");
  await executable(custom);
  await executable(jj);
  assert.deepEqual(
    await developmentAgentEnvironment(
      root,
      {
        JJ_STORE_ACP_EXECUTABLE: custom,
      },
      join(root, "custom/jj-store"),
    ),
    { JJ_STORE_ACP_EXECUTABLE: custom, JJ_STORE_AGENT_JJ_EXECUTABLE: jj },
  );
});

test("missing or stale bundled ACP fails before a service can be replaced", async (t) => {
  const { root, acp } = await fixture(t);
  await executable(acp, "1|2");
  await assert.rejects(
    developmentAgentEnvironment(root, {}),
    /control version 3/u,
  );
  await rm(acp);
  await assert.rejects(
    developmentAgentEnvironment(root, {}),
    /desktop:prepare/u,
  );
});

test("version 3 alone does not qualify a helper that rejects prepared context", async (t) => {
  const { root, acp } = await fixture(t);
  await executable(acp, "1|2|3", false);
  await assert.rejects(
    developmentAgentEnvironment(root, {}),
    /prepared context/u,
  );
});

test("agent preflight refuses symlinks and never silently falls back from an explicit selection", async (t) => {
  const { root, acp, jj } = await fixture(t);
  await rm(acp);
  await symlink(jj, acp);
  await assert.rejects(
    developmentAgentEnvironment(root, {}),
    /regular agent runtime/u,
  );
  await assert.rejects(
    developmentAgentEnvironment(root, {
      JJ_STORE_ACP_EXECUTABLE: join(root, "missing-acp"),
    }),
    /missing-acp/u,
  );
});
