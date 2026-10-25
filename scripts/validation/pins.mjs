import {
  repositoryGitRoot,
  repositoryParent,
  snapshotFile,
} from "./repository.mjs";
import { execFileSync } from "node:child_process";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { join, resolve } from "node:path";
import { hash } from "./model.mjs";
const sources = [
  {
    name: "lapis-ai",
    packageName: "@lapismd/lapis-ai",
    file: "scripts/lapis-ai-source.json",
  },
  {
    name: "agent-redaction",
    packageName: "@lapismd/agent-redaction",
    file: "scripts/agent-redaction-source.json",
  },
];
export async function exportPinnedSources(root, input, revision) {
  const entries = [];
  for (const source of sources) {
    const pin = JSON.parse(
      snapshotFile(root, revision, source.file).toString("utf8"),
    );
    if (
      pin.repository !== `lapismd/${source.name}` ||
      !/^[a-f0-9]{40}$/.test(pin.revision)
    )
      throw Error("Invalid sibling source pin");
    const sibling = join(repositoryParent(root), source.name);
    const pkg = JSON.parse(
      snapshotFile(sibling, pin.revision, "package.json").toString("utf8"),
    );
    if (pkg.name !== source.packageName || pkg.version !== pin.version)
      throw Error(`Pinned ${source.name} package identity mismatch`);
    const path = `source-${source.name}.tar`;
    execFileSync(
      "git",
      [
        "--git-dir",
        repositoryGitRoot(sibling),
        "archive",
        "--format=tar",
        `--output=${join(input, path)}`,
        pin.revision,
      ],
      { cwd: sibling },
    );
    entries.push({
      ...source,
      pin,
      path,
      sha256: hash(await readFile(join(input, path))),
    });
  }
  return entries;
}
export async function restorePinnedSources(root, input, entries) {
  for (const source of sources) {
    const entry = entries?.find((item) => item.name === source.name);
    if (!entry) throw Error(`Missing pinned ${source.name} source transfer`);
    const pin = JSON.parse(await readFile(join(root, source.file), "utf8"));
    if (
      JSON.stringify(entry.pin) !== JSON.stringify(pin) ||
      entry.path !== `source-${source.name}.tar`
    )
      throw Error("Sibling source pin mismatch");
    const archive = join(input, entry.path);
    if (hash(await readFile(archive)) !== entry.sha256)
      throw Error("Sibling source checksum mismatch");
    const destination = resolve(root, `../${source.name}`);
    await mkdir(destination);
    execFileSync("tar", ["-xf", archive, "-C", destination]);
    const pkg = JSON.parse(
      await readFile(join(destination, "package.json"), "utf8"),
    );
    if (pkg.name !== source.packageName || pkg.version !== pin.version)
      throw Error("Restored sibling package identity mismatch");
    await writeFile(
      join(destination, ".jj-store-source.json"),
      JSON.stringify(pin),
    );
  }
}
