import { stageDesktopResources } from "./desktop-resources.mjs";
import { spawn } from "node:child_process";
import { resolve, join } from "node:path";
import { parseArgs } from "node:util";

const root = resolve(import.meta.dirname, "..");
const { positionals, values } = parseArgs({
  allowPositionals: true,
  options: {
    "server-dir": { type: "string" },
    "rustfs-runtime": { type: "string" },
    "web-root": { type: "string" },
    "state-dir": { type: "string" },
  },
});
const mode = positionals[0];
function run(command, args, cwd = root) {
  return new Promise((resolveRun, reject) => {
    const child = spawn(command, args, { cwd, stdio: "inherit" });
    child.once("error", reject);
    child.once("exit", (code, signal) =>
      code === 0
        ? resolveRun()
        : reject(Error(`${command} exited with ${code ?? signal}`)),
    );
  });
}
async function stage() {
  const target = resolve(values["server-dir"] ?? join(root, "target/debug"));
  const runtime = resolve(
    values["rustfs-runtime"] ??
      join(root, "runtime/embedded-rustfs/target/debug/jj-store-rustfs"),
  );
  const web = resolve(values["web-root"] ?? join(root, "web/dist"));
  const destination = join(root, "desktop/resources");
  await stageDesktopResources({ target, runtime, web, destination });
}
if (mode === "prepare") {
  await run("pnpm", ["web:build"]);
  await run("cargo", ["build", "--locked", "-p", "jj-store-cli", "--bins"]);
  await run("cargo", [
    "build",
    "--manifest-path",
    "runtime/embedded-rustfs/Cargo.toml",
    "--locked",
  ]);
  await stage();
} else if (mode === "stage") await stage();
else if (mode === "check")
  await run("cargo", [
    "check",
    "--manifest-path",
    "desktop/Cargo.toml",
    "--locked",
  ]);
else if (mode === "test")
  await run("cargo", [
    "test",
    "--manifest-path",
    "desktop/Cargo.toml",
    "--locked",
    "--no-default-features",
  ]);
else if (mode === "bundle") {
  await run(
    "pnpm",
    ["dlx", "@tauri-apps/cli@2.11.4", "build", "--debug", "--bundles", "app"],
    join(root, "desktop"),
  );
} else if (mode === "run") {
  if (values["state-dir"])
    process.env.JJ_STORE_DESKTOP_DATA_DIR = resolve(values["state-dir"]);
  await run("cargo", [
    "run",
    "--manifest-path",
    "desktop/Cargo.toml",
    "--locked",
  ]);
} else
  throw Error(
    "Usage: node scripts/desktop.mjs prepare|stage|check|test|bundle|run [--server-dir PATH --rustfs-runtime PATH --web-root PATH --state-dir PATH]",
  );
