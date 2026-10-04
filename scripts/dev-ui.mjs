#!/usr/bin/env node
import { constants } from "node:fs";
import { access, lstat, readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { developmentLayout, loadFixture } from "./dev-demo-state.mjs";
import { stopPrevious, processTreeMetrics } from "./dev-demo-services.mjs";
import {
  inspectRuntimeProcesses,
  stopRuntimeProcesses,
} from "./dev-demo-processes.mjs";
import { waitForSearch } from "./dev-demo-search.mjs";
import { assertPortAvailable, start, ready } from "./dev-ui-services.mjs";
import {
  uiDevelopmentSettings,
  rewriteUiBackendConfiguration,
} from "./dev-ui-config.mjs";

async function readRuntime(layout) {
  try {
    return JSON.parse(await readFile(layout.runtimeFile, "utf8"));
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    return null;
  }
}

export async function prepareUiDevelopment(
  repositoryRoot,
  environment = process.env,
) {
  const settings = uiDevelopmentSettings(environment);
  const layout = developmentLayout(repositoryRoot, environment);
  const fixture = await loadFixture(layout);
  if (!fixture)
    throw new Error(
      "UI development requires an existing fixture. Run pnpm dev:restart once, then pnpm dev:ui.",
    );
  const executable = resolve(
    repositoryRoot,
    environment.JJ_STORE_DEV_SERVER ?? "target/debug/jj-store",
  );
  try {
    await access(executable, constants.X_OK);
  } catch {
    throw new Error(
      `Missing backend executable ${executable}. Run pnpm dev:backend:build or pnpm dev:restart once.`,
    );
  }
  const previous = await readRuntime(layout);
  const serverConfig = join(layout.runtimeRoot, "ui-server.toml");
  const retained =
    previous?.mode === "vite" &&
    previous.workspaceRoot === repositoryRoot &&
    previous.runtimeRoot === layout.runtimeRoot &&
    previous.repositoryId === fixture.repositoryId &&
    previous.serverConfig === serverConfig;
  const sourcePath = retained ? serverConfig : layout.serverConfig;
  const metadata = await lstat(sourcePath);
  if (
    !metadata.isFile() ||
    metadata.isSymbolicLink() ||
    metadata.size > 64 * 1024
  )
    throw new Error(
      "UI development requires a bounded regular configuration file",
    );
  // Reuse app-saved settings when restarting this exact fixture. Port and binary
  // overrides below remain explicit; a missing retained file never resets it.
  let source = await readFile(sourcePath, "utf8");
  if (environment.JJ_STORE_DEV_RUSTFS_RUNTIME)
    source = source.replace(
      /^runtime_executable = .*$/mu,
      `runtime_executable = ${JSON.stringify(resolve(repositoryRoot, environment.JJ_STORE_DEV_RUSTFS_RUNTIME))}`,
    );
  const runtimePath = /^runtime_executable = (".*")$/mu.exec(source)?.[1];
  if (!runtimePath)
    throw new Error(
      "UI development requires the existing embedded RustFS fixture",
    );
  try {
    await access(JSON.parse(runtimePath), constants.X_OK);
  } catch {
    throw new Error(
      "Missing embedded RustFS executable. Run pnpm dev:restart once or select JJ_STORE_DEV_RUSTFS_RUNTIME.",
    );
  }
  return {
    settings,
    layout,
    fixture,
    executable,
    serverConfig,
    configuration: rewriteUiBackendConfiguration(source, settings),
    previous,
  };
}

export async function runUiDevelopment(
  repositoryRoot,
  environment = process.env,
  backendOnly = false,
) {
  const prepared = await prepareUiDevelopment(repositoryRoot, environment);
  const {
    settings,
    layout,
    fixture,
    executable,
    serverConfig,
    configuration,
    previous,
  } = prepared;
  const owned = previous
    ? await inspectRuntimeProcesses(previous, {
        owner: repositoryRoot,
        runtimeRoot: layout.runtimeRoot,
      })
    : [];
  const ownsUiPort =
    previous?.origin === settings.origin &&
    owned.some(
      (item) =>
        item.kind === "ui" ||
        (previous.mode !== "vite" && item.kind === "server"),
    );
  const ownsBackendPort =
    previous?.mode === "vite" &&
    previous.apiOrigin === settings.backendOrigin &&
    owned.some((item) => item.kind === "server");
  // Check unowned target ports before stopping any existing development process.
  if (!ownsUiPort) await assertPortAvailable(settings.uiPort);
  if (!ownsBackendPort) await assertPortAvailable(settings.backendPort);
  if (backendOnly) {
    if (
      previous?.mode !== "vite" ||
      previous.origin !== settings.origin ||
      previous.apiOrigin !== settings.backendOrigin
    )
      throw new Error(
        "Backend-only restart requires a matching active pnpm dev:ui runtime",
      );
    const processes = await inspectRuntimeProcesses(previous, {
      owner: repositoryRoot,
      runtimeRoot: layout.runtimeRoot,
    });
    if (!processes.some((item) => item.kind === "ui"))
      throw new Error("Vite is not running; use pnpm dev:ui");
    await stopRuntimeProcesses(previous, {
      owner: repositoryRoot,
      runtimeRoot: layout.runtimeRoot,
      ui: false,
    });
  } else {
    await stopPrevious({
      owner: repositoryRoot,
      runtimeFile: layout.runtimeFile,
      runtimeRoot: layout.runtimeRoot,
    });
    await assertPortAvailable(settings.uiPort);
  }
  await assertPortAvailable(settings.backendPort);
  await writeFile(serverConfig, configuration, { mode: 0o600 });
  let runtime = {
    ...previous,
    format: 2,
    mode: "vite",
    accessMode: "local-trusted",
    storageMode: "embedded-rustfs",
    origin: settings.origin,
    apiOrigin: settings.backendOrigin,
    workspaceRoot: repositoryRoot,
    runtimeRoot: layout.runtimeRoot,
    serverConfig,
    repositoryId: fixture.repositoryId,
    uiRunner: join(repositoryRoot, "scripts/dev-ui-vite.mjs"),
    serverPid: undefined,
    uiPid: backendOnly ? previous.uiPid : undefined,
  };
  const started = performance.now();
  try {
    runtime.serverPid = await start(
      executable,
      ["run", "--config", serverConfig, "--format=json"],
      repositoryRoot,
      layout,
      environment,
      layout.logPath,
    );
    if (!backendOnly)
      runtime.uiPid = await start(
        process.execPath,
        [runtime.uiRunner, "--runtime-root", layout.runtimeRoot],
        repositoryRoot,
        layout,
        {
          ...environment,
          JJ_STORE_DEV_PORT: String(settings.uiPort),
          JJ_STORE_DEV_BACKEND_PORT: String(settings.backendPort),
        },
        join(layout.runtimeRoot, "ui.log"),
      );
    await writeFile(
      layout.runtimeFile,
      JSON.stringify(runtime, null, 2) + "\n",
      { mode: 0o600 },
    );
    await ready(settings.origin);
    const search = await waitForSearch(settings.origin, fixture.repositoryId);
    runtime.resources = {
      ...previous?.resources,
      ...(await processTreeMetrics(runtime.serverPid)),
      restartStartupMillis: Math.round(performance.now() - started),
    };
    await writeFile(
      layout.runtimeFile,
      JSON.stringify(runtime, null, 2) + "\n",
      { mode: 0o600 },
    );
    console.log(
      `JJ Store UI development ready.\nURL: ${settings.origin}\nBackend: ${settings.backendOrigin}\nWorkspace: ${repositoryRoot}\nRuntime: ${layout.runtimeRoot}\nUI: Vite HMR; no builds\nFixture: preserved\nSearch: ${search.configured === false ? "disabled" : "ready"}\nUse pnpm dev:backend:restart after rebuilding Rust; pnpm dev:restart rebuilds the whole stack; pnpm dev:stop stops both processes.`,
    );
    return runtime;
  } catch (error) {
    await stopRuntimeProcesses(runtime, {
      owner: repositoryRoot,
      runtimeRoot: layout.runtimeRoot,
      ui: !backendOnly,
    });
    await writeFile(
      layout.runtimeFile,
      JSON.stringify(
        {
          ...runtime,
          serverPid: undefined,
          uiPid: backendOnly ? runtime.uiPid : undefined,
        },
        null,
        2,
      ) + "\n",
      { mode: 0o600 },
    );
    throw error;
  }
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const action = process.argv[2] ?? "start";
  if (!["start", "backend"].includes(action))
    throw new Error(`unknown UI development action ${action}`);
  try {
    await runUiDevelopment(
      resolve(import.meta.dirname, ".."),
      process.env,
      action === "backend",
    );
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
