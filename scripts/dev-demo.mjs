#!/usr/bin/env node

import { spawn } from "node:child_process";
import { openSync } from "node:fs";
import { chmod, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import {
  execute,
  processTreeMetrics,
  stopPrevious,
  waitForServer,
} from "./dev-demo-services.mjs";
import { seedRepository } from "./dev-demo-repository.mjs";

const repositoryRoot = resolve(import.meta.dirname, "..");
const userSuffix =
  typeof process.getuid === "function" ? process.getuid() : "user";
const runtimeRoot = join(tmpdir(), `jj-store-dev-${userSuffix}`);
const runtimeFile = join(runtimeRoot, "runtime.json");
const serverPort = Number(process.env.JJ_STORE_DEV_PORT ?? "7460");

async function run(command, args, options = {}) {
  return execute(command, args, {
    cwd: repositoryRoot,
    env: options.env,
  });
}

function startServer(serverConfig, logHandle) {
  const server = spawn(
    resolve(repositoryRoot, "target/debug/jj-store"),
    ["run", "--config", serverConfig, "--format=json"],
    {
      cwd: repositoryRoot,
      detached: true,
      env: process.env,
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

async function restart() {
  await stopPrevious({
    owner: repositoryRoot,
    runtimeFile,
    runtimeRoot,
  });
  try {
    await fetch(`http://127.0.0.1:${serverPort}/`);
    throw new Error(`port ${serverPort} is occupied by an unowned service`);
  } catch (error) {
    if (!String(error.message).includes("fetch failed")) throw error;
  }
  await rm(runtimeRoot, { recursive: true, force: true });
  await mkdir(runtimeRoot, { recursive: true, mode: 0o700 });
  await run("pnpm", ["web:build"]);
  await run("pnpm", ["build:embedded-rustfs"]);
  await run("cargo", ["build", "-p", "jj-store-cli", "--bins", "--locked"]);
  const origin = `http://127.0.0.1:${serverPort}/`;
  const stateRoot = join(runtimeRoot, "local");
  const rustfsRuntime = resolve(
    repositoryRoot,
    "runtime/embedded-rustfs/target/debug/jj-store-rustfs",
  );
  await run(resolve(repositoryRoot, "target/debug/jj-store"), [
    "init",
    "--local",
    "--state-dir",
    stateRoot,
    "--rustfs-executable",
    rustfsRuntime,
    "--format=json",
  ]);
  const serverConfig = join(stateRoot, "server.toml");
  const unixSocket = join(stateRoot, "native.sock");
  const generated = await readFile(serverConfig, "utf8");
  await writeFile(
    serverConfig,
    generated
      .replace('bind = "127.0.0.1:7443"', `bind = "127.0.0.1:${serverPort}"`)
      .replace(
        'public_url = "http://127.0.0.1:7443/"',
        `public_url = "${origin}"`,
      )
      .replace(
        /operator_policy = (.+)\n/u,
        `operator_policy = $1\nweb_root = ${JSON.stringify(join(repositoryRoot, "web/dist"))}\n`,
      ),
    { mode: 0o600 },
  );
  const logPath = join(runtimeRoot, "server.log");
  const logHandle = openSync(logPath, "a", 0o600);
  let started = performance.now();
  let server = startServer(serverConfig, logHandle);
  await waitForServer(origin);
  const firstStartupMillis = Math.round(performance.now() - started);
  const repositoryId = await seedRepository({
    repositoryRoot,
    runtimeRoot,
    run,
    unixSocket,
  });
  await stopServer(server);
  await run(resolve(repositoryRoot, "target/debug/jj-store"), [
    "repository",
    "register",
    "--config",
    serverConfig,
    "--repo-id",
    repositoryId,
    "--name",
    "jj-store demo",
    "--format=json",
  ]);
  started = performance.now();
  server = startServer(serverConfig, logHandle);
  await waitForServer(origin);
  const restartStartupMillis = Math.round(performance.now() - started);
  const processes = await processTreeMetrics(server.pid);
  const runtime = {
    format: 2,
    accessMode: "local-trusted",
    storageMode: "embedded-rustfs",
    serverPid: server.pid,
    origin,
    repositoryId,
    runtimeRoot,
    serverConfig,
    resources: {
      firstStartupMillis,
      restartStartupMillis,
      ...processes,
    },
  };
  await writeFile(runtimeFile, `${JSON.stringify(runtime, null, 2)}\n`, {
    mode: 0o600,
  });
  await chmod(runtimeRoot, 0o700);
  console.log(
    `JJ Store LocalTrusted development demo is ready.\n\nURL: ${origin}\nRepository ID: ${repositoryId}\nCredentials: none (local loopback access)\nRuntime: ${runtimeRoot}\nStorage: embedded RustFS\nStartup: ${firstStartupMillis} ms cold, ${restartStartupMillis} ms restart\nResident memory: ${processes.totalRssKiB} KiB server plus storage\n\nThe repository, installation identity and RustFS volume persist under the runtime directory until pnpm dev:stop.`,
  );
}

async function stop() {
  await stopPrevious({
    owner: repositoryRoot,
    runtimeFile,
    runtimeRoot,
  });
  await rm(runtimeRoot, { recursive: true, force: true });
  console.log("JJ Store development demo stopped.");
}

const action = process.argv[2] ?? "restart";
try {
  if (action === "restart") await restart();
  else if (action === "stop") await stop();
  else throw new Error(`unknown action ${action}; expected restart or stop`);
} catch (error) {
  console.error(error.stderr || error.stack || error.message);
  process.exitCode = 1;
}
