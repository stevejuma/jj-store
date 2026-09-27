import { readdir, writeFile } from "node:fs/promises";
const root = new URL("../web/dist/", import.meta.url);
const assets = await readdir(new URL("assets/", root));
if (assets.some((name) => !/^[a-zA-Z0-9_.-]+\.(js|css)$/.test(name)))
  throw new Error("Unexpected browser asset type");
await writeFile(
  new URL("asset-manifest.json", root),
  JSON.stringify({
    version: 1,
    files: ["index.html", ...assets.sort().map((name) => `assets/${name}`)],
  }),
);
