import { readFileSync, writeFileSync } from "node:fs";
import { metadata, validateVersion } from "./release-metadata.mjs";
const selected = JSON.parse(readFileSync("package.json", "utf8")).version;
if (!/^\d+\.\d+\.\d+$/.test(selected))
  throw Error("invalid Changesets version");
const data = metadata();
let source = readFileSync("Cargo.toml", "utf8");
source = source
  .replace(/^(version = )"[^"]+"/m, `$1"${selected}"`)
  .replace(/(path = "crates\/[^"]+", version = )"[^"]+"/g, `$1"${selected}"`);
writeFileSync("Cargo.toml", source);
const helper = "runtime/embedded-rustfs/Cargo.toml";
writeFileSync(
  helper,
  readFileSync(helper, "utf8").replace(
    /^(version = )"[^"]+"/m,
    `$1"${selected}"`,
  ),
);
for (const p of data.packages) {
  const lock = `${p.path === "runtime/embedded-rustfs" ? p.path + "/" : ""}Cargo.lock`;
  let text = readFileSync(lock, "utf8");
  for (const item of data.packages)
    text = text.replace(
      new RegExp(`(name = "${item.name}"\\nversion = )"[^"]+"`, "g"),
      `$1"${selected}"`,
    );
  writeFileSync(lock, text);
}
validateVersion(metadata(), selected);
console.log(
  `Synchronized coordinated release ${selected}; npm publication remains disabled.`,
);
