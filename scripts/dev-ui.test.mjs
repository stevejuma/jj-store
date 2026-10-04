import assert from "node:assert/strict";
import { mkdtemp, rm, mkdir, writeFile, readFile } from "node:fs/promises";
import { spawn } from "node:child_process";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import {
  uiDevelopmentSettings,
  uiDevelopmentServer,
  rewriteUiBackendConfiguration,
} from "./dev-ui-config.mjs";
import { prepareUiDevelopment, runUiDevelopment } from "./dev-ui.mjs";
import { developmentLayout, writeFixture } from "./dev-demo-state.mjs";

test("HMR proxy preserves the public origin and streams every versioned API route", () => {
  const settings = uiDevelopmentSettings({});
  assert.equal(settings.origin, "http://127.0.0.1:7460/");
  assert.equal(settings.backendOrigin, "http://127.0.0.1:7462/");
  const server = uiDevelopmentServer({});
  assert.equal(server.host, "127.0.0.1");
  assert.equal(server.port, 7460);
  assert.equal(server.strictPort, true);
  assert.equal(server.cors, false);
  assert.ok(server.fs.deny.includes("**/.local/**"));
  assert.ok(server.fs.deny.includes("**/.jj/**"));
  assert.deepEqual(server.proxy, {
    "/v1": { target: settings.backendOrigin, changeOrigin: false },
  });
  assert.match(
    server.headers["Content-Security-Policy"],
    /ws:\/\/127\.0\.0\.1:7460/u,
  );
  assert.doesNotMatch(
    server.headers["Content-Security-Policy"],
    /unsafe-eval/u,
  );
});

test("UI development rejects invalid and overlapping ports", () => {
  for (const port of ["0", "65536", "7460oops", "1.5", ""]) {
    assert.throws(
      () => uiDevelopmentSettings({ JJ_STORE_DEV_PORT: port }),
      /port/u,
    );
  }
  assert.throws(
    () => uiDevelopmentSettings({ JJ_STORE_DEV_BACKEND_PORT: "7460" }),
    /different/u,
  );
  assert.equal(
    uiDevelopmentSettings({ JJ_STORE_DEV_PORT: "7500" }).backendPort,
    7502,
  );
});

test("UI backend configuration keeps storage paths and removes built UI dependence", () => {
  const source = `format = 2
bind = "127.0.0.1:7460"
public_url = "http://127.0.0.1:7460/"
state_dir = "/fixture/local"
web_root = "/workspace/web/dist"
development_browser_annotations = true
runtime_executable = "/fixture/rustfs"
`;
  const settings = uiDevelopmentSettings({});
  const rewritten = rewriteUiBackendConfiguration(source, settings);
  assert.match(rewritten, /^bind = "127\.0\.0\.1:7462"$/mu);
  assert.match(rewritten, /^public_url = "http:\/\/127\.0\.0\.1:7460\/"$/mu);
  assert.match(rewritten, /^state_dir = "\/fixture\/local"$/mu);
  assert.match(rewritten, /^runtime_executable = "\/fixture\/rustfs"$/mu);
  assert.match(rewritten, /^development_browser_annotations = false$/mu);
  assert.doesNotMatch(rewritten, /^web_root =/mu);
  assert.equal(rewriteUiBackendConfiguration(rewritten, settings), rewritten);
});

test("UI-only startup requires an existing fixture before touching a running service", async (t) => {
  const root = await mkdtemp(join(tmpdir(), "jj-store-ui-preflight-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  await assert.rejects(prepareUiDevelopment(root, {}), /pnpm dev:restart/u);
});

test("an occupied backend port leaves the current service and runtime marker intact", async (t) => {
  const root = await mkdtemp(join(tmpdir(), "jj-store-ui-port-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  const blocker = createServer();
  await new Promise((ready) => blocker.listen(0, "127.0.0.1", ready));
  t.after(() => new Promise((ready) => blocker.close(ready)));
  const backendPort = blocker.address().port;
  const uiPort = backendPort - 1;
  const layout = developmentLayout(root, {});
  await mkdir(layout.repository, { recursive: true });
  await mkdir(layout.rustfsVolume, { recursive: true });
  await writeFile(
    layout.serverConfig,
    `format = 2\nbind = "127.0.0.1:${uiPort}"\npublic_url = "http://127.0.0.1:${uiPort}/"\nruntime_executable = ${JSON.stringify(process.execPath)}\n`,
  );
  await writeFixture(layout, {
    repositoryId: "11".repeat(16),
    firstStartupMillis: 0,
  });
  const running = spawn(
    process.execPath,
    [
      "-e",
      "setInterval(() => {}, 1000)",
      "--",
      "jj-store",
      "run",
      "--config",
      layout.serverConfig,
    ],
    { stdio: "ignore" },
  );
  await new Promise((ready, reject) => {
    running.once("spawn", ready);
    running.once("error", reject);
  });
  t.after(() => running.kill());
  const marker = JSON.stringify({
    serverPid: running.pid,
    serverConfig: layout.serverConfig,
    origin: `http://127.0.0.1:${uiPort}/`,
  });
  await writeFile(layout.runtimeFile, marker);
  await assert.rejects(
    runUiDevelopment(root, {
      JJ_STORE_DEV_PORT: String(uiPort),
      JJ_STORE_DEV_BACKEND_PORT: String(backendPort),
      JJ_STORE_DEV_SERVER: process.execPath,
    }),
    { code: "EADDRINUSE" },
  );
  assert.doesNotThrow(() => process.kill(running.pid, 0));
  assert.equal(await readFile(layout.runtimeFile, "utf8"), marker);
});

test("matching HMR backend restart retains app-saved configuration and refuses a missing retained file", async (t) => {
  const root = await mkdtemp(join(tmpdir(), "jj-store-ui-settings-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  const layout = developmentLayout(root, {});
  await mkdir(layout.repository, { recursive: true });
  await mkdir(layout.rustfsVolume, { recursive: true });
  const base = `format = 2\nbind = "127.0.0.1:7460"\npublic_url = "http://127.0.0.1:7460/"\nruntime_executable = ${JSON.stringify(process.execPath)}\n[agent_capture]\nenabled = false\n`;
  await writeFile(layout.serverConfig, base);
  const fixture = { repositoryId: "11".repeat(16), firstStartupMillis: 0 };
  await writeFixture(layout, fixture);
  const initial = await prepareUiDevelopment(root, {
    JJ_STORE_DEV_SERVER: process.execPath,
  });
  await writeFile(
    initial.serverConfig,
    initial.configuration.replace("enabled = false", "enabled = true"),
  );
  await writeFile(
    layout.runtimeFile,
    JSON.stringify({
      mode: "vite",
      workspaceRoot: root,
      runtimeRoot: layout.runtimeRoot,
      serverConfig: initial.serverConfig,
      repositoryId: fixture.repositoryId,
    }),
  );
  const saved = await prepareUiDevelopment(root, {
    JJ_STORE_DEV_SERVER: process.execPath,
  });
  assert.match(saved.configuration, /enabled = true/u);
  assert.match(await readFile(layout.serverConfig, "utf8"), /enabled = false/u);
  await rm(initial.serverConfig);
  await assert.rejects(
    prepareUiDevelopment(root, { JJ_STORE_DEV_SERVER: process.execPath }),
    { code: "ENOENT" },
  );
});
