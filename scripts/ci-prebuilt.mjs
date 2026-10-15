import { constants } from "node:fs";
import { access, lstat, readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { helpAssets } from "./docs/assets.mjs";

/** CI consumes artifacts from this workflow run; ordinary development still builds. */
export async function prebuiltBrowserResources(
  root,
  environment = process.env,
) {
  if (environment.JJ_STORE_CI_PREBUILT !== "1") return false;
  if (environment.CI !== "true")
    throw Error("Prebuilt browser resources require explicit CI execution");
  const runtime = environment.JJ_STORE_DEV_RUSTFS_RUNTIME;
  if (!runtime) throw Error("Prebuilt CI requires an explicit RustFS runtime");
  for (const path of [
    join(root, "target/debug/jj-store"),
    join(root, "target/debug/jj-store-jj"),
    resolve(root, runtime),
  ]) {
    const stat = await lstat(path);
    if (!stat.isFile() || stat.isSymbolicLink())
      throw Error(`Unsafe prebuilt executable: ${path}`);
    await access(path, constants.X_OK);
  }
  const assets = join(root, "web/dist");
  const manifestPath = join(assets, "asset-manifest.json");
  const metadata = await lstat(manifestPath);
  if (!metadata.isFile() || metadata.isSymbolicLink() || metadata.size > 16384)
    throw Error("Invalid prebuilt browser manifest");
  const manifest = JSON.parse(await readFile(manifestPath, "utf8"));
  if (
    manifest.version !== 1 ||
    !Array.isArray(manifest.files) ||
    !manifest.files.includes("index.html") ||
    manifest.files.length > 256 ||
    new Set(manifest.files).size !== manifest.files.length
  )
    throw Error("Invalid prebuilt browser manifest");
  for (const name of manifest.files) {
    if (
      typeof name !== "string" ||
      !/^(?:index\.html|assets\/[a-zA-Z0-9_.-]+\.(?:js|css|woff2?|ttf)|agent-icons\/[a-zA-Z0-9_.-]+\.svg)$/.test(
        name,
      ) ||
      name.includes("..")
    )
      throw Error("Unsafe prebuilt browser path");
    let path = assets;
    for (const part of name.split("/")) {
      path = join(path, part);
      if ((await lstat(path)).isSymbolicLink())
        throw Error("Prebuilt asset symlink");
    }
    if (!(await lstat(path)).isFile())
      throw Error("Invalid prebuilt browser asset");
  }
  await helpAssets(join(assets, "help"));
  return true;
}
