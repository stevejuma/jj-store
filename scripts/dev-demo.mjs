#!/usr/bin/env node

import { randomBytes } from "node:crypto";
import { spawn } from "node:child_process";
import { openSync } from "node:fs";
import { appendFile, chmod, mkdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import {
  createBucket,
  credential,
  execute,
  stopPrevious,
  waitForServer,
} from "./dev-demo-services.mjs";
import { seedRepository } from "./dev-demo-repository.mjs";

const repositoryRoot = resolve(import.meta.dirname, "..");
const userSuffix =
  typeof process.getuid === "function" ? process.getuid() : "user";
const runtimeRoot = join(tmpdir(), `jj-store-dev-${userSuffix}`);
const runtimeFile = join(runtimeRoot, "runtime.json");
const containerName = `jj-store-dev-${userSuffix}`;
const serverPort = Number(process.env.JJ_STORE_DEV_PORT ?? "7460");
const image =
  "rustfs/rustfs@sha256:b7014e0ce2bc703c1316b3ef760e29dfae61fe4a50d1a66fa89638e0f8ea211f";
const accessKey = "jj-store-disposable-test";
const secretKey = "jj-store-disposable-test-secret-only";
const bucket = "jj-store-dev";

async function run(command, args, options = {}) {
  return execute(command, args, {
    cwd: repositoryRoot,
    env: options.env,
  });
}

function startServer(serverConfig, logHandle, environment = {}) {
  const server = spawn(
    resolve(repositoryRoot, "target/debug/jj-store"),
    ["run", "--config", serverConfig, "--format=json"],
    {
      cwd: repositoryRoot,
      detached: true,
      env: {
        ...process.env,
        JJ_STORE_S3_ACCESS_KEY: accessKey,
        JJ_STORE_S3_SECRET_KEY: secretKey,
        ...environment,
      },
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

async function waitForSearch(origin, repositoryId, token) {
  for (let attempt = 0; attempt < 300; attempt += 1) {
    try {
      const response = await fetch(
        `${origin}v1/repos/${repositoryId}/search/status`,
        { headers: { authorization: `Bearer ${token}` } },
      );
      if (response.ok) {
        const status = await response.json();
        if (status.indexed_position !== null && status.is_stale === false)
          return;
      } else await response.body?.cancel();
    } catch {
      // The search worker is rebuilding its first generation.
    }
    await new Promise((resolveWait) => setTimeout(resolveWait, 100));
  }
  throw new Error("jj-store search worker did not become ready");
}

async function restart() {
  await stopPrevious({
    containerName,
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
  await run("cargo", ["build", "-p", "jj-store-cli", "--bins", "--locked"]);
  const { stdout: containerId } = await run("docker", [
    "run",
    "--detach",
    "--name",
    containerName,
    "--label",
    `org.lapismd.jj-store.dev=${repositoryRoot}`,
    "--publish",
    "127.0.0.1::9000",
    "--env",
    `RUSTFS_ACCESS_KEY=${accessKey}`,
    "--env",
    `RUSTFS_SECRET_KEY=${secretKey}`,
    "--env",
    "RUSTFS_VOLUMES=/data",
    "--env",
    "RUSTFS_ADDRESS=0.0.0.0:9000",
    "--env",
    "RUSTFS_CONSOLE_ENABLE=false",
    "--tmpfs",
    "/data:rw,mode=1777",
    image,
  ]);
  const { stdout: portText } = await run("docker", [
    "port",
    containerName,
    "9000/tcp",
  ]);
  const storagePort = Number(portText.trim().split(":").at(-1));
  await createBucket({ accessKey, bucket, secretKey, storagePort });
  const origin = `http://127.0.0.1:${serverPort}/`;
  const operatorToken = credential();
  const adminToken = credential();
  const operatorPolicy = join(runtimeRoot, "operators.json");
  await run(
    resolve(repositoryRoot, "target/debug/jj-store"),
    [
      "operator",
      "init",
      "--policy",
      operatorPolicy,
      "--endpoint",
      origin.slice(0, -1),
      "--insecure-loopback",
      "--principal",
      randomBytes(16).toString("hex"),
      "--format=json",
    ],
    { env: { JJ_STORE_OPERATOR_TOKEN: operatorToken } },
  );
  const serverConfig = join(runtimeRoot, "server.toml");
  await writeFile(
    serverConfig,
    `format = 1\nbind = "127.0.0.1:${serverPort}"\npublic_url = "${origin}"\ninsecure_loopback = true\nstate_dir = "${join(runtimeRoot, "state")}"\noperator_policy = "${operatorPolicy}"\nweb_root = "${join(repositoryRoot, "web/dist")}"\n\n[storage]\nendpoint = "http://127.0.0.1:${storagePort}/"\nbucket = "${bucket}"\nregion = "us-east-1"\ninsecure_loopback = true\n\n[limits]\nconcurrent_requests = 8\nrequest_seconds = 30\nmax_blob_bytes = 33554432\ninventory_nodes = 100000\n\n[protection]\nbookmarks = []\ncommits = []\n`,
  );
  const logPath = join(runtimeRoot, "server.log");
  const logHandle = openSync(logPath, "a", 0o600);
  let server = startServer(serverConfig, logHandle);
  await waitForServer(origin);
  const repositoryId = await seedRepository({
    adminToken,
    operatorToken,
    origin,
    repositoryRoot,
    runtimeRoot,
    run,
  });
  await stopServer(server);
  await appendFile(
    serverConfig,
    `\n[[search]]\nrepository = "${repositoryId}"\ntoken_environment = "JJ_STORE_SEARCH_TOKEN_DEMO"\npoll_millis = 50\ntimeout_seconds = 30\nindex_bytes = 67108864\n\n[search.source]\nindex_source = true\nmax_text_bytes = 262144\nmax_files = 10000\nmax_work = 100000\nmax_output_bytes = 16777216\n\n[search.source.scopes.main]\nkind = "bookmark"\nname = "main"\n`,
  );
  server = startServer(serverConfig, logHandle, {
    JJ_STORE_SEARCH_TOKEN_DEMO: adminToken,
  });
  await waitForServer(origin);
  await waitForSearch(origin, repositoryId, adminToken);
  const runtime = {
    format: 1,
    serverPid: server.pid,
    containerId,
    containerName,
    origin,
    repositoryId,
    runtimeRoot,
  };
  await writeFile(runtimeFile, `${JSON.stringify(runtime, null, 2)}\n`, {
    mode: 0o600,
  });
  await chmod(runtimeRoot, 0o700);
  console.log(
    `JJ Store development demo is ready.\n\nURL: ${origin}\nRepository ID: ${repositoryId}\nToken: ${adminToken}\nRuntime: ${runtimeRoot}\nRustFS: ${containerName}\n\nThe token is shown once and is kept out of the repository. Reloading the browser requires entering it again.`,
  );
}

async function stop() {
  await stopPrevious({
    containerName,
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
