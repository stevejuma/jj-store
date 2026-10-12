#!/usr/bin/env node
import { spawn } from "node:child_process";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import {
  storageContext,
  targets,
  helper,
  targetLock,
  canonical,
} from "./build-storage-inventory.mjs";
import { maintain, storageFailure } from "./build-storage-policy.mjs";

function execute(command, args, options = {}) {
  return new Promise((accept, reject) => {
    const child = spawn(command, args, { stdio: "inherit", ...options });
    const forward = (signal) => child.kill(signal);
    const interrupt = () => forward("SIGINT");
    const terminate = () => forward("SIGTERM");
    process.on("SIGINT", interrupt);
    process.on("SIGTERM", terminate);
    child.once("error", reject);
    child.once("exit", (code, signal) => {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", terminate);
      code === 0
        ? accept()
        : reject(Error(`${command} exited with ${code ?? signal}`));
    });
  });
}

export async function cargoSelection(context, args, environment = process.env) {
  const option = (name) => {
    const index = args.indexOf(name);
    return index >= 0
      ? args[index + 1]
      : args.find((a) => a.startsWith(name + "="))?.slice(name.length + 1);
  };
  if (
    args.includes("--build-dir") ||
    args.some((a) => a.startsWith("--config")) ||
    environment.CARGO_BUILD_BUILD_DIR
  )
    throw Error(
      "Managed Cargo does not accept build directory configuration; select a registered --target-dir",
    );
  const manifest = await canonical(
    resolve(context.root, option("--manifest-path") ?? "Cargo.toml"),
  );
  const known = await targets(context);
  const native = known.find((e) => !e.temporary && e.manifest === manifest);
  if (!native)
    throw Error("Cargo manifest is outside the managed workspace family");
  const path = await canonical(
    resolve(
      context.root,
      option("--target-dir") ?? (environment.CARGO_TARGET_DIR || native.path),
    ),
  );
  const entry = known.find((e) => e.path === path);
  if (!entry || (entry.temporary && entry.completed))
    throw Error(
      "Select an active registered build target; temporary targets require dev:target:register",
    );
  if (entry.root !== native.root)
    throw Error("Cargo targets cannot be shared between workspaces");
  if (!entry.temporary && entry.manifest !== native.manifest)
    throw Error(
      "Independent Cargo workspaces require their own target directory",
    );
  const pinned = args.slice();
  const separator = pinned.indexOf("--");
  if (!option("--target-dir"))
    pinned.splice(
      separator < 0 ? pinned.length : separator,
      0,
      "--target-dir",
      path,
    );
  return { path, args: pinned, manifest };
}

export function cargoCommand(args) {
  return [process.execPath, [resolve(import.meta.filename), ...args]];
}

export async function managedCargo(
  args,
  root = resolve(import.meta.dirname, ".."),
) {
  if (args[0] === "--") args = args.slice(1);
  if (!args.length)
    throw Error(
      "Usage: pnpm dev:cargo -- build|check|clippy|test|run [Cargo arguments]",
    );
  if (["metadata", "fmt", "--version", "--help"].includes(args[0])) {
    await execute("cargo", args, { cwd: root });
    return;
  }
  const bundle = args[0] === "desktop-bundle";
  if (!bundle && !["build", "check", "clippy", "test", "run"].includes(args[0]))
    throw Error("Unsupported managed Cargo command");
  const context = await storageContext(root);
  const selection = await cargoSelection(
    context,
    bundle ? ["build", "--manifest-path", "desktop/Cargo.toml"] : args,
  );
  const registryLock = join(context.store, "registry.lock");
  const preflight = [
    process.execPath,
    resolve(import.meta.filename),
    "--maintenance",
    root,
  ];
  await execute(
    "python3",
    [helper, "lock", registryLock, "exclusive-wait", ...preflight],
    { cwd: root },
  );
  const command = bundle
    ? [
        "pnpm",
        "dlx",
        "@tauri-apps/cli@2.11.4",
        "build",
        "--debug",
        "--bundles",
        "app",
      ]
    : ["cargo", ...selection.args];
  let failure;
  try {
    await execute(
      "python3",
      [
        helper,
        "lock",
        targetLock(context, selection.path),
        "shared",
        ...command,
      ],
      {
        cwd: bundle ? join(root, "desktop") : root,
        env: {
          ...process.env,
          CARGO_TARGET_DIR: selection.path,
          CARGO_BUILD_BUILD_DIR: selection.path,
        },
      },
    );
  } catch (error) {
    failure = error;
  }
  try {
    await execute(
      "python3",
      [
        helper,
        "lock",
        registryLock,
        "exclusive-wait",
        ...preflight,
        selection.path,
      ],
      { cwd: root },
    );
  } catch (error) {
    failure ??= error;
  }
  if (failure) throw failure;
}

if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
) {
  try {
    if (process.argv[2] === "--maintenance") {
      const context = await storageContext(process.argv[3]);
      const result = await maintain(context, {
        preserve: process.argv[4] ? [process.argv[4]] : [],
      });
      if (result.blocked) throw Error(storageFailure(result));
    } else
      await managedCargo(
        process.argv.slice(2),
        process.env.JJ_STORE_BUILD_ROOT ?? resolve(import.meta.dirname, ".."),
      );
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
