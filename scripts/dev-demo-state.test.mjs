import assert from "node:assert/strict";
import { mkdtemp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import {
  developmentLayout,
  loadFixture,
  rewriteServerConfiguration,
  writeFixture,
} from "./dev-demo-state.mjs";

test("development layout defaults to ignored checkout state and accepts isolation", () => {
  const root = "/workspace/jj-store";
  assert.equal(
    developmentLayout(root, {}).runtimeRoot,
    "/workspace/jj-store/.local/dev-demo",
  );
  assert.equal(
    developmentLayout(root, { JJ_STORE_DEV_ROOT: "isolated/demo" }).runtimeRoot,
    "/workspace/jj-store/isolated/demo",
  );
});

test("fixture marker distinguishes reusable state from partial state", async () => {
  const root = await mkdtemp(join(tmpdir(), "jj-store-demo-state-"));
  const layout = developmentLayout(root, {});
  try {
    assert.equal(await loadFixture(layout), null);
    await mkdir(layout.runtimeRoot, { recursive: true });
    await writeFile(join(layout.runtimeRoot, "unexpected"), "partial");
    await assert.rejects(loadFixture(layout), /pnpm dev:reset/u);

    await rm(layout.runtimeRoot, { recursive: true });
    await mkdir(layout.repository, { recursive: true });
    await mkdir(layout.rustfsVolume, { recursive: true });
    await mkdir(layout.stateRoot, { recursive: true });
    await writeFile(layout.serverConfig, "format = 2\n");
    const fixture = await writeFixture(layout, {
      firstStartupMillis: 100,
      repositoryId: "0123456789abcdef0123456789abcdef",
    });
    assert.deepEqual(await loadFixture(layout), fixture);
    assert.equal(JSON.parse(await readFile(layout.fixtureFile)).format, 1);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("server configuration refresh is idempotent", () => {
  const original = `format = 2
bind = "127.0.0.1:7443"
public_url = "http://127.0.0.1:7443/"
operator_policy = "operator.toml"
search = []
`;
  const expected = `format = 2
bind = "127.0.0.1:7460"
public_url = "http://127.0.0.1:7460/"
operator_policy = "operator.toml"
web_root = "/workspace/web/dist"
search = []
`;
  const options = {
    origin: "http://127.0.0.1:7460/",
    webRoot: "/workspace/web/dist",
  };
  assert.equal(rewriteServerConfiguration(original, options), expected);
  assert.equal(rewriteServerConfiguration(expected, options), expected);
});
