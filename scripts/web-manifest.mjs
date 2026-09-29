import { readdir, writeFile } from "node:fs/promises";
const root = new URL("../web/dist/", import.meta.url);
const assets = await readdir(new URL("assets/", root));
const agentIconEntries = await readdir(new URL("agent-icons/", root), {
  withFileTypes: true,
});
if (
  assets.some(
    (name) => !/^[a-zA-Z0-9_.-]+\.(js|css|woff2|woff|ttf)$/.test(name),
  )
)
  throw new Error("Unexpected browser asset type");
if (
  agentIconEntries.some(
    (entry) =>
      !entry.isFile() ||
      (!/^[a-zA-Z0-9_.-]+\.svg$/.test(entry.name) &&
        entry.name !== "SOURCES.md"),
  )
)
  throw new Error("Unexpected agent icon asset type");
const agentIcons = agentIconEntries
  .map((entry) => entry.name)
  .filter((name) => name.endsWith(".svg"));
await writeFile(
  new URL("asset-manifest.json", root),
  JSON.stringify({
    version: 1,
    files: [
      "index.html",
      ...assets.sort().map((name) => `assets/${name}`),
      ...agentIcons.sort().map((name) => `agent-icons/${name}`),
    ],
  }),
);
