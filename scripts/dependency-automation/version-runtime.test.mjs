import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import test from "node:test";
import YAML from "yaml";
const read = (p) =>
  readFileSync(new URL("../../" + p, import.meta.url), "utf8");
const digest = (s) => createHash("sha256").update(s).digest("hex");
test("complete Node22 script recipe remains source-bound before repository writer admission", () => {
  const q = JSON.parse(read(".github/versioning-runtime.json")),
    m = JSON.parse(read("package.json"));
  assert.equal(q.nodeMajor, 22);
  assert.equal(q.cliVersion, "2.31.1");
  assert.equal(q.packageManager, m.packageManager);
  assert.deepEqual(q.scripts, {
    "release:version": m.scripts["release:version"],
  });
  assert.equal(q.configSha256, digest(read(".changeset/config.json")));
  assert.deepEqual(
    Object.keys(q.files).sort(),
    [
      "pnpm-workspace.yaml",
      ".prettierrc.json",
      ".prettierignore",
      "scripts/release-version.mjs",
      "scripts/release-metadata.mjs",
    ].sort(),
  );
  for (const [p, h] of Object.entries(q.files))
    assert.equal(h, digest(read(p)), p);
  assert.equal(q.mode, undefined);
  assert.equal(q.workspacePolicy, undefined);
  const w = YAML.parse(read(".github/workflows/ci.yml"));
  const step = w.jobs.governance.steps.find(
    (s) => s.run === "pnpm check:dependency-intake",
  );
  assert.ok(step);
  assert.equal(step.if, undefined);
  const lanes = JSON.parse(read("scripts/validation/lanes.json"));
  for (const lane of ["check", "spec:check"])
    assert.ok(lanes[lane].startsWith("pnpm check:dependency-intake && "));
});
