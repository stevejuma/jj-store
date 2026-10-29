import assert from "node:assert/strict";
import { mkdtemp, rm, writeFile, symlink, realpath } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { verifyAiBundle } from "./lapis-ai.mjs";
import { developmentAgentEnvironment } from "./dev-agent-runtime.mjs";
import {
  stubAiBundle,
  stubAiExecutable as executable,
} from "./lapis-ai-test-fixture.mjs";

async function fixture(t) {
  const root = await realpath(
    await mkdtemp(join(tmpdir(), "jj-store-agent-startup-")),
  );
  t.after(() => rm(root, { recursive: true, force: true }));
  const { acp, bundle } = await stubAiBundle(root);
  const jj = join(root, "target/debug/jj-store-jj");
  await executable(jj);
  return { root, acp, jj, bundle };
}

test("development restart selects bundled ACP without inherited shell configuration", async (t) => {
  const { root, acp, jj, bundle } = await fixture(t);
  for (let restart = 0; restart < 2; restart++)
    assert.deepEqual(await developmentAgentEnvironment(root, {}), {
      JJ_STORE_ACP_EXECUTABLE: acp,
      JJ_STORE_AGENT_JJ_EXECUTABLE: jj,
      JJ_STORE_AI_BUNDLE_DIR: bundle,
      JJ_STORE_LOCAL_AGENT_EXECUTABLE: join(bundle, "lapis-agent"),
      JJ_STORE_AGENT_TOOLS_EXECUTABLE: join(bundle, "lapis-agent-tools"),
      JJ_STORE_COMPUTE_EXECUTABLE: join(bundle, "lapis-compute"),
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
    /Invalid Lapis AI bundle/u,
  );
  await rm(acp);
  await assert.rejects(developmentAgentEnvironment(root, {}), /ai:prepare/u);
});

test("version 3 alone does not qualify a helper that rejects prepared context", async (t) => {
  const { root, acp } = await fixture(t);
  await executable(acp, "1|2|3", false);
  await assert.rejects(
    developmentAgentEnvironment(root, { JJ_STORE_ACP_EXECUTABLE: acp }),
    /prepared context/u,
  );
});

test("agent preflight refuses symlinks and never silently falls back from an explicit selection", async (t) => {
  const { root, acp, jj } = await fixture(t);
  await rm(acp);
  await symlink(jj, acp);
  await assert.rejects(
    developmentAgentEnvironment(root, { JJ_STORE_ACP_EXECUTABLE: acp }),
    /regular agent runtime/u,
  );
  await assert.rejects(
    developmentAgentEnvironment(root, {
      JJ_STORE_ACP_EXECUTABLE: join(root, "missing-acp"),
    }),
    /missing-acp/u,
  );
});

test("tampering with a non-ACP helper fails before replacing a service", async (t) => {
  const { root, bundle } = await fixture(t);
  await writeFile(join(bundle, "lapis-compute"), "tampered");
  await assert.rejects(verifyAiBundle(bundle), /checksum/);
  await assert.rejects(
    developmentAgentEnvironment(root, {}),
    /Invalid Lapis AI bundle/,
  );
});
