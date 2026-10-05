#!/usr/bin/env node
import { execFileSync, spawn } from "node:child_process";
import {
  mkdtempSync,
  readFileSync,
  writeFileSync,
  mkdirSync,
  rmSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { rewriteServerConfiguration } from "./dev-demo-state.mjs";
import { registerDemoLocalFolder } from "./dev-demo-local-folder.mjs";

// Uses existing explicitly built development executables; no installed service changes.
const repositoryRoot = resolve(import.meta.dirname, "..");
const root = mkdtempSync(join(tmpdir(), "jj-store-stacks-browser-"));
const target = resolve(
  process.env.CARGO_TARGET_DIR ?? join(repositoryRoot, "target"),
);
const server = join(target, "debug/jj-store"),
  jj = join(target, "debug/jj");
const port = Number(process.env.JJ_STORE_STACKS_BROWSER_PORT ?? "7489");
const origin = `http://127.0.0.1:${port}/`;
const run = (binary, args) =>
  execFileSync(binary, args, {
    cwd: repositoryRoot,
    encoding: "utf8",
    env: { ...process.env, JJ_CONFIG: join(root, "jj.toml") },
  });
writeFileSync(
  join(root, "jj.toml"),
  '[user]\nname = "Stacks browser acceptance"\nemail = "stacks@example.test"\n',
);
let child;
try {
  const folder = join(root, "repository"),
    state = join(root, "state");
  run(jj, ["git", "init", "--colocate", folder]);
  const command = (...args) => run(jj, ["-R", folder, ...args]);
  writeFileSync(join(folder, "README.md"), "# Local stacks fixture\n");
  writeFileSync(join(folder, ".gitignore"), "obstacle.tmp\n");
  writeFileSync(
    join(folder, "hunks.txt"),
    Array.from({ length: 40 }, (_, index) => `line-${index}\n`).join(""),
  );
  command("describe", "-m", "Workspace target");
  command("bookmark", "create", "main");
  command("new", "main", "-m", "Topic ancestor");
  mkdirSync(join(folder, "src"));
  writeFileSync(join(folder, "src/topic.ts"), "export const one = 1;\n");
  command("describe", "-m", "Topic ancestor");
  command("new", "-m", "Topic tip");
  writeFileSync(
    join(folder, "src/topic.ts"),
    "export const one = 1;\nexport const two = 2;\n",
  );
  command("describe", "-m", "Topic tip");
  command("bookmark", "create", "topic");
  command("new", "main", "-m", "Docs tip");
  writeFileSync(
    join(folder, "README.md"),
    "# Local stacks fixture\n\nDocumentation stack.\n",
  );
  command("describe", "-m", "Docs tip");
  command("bookmark", "create", "docs");
  command("new", "main", "-m", "Physical working copy");
  command(
    "agent",
    "initialize",
    "--request-id",
    "11".repeat(16),
    "--format=json",
  );
  const doctor = JSON.parse(command("store", "doctor", "--format=json"));
  run(server, [
    "init",
    "--local",
    "--storage",
    "filesystem",
    "--state-dir",
    state,
    "--format=json",
  ]);
  const config = join(state, "server.toml");
  writeFileSync(
    config,
    rewriteServerConfiguration(readFileSync(config, "utf8"), {
      origin,
      webRoot: join(repositoryRoot, "web/dist"),
      browserAnnotations: false,
    }),
  );
  child = spawn(server, ["run", "--config", config, "--format=json"], {
    cwd: repositoryRoot,
    stdio: ["ignore", "pipe", "pipe"],
    env: {
      ...process.env,
      JJ_CONFIG: join(root, "jj.toml"),
      GIT_CONFIG_NOSYSTEM: "1",
      GIT_CONFIG_GLOBAL: "/dev/null",
    },
  });
  let output = "";
  child.stdout.on("data", (value) => (output += value));
  child.stderr.on("data", (value) => (output += value));
  let ready = false,
    last = "no response";
  for (let attempt = 0; attempt < 100; attempt++) {
    if (child.exitCode !== null) throw new Error(output);
    try {
      const response = await fetch(origin + "v1/bootstrap", {
        headers: {
          "x-jj-store-protocol": "1",
          Origin: new URL(origin).origin,
          "Sec-Fetch-Site": "same-origin",
          "Sec-Fetch-Mode": "cors",
          "Sec-Fetch-Dest": "empty",
        },
      });
      last = `${response.status}: ${await response.text()}`;
      if (response.ok) {
        ready = true;
        break;
      }
    } catch (error) {
      last = String(error);
    }
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  if (!ready)
    throw new Error(`Isolated stacks server did not become ready: ${last}
${output}`);
  await registerDemoLocalFolder({
    origin,
    repositoryId: doctor.repository,
    path: folder,
  });
  const operation = command(
    "--ignore-working-copy",
    "--no-pager",
    "op",
    "log",
    "-n",
    "1",
    "--no-graph",
    "-T",
    "id",
  ).trim();
  // Dirty disk after the last JJ command; active Stacks refresh snapshots it.
  writeFileSync(join(folder, "README.md"), "# Unsnapshotted physical change\n");
  writeFileSync(join(folder, "unsnapshotted.txt"), "new physical file\n");
  writeFileSync(join(folder, "obstacle.tmp"), "ignored checkout obstacle\n");
  const fixture = join(root, "browser-fixture.json");
  writeFileSync(
    fixture,
    JSON.stringify({
      repository: doctor.repository,
      folder,
      operation,
      origin,
      jj,
      jjConfig: join(root, "jj.toml"),
    }),
  );
  execFileSync(
    "pnpm",
    [
      "exec",
      "playwright",
      "test",
      "--config",
      "web/playwright.real.config.ts",
      "stacks-browser.spec.ts",
      "--output",
      ".scratch/test-results-stacks-browser",
      ...process.argv.slice(2),
    ],
    {
      cwd: repositoryRoot,
      stdio: "inherit",
      env: {
        ...process.env,
        JJ_STORE_REAL_BASE_URL: origin,
        JJ_STORE_STACKS_BROWSER_FIXTURE: fixture,
      },
    },
  );
  console.log(
    "Cold real-folder Stacks core commands passed; local TOML and independent WIP verified.",
  );
} finally {
  if (child && child.exitCode === null) {
    child.kill("SIGTERM");
    await new Promise((resolve) => {
      child.once("exit", resolve);
      setTimeout(resolve, 10000);
    });
  }
  if (process.env.JJ_STORE_STACKS_BROWSER_KEEP !== "1")
    rmSync(root, { recursive: true, force: true });
  else console.log(`Retained fixture: ${root}`);
}
