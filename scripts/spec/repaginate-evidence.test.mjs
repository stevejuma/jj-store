import test from "node:test";
import assert from "node:assert/strict";
import {
  mkdtempSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  writeFileSync,
  rmSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { repaginateEvidence } from "./repaginate-evidence.mjs";
import { mergeEvidencePages, renderField } from "./evidence-format.mjs";
import { signature } from "./evidence-signature.mjs";

test("repagination_is_bounded_lossless_and_repeatable_for_committed_evidence", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-repaginate-"));
  const directory = "spec/src/history/2000/01/01/E-TEST-001";
  const entry = {
    id: "E-TEST-001",
    date: "2000-01-01",
    result: "fail",
    files: Array.from({ length: 500 }, (_, i) => `file-${i}.ts`),
    runs: [
      {
        command:
          "cat <<'EOF'\n## Pages\n<!-- /evidence-field -->\nλ exact\nEOF\n",
        result: "fail",
        failures: 1,
        note: "Retain the failure & source.",
      },
    ],
  };
  try {
    mkdirSync(join(root, directory), { recursive: true });
    writeFileSync(
      join(root, directory, "index.md"),
      Object.entries(entry)
        .map(([key, value]) => renderField(key, value))
        .join("\n"),
    );
    repaginateEvidence(directory, root);
    const snapshot = () =>
      readdirSync(join(root, directory))
        .sort((a, b) =>
          a === "index.md" ? -1 : b === "index.md" ? 1 : a.localeCompare(b),
        )
        .map((name) => readFileSync(join(root, directory, name), "utf8"));
    const pages = snapshot();
    assert.ok(pages.length > 1);
    for (const page of pages) {
      assert.ok(Buffer.byteLength(page) <= 32768);
      assert.ok(page.split("\n").length <= 401);
    }
    assert.equal(signature(mergeEvidencePages(pages)), signature(entry));
    repaginateEvidence(directory, root);
    assert.deepEqual(snapshot(), pages);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
