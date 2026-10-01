#!/usr/bin/env node

import { spawn } from "node:child_process";
import { openSync } from "node:fs";
import { chmod, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import {
  execute,
  executeWithInput,
  processTreeMetrics,
  stopPrevious,
  waitForServer,
} from "./dev-demo-services.mjs";
import { seedAgentActivity } from "./dev-demo-agents.mjs";
import { seedReviews } from "./dev-demo-reviews.mjs";
import { seedRepository } from "./dev-demo-repository.mjs";
import { seedWork } from "./dev-demo-work.mjs";
import {
  configureSearch,
  repositoryPosition,
  waitForSearch,
} from "./dev-demo-search.mjs";
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

async function runInput(command, args, input, options = {}) {
  return executeWithInput(command, args, input, {
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
      browserAnnotations: process.env.JJ_STORE_DEV_BROWSER_ANNOTATIONS !== "0",
    }),
    { mode: 0o600 },
  );
}

async function initializeFixture(origin, logHandle) {
  const rustfsRuntime = resolve(
    process.env.JJ_STORE_DEV_RUSTFS_RUNTIME ??
      join(
        repositoryRoot,
        "runtime/embedded-rustfs/target/debug/jj-store-rustfs",
      ),
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
  let seed;
  let firstStartupMillis;
  try {
    await waitForServer(origin);
    firstStartupMillis = Math.round(performance.now() - started);
    seed = await seedRepository({
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
    seed.repositoryId,
    "--name",
    "jj-store demo",
    "--format=json",
  ]);
  await configureSearch(layout.serverConfig, seed.repositoryId);
  return {
    fixture: {
      firstStartupMillis,
      repositoryId: seed.repositoryId,
    },
    references: seed.references,
  };
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
  if (!process.env.JJ_STORE_DEV_RUSTFS_RUNTIME)
    await run("pnpm", ["build:embedded-rustfs"]);
  await run("cargo", ["build", "-p", "jj-store-cli", "--bins", "--locked"]);
  const origin = `http://127.0.0.1:${serverPort}/`;
  let fixture = await loadFixture(layout);
  let references;
  await mkdir(layout.runtimeRoot, { recursive: true, mode: 0o700 });
  const logHandle = openSync(layout.logPath, "a", 0o600);
  if (fixture) await refreshServerConfiguration(origin);
  else {
    const initialized = await initializeFixture(origin, logHandle);
    fixture = initialized.fixture;
    references = initialized.references;
  }
  const started = performance.now();
  let server = startServer(layout.serverConfig, logHandle);
  let restartStartupMillis;
  let fixturePreparationMillis = 0;
  let searchConfigured = true;
  try {
    await waitForServer(origin);
    searchConfigured =
      (await waitForSearch(origin, fixture.repositoryId)).configured !== false;
    restartStartupMillis = Math.round(performance.now() - started);
    if (references) {
      await seedAgentActivity({
        repositoryRoot,
        runtimeRoot: layout.runtimeRoot,
        repository: layout.repository,
        run,
        runInput,
      });
      await seedReviews({
        origin,
        repositoryId: fixture.repositoryId,
        references,
      });
      await seedWork({ origin, repositoryId: fixture.repositoryId });
      await waitForSearch(
        origin,
        fixture.repositoryId,
        await repositoryPosition(origin, fixture.repositoryId),
      );
      fixturePreparationMillis = Math.round(performance.now() - started);
      await stopServer(server);
      const restartStarted = performance.now();
      server = startServer(layout.serverConfig, logHandle);
      await waitForServer(origin);
      await waitForSearch(origin, fixture.repositoryId);
      restartStartupMillis = Math.round(performance.now() - restartStarted);
    }
  } catch (error) {
    await stopServer(server);
    throw error;
  }
  if (references) fixture = await writeFixture(layout, fixture);
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
      fixturePreparationMillis,
      ...processes,
    },
  };
  await writeFile(layout.runtimeFile, `${JSON.stringify(runtime, null, 2)}\n`, {
    mode: 0o600,
  });
  await chmod(layout.runtimeRoot, 0o700);
  console.log(
    `JJ Store LocalTrusted development demo is ready.\n\nURL: ${origin}\nRepository ID: ${fixture.repositoryId}\nCredentials: none (local loopback access)\nRuntime: ${layout.runtimeRoot}\nStorage: embedded RustFS\nFixture: ${reset ? "reset and seeded" : "reused or initialized"}\nSearch: ${searchConfigured ? "ready" : "disabled"}\nStartup: ${fixture.firstStartupMillis} ms cold, ${restartStartupMillis} ms restart\nResident memory: ${processes.totalRssKiB} KiB server plus storage\n\nThe repository, installation identity and RustFS volume persist across restart and stop. Run pnpm dev:reset to replace them with a deterministic sample fixture.`,
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

async function seedSampleWork() {
  const fixture = await loadFixture(layout);
  if (!fixture)
    throw new Error(
      "start the owned development demo with pnpm dev:restart first",
    );
  const runtime = JSON.parse(await readFile(layout.runtimeFile, "utf8"));
  if (
    runtime.runtimeRoot !== layout.runtimeRoot ||
    runtime.serverConfig !== layout.serverConfig ||
    runtime.repositoryId !== fixture.repositoryId ||
    runtime.origin !== `http://127.0.0.1:${serverPort}/`
  )
    throw new Error(
      "development work seed requires the matching owned LocalTrusted demo runtime",
    );
  const ownedProcess = await run("ps", [
    "-p",
    String(runtime.serverPid),
    "-o",
    "command=",
  ]);
  if (!ownedProcess.stdout.includes(layout.serverConfig))
    throw new Error("development work seed refuses an unowned process");
  const result = await seedWork({
    origin: runtime.origin,
    repositoryId: fixture.repositoryId,
  });
  console.log(
    `Development work samples ready: ${result.projects} projects, ${result.tasks} tasks, ${result.comments} comments; ${result.publications} new publications. Existing records preserved. Refresh projects to load the samples.`,
  );
}

const action = process.argv[2] ?? "restart";
try {
  if (action === "restart") await restart();
  else if (action === "reset") await restart({ reset: true });
  else if (action === "stop") await stop();
  else if (action === "seed-work") await seedSampleWork();
  else
    throw new Error(
      `unknown action ${action}; expected restart, reset, stop or seed-work`,
    );
} catch (error) {
  console.error(error.stderr || error.stack || error.message);
  process.exitCode = 1;
}
