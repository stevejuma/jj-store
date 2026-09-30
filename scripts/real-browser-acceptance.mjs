#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { arch, platform, tmpdir } from "node:os";
import { join } from "node:path";
import { resolve } from "node:path";

const repositoryRoot = resolve(import.meta.dirname, "..");
const keep = process.env.JJ_STORE_REAL_KEEP === "1";
const acceptanceRoot = mkdtempSync(join(tmpdir(), "jj-store-real-browser-"));
const demoEnvironment = { JJ_STORE_DEV_ROOT: acceptanceRoot };

function execute(command, args, options = {}) {
  return execFileSync(command, args, {
    cwd: repositoryRoot,
    encoding: options.encoding,
    env: { ...process.env, ...options.env },
    maxBuffer: 16 * 1024 * 1024,
    stdio: options.stdio,
  });
}

function field(output, label) {
  const match = output.match(new RegExp(`^${label}: (.+)$`, "m"));
  if (!match?.[1]) throw new Error(`development demo omitted ${label}`);
  return match[1].trim();
}

let started = false;
try {
  const output = execute("pnpm", ["dev:reset"], {
    encoding: "utf8",
    env: demoEnvironment,
  });
  started = true;
  const baseURL = field(output, "URL");
  const repository = field(output, "Repository ID");
  const runtime = field(output, "Runtime");
  const metadata = JSON.parse(
    readFileSync(join(runtime, "runtime.json"), "utf8"),
  );
  if (
    metadata.format !== 2 ||
    metadata.accessMode !== "local-trusted" ||
    metadata.storageMode !== "embedded-rustfs"
  )
    throw new Error(
      "cold browser stack did not use the local release topology",
    );
  if (/token|credential/iu.test(JSON.stringify(metadata)))
    throw new Error(
      "local runtime metadata contains a credential-shaped field",
    );
  if (
    metadata.resources.firstStartupMillis > 30_000 ||
    metadata.resources.restartStartupMillis > 30_000 ||
    metadata.resources.totalRssKiB > 512 * 1024
  )
    throw new Error(
      `local release startup or resident-memory budget exceeded: ${JSON.stringify(metadata.resources)}`,
    );
  if (
    process.env.JJ_STORE_LOCAL_RELEASE_QUALIFY === "1" &&
    (platform() !== "darwin" || arch() !== "arm64")
  )
    throw new Error("local release qualification requires macOS ARM64");
  console.log(`Real browser stack ready at ${baseURL} for ${repository}.`);
  console.log(
    `Local resources: cold ${metadata.resources.firstStartupMillis} ms, restart ${metadata.resources.restartStartupMillis} ms, ${metadata.resources.totalRssKiB} KiB resident.`,
  );
  console.log(
    `Full fixture preparation: ${metadata.resources.fixturePreparationMillis} ms.`,
  );
  execute(
    "pnpm",
    [
      "exec",
      "playwright",
      "test",
      "--config",
      "web/playwright.real.config.ts",
      ...(process.env.JJ_STORE_REAL_TEST
        ? [process.env.JJ_STORE_REAL_TEST]
        : []),
    ],
    {
      env: {
        JJ_STORE_REAL_BASE_URL: baseURL,
        JJ_STORE_REAL_REPOSITORY: repository,
        JJ_STORE_REAL_RUNTIME: runtime,
        JJ_STORE_REAL_ROOT: repositoryRoot,
      },
      stdio: "inherit",
    },
  );
} finally {
  if (started && !keep)
    execute("pnpm", ["dev:stop"], {
      env: demoEnvironment,
      stdio: "inherit",
    });
  if (!keep) rmSync(acceptanceRoot, { force: true, recursive: true });
  else if (started)
    console.log(
      `JJ_STORE_REAL_KEEP=1 left the development stack running at ${acceptanceRoot}.`,
    );
}
