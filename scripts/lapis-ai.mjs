import { spawn } from "node:child_process";
import { constants } from "node:fs";
import { cp, lstat, mkdir, rename, rm } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { parseArgs } from "node:util";

export const repositoryRoot = resolve(import.meta.dirname, "..");
export const aiSource = resolve(repositoryRoot, "../lapis-ai");
export async function verifyAiBundle(directory) {
  const { verifyBundle } = await import(
    pathToFileURL(join(aiSource, "scripts/bundle.mjs"))
  );
  return verifyBundle(directory);
}
export async function exists(path) {
  try {
    await lstat(path);
    return true;
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    return false;
  }
}
export async function selectedAiBundle(
  root = repositoryRoot,
  environment = process.env,
) {
  if (environment.JJ_STORE_AI_BUNDLE_DIR)
    return resolve(root, environment.JJ_STORE_AI_BUNDLE_DIR);
  const local = join(root, ".local/lapis-ai");
  return (await exists(local))
    ? local
    : join(root, "desktop/resources/lapis-ai");
}
export async function replaceGeneratedDirectory(candidate, destination) {
  const backup = destination + `.previous-${process.pid}`;
  const retained = await exists(destination);
  if (retained) await rename(destination, backup);
  try {
    await rename(candidate, destination);
  } catch (error) {
    if (retained) await rename(backup, destination);
    throw error;
  }
  if (retained) await rm(backup, { recursive: true });
}
async function run(args) {
  await new Promise((accept, reject) => {
    const child = spawn("pnpm", args, { cwd: aiSource, stdio: "inherit" });
    child.once("error", reject);
    child.once("exit", (code, signal) =>
      code === 0
        ? accept()
        : reject(Error(`Lapis AI preparation failed: ${code ?? signal}`)),
    );
  });
}
export async function prepareAi({
  destination = join(repositoryRoot, ".local/lapis-ai"),
  nativeRuntime,
} = {}) {
  destination = resolve(destination);
  if (!destination.startsWith(repositoryRoot + "/") || destination === aiSource)
    throw Error("AI staging must use a generated directory inside jj-store");
  if (!(await exists(join(aiSource, "Cargo.toml"))))
    throw Error("Local lapis-ai checkout is required next to jj-store");
  const staged = join(
    aiSource,
    "dist",
    `jj-store-${process.pid}-${Date.now()}`,
  );
  const candidate = destination + `.pending-${process.pid}`;
  if (await exists(candidate))
    throw Error("AI staging candidate already exists");
  const candidates = [
    nativeRuntime,
    process.env.JJ_STORE_COMPUTE_NATIVE_RUNTIME_DIR,
    join(destination, "native-runtime"),
    join(repositoryRoot, "desktop/resources/native-runtime"),
  ].filter(Boolean);
  const native = await candidates.reduce(
    async (selected, path) =>
      (await selected) ?? ((await exists(path)) ? resolve(path) : undefined),
    Promise.resolve(undefined),
  );
  if (nativeRuntime && native !== resolve(nativeRuntime))
    throw Error("Selected native runtime is unavailable");
  await run([
    "dev:cargo",
    "--",
    "build",
    "-p",
    "lapis-acp",
    "-p",
    "lapis-agent",
    "-p",
    "lapis-agent-tools",
    "-p",
    "lapis-compute",
    "--locked",
  ]);
  try {
    await run([
      "package:stage",
      "--",
      "--profile",
      "debug",
      "--destination",
      staged,
      ...(native ? ["--native-runtime", native] : []),
    ]);
    await verifyAiBundle(staged);
    await mkdir(dirname(destination), { recursive: true });
    await cp(staged, candidate, {
      recursive: true,
      mode: constants.COPYFILE_FICLONE,
    });
    await verifyAiBundle(candidate);
    await replaceGeneratedDirectory(candidate, destination);
  } finally {
    await rm(staged, { recursive: true, force: true });
    await rm(candidate, { recursive: true, force: true });
  }
  console.log(`Prepared local Lapis AI bundle at ${destination}`);
  return destination;
}
if (
  process.argv[1] &&
  fileURLToPath(import.meta.url) === resolve(process.argv[1])
) {
  const { values } = parseArgs({
    args: process.argv.slice(2).filter((v) => v !== "--"),
    options: {
      destination: { type: "string" },
      "native-runtime-dir": { type: "string" },
    },
  });
  await prepareAi({
    destination: values.destination,
    nativeRuntime: values["native-runtime-dir"],
  });
}
