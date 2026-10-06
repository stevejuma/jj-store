import test from "node:test";
import assert from "node:assert/strict";
import {
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  readFileSync,
  readdirSync,
  rmSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, relative } from "node:path";
import { expandSummary } from "@lapismd/spec-validator";
import { updateNavigation } from "./navigation-update.mjs";
import { writeEvidence } from "./evidence-new.mjs";
import { mergeEvidencePages } from "./evidence-format.mjs";
const files = (dir) =>
  readdirSync(dir, { withFileTypes: true }).flatMap((item) =>
    item.isDirectory() ? files(join(dir, item.name)) : [join(dir, item.name)],
  );
test("wide_navigation_and_index_pages_stay_bounded_complete_and_repeatable", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-wide-navigation-"));
  try {
    mkdirSync(join(root, "topics"));
    writeFileSync(join(root, "index.md"), "# Home\n");
    writeFileSync(join(root, "topics/index.md"), "# Topics\n");
    for (let i = 0; i < 450; i++)
      writeFileSync(join(root, `topics/topic-${i}.md`), `# Topic ${i}\n`);
    updateNavigation(root);
    const snapshot = () =>
      files(root)
        .map((path) => [relative(root, path), readFileSync(path, "utf8")])
        .sort((a, b) => a[0].localeCompare(b[0]));
    const first = snapshot();
    updateNavigation(root);
    assert.deepEqual(snapshot(), first);
    const authored = files(root).map((path) => ({
      chapterPath: relative(root, path),
      source: readFileSync(path, "utf8"),
    }));
    for (const page of authored) {
      assert.ok(Buffer.byteLength(page.source) <= 32768, page.chapterPath);
      assert.ok(page.source.split("\n").length <= 401, page.chapterPath);
    }
    const summary = expandSummary(authored).source;
    for (let i = 0; i < 450; i++)
      assert.equal(summary.split(`](topics/topic-${i}.md)`).length - 1, 1);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
test("navigation_does_not_strip_a_pages_heading_from_exact_command_evidence", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-evidence-navigation-"));
  try {
    const entry = {
      id: "E-TEST-006",
      date: "2000-01-01",
      runs: [
        {
          command: "cat <<'EOF'\n## Pages\nkeep exact\nEOF\n",
          result: "pass",
          failures: 0,
        },
      ],
    };
    const directory = writeEvidence(entry, root);
    updateNavigation(join(root, "spec/src"));
    const pages = files(directory)
      .filter((path) => /\/(?:index|details-\d+)\.md$/.test(path))
      .map((path) => readFileSync(path, "utf8"));
    assert.deepEqual(mergeEvidencePages(pages), entry);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
