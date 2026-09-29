#!/usr/bin/env node

import { spawn } from "node:child_process";
import { openSync } from "node:fs";
import { chmod, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import {
  execute,
  processTreeMetrics,
  stopPrevious,
  waitForServer,
} from "./dev-demo-services.mjs";
import { seedRepository } from "./dev-demo-repository.mjs";
import {
  developmentLayout,
  loadFixture,
  rewriteServerConfiguration,
  writeFixture,
} from "./dev-demo-state.mjs";

const repositoryRoot = resolve(import.meta.dirname, "..");
const layout = developmentLayout(repositoryRoot);
const serverPort = Number(process.env.JJ_STORE_DEV_PORT ?? "7460");
const developmentEnvironment = {
  JJ_STORE_DEVELOPMENT_STATE_ROOT: layout.runtimeRoot,
};

async function run(command, args, options = {}) {
  return execute(command, args, {
    cwd: repositoryRoot,
    env: { ...developmentEnvironment, ...options.env },
  });
}

function startServer(serverConfig, logHandle) {
  const server = spawn(
    resolve(repositoryRoot, "target/debug/jj-store"),
    ["run", "--config", serverConfig, "--format=json"],
    {
      cwd: repositoryRoot,
      detached: true,
      env: { ...process.env, ...developmentEnvironment },
      stdio: ["ignore", logHandle, logHandle],
    },
  );
  server.unref();
  return server;
}

async function stopServer(server) {
  if (!server.pid) return;
  process.kill(server.pid, "SIGTERM");
  for (let attempt = 0; attempt < 100; attempt += 1) {
    try {
      process.kill(server.pid, 0);
      await new Promise((resolveWait) => setTimeout(resolveWait, 100));
    } catch {
      return;
    }
  }
  throw new Error("temporary native server did not stop");
}

async function configureSearch(serverConfig, repositoryId) {
  const generated = await readFile(serverConfig, "utf8");
  const search = `
[[search]]
repository = "${repositoryId}"
poll_millis = 100
timeout_seconds = 30
snapshot_every = 32

[search.source.scopes.main]
kind = "bookmark"
name = "main"
`;
  await writeFile(
    serverConfig,
    `${generated.replace(/^search = \[\]\n/mu, "")}${search}`,
    { mode: 0o600 },
  );
}

async function waitForSearch(origin, repositoryId) {
  const deadline = performance.now() + 120_000;
  const url = `${origin}v1/repos/${repositoryId}/search/status`;
  let lastResponse = "no response";
  while (performance.now() < deadline) {
    try {
      const response = await fetch(url, {
        headers: {
          Origin: new URL(origin).origin,
          "Sec-Fetch-Dest": "empty",
          "Sec-Fetch-Mode": "cors",
          "Sec-Fetch-Site": "same-origin",
          "X-JJ-Store-Protocol": "1",
        },
      });
      const searchState = await response.json();
      lastResponse = `${response.status} ${JSON.stringify(searchState)}`;
      if (
        response.ok &&
        searchState.index_generation &&
        searchState.is_stale === false &&
        searchState.degraded === false
      ) {
        return searchState;
      }
    } catch (error) {
      lastResponse = String(error.message ?? error);
    }
    await new Promise((resolveWait) => setTimeout(resolveWait, 100));
  }
  throw new Error(
    `native search did not become ready within 120 seconds: ${lastResponse}`,
  );
}

async function assertPortAvailable() {
  try {
    await fetch(`http://127.0.0.1:${serverPort}/`);
    throw new Error(`port ${serverPort} is occupied by an unowned service`);
  } catch (error) {
    if (!String(error.message).includes("fetch failed")) throw error;
  }
}

async function refreshServerConfiguration(origin) {
  const generated = await readFile(layout.serverConfig, "utf8");
  await writeFile(
    layout.serverConfig,
    rewriteServerConfiguration(generated, {
      origin,
      webRoot: join(repositoryRoot, "web/dist"),
    }),
    { mode: 0o600 },
  );
}

async function initializeFixture(origin, logHandle) {
  const rustfsRuntime = resolve(
    repositoryRoot,
    "runtime/embedded-rustfs/target/debug/jj-store-rustfs",
  );
  await run(resolve(repositoryRoot, "target/debug/jj-store"), [
    "init",
    "--local",
    "--state-dir",
    layout.stateRoot,
    "--rustfs-executable",
    rustfsRuntime,
    "--format=json",
  ]);
  await refreshServerConfiguration(origin);
  const started = performance.now();
  const server = startServer(layout.serverConfig, logHandle);
  let repositoryId;
  let firstStartupMillis;
  try {
    await waitForServer(origin);
    firstStartupMillis = Math.round(performance.now() - started);
    repositoryId = await seedRepository({
      repositoryRoot,
      runtimeRoot: layout.runtimeRoot,
      run,
      unixSocket: layout.unixSocket,
    });
  } finally {
    await stopServer(server);
  }
  await run(resolve(repositoryRoot, "target/debug/jj-store"), [
    "repository",
    "register",
    "--config",
    layout.serverConfig,
    "--repo-id",
    repositoryId,
    "--name",
    "jj-store demo",
    "--format=json",
  ]);
  await configureSearch(layout.serverConfig, repositoryId);
  return writeFixture(layout, {
    firstStartupMillis,
    repositoryId,
  });
}

async function restart({ reset = false } = {}) {
  await stopPrevious({
    owner: repositoryRoot,
    runtimeFile: layout.runtimeFile,
    runtimeRoot: layout.runtimeRoot,
  });
  if (reset) await rm(layout.runtimeRoot, { recursive: true, force: true });
  await assertPortAvailable();
  await run("pnpm", ["web:build"]);
  await run("pnpm", ["build:embedded-rustfs"]);
  await run("cargo", ["build", "-p", "jj-store-cli", "--bins", "--locked"]);
  const origin = `http://127.0.0.1:${serverPort}/`;
  let fixture = await loadFixture(layout);
  await mkdir(layout.runtimeRoot, { recursive: true, mode: 0o700 });
  const logHandle = openSync(layout.logPath, "a", 0o600);
  if (fixture) await refreshServerConfiguration(origin);
  else fixture = await initializeFixture(origin, logHandle);
  const started = performance.now();
  const server = startServer(layout.serverConfig, logHandle);
  try {
    await waitForServer(origin);
    await waitForSearch(origin, fixture.repositoryId);
  } catch (error) {
    await stopServer(server);
    throw error;
  }
  const restartStartupMillis = Math.round(performance.now() - started);
  const processes = await processTreeMetrics(server.pid);
  const runtime = {
    format: 2,
    accessMode: "local-trusted",
    storageMode: "embedded-rustfs",
    serverPid: server.pid,
    origin,
    repositoryId: fixture.repositoryId,
    runtimeRoot: layout.runtimeRoot,
    serverConfig: layout.serverConfig,
    resources: {
      firstStartupMillis: fixture.firstStartupMillis,
      restartStartupMillis,
      ...processes,
    },
  };
  await writeFile(layout.runtimeFile, `${JSON.stringify(runtime, null, 2)}\n`, {
    mode: 0o600,
  });
  await chmod(layout.runtimeRoot, 0o700);
  console.log(
    `JJ Store LocalTrusted development demo is ready.\n\nURL: ${origin}\nRepository ID: ${fixture.repositoryId}\nCredentials: none (local loopback access)\nRuntime: ${layout.runtimeRoot}\nStorage: embedded RustFS\nFixture: ${reset ? "reset and seeded" : "reused or initialized"}\nSearch: ready\nStartup: ${fixture.firstStartupMillis} ms cold, ${restartStartupMillis} ms restart\nResident memory: ${processes.totalRssKiB} KiB server plus storage\n\nThe repository, installation identity and RustFS volume persist across restart and stop. Run pnpm dev:reset to replace them with a deterministic sample fixture.`,
  );
}

async function stop() {
  await stopPrevious({
    owner: repositoryRoot,
    runtimeFile: layout.runtimeFile,
    runtimeRoot: layout.runtimeRoot,
  });
  console.log(
    `JJ Store development demo stopped. Preserved state: ${layout.runtimeRoot}`,
  );
}

const action = process.argv[2] ?? "restart";
try {
  if (action === "restart") await restart();
  else if (action === "reset") await restart({ reset: true });
  else if (action === "stop") await stop();
  else
    throw new Error(
      `unknown action ${action}; expected restart, reset or stop`,
    );
} catch (error) {
  console.error(error.stderr || error.stack || error.message);
  process.exitCode = 1;
}
