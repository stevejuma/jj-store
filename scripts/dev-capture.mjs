#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { developmentLayout, loadFixture } from "./dev-demo-state.mjs";
import { runUiDevelopment } from "./dev-ui.mjs";

const root = resolve(import.meta.dirname, "..");
const previewRoot = resolve(
  root,
  process.env.JJ_STORE_CAPTURE_PREVIEW_ROOT ?? ".local/codex-preview",
);
const environment = {
  ...process.env,
  JJ_STORE_DEV_ROOT: previewRoot,
  JJ_STORE_DEV_PORT: process.env.JJ_STORE_CAPTURE_PREVIEW_PORT ?? "7470",
  JJ_STORE_DEV_BACKEND_PORT:
    process.env.JJ_STORE_CAPTURE_PREVIEW_BACKEND_PORT ?? "7472",
};
const layout = developmentLayout(root, environment);
function run(command, args, extra = {}) {
  const result = spawnSync(command, args, {
    cwd: root,
    env: { ...environment, ...extra },
    stdio: "inherit",
  });
  if (result.status !== 0)
    throw Error(
      `${command} failed (${result.status ?? result.error?.message})`,
    );
}
function requireCodex() {
  const result = spawnSync(
    process.env.JJ_STORE_CAPTURE_CODEX ?? "codex",
    ["--version"],
    { encoding: "utf8" },
  );
  if (result.status !== 0 || result.stdout.trim() !== "codex-cli 0.156.1")
    throw Error(
      "This preview requires Codex CLI 0.156.1. No other version is enabled.",
    );
}
async function configure() {
  const source = await readFile(layout.serverConfig, "utf8");
  if (/^\[agent_capture(?:\]|\.)/m.test(source)) return;
  await writeFile(
    layout.serverConfig,
    source +
      `
# Codex CLI preview: known CODEX_HOME/HOME locations, exact linked-directory admission.
[agent_capture]
enabled = true
max_readers = 4
rescan_seconds = 5
max_sources = 1024
max_pending_bytes = 67108864
[[agent_capture.harnesses]]
profile = "codex-cli-jsonl-v1"
product_version = "0.156.1"
`,
    { mode: 0o600 },
  );
}
const action = process.argv[2] ?? "start";
if (!["setup", "start", "restart", "stop"].includes(action))
  throw Error("Use setup, start, restart or stop");
if (action === "stop") run(process.execPath, ["scripts/dev-demo.mjs", "stop"]);
else {
  requireCodex();
  let fixture = await loadFixture(layout);
  if (!fixture) {
    if (action !== "setup")
      throw Error(
        "Run pnpm dev:capture:setup once to build the isolated fixture.",
      );
    run(process.execPath, ["scripts/dev-demo.mjs", "restart"]);
    run(process.execPath, ["scripts/dev-demo.mjs", "stop"]);
    fixture = await loadFixture(layout);
  }
  await configure();
  await runUiDevelopment(root, environment, action === "restart");
  console.log(
    `\nCodex capture preview (CLI 0.156.1)\nLinked folder: ${layout.repository}\nConfiguration: ${layout.serverConfig}\nStart Codex in that folder, then open Agents.\nSettings > Agents has Capture, Sources, Imports and Diagnostics tabs. Capture shows the configured root paths and pause/resume.\nStop: pnpm dev:capture:stop\nRestart backend: pnpm dev:capture:restart\nFixture and captured history are preserved.`,
  );
}
