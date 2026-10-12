import test from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { execFileSync } from "node:child_process";
import { revisionReader } from "./revision-reader.mjs";
import { taskHistory, integrityErrors } from "./progress-integrity.mjs";
import { migrationAudit } from "./migration-audit.mjs";
import { signature } from "./progress-validation.mjs";
import { relationContext } from "./progress-history.mjs";
import { writeEvidence } from "./evidence-new.mjs";
import { confirmationIsQualified } from "./change-confirmations.mjs";
test("unchanged_confirmation_requires_completed_validation_and_retains_red_evidence", () => {
  const entry = {
    result: "pending",
    remaining: ["Run the acceptance target"],
    runs: [],
  };
  assert.equal(confirmationIsQualified(entry), false);
  assert.equal(
    confirmationIsQualified({ ...entry, result: "pass", remaining: [] }),
    false,
  );
  assert.equal(
    confirmationIsQualified({
      result: "pass",
      remaining: [],
      runs: [
        { command: "regression before repair", result: "fail", failures: 1 },
        { command: "regression after repair", result: "pass", failures: 0 },
      ],
    }),
    true,
  );
});
import { readdirSync } from "node:fs";
test("historical_resolution_context_keeps_original_failure_and_links_later_proof", () => {
  const ledger = {
    evidencePaths: new Map([
      ["E-TEST-001", "history/old/index.md"],
      ["E-TEST-002", "history/new/index.md"],
    ]),
  };
  const context = relationContext(
    ledger,
    {
      "E-TEST-002": {
        resolves: ["E-TEST-001"],
        reason: "A later run qualifies the repair.",
      },
    },
    { id: "E-TEST-001", result: "fail" },
  );
  assert.match(context, /original result remains \*\*fail\*\*/);
  assert.match(context, /\[E-TEST-002\]\(\.\.\/new\/index\.md\)/);
  assert.match(context, /qualifies the repair/);
});
test("regenerating_a_draft_removes_obsolete_detail_fragments", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-draft-parts-"));
  try {
    const entry = {
      id: "E-TEST-003",
      date: "2000-01-01",
      scope: "proof\n".repeat(1600),
    };
    const path = writeEvidence(entry, root);
    assert.ok(readdirSync(path).length > 1);
    writeEvidence({ ...entry, scope: "short proof" }, root);
    assert.deepEqual(readdirSync(path), ["index.md"]);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
test("revision_reader_keeps_selected_content_and_detects_mid_read_changes", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-revision-"));
  const jj = (...args) =>
    execFileSync("jj", ["--no-pager", ...args], {
      cwd: root,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    }).trim();
  try {
    jj("git", "init");
    writeFileSync(join(root, "proof.txt"), "original\n");
    jj("commit", "-m", "baseline");
    const base = jj("log", "-r", "@-", "--no-graph", "-T", "commit_id");
    writeFileSync(join(root, "proof.txt"), "later\n");
    const reader = revisionReader(root, base);
    assert.equal(reader.read("proof.txt"), "original\n");
    reader.verify();
    writeFileSync(join(root, "proof.txt"), "changed during read\n");
    assert.throws(reader.verify, /changed during/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
test("task_histories_include_unlisted_evidence_with_historical_mapping", () => {
  const ledger = {
    source: "| E-M17-* | M17 | M18 |",
    tasks: [{ id: "M18" }],
    entries: [
      { id: "E-M17-001", tasks: ["M17"] },
      { id: "E-READ-096", tasks: ["M18"] },
    ],
  };
  assert.deepEqual(
    taskHistory(ledger, "M18").map((e) => e.id),
    ["E-M17-001", "E-READ-096"],
  );
});
test("migration_preserves_original_evidence_tasks_requirements_and_byte_archives", () => {
  // Audit the migration itself. Later governed requirement/task updates are
  // checked by the current ledger validator, not frozen at their migration state.
  const result = migrationAudit(
    ".",
    "40382d3599db56b4bdb33f23862e737bc521fe66",
  );
  assert.equal(result.evidence, 615);
  assert.equal(result.tasks, 142);
  assert.equal(result.requirements, 173);
});
test("canonical_evidence_signatures_ignore_key_order_without_losing_values", () => {
  assert.equal(
    signature({ b: [{ y: 2, x: 1 }], a: 1 }),
    signature({ a: 1, b: [{ x: 1, y: 2 }] }),
  );
  assert.notEqual(signature({ a: "proof" }), signature({ a: "edited proof" }));
});
test("record_authoring_rejects_an_id_from_another_date", () => {
  const root = mkdtempSync(join(tmpdir(), "spec-id-collision-"));
  try {
    mkdirSync(join(root, "spec/src/history/2000/01/01/E-TEST-001"), {
      recursive: true,
    });
    assert.throws(
      () =>
        execFileSync(
          process.execPath,
          [
            join(process.cwd(), "scripts/spec/evidence-new.mjs"),
            "--id",
            "E-TEST-001",
            "--task",
            "M0",
            "--requirement",
            "JST-GOV-001",
          ],
          { cwd: root, stdio: "pipe" },
        ),
      /evidence ID already exists/,
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
