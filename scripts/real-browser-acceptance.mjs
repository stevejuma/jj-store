#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { resolve } from "node:path";

const repositoryRoot = resolve(import.meta.dirname, "..");
const keep = process.env.JJ_STORE_REAL_KEEP === "1";

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
  const output = execute("pnpm", ["dev:restart"], { encoding: "utf8" });
  started = true;
  const baseURL = field(output, "URL");
  const repository = field(output, "Repository ID");
  const token = field(output, "Token");
  const runtime = field(output, "Runtime");
  console.log(`Real browser stack ready at ${baseURL} for ${repository}.`);
  execute(
    "pnpm",
    ["exec", "playwright", "test", "--config", "web/playwright.real.config.ts"],
    {
      env: {
        JJ_STORE_REAL_BASE_URL: baseURL,
        JJ_STORE_REAL_REPOSITORY: repository,
        JJ_STORE_REAL_TOKEN: token,
        JJ_STORE_REAL_RUNTIME: runtime,
        JJ_STORE_REAL_ROOT: repositoryRoot,
      },
      stdio: "inherit",
    },
  );
} finally {
  if (started && !keep) execute("pnpm", ["dev:stop"], { stdio: "inherit" });
  else if (started)
    console.log("JJ_STORE_REAL_KEEP=1 left the development stack running.");
}
