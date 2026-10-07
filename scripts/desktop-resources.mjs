import { cp, mkdir, readFile, rm, stat } from "node:fs/promises";
import { constants } from "node:fs";
import { resolve, join } from "node:path";
import { helpAssets } from "./docs/assets.mjs";
export async function stageDesktopResources({
  target,
  runtime,
  web,
  destination,
  acp,
}) {
  const binaries = [
    [join(target, "jj-store"), "jj-store"],
    [join(target, "jj"), "jj"],
    [runtime, "jj-store-rustfs"],
  ];
  if (acp) binaries.push([acp, "jj-acp"]);
  const manifest = JSON.parse(
    await readFile(join(web, "asset-manifest.json"), "utf8"),
  );
  if (
    manifest.version !== 1 ||
    !Array.isArray(manifest.files) ||
    manifest.files.length > 256
  )
    throw Error("Invalid production browser manifest");
  for (const [source] of binaries)
    if (!(await stat(source)).isFile())
      throw Error(`Missing executable: ${source}`);
  const help = await helpAssets(join(web, "help"));
  const files = ["asset-manifest.json", ...manifest.files];
  for (const file of files) {
    if (
      !/^(?:asset-manifest\.json|index\.html|(?:assets|agent-icons)\/[a-zA-Z0-9_.-]+)$/.test(
        file,
      ) ||
      file.includes("..")
    )
      throw Error("Unsafe browser asset path");
    if (!(await stat(join(web, file))).isFile())
      throw Error(`Missing browser asset: ${file}`);
  }
  await rm(destination, { recursive: true, force: true });
  await mkdir(join(destination, "web"), { recursive: true });
  for (const [source, name] of binaries)
    await cp(source, join(destination, name), {
      mode: constants.COPYFILE_FICLONE,
    });
  for (const file of files) {
    await mkdir(resolve(destination, "web", file, ".."), { recursive: true });
    await cp(join(web, file), join(destination, "web", file));
  }
  for (const name of ["asset-manifest.json", ...help.keys()]) {
    const file = join("help", name);
    await mkdir(resolve(destination, "web", file, ".."), { recursive: true });
    await cp(join(web, file), join(destination, "web", file));
  }
  console.log(
    `Staged paired runtime and ${manifest.files.length} production browser assets at ${destination}`,
  );
}
