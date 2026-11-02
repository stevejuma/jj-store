import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

// Agent fixture commands require a JJ workspace, including on fresh Git checkouts.
export function ensureCiWorkspace(root, env = process.env) {
  assert.equal(
    env.CI,
    "true",
    "Workspace preparation is limited to disposable CI checkouts",
  );
  const gitRoot = execFileSync("git", ["rev-parse", "--show-toplevel"], {
    cwd: root,
    encoding: "utf8",
  }).trim();
  assert.equal(resolve(gitRoot), resolve(root), "Expected a checkout root");
  if (!existsSync(resolve(root, ".jj")))
    execFileSync("jj", ["git", "init", "--colocate", root], {
      cwd: root,
      stdio: "inherit",
    });
  const jjRoot = execFileSync("jj", ["root"], {
    cwd: root,
    encoding: "utf8",
  }).trim();
  assert.equal(
    resolve(jjRoot),
    resolve(root),
    "Agent commands require the selected JJ workspace",
  );
  console.log(`Verified disposable JJ workspace: ${jjRoot}`);
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  ensureCiWorkspace(process.cwd());
