import assert from "node:assert/strict";
import { readFileSync, realpathSync } from "node:fs";
import { gunzipSync } from "node:zlib";
import test from "node:test";
import { intakeFacts, intakeSlot, verifyIntake } from "./workspace-intake.mjs";
const f = JSON.parse(
  gunzipSync(
    readFileSync(
      new URL("../fixtures/jj-store-registry-intake.json.gz", import.meta.url),
    ),
  ),
);
const policyBytes = readFileSync(
    new URL(
      "../../.github/dependency-automation/canonical-intake.json",
      import.meta.url,
    ),
    "utf8",
  ),
  p = JSON.parse(policyBytes);
const document = (files) =>
  "# Source\n\n" + intakeSlot(p, intakeFacts(p, files)) + "\n";
function data() {
  const { before, after: files } = structuredClone(f);
  return {
    before,
    files,
    policyBytes,
    basePolicyBytes: policyBytes,
    documents: Object.fromEntries(p.chapters.map((n) => [n, document(files)])),
    baseDocuments: Object.fromEntries(
      p.chapters.map((n) => [n, document(before)]),
    ),
    changedFiles: ["package.json", "pnpm-lock.yaml", ...p.chapters],
  };
}
const path = ".changeset/lapis-dependencies-012345abcdef.md";
test("actual changed SDK candidate rejects oversized UTF8 maintenance note", () => {
  const d = data();
  d.changedFiles.push(path);
  d.files[path] = "---\n---\n" + "é".repeat(4096);
  assert.throws(() => verifyIntake(d));
});
test("same-fact repeat cannot admit managed product intent", () => {
  const d = data();
  d.before = structuredClone(d.files);
  d.baseDocuments = structuredClone(d.documents);
  d.changedFiles = [path];
  d.files[path] = '---\n"product": patch\n---\n';
  assert.throws(() => verifyIntake(d));
});

const notes = await import("./maintenance-note.mjs");
const { mkdtempSync, mkdirSync, writeFileSync, symlinkSync, rmSync } =
  await import("node:fs");
const { tmpdir } = await import("node:os");
const { join } = await import("node:path");
const generated =
  "---\n---\n\nRefresh first-party Lapis dependencies to the published versions recorded in this pull request.\n";
test("actual generated empty-note bytes survive changed candidate and repeat", () => {
  const d = data();
  d.changedFiles.push(path);
  d.files[path] = generated;
  verifyIntake(d);
  d.before = structuredClone(d.files);
  d.baseDocuments = structuredClone(d.documents);
  d.changedFiles = [path];
  verifyIntake(d);
  assert.equal(d.files[path], generated);
});
for (const [name, text] of [
  ["null", "---\nnull\n---\n"],
  ["array", "---\n[]\n---\n"],
  ["preamble", "Before\n---\n---\n"],
  ["malformed", "---\n{\n---\n"],
  ["BOM", "\ufeff---\n---\n"],
])
  test("bounded maintenance helper rejects " + name, () =>
    assert.throws(() => notes.verifyMaintenanceNotes([path], { [path]: text })),
  );
test("reserved namespace remains narrow and at most one note", () => {
  assert.throws(() =>
    notes.verifyMaintenanceNotes(
      [".changeset/lapis-dependencies-unknown.md"],
      {},
    ),
  );
  assert.throws(() =>
    notes.verifyMaintenanceNotes(
      [path, ".changeset/lapis-dependencies-ffffffffffff.md"],
      { [path]: generated },
    ),
  );
  assert.throws(() => notes.verifyMaintenanceNotes([path], {}));
});
test("regular UTF8 note reader rejects symlinks, malformed bytes, missing and oversized files", (t) => {
  const root = realpathSync(
    mkdtempSync(join(tmpdir(), "source-managed-note-")),
  );
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const file = join(root, "note.md");
  writeFileSync(file, generated);
  assert.equal(notes.readMaintenanceNote(file), generated);
  const link = join(root, "link.md");
  symlinkSync(file, link);
  assert.throws(() => notes.readMaintenanceNote(link));
  assert.throws(() => notes.readMaintenanceNote(join(root, "missing.md")));
  const directory = join(root, "directory.md");
  mkdirSync(directory);
  assert.throws(() => notes.readMaintenanceNote(directory));
  writeFileSync(
    file,
    Buffer.concat([Buffer.from("---\n---\n"), Buffer.from([255])]),
  );
  assert.throws(() => notes.readMaintenanceNote(file));
  writeFileSync(file, "---\n---\n" + "é".repeat(4096));
  assert.throws(() => notes.readMaintenanceNote(file));
});
test("UTF88192 boundary is enforced without counting code units", () => {
  const bytes = "---\n---\n" + "é".repeat(4092);
  assert.equal(Buffer.byteLength(bytes), 8192);
  assert.deepEqual(notes.verifyMaintenanceNotes([path], { [path]: bytes }), [
    path,
  ]);
  assert.throws(() =>
    notes.verifyMaintenanceNotes([path], { [path]: bytes + "é" }),
  );
});

test("bounded note helper preserves only the exact computed runtime release map", () => {
  const bytes =
    '---\n"@lapismd/jj-history": patch\n---\nSource-owned runtime intent.\n';
  assert.deepEqual(
    notes.verifyMaintenanceNotes(
      [path],
      { [path]: bytes },
      { "@lapismd/jj-history": "patch" },
    ),
    [path],
  );
  assert.throws(() => notes.verifyMaintenanceNotes([path], { [path]: bytes }));
});
