import test from "node:test";
import assert from "node:assert/strict";
import {
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  readFileSync,
  rmSync,
  statSync,
  utimesSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { bundleReferences } from "./book-references.mjs";
test("book_references_close_transitive_repository_links_and_preserve_authored_sources", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-book-references-"));
  const stage = join(root, "spec/.generated");
  try {
    for (const directory of ["docs", "spec/src/current", "spec/.generated/src"])
      mkdirSync(join(root, directory), { recursive: true });
    writeFileSync(
      join(root, "docs/a.md"),
      "# A\n\n[B](b.md#b) and [bytes](bytes.json).\n",
    );
    writeFileSync(
      join(root, "docs/b.md"),
      "# B\n\n[A](a.md) and [current](../spec/src/current/index.md).\n",
    );
    writeFileSync(join(root, "docs/bytes.json"), '{"preserve":true}\n');
    const content =
      "# Current\n\n[reference](../../../docs/a.md#a)\n\n```shell\n[example](../../../docs/a.md)\n```\n";
    writeFileSync(join(root, "spec/src/current/index.md"), content);
    const book = {
      sections: [
        {
          Chapter: {
            name: "Current",
            content,
            path: "current/index.md",
            source_path: "current/index.md",
            sub_items: [],
          },
        },
      ],
    };
    const result = bundleReferences(book, {
      repoRoot: root,
      stageRoot: stage,
      tracked: new Set(["docs/a.md", "docs/b.md", "docs/bytes.json"]),
    });
    assert.match(
      result.sections[0].Chapter.content,
      /\(\.\.\/_repository\/docs\/a\.md#a\)/,
    );
    assert.match(
      result.sections[0].Chapter.content,
      /```shell\n\[example\]\(\.\.\/\.\.\/\.\.\/docs\/a\.md\)/,
    );
    assert.equal(
      result.sections.filter((section) => section.Chapter).length,
      3,
    );
    assert.equal(
      readFileSync(join(root, "spec/src/current/index.md"), "utf8"),
      content,
    );
    assert.equal(
      readFileSync(join(stage, "src/_repository/docs/bytes.json"), "utf8"),
      '{"preserve":true}\n',
    );
    assert.match(
      readFileSync(join(stage, "src/_repository/docs/b.md"), "utf8"),
      /\(\.\.\/\.\.\/current\/index\.md\)/,
    );
    assert.throws(
      () =>
        bundleReferences(
          {
            sections: [
              {
                Chapter: {
                  content: "[untracked](../../../docs/a.md)",
                  source_path: "current/index.md",
                  sub_items: [],
                },
              },
            ],
          },
          { repoRoot: root, stageRoot: stage, tracked: new Set() },
        ),
      /tracked repository file/,
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
test("book_reference_refresh_keeps_unchanged_staging_quiet_for_live_serving", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-book-quiet-"));
  const stage = join(root, "spec/.generated");
  try {
    mkdirSync(join(root, "docs"), { recursive: true });
    writeFileSync(join(root, "docs/a.md"), "# A\n");
    writeFileSync(join(root, "docs/bytes.json"), "original\n");
    const book = () => ({
      items: [
        {
          Chapter: {
            content:
              "[doc](../../../docs/a.md) and [bytes](../../../docs/bytes.json)",
            source_path: "current/index.md",
            path: "current/index.md",
            sub_items: [],
          },
        },
      ],
    });
    const options = {
      repoRoot: root,
      stageRoot: stage,
      tracked: new Set(["docs/a.md", "docs/bytes.json"]),
    };
    bundleReferences(book(), options);
    const doc = join(stage, "src/_repository/docs/a.md"),
      asset = join(stage, "src/_repository/docs/bytes.json");
    utimesSync(doc, 0, 0);
    utimesSync(asset, 0, 0);
    bundleReferences(book(), options);
    assert.equal(statSync(doc).mtimeMs, 0);
    assert.equal(statSync(asset).mtimeMs, 0);
    writeFileSync(join(root, "docs/bytes.json"), "updated\n");
    bundleReferences(book(), options);
    assert.equal(readFileSync(asset, "utf8"), "updated\n");
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
