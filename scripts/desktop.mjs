import { stageAgentResources } from "./agent-runtime-resources.mjs";
import { stageDesktopResources } from "./desktop-resources.mjs";
import { spawn } from "node:child_process";
import { resolve, join } from "node:path";
import { parseArgs } from "node:util";
import { cargoCommand, managedCargo } from "./managed-cargo.mjs";

const root = resolve(import.meta.dirname, "..");
const { positionals, values } = parseArgs({
  args: process.argv.slice(2).filter((value) => value !== "--"),
  allowPositionals: true,
  options: {
    "server-dir": { type: "string" },
    "rustfs-runtime": { type: "string" },
    "web-root": { type: "string" },
    "state-dir": { type: "string" },
    "acp-executable": { type: "string" },
    "native-runtime-dir": { type: "string" },
    destination: { type: "string" },
  },
});
const mode = positionals[0];
function run(command, args, cwd = root) {
  if (command === "cargo") [command, args] = cargoCommand(args);
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
  const destination = resolve(
    values.destination ?? join(root, "desktop/resources"),
  );
  const acp = values["acp-executable"] ?? process.env.JJ_STORE_ACP_EXECUTABLE;
  if (!acp)
    throw Error("Provide the pinned jj-acp executable using --acp-executable");
  await stageDesktopResources({
    target,
    runtime,
    web,
    destination,
    acp: resolve(acp),
  });
  await stageAgentResources(
    destination,
    {
      "jj-acp": resolve(acp),
      "jj-store-local-agent": join(
        root,
        "runtime/local-agent/target/debug/jj-store-local-agent",
      ),
      "jj-store-agent-tools": join(
        root,
        "runtime/local-agent/target/debug/jj-store-agent-tools",
      ),
      "jj-store-compute": join(
        root,
        "runtime/compute/target/debug/jj-store-compute",
      ),
    },
    values["native-runtime-dir"]
      ? resolve(values["native-runtime-dir"])
      : undefined,
  );
}
if (mode === "prepare") {
  await run("pnpm", ["web:build"]);
  await run("cargo", ["build", "--locked", "-p", "jj-store", "--bins"]);
  await run("cargo", [
    "build",
    "--manifest-path",
    "runtime/embedded-rustfs/Cargo.toml",
    "--locked",
  ]);
  await run("cargo", [
    "build",
    "--manifest-path",
    "runtime/local-agent/Cargo.toml",
    "--workspace",
    "--locked",
  ]);
  await run("cargo", [
    "build",
    "--manifest-path",
    "runtime/compute/Cargo.toml",
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
  await managedCargo(["desktop-bundle"], root);
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
