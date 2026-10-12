/** Cold built-UI directory acceptance on an isolated, empty real RustFS installation. */
import { execFileSync, spawn } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { createServer } from "node:net";
import { rewriteServerConfiguration } from "./dev-demo-state.mjs";
const root = resolve(import.meta.dirname, ".."),
  runtime = mkdtempSync(join(tmpdir(), "jj-store-profile-"));
const binary = join(root, "target/debug/jj-store"),
  rustfs = join(root, "runtime/embedded-rustfs/target/debug/jj-store-rustfs");
const listener = createServer();
await new Promise((r) => listener.listen(0, "127.0.0.1", r));
const port = listener.address().port;
await new Promise((r) => listener.close(r));
const base = `http://127.0.0.1:${port}`,
  config = join(runtime, "local/server.toml");
const run = (cmd, args, env = {}) =>
  execFileSync(cmd, args, {
    cwd: root,
    env: { ...process.env, ...env },
    stdio: "inherit",
    maxBuffer: 8 * 1024 * 1024,
  });
let service;
try {
  run("cargo", ["build", "-p", "jj-store-cli", "--bins", "--locked"]);
  run("pnpm", ["web:build"]);
  run(binary, [
    "init",
    "--local",
    "--state-dir",
    join(runtime, "local"),
    "--rustfs-executable",
    rustfs,
  ]);
  writeFileSync(
    config,
    rewriteServerConfiguration(readFileSync(config, "utf8"), {
      origin: base + "/",
      webRoot: join(root, "web/dist"),
      browserAnnotations: false,
    }),
    { mode: 0o600 },
  );
  service = spawn(binary, ["run", "--config", config, "--loopback-only"], {
    cwd: root,
    stdio: "inherit",
  });
  const deadline = Date.now() + 30000;
  let ready = false;
  while (Date.now() < deadline) {
    try {
      ready = (
        await fetch(base + "/v1/bootstrap", {
          headers: {
            "x-jj-store-protocol": "1",
            Origin: base,
            "Sec-Fetch-Site": "same-origin",
            "Sec-Fetch-Mode": "cors",
            "Sec-Fetch-Dest": "empty",
          },
        })
      ).ok;
      if (ready) break;
    } catch {}
    await new Promise((r) => setTimeout(r, 100));
  }
  if (!ready) throw Error("isolated profile server startup failed");
  run(
    "pnpm",
    [
      "exec",
      "playwright",
      "test",
      "--config",
      "web/playwright.real.config.ts",
      "profiles-emoji.spec.ts",
    ],
    {
      JJ_STORE_REAL_RUNTIME: runtime,
      JJ_STORE_REAL_ROOT: root,
      JJ_STORE_REAL_BASE_URL: base,
      JJ_STORE_REAL_EMPTY: "1",
      JJ_STORE_REAL_PID: String(service.pid),
    },
  );
} finally {
  if (service?.exitCode === null) {
    service.kill("SIGTERM");
    await new Promise((r) => service.once("exit", r));
  }
  rmSync(runtime, { force: true, recursive: true });
}
