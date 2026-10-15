import { readFileSync, appendFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { metadata, orderedPackages, version } from "./release-metadata.mjs";
const selected = version();
const hash = (bytes) => createHash("sha256").update(bytes).digest("hex");
export async function existingPackage(
  name,
  selectedVersion,
  archive,
  request = fetch,
) {
  const options = {
    headers: {
      "User-Agent":
        "jj-store release verification (github.com/stevejuma/jj-store)",
    },
  };
  const response = await request(
    `https://crates.io/api/v1/crates/${name}/${selectedVersion}`,
    options,
  );
  if (response.status === 404) return false;
  if (!response.ok)
    throw Error(`registry metadata ${name}: HTTP ${response.status}`);
  const info = await response.json();
  if (info.version.yanked) throw Error(`existing release ${name} is yanked`);
  const remote = await request(
    `https://static.crates.io/crates/${name}/${name}-${selectedVersion}.crate`,
    options,
  );
  if (
    !remote.ok ||
    hash(Buffer.from(await remote.arrayBuffer())) !==
      hash(readFileSync(archive))
  )
    throw Error(
      `${name} exists with different package contents; stop publication`,
    );
  return true;
}
const missing = [];
for (const item of orderedPackages(metadata())) {
  const archive = `${item.path === "runtime/embedded-rustfs" ? item.path + "/" : ""}target/package/${item.name}-${selected}.crate`;
  if (await existingPackage(item.name, selected, archive)) continue;
  missing.push(item.name);
  if (process.argv.includes("--publish")) {
    execFileSync(
      "cargo",
      ["publish", "--locked", "--manifest-path", `${item.path}/Cargo.toml`],
      { stdio: "inherit" },
    );
    let accepted = false;
    for (let i = 0; i < 24; i++) {
      if (await existingPackage(item.name, selected, archive)) {
        accepted = true;
        break;
      }
      await new Promise((resolve) => setTimeout(resolve, 2500));
    }
    if (!accepted)
      throw Error(
        `publication of ${item.name} has an unknown outcome; rerun verification before retrying`,
      );
  }
}
if (process.env.GITHUB_OUTPUT)
  appendFileSync(process.env.GITHUB_OUTPUT, `missing=${missing.length}\n`);
if (
  missing.length &&
  !process.argv.includes("--status") &&
  !process.argv.includes("--publish")
)
  throw Error(`missing releases: ${missing.join(", ")}`);
console.log(
  `${missing.length} missing package versions selected in dependency order.`,
);
