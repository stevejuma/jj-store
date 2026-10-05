#!/usr/bin/env node
import { execFileSync, spawn } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { rewriteServerConfiguration } from "./dev-demo-state.mjs";
import { registerDemoLocalFolder } from "./dev-demo-local-folder.mjs";
import { seedAgentActivity } from "./dev-demo-agents.mjs";
const repositoryRoot = resolve(import.meta.dirname, "..");
const root = mkdtempSync(join(tmpdir(), "jj-store-filesystem-browser-"));
const target = resolve(
  process.env.CARGO_TARGET_DIR ?? join(repositoryRoot, "target"),
);
const server = join(target, "debug/jj-store"),
  jj = join(target, "debug/jj");
const port = Number(process.env.JJ_STORE_FILESYSTEM_PORT ?? "7478"),
  origin = `http://127.0.0.1:${port}/`;
const run = (binary, args, cwd = repositoryRoot) =>
  execFileSync(binary, args, {
    cwd,
    encoding: "utf8",
    env: { ...process.env, JJ_CONFIG: join(root, "jj.toml") },
  });
writeFileSync(
  join(root, "jj.toml"),
  '[user]\nname = "Local browser acceptance"\nemail = "browser@example.test"\n',
);
let processHandle;
try {
  const folder = join(root, "repository"),
    state = join(root, "state");
  run(jj, ["git", "init", "--colocate", folder]);
  writeFileSync(
    join(folder, "README.md"),
    "# Filesystem repository\n\nLocal browser acceptance.\n",
  );
  run(jj, ["-R", folder, "describe", "-m", "Local browser fixture"]);
  run(jj, ["-R", folder, "bookmark", "create", "main"]);
  const initialized = JSON.parse(
    run(jj, [
      "-R",
      folder,
      "agent",
      "initialize",
      "--request-id",
      "11".repeat(16),
      "--format=json",
    ]),
  );
  if (initialized.status !== "committed")
    throw Error("fixture initialization did not commit");
  const captureRun = async (binary, args, input, options = {}) => ({
    stdout: execFileSync(binary, args, {
      cwd: options.cwd ?? repositoryRoot,
      encoding: "utf8",
      env: { ...process.env, JJ_CONFIG: join(root, "jj.toml"), ...options.env },
      ...(input ? { input } : {}),
    }),
  });
  await seedAgentActivity({
    repositoryRoot,
    runtimeRoot: root,
    repository: folder,
    run: (binary, args, options) => captureRun(binary, args, null, options),
    runInput: captureRun,
  });
  const doctor = JSON.parse(
    run(jj, ["-R", folder, "store", "doctor", "--format=json"]),
  );
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
  processHandle = spawn(server, ["run", "--config", config, "--format=json"], {
    cwd: repositoryRoot,
    stdio: ["ignore", "pipe", "pipe"],
  });
  let output = "";
  processHandle.stdout.on("data", (b) => (output += b));
  processHandle.stderr.on("data", (b) => (output += b));
  for (let i = 0; i < 100; i++) {
    if (processHandle.exitCode !== null) throw Error(output);
    try {
      if ((await fetch(origin + "v1/bootstrap")).ok) break;
    } catch {}
    await new Promise((r) => setTimeout(r, 100));
  }
  await registerDemoLocalFolder({
    origin,
    repositoryId: doctor.repository,
    path: folder,
  });
  const operation = run(jj, [
    "-R",
    folder,
    "--ignore-working-copy",
    "--no-pager",
    "op",
    "log",
    "-n",
    "1",
    "--no-graph",
    "-T",
    "id",
  ]).trim();
  if (!/^[0-9a-f]{128}$/.test(operation))
    throw Error("invalid fixture operation");
  const fixture = {
    repository: doctor.repository,
    folder,
    operation,
    root,
    origin,
    profile: "filesystem",
  };
  writeFileSync(join(root, "browser-fixture.json"), JSON.stringify(fixture));
  execFileSync(
    "pnpm",
    [
      "exec",
      "playwright",
      "test",
      "--config",
      "web/playwright.real.config.ts",
      "local-browser.spec.ts",
      "--output",
      ".scratch/test-results-local-browser",
    ],
    {
      cwd: repositoryRoot,
      stdio: "inherit",
      env: {
        ...process.env,
        JJ_STORE_REAL_BASE_URL: origin,
        JJ_STORE_LOCAL_BROWSER_FIXTURE: join(root, "browser-fixture.json"),
      },
    },
  );
  if (
    readFileSync(join(state, "server.toml"), "utf8").includes(
      'mode = "embedded"',
    )
  )
    throw Error("filesystem fixture configured native storage");
  console.log(
    "Cold filesystem-only browser acceptance passed; no RustFS or native RPC.",
  );
} finally {
  if (processHandle && processHandle.exitCode === null) {
    processHandle.kill("SIGTERM");
    await new Promise((resolveWait) => {
      processHandle.once("exit", resolveWait);
      setTimeout(resolveWait, 10000);
    });
  }
  if (process.env.JJ_STORE_LOCAL_BROWSER_KEEP !== "1")
    rmSync(root, { recursive: true, force: true });
  else console.log(`Retained fixture: ${root}`);
}
