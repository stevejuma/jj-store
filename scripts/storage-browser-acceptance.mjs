// Focused real-RustFS maintenance fixture: native code, no harness setup or sample agents.
import { spawn } from "node:child_process";
import { openSync, closeSync } from "node:fs";
import { mkdtemp, readFile, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve, basename } from "node:path";
import { execute, waitForServer } from "./dev-demo-services.mjs";
import { stopRuntimeProcesses } from "./dev-demo-processes.mjs";
import { rewriteServerConfiguration } from "./dev-demo-state.mjs";
const owner = resolve(import.meta.dirname, "..");
const restartOnly = process.argv[2] === "restart";
const root = restartOnly
  ? process.env.JJ_STORE_REAL_RUNTIME
  : await mkdtemp(
      join(
        process.platform === "darwin" ? "/tmp" : tmpdir(),
        "jj-store-storage-browser-",
      ),
    );
if (!root || !basename(root).startsWith("jj-store-storage-browser-"))
  throw new Error("expected isolated Storage fixture");
const marker = join(root, "storage-runtime.json");
const run = (command, args, env = {}) =>
  execute(command, args, { cwd: owner, env });
const server = join(owner, "target/debug/jj-store");
async function readRuntime() {
  const value = JSON.parse(await readFile(marker, "utf8"));
  if (
    value.owner !== owner ||
    value.serverConfig !== join(root, "local/server.toml")
  )
    throw new Error("Storage fixture ownership mismatch");
  return value;
}
async function start(value) {
  const log = openSync(join(root, "server.log"), "a", 0o600);
  const child = spawn(
    server,
    ["run", "--config", value.serverConfig, "--format=json"],
    { cwd: owner, detached: true, stdio: ["ignore", log, log] },
  );
  child.unref();
  closeSync(log);
  const runtime = { ...value, serverPid: child.pid };
  await writeFile(marker, JSON.stringify(runtime), { mode: 0o600 });
  await waitForServer(value.origin);
}
async function stop() {
  let runtime;
  try {
    runtime = await readRuntime();
  } catch (error) {
    if (error.code === "ENOENT") return;
    throw error;
  }
  await stopRuntimeProcesses(runtime, { owner, runtimeRoot: root, ui: false });
}
if (restartOnly) {
  const runtime = await readRuntime();
  await stop();
  await start(runtime);
} else {
  try {
    const runtimeBinary = process.env.JJ_STORE_DEV_RUSTFS_RUNTIME;
    if (!runtimeBinary)
      throw new Error(
        "set JJ_STORE_DEV_RUSTFS_RUNTIME to the qualified embedded runtime",
      );
    await run("pnpm", ["web:build"]);
    await run("cargo", ["build", "--locked", "-p", "jj-store", "--bins"]);
    const origin = `http://127.0.0.1:${Number(process.env.JJ_STORE_DEV_PORT ?? "7570")}/`;
    await run(server, [
      "init",
      "--local",
      "--state-dir",
      join(root, "local"),
      "--rustfs-executable",
      resolve(runtimeBinary),
      "--format=json",
    ]);
    const serverConfig = join(root, "local/server.toml");
    const config = rewriteServerConfiguration(
      await readFile(serverConfig, "utf8"),
      { origin, webRoot: join(owner, "web/dist"), browserAnnotations: false },
    );
    await writeFile(serverConfig, config, { mode: 0o600 });
    await start({ owner, origin, serverConfig });
    const jjConfig = join(root, "jj.toml");
    await writeFile(
      jjConfig,
      '[user]\nname = "Storage acceptance"\nemail = "storage@example.invalid"\n',
      { mode: 0o600 },
    );
    const env = { JJ_CONFIG: jjConfig };
    const created = await run(
      join(owner, "target/debug/jj-store-jj"),
      [
        "store",
        "init",
        join(root, "repository"),
        "--unix-socket",
        join(root, "local/native.sock"),
        "--cache",
        join(root, "cache"),
        "--state-dir",
        join(root, "init-state"),
        "--workspace",
        "storage",
        "--format=json",
      ],
      env,
    );
    const repo = JSON.parse(created.stdout).repository;
    await writeFile(
      join(root, "repository/README.md"),
      "# Storage acceptance\n\nRetained native data.\n",
    );
    await run(
      join(owner, "target/debug/jj-store-jj"),
      ["-R", join(root, "repository"), "describe", "-m", "Storage acceptance"],
      env,
    );
    await stop();
    await run(server, [
      "repository",
      "register",
      "--config",
      serverConfig,
      "--repo-id",
      repo,
      "--name",
      "Storage acceptance",
      "--format=json",
    ]);
    await start(await readRuntime());
    console.log(`Cold native Storage fixture ready at ${origin} (${repo}).`);
    const result = await run(
      "pnpm",
      [
        "exec",
        "playwright",
        "test",
        "--config",
        "web/playwright.real.config.ts",
        "web/tests/real/storage-maintenance.spec.ts",
      ],
      {
        JJ_STORE_REAL_BASE_URL: origin,
        JJ_STORE_REAL_REPOSITORY: repo,
        JJ_STORE_REAL_RUNTIME: root,
        JJ_STORE_REAL_ROOT: owner,
        JJ_STORE_STORAGE_FIXTURE: "1",
      },
    );
    console.log(result.stdout);
    if (result.stderr) console.error(result.stderr);
  } catch (error) {
    if (error.stdout) console.error(error.stdout);
    if (error.stderr) console.error(error.stderr);
    const serverLog = await readFile(join(root, "server.log"), "utf8").catch(
      () => "",
    );
    if (serverLog) console.error(serverLog.slice(-16384));
    throw error;
  } finally {
    await stop();
    await rm(root, { recursive: true, force: true });
  }
}
