import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { gunzipSync } from "node:zlib";
import test from "node:test";
import YAML from "yaml";
import { intakeFacts, intakeSlot, verifyIntake } from "./workspace-intake.mjs";
const policyBytes = readFileSync(
  new URL(
    "../../.github/dependency-automation/canonical-intake.json",
    import.meta.url,
  ),
  "utf8",
);
const policy = JSON.parse(policyBytes);
const fixture = JSON.parse(
  gunzipSync(
    readFileSync(
      new URL("../fixtures/jj-store-registry-intake.json.gz", import.meta.url),
    ),
  ),
);
const document = (files) =>
  "# Source\n\n" + intakeSlot(policy, intakeFacts(policy, files)) + "\n";
function data() {
  const { before, after: files } = structuredClone(fixture);
  return {
    policyBytes,
    basePolicyBytes: policyBytes,
    files,
    before,
    documents: Object.fromEntries(
      policy.chapters.map((p) => [p, document(files)]),
    ),
    baseDocuments: Object.fromEntries(
      policy.chapters.map((p) => [p, document(before)]),
    ),
    changedFiles: ["package.json", "pnpm-lock.yaml", ...policy.chapters],
  };
}
const lock = (d, fn) => {
  const v = YAML.parse(d.files["pnpm-lock.yaml"]);
  fn(v);
  d.files["pnpm-lock.yaml"] = YAML.stringify(v);
};
test("actual released SDK0.2.0→0.2.1 transition retains four registry cohorts and both public contracts", () => {
  const d = data();
  const facts = verifyIntake(d);
  assert.equal(
    facts.manifests["package.json"].devDependencies["@lapismd/spec-validator"]
      .version,
    "0.2.1",
  );
  for (const path of policy.manifests.slice(1))
    assert.equal(d.before[path], d.files[path]);
  assert.equal(
    YAML.parse(d.files["pnpm-lock.yaml"]).importers["."].devDependencies[
      "@lapismd/jj-conversations"
    ].version,
    "link:packages/jj-conversations",
  );
});
for (const [name, mutate] of [
  ["missing facts", (d) => (d.documents[policy.chapters[0]] = "none")],
  [
    "stale facts",
    (d) =>
      (d.documents[policy.chapters[0]] = d.baseDocuments[policy.chapters[0]]),
  ],
  [
    "manual canonical prose",
    (d) => (d.documents[policy.chapters[0]] += "Manual"),
  ],
  ["changed opt-in", (d) => (d.basePolicyBytes += " ")],
  ["unexpected source", (d) => d.changedFiles.push("web/src/lib/unknown.ts")],
  [
    "public exports",
    (d) => {
      const p = JSON.parse(d.files[policy.manifests[1]]);
      p.exports = { ".": "./wrong.js" };
      d.files[policy.manifests[1]] = JSON.stringify(p);
    },
  ],
  [
    "owned Conversations projection",
    (d) =>
      lock(
        d,
        (l) =>
          (l.importers["."].devDependencies[
            "@lapismd/jj-conversations"
          ].version = "0.1.0"),
      ),
  ],
  [
    "immutable Core metadata",
    (d) =>
      lock(
        d,
        (l) =>
          (l.packages["@lapismd/design-core@0.13.5"].engines = {
            node: ">=999",
          }),
      ),
  ],
  [
    "unrelated direct edge",
    (d) =>
      lock(d, (l) => (l.importers["."].devDependencies.yaml.version = "2.9.0")),
  ],
  [
    "first-party catalog edge",
    (d) =>
      lock(
        d,
        (l) =>
          (l.snapshots[
            Object.keys(l.snapshots).find((k) => k.startsWith("@lapismd/mira@"))
          ].dependencies["@lapismd/unknown"] = "0.1.0"),
      ),
  ],
  [
    "unsupported SDK cohort",
    (d) => {
      const p = JSON.parse(d.files["package.json"]);
      p.devDependencies["@lapismd/spec-validator"] = "^0.3.0";
      d.files["package.json"] = JSON.stringify(p);
    },
  ],
])
  test("source guard rejects " + name, () => {
    const d = data();
    mutate(d);
    assert.throws(() => verifyIntake(d));
  });
test("actual adjacent-delimiter generated empty note adds no runtime release intent", () => {
  const d = data(),
    p = ".changeset/lapis-dependencies-012345abcdef.md";
  d.changedFiles.push(p);
  d.files[p] = "---\n---\n\nRefresh first-party Lapis dependencies.\n";
  assert.ok(verifyIntake(d));
});
for (const [name, body] of [
  ["null", "null\n"],
  ["runtime", '"@lapismd/jj-store": patch\n'],
  ["malformed", "{\n"],
])
  test("managed note rejects " + name + " release metadata", () => {
    const d = data(),
      p = ".changeset/lapis-dependencies-012345abcdef.md";
    d.changedFiles.push(p);
    d.files[p] = "---\n" + body + "---\n";
    assert.throws(() => verifyIntake(d));
  });
test("source policy cannot admit the owned workspace output as a registry input", () => {
  const d = data(),
    p = JSON.parse(d.policyBytes);
  p.dependencies.push("@lapismd/jj-conversations");
  d.policyBytes = JSON.stringify(p);
  assert.throws(() => verifyIntake(d));
});

test("source-owned prose formatter may wrap only the exact digest paragraph", () => {
  const d = data();
  for (const p of policy.chapters)
    d.documents[p] = d.documents[p].replace(
      "Manifest, peer and integrity facts SHA256: `",
      "Manifest, peer and integrity facts SHA256:\n`",
    );
  assert.ok(verifyIntake(d));
  d.documents[policy.chapters[0]] = d.documents[policy.chapters[0]].replace(
    /SHA256:\n`[a-f0-9]{64}`/,
    "SHA256:\n`" + "0".repeat(64) + "`",
  );
  assert.throws(() => verifyIntake(d));
});
