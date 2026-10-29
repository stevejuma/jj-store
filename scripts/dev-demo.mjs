#!/usr/bin/env node

import {
  startDevelopmentServer,
  stopDevelopmentServer,
} from "./dev-demo-server.mjs";
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
import { seedRepositoryProfiles } from "./dev-demo-profiles.mjs";
import { registerDemoLocalFolder } from "./dev-demo-local-folder.mjs";
import { linkDemoFolder } from "./dev-demo-folder.mjs";
import { seedSampleWork } from "./dev-demo-work-run.mjs";
import { seedThreads } from "./dev-demo-threads.mjs";
import { seedWork } from "./dev-demo-work.mjs";
import { developmentAgentEnvironment } from "./dev-agent-runtime.mjs";
import { prebuiltBrowserResources } from "./ci-prebuilt.mjs";
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
    cwd: options.cwd ?? repositoryRoot,
    env: { ...developmentEnvironment, ...options.env },
  });
}

async function runInput(command, args, input, options = {}) {
  return executeWithInput(command, args, input, {
    cwd: options.cwd ?? repositoryRoot,
    env: { ...developmentEnvironment, ...options.env },
  });
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
  const server = startDevelopmentServer(
    repositoryRoot,
    developmentEnvironment,
    layout.serverConfig,
    logHandle,
  );
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
    await stopDevelopmentServer(server);
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
  const prebuilt = await prebuiltBrowserResources(repositoryRoot);
  if (!prebuilt) {
    await run("pnpm", ["ai:prepare"]);
    await run("cargo", ["build", "-p", "jj-store", "--bins", "--locked"]);
  }
  Object.assign(
    developmentEnvironment,
    await developmentAgentEnvironment(repositoryRoot),
  );
  await stopPrevious({
    owner: repositoryRoot,
    runtimeFile: layout.runtimeFile,
    runtimeRoot: layout.runtimeRoot,
  });
  if (reset) await rm(layout.runtimeRoot, { recursive: true, force: true });
  await assertPortAvailable();
  if (!prebuilt) await run("pnpm", ["web:build"]);
  if (!process.env.JJ_STORE_DEV_RUSTFS_RUNTIME)
    await run("pnpm", ["build:embedded-rustfs"]);
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
  let server = startDevelopmentServer(
    repositoryRoot,
    developmentEnvironment,
    layout.serverConfig,
    logHandle,
  );
  let restartStartupMillis;
  let fixturePreparationMillis = 0;
  let searchConfigured = true;
  let profiles;
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
    }
    profiles = await seedRepositoryProfiles({
      repositoryRoot,
      runtimeRoot: layout.runtimeRoot,
      repositoryId: fixture.repositoryId,
      unixSocket: layout.unixSocket,
      run,
    });
    await registerDemoLocalFolder({
      origin,
      repositoryId: profiles.localGit,
      path: profiles.localFolder,
    });
    await linkDemoFolder({
      origin,
      repositoryId: fixture.repositoryId,
      path: layout.repository,
    });
    await seedThreads({
      origin,
      runtimeRoot: layout.runtimeRoot,
      repositories: [fixture.repositoryId, profiles.localGit],
    });
    const work = await seedWork({
      origin,
      repositoryId: fixture.repositoryId,
      repositoryLinks: profiles.repositoryLinks,
    });
    if (references || work.publications) {
      await waitForSearch(
        origin,
        fixture.repositoryId,
        await repositoryPosition(origin, fixture.repositoryId),
      );
      fixturePreparationMillis = Math.round(performance.now() - started);
      await stopDevelopmentServer(server);
      const restartStarted = performance.now();
      server = startDevelopmentServer(
        repositoryRoot,
        developmentEnvironment,
        layout.serverConfig,
        logHandle,
      );
      await waitForServer(origin);
      await waitForSearch(origin, fixture.repositoryId);
      restartStartupMillis = Math.round(performance.now() - restartStarted);
    }
  } catch (error) {
    await stopDevelopmentServer(server);
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
    repositoryProfiles: profiles,
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
    `JJ Store LocalTrusted development demo is ready.\n\nURL: ${origin}\nRepository ID: ${fixture.repositoryId}\nServer-only native: ${profiles.nativeServer}\nLocal Git-backed JJ: ${profiles.localGit} (${profiles.localFolder}; browser ready)\nCredentials: none (local loopback access)\nRuntime: ${layout.runtimeRoot}\nStorage: embedded RustFS\nFixture: ${reset ? "reset and seeded" : "reused or initialized"}\nSearch: ${searchConfigured ? "ready" : "disabled"}\nStartup: ${fixture.firstStartupMillis} ms cold, ${restartStartupMillis} ms restart\nResident memory: ${processes.totalRssKiB} KiB server plus storage\n\nThe repository, installation identity and RustFS volume persist across restart and stop. Run pnpm dev:reset to replace them with a deterministic sample fixture.`,
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
  else if (action === "seed-work")
    await seedSampleWork({ layout, repositoryRoot, serverPort, run });
  else
    throw new Error(
      `unknown action ${action}; expected restart, reset, stop or seed-work`,
    );
} catch (error) {
  console.error(error.stderr || error.stack || error.message);
  process.exitCode = 1;
}
