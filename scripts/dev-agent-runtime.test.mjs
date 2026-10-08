import assert from "node:assert/strict";
import { mkdtemp, mkdir, rm, writeFile, symlink } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { developmentAgentEnvironment } from "./dev-agent-runtime.mjs";

async function executable(path, versions = "1|2|3") {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(
    path,
    `#!/bin/sh\n[ "$1" = "--help" ] || exit 17\nprintf '%s\\n' '--control-version <${versions}>'\n`,
    { mode: 0o755 },
  );
}
async function fixture(t) {
  const root = await mkdtemp(join(tmpdir(), "jj-store-agent-startup-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  const acp = join(root, "desktop/resources/jj-acp");
  const jj = join(root, "target/debug/jj");
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
  const jj = join(root, "custom/jj");
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
