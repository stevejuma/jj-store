import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { restorePinnedSources } from "./pins.mjs";
import { execute } from "./process.mjs";
import { ensureAiSource } from "../ci-lapis-source.mjs";
import { ensureRedactionSource } from "../ci-redaction-source.mjs";
export async function prepareFull(root, input, source, lane) {
  await restorePinnedSources(root, input, source.pins);
  await ensureAiSource(root, { CI: "true" });
  await ensureRedactionSource(root, { CI: "true" });
  await execute("pnpm", ["install", "--frozen-lockfile"], {
    cwd: "/work/lapis-ai",
  });
  if (lane === "test:governance") {
    // Offline clean-install regressions need registry metadata as well as locked tarballs.
    const dependencies = new Set();
    for (const name of ["jj-conversations", "jj-history"]) {
      const pkg = JSON.parse(
        await readFile(join(root, `packages/${name}/package.json`), "utf8"),
      );
      for (const [name, range] of Object.entries({
        ...pkg.dependencies,
        ...pkg.devDependencies,
      }))
        dependencies.add(`${name}@${range}`);
    }
    await execute("pnpm", ["store", "add", ...dependencies], { cwd: root });
    await execute("cargo", ["fetch", "--locked"], {
      cwd: root,
    });
  }
}
