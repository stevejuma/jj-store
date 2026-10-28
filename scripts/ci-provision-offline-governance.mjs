import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const root = resolve(import.meta.dirname, "..");
const dependencies = new Set();
for (const name of ["jj-conversations", "jj-history"]) {
  const pkg = JSON.parse(
    await readFile(join(root, `packages/${name}/package.json`), "utf8"),
  );
  for (const [dep, range] of Object.entries({
    ...pkg.dependencies,
    ...pkg.devDependencies,
  }))
    dependencies.add(`${dep}@${range}`);
}
const args = ["store", "add", ...dependencies];
const result = spawnSync("pnpm", args, {
  cwd: root,
  encoding: "utf8",
  stdio: "inherit",
});
if (result.status)
  throw new Error(
    `pnpm ${args.join(" ")} failed with status ${result.status ?? "signal"}`,
  );
