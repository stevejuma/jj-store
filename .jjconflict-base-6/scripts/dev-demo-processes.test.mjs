import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { stopRuntimeProcesses } from "./dev-demo-processes.mjs";
import { stopPrevious } from "./dev-demo-services.mjs";

async function child(t, args) {
  const child = spawn(
    process.execPath,
    ["-e", "setInterval(() => {}, 1000)", "--", ...args],
    { stdio: "ignore" },
  );
  await new Promise((ready, reject) => {
    child.once("spawn", ready);
    child.once("error", reject);
  });
  t.after(() => {
    if (child.exitCode === null) child.kill();
  });
  return child.pid;
}

test("stop validates both process owners before signaling either", async (t) => {
  const runtimeRoot = "/fixture/owned";
  const uiRunner = "/workspace/scripts/dev-ui-vite.mjs";
  const uiPid = await child(t, [uiRunner, "--runtime-root", runtimeRoot]);
  const serverPid = await child(t, ["unrelated-service"]);
  await assert.rejects(
    stopRuntimeProcesses(
      { uiPid, uiRunner, serverPid, serverConfig: "/fixture/server.toml" },
      { owner: process.cwd(), runtimeRoot },
    ),
    /unrelated server/u,
  );
  assert.doesNotThrow(() => process.kill(uiPid, 0));
  assert.doesNotThrow(() => process.kill(serverPid, 0));
});

test("development stop closes Vite and backend while preserving the fixture", async (t) => {
  const runtimeRoot = await mkdtemp(join(tmpdir(), "jj-store-process-stop-"));
  t.after(() => rm(runtimeRoot, { recursive: true, force: true }));
  const uiRunner = "/workspace/scripts/dev-ui-vite.mjs";
  const serverConfig = join(runtimeRoot, "ui-server.toml");
  const uiPid = await child(t, [uiRunner, "--runtime-root", runtimeRoot]);
  const serverPid = await child(t, [
    "jj-store",
    "run",
    "--config",
    serverConfig,
  ]);
  const runtimeFile = join(runtimeRoot, "runtime.json");
  await writeFile(
    runtimeFile,
    JSON.stringify({ uiPid, uiRunner, serverPid, serverConfig }),
  );
  await writeFile(join(runtimeRoot, "fixture.json"), "preserved");
  await stopPrevious({ owner: process.cwd(), runtimeRoot, runtimeFile });
  assert.throws(() => process.kill(uiPid, 0), { code: "ESRCH" });
  assert.throws(() => process.kill(serverPid, 0), { code: "ESRCH" });
  assert.equal(
    await readFile(join(runtimeRoot, "fixture.json"), "utf8"),
    "preserved",
  );
  await assert.rejects(readFile(runtimeFile), { code: "ENOENT" });
});

test("backend-only restart leaves the owned Vite process running", async (t) => {
  const runtimeRoot = "/fixture/backend-only";
  const uiRunner = "/workspace/scripts/dev-ui-vite.mjs";
  const serverConfig = `${runtimeRoot}/ui-server.toml`;
  const uiPid = await child(t, [uiRunner, "--runtime-root", runtimeRoot]);
  const serverPid = await child(t, [
    "jj-store",
    "run",
    "--config",
    serverConfig,
  ]);
  await stopRuntimeProcesses(
    { uiPid, uiRunner, serverPid, serverConfig },
    { owner: process.cwd(), runtimeRoot, ui: false },
  );
  assert.doesNotThrow(() => process.kill(uiPid, 0));
  assert.throws(() => process.kill(serverPid, 0), { code: "ESRCH" });
});
