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
import { indexBody } from "./navigation-pages.mjs";
const files = (dir) =>
  readdirSync(dir, { withFileTypes: true }).flatMap((item) =>
    item.isDirectory() ? files(join(dir, item.name)) : [join(dir, item.name)],
  );
test("bounded_legacy_leaf_evidence_remains_byte_identical_during_navigation_refresh", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-bounded-leaf-"));
  try {
    const original = readFileSync(
      new URL(
        "../../spec/src/history/2026/11/03/E-UI-CATALOG-HELP-001/index.md",
        import.meta.url,
      ),
      "utf8",
    );
    assert.equal(original.split("\n").length - 1, 399);
    const directory = join(root, "history/entry");
    mkdirSync(directory, { recursive: true });
    writeFileSync(join(root, "index.md"), "# Specification\n");
    const file = join(directory, "index.md");
    writeFileSync(file, original);
    updateNavigation(root);
    assert.equal(readFileSync(file, "utf8"), original);
    for (const oversized of [
      original + "extra one\nextra two\n",
      "# Oversized\n" + "x".repeat(32768) + "\n",
    ]) {
      writeFileSync(file, oversized);
      assert.throws(() => updateNavigation(root), /oversized authored index/);
      assert.equal(readFileSync(file, "utf8"), oversized);
    }
    writeFileSync(file, original);
    updateNavigation(root);
    assert.equal(readFileSync(file, "utf8"), original);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
test("navigation_refresh_preserves_authored_reviews_appended_after_generated_links", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-authored-reviews-"));
  try {
    const before = "# Milestone\n\nOriginal contract.\n";
    const reviews =
      "\nOld review stays In progress.\n\nNew review preserves all gates.\n";
    const source =
      before +
      "\n<!-- generated-navigation -->\n\n## Pages\n\n- [History](history/index.md)\n" +
      reviews;
    assert.ok(indexBody(source).includes(reviews));
    writeFileSync(join(root, "index.md"), "# Root\n");
    mkdirSync(join(root, "milestone/history"), { recursive: true });
    writeFileSync(join(root, "milestone/history/index.md"), "# History\n");
    const file = join(root, "milestone/index.md");
    writeFileSync(file, source);
    updateNavigation(root);
    const first = readFileSync(file, "utf8");
    assert.ok(first.includes(reviews));
    assert.equal(
      first,
      source,
      "unchanged navigation preserves all authored bytes",
    );
    updateNavigation(root);
    assert.equal(readFileSync(file, "utf8"), first);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
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
