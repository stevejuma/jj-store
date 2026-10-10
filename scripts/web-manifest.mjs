import { readdir, readFile, writeFile } from "node:fs/promises";
import { pathToFileURL } from "node:url";
const assetPattern = /^assets\/[a-zA-Z0-9_.-]+\.(js|css|woff2|woff|ttf)$/;

/** Traverse compiler dependencies from the application entry; unrelated emitted workers are not served. */
export function applicationAssets(manifest, entry = "index.html") {
  const files = new Set([entry]),
    visited = new Set();
  function add(file) {
    if (!assetPattern.test(file) || file.includes(".."))
      throw new Error("Unexpected browser asset path");
    files.add(file);
  }
  function visit(key) {
    if (visited.has(key)) return;
    visited.add(key);
    const chunk = manifest[key];
    if (!chunk) throw new Error(`Missing browser build entry: ${key}`);
    add(chunk.file);
    for (const file of [...(chunk.css ?? []), ...(chunk.assets ?? [])])
      add(file);
    for (const dependency of [
      ...(chunk.imports ?? []),
      ...(chunk.dynamicImports ?? []),
    ])
      visit(dependency);
  }
  visit(entry);
  // Vite emits the unsplit stylesheet as its own manifest entry.
  if (manifest["style.css"]) visit("style.css");
  return [...files].sort();
}

async function writeManifest() {
  const root = new URL("../web/dist/", import.meta.url);
  const manifest = JSON.parse(
    await readFile(new URL(".vite/manifest.json", root), "utf8"),
  );
  const files = applicationAssets(manifest);
  const available = new Set(await readdir(new URL("assets/", root)));
  for (const file of files)
    if (file !== "index.html" && !available.has(file.slice(7)))
      throw new Error(`Missing browser asset: ${file}`);
  const icons = await readdir(new URL("agent-icons/", root), {
    withFileTypes: true,
  });
  if (
    icons.some(
      (entry) =>
        !entry.isFile() ||
        (!/^[a-zA-Z0-9_.-]+\.svg$/.test(entry.name) &&
          entry.name !== "SOURCES.md"),
    )
  )
    throw new Error("Unexpected agent icon asset type");
  await writeFile(
    new URL("asset-manifest.json", root),
    JSON.stringify({
      version: 1,
      files: [
        ...files,
        ...icons
          .map((entry) => entry.name)
          .filter((name) => name.endsWith(".svg"))
          .sort()
          .map((name) => `agent-icons/${name}`),
      ],
    }),
  );
}
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href)
  await writeManifest();
