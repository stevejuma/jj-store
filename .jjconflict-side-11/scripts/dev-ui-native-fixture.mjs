import { spawn } from "node:child_process";
import { once } from "node:events";
import { mkdtemp, realpath, readFile, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { execute } from "./dev-demo-services.mjs";
import { developmentLayout, writeFixture } from "./dev-demo-state.mjs";

export async function createUiNativeFixture(repositoryRoot, environment) {
  const runtimeRoot = await realpath(
    await mkdtemp(join(tmpdir(), "jj-store-hmr-native-")),
  );
  try {
    return await initializeFixture(repositoryRoot, environment, runtimeRoot);
  } catch (error) {
    await rm(runtimeRoot, { recursive: true, force: true });
    throw error;
  }
}

async function initializeFixture(repositoryRoot, environment, runtimeRoot) {
  const layout = developmentLayout(repositoryRoot, {
    JJ_STORE_DEV_ROOT: runtimeRoot,
  });
  const executable = resolve(
    repositoryRoot,
    environment.JJ_STORE_DEV_SERVER ?? "target/debug/jj-store",
  );
  const nativeJj = resolve(
    repositoryRoot,
    environment.JJ_STORE_UI_TEST_JJ ?? "target/debug/jj",
  );
  const rustfs = resolve(
    repositoryRoot,
    environment.JJ_STORE_DEV_RUSTFS_RUNTIME ??
      "runtime/embedded-rustfs/target/debug/jj-store-rustfs",
  );
  const backendPort = Number(environment.JJ_STORE_DEV_BACKEND_PORT ?? "7482");
  const origin = `http://127.0.0.1:${backendPort}/`;
  const env = {
    ...environment,
    JJ_STORE_DEV_ROOT: runtimeRoot,
    JJ_STORE_DEVELOPMENT_STATE_ROOT: runtimeRoot,
  };
  const run = (command, args, options = {}) =>
    execute(command, args, {
      cwd: repositoryRoot,
      env: { ...env, ...options.env },
    });
  await run(executable, [
    "init",
    "--local",
    "--state-dir",
    layout.stateRoot,
    "--rustfs-executable",
    rustfs,
    "--format=json",
  ]);
  const configuration = (await readFile(layout.serverConfig, "utf8"))
    .replace(/^bind = ".*"$/mu, `bind = "127.0.0.1:${backendPort}"`)
    .replace(/^public_url = ".*"$/mu, `public_url = "${origin}"`);
  await writeFile(layout.serverConfig, configuration, { mode: 0o600 });
  const server = spawn(
    executable,
    ["run", "--config", layout.serverConfig, "--format=json"],
    { cwd: repositoryRoot, env, stdio: "ignore" },
  );
  const exited = once(server, "exit");
  let repositoryId;
  try {
    for (let attempt = 0; attempt < 100; attempt += 1) {
      try {
        const response = await fetch(`${origin}v1/bootstrap`, {
          headers: {
            "x-jj-store-protocol": "1",
            Origin: new URL(origin).origin,
            "Sec-Fetch-Site": "same-origin",
            "Sec-Fetch-Mode": "cors",
            "Sec-Fetch-Dest": "empty",
          },
          signal: AbortSignal.timeout(1000),
        });
        if (response.ok) {
          await response.body?.cancel();
          break;
        }
      } catch {
        /* Fixture server is starting. */
      }
      if (attempt === 99)
        throw new Error("native HMR fixture server did not start");
      await new Promise((resolveWait) => setTimeout(resolveWait, 100));
    }
    const jjConfig = join(runtimeRoot, "jj.toml");
    await writeFile(
      jjConfig,
      '[user]\nname = "HMR acceptance"\nemail = "hmr@example.invalid"\n',
    );
    env.JJ_CONFIG = jjConfig;
    repositoryId = JSON.parse(
      (
        await run(nativeJj, [
          "store",
          "init",
          layout.repository,
          "--unix-socket",
          layout.unixSocket,
          "--cache",
          join(runtimeRoot, "cache"),
          "--state-dir",
          join(runtimeRoot, "init-state"),
          "--workspace",
          "hmr",
          "--format=json",
        ])
      ).stdout,
    ).repository;
    await writeFile(
      join(layout.repository, "README.md"),
      "# Native HMR acceptance\n",
    );
    await run(nativeJj, [
      "-R",
      layout.repository,
      "describe",
      "-m",
      "Native HMR acceptance",
    ]);
    await run(nativeJj, [
      "-R",
      layout.repository,
      "bookmark",
      "create",
      "main",
      "-r",
      "@",
    ]);
  } finally {
    if (server.exitCode === null && server.signalCode === null)
      server.kill("SIGTERM");
    await exited;
  }
  await run(executable, [
    "repository",
    "register",
    "--config",
    layout.serverConfig,
    "--repo-id",
    repositoryId,
    "--name",
    "HMR acceptance repository",
    "--format=json",
  ]);
  await writeFixture(layout, { repositoryId, firstStartupMillis: 0 });
  return { layout, repositoryId, environment: env };
}
