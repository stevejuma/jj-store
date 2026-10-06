import test from "node:test";
import assert from "node:assert/strict";
import { evidencePages, mergeEvidencePages } from "./evidence-format.mjs";
test("literal_evidence_markers_inside_commands_are_not_record_structure", () => {
  const entry = {
    id: "E-TEST-005",
    scope: "Read `<!-- /evidence-field -->` literally.",
    runs: [
      {
        command:
          "cat <<'EOF'\n<!-- evidence-field:fake text -->\n<!-- /evidence-field -->\n<!-- /run -->\nEOF\n",
        result: "pass",
        failures: 0,
        note: "Keep `<!-- /run -->` in the note.",
      },
    ],
  };
  assert.deepEqual(mergeEvidencePages(evidencePages(entry)), entry);
});
test("markdown_evidence_preserves_all_fields_and_exact_multiline_commands", () => {
  const entry = {
    id: "E-TEST-001",
    tasks: ["M0"],
    scope: "A paragraph.\n\nA second line.\n",
    versions: { native: ["a", "b"] },
    runs: [
      {
        command: "printf 'a|b'\ncat <<'EOF'\n```\nEOF\n",
        result: "fail",
        failures: 2,
        note: "Exact note\n",
        detail: "retained extra",
      },
      { command: "check", result: "pass", failures: 0 },
    ],
    remaining: [],
  };
  assert.deepEqual(mergeEvidencePages(evidencePages(entry)), entry);
});
test("prose_entities_and_type_placeholders_render_without_losing_original_values", () => {
  const entry = {
    id: "E-TEST-003",
    scope: "Result<String> and <request>; literal &lt; and `Result<String>`.",
    runs: [
      {
        command: "echo '<value>'",
        result: "pass",
        failures: 0,
        note: "Use <cwd> and `Option<String>`; retain &amp;.",
      },
    ],
  };
  const pages = evidencePages(entry);
  assert.match(pages.join("\n"), /Result&lt;String&gt;/);
  assert.match(pages.join("\n"), /`Result<String>`/);
  assert.deepEqual(mergeEvidencePages(pages), entry);
});
test("long_unicode_and_many_line_fields_split_at_safe_character_boundaries", () => {
  const entry = {
    id: "E-TEST-004",
    scope: "😀 <request> & literal\n".repeat(1800),
  };
  const pages = evidencePages(entry);
  for (const page of pages) {
    assert.ok(Buffer.byteLength(page) <= 32768);
    assert.ok(page.split("\n").length <= 400);
  }
  assert.deepEqual(mergeEvidencePages(pages), entry);
});
test("oversized_history_splits_at_runs_without_losing_order", () => {
  const entry = {
    id: "E-TEST-002",
    files: ["test"],
    runs: Array.from({ length: 300 }, (_, n) => ({
      command: `test-${n}`,
      result: "pass",
      failures: 0,
      note: "complete original proof",
    })),
  };
  const pages = evidencePages(entry);
  assert.ok(pages.length > 1);
  for (const page of pages) {
    assert.ok(Buffer.byteLength(page) <= 32768);
    assert.ok(page.split("\n").length <= 400);
  }
  assert.deepEqual(mergeEvidencePages(pages), entry);
});
