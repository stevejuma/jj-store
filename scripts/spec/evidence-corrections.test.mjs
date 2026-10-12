import test from "node:test";
import assert from "node:assert/strict";
import { validateLedger, signature } from "./progress-validation.mjs";
const original = {
  id: "E-OLD-001",
  tasks: ["M0"],
  date: "2000-01-01",
  kind: "verification",
  files: ["proof.js"],
  tests: ["Descriptive observation"],
  runs: [{ command: "node proof.js", outcome: "failed", note: "One failure." }],
  result: "partial",
  remaining: ["Original gap"],
  versions: "v1",
  formats: "v1",
};
const correction = {
  ...original,
  id: "E-FIX-001",
  date: "2000-01-02",
  tests: [],
  runs: [{ command: "check correction", result: "pass", failures: 0 }],
  result: "pass",
  remaining: [],
  corrections: [
    {
      target: original.id,
      reason:
        "Normalize malformed metadata without altering recorded commands or results.",
      fields: {
        tests: { before: original.tests, after: [] },
        runs: {
          before: original.runs,
          after: [{ ...original.runs[0], result: "fail", failures: 1 }],
        },
        result: { before: "partial", after: "pending" },
      },
    },
  ],
};
function errors(record = correction, entry = original) {
  return validateLedger({
    parsed: {
      tasks: [
        {
          id: "M0",
          status: "In progress",
          depends: [],
          requirements: ["JST-GOV-010"],
          gate: "Original gate",
          next: "Qualify",
          evidence: [entry.id],
        },
      ],
      entries: [entry, record],
      source: "",
    },
    source: "",
    read: () => "",
    requirements: new Set(["JST-GOV-010"]),
  });
}
test("appended_metadata_correction_preserves_original_payload_and_outcome", () => {
  const before = signature(original);
  assert.deepEqual(errors(), []);
  assert.equal(signature(original), before);
  assert.equal(original.result, "partial");
  assert.equal(original.runs[0].outcome, "failed");
});
test("correction_requires_exact_prior_values_and_reason", () => {
  const record = structuredClone(correction);
  record.corrections[0].fields.result.before = "pass";
  assert.ok(errors(record).some((e) => e.includes("prior value")));
  record.corrections[0].reason = "";
  assert.ok(errors(record).some((e) => e.includes("reason")));
});
test("corrections_cannot_promote_a_result_or_change_valid_run_history", () => {
  const record = structuredClone(correction);
  record.corrections[0].fields.result.after = "pass";
  assert.ok(errors(record).some((e) => e.includes("promote")));
  record.corrections[0].fields.runs.after[0].command = "different command";
  assert.ok(errors(record).some((e) => e.includes("recorded run")));
});
test("corrections_reject_unknown_fields_conflicts_and_missing_targets", () => {
  const record = structuredClone(correction);
  record.corrections[0].fields.tasks = {
    before: original.tasks,
    after: ["M1"],
  };
  assert.ok(errors(record).some((e) => e.includes("field tasks")));
  record.corrections.push(record.corrections[0]);
  assert.ok(errors(record).some((e) => e.includes("duplicate")));
  record.corrections[0].target = "E-MISSING-001";
  assert.ok(errors(record).some((e) => e.includes("target")));
});
test("corrected_references_and_results_still_receive_normal_validation", () => {
  const record = structuredClone(correction);
  record.corrections[0].fields.tests.after = ["../escape#test"];
  assert.ok(errors(record).some((e) => e.includes("unsafe evidence path")));
  record.corrections[0].fields.runs.after[0].failures = 0;
  assert.ok(errors(record).some((e) => e.includes("invalid run result")));
});
test("metadata_projection_never_supplies_a_passing_completion_gate", () => {
  const record = structuredClone(correction);
  record.corrections[0].fields.result.after = "pending";
  const issues = validateLedger({
    parsed: {
      tasks: [
        {
          id: "M0",
          status: "Done",
          depends: [],
          requirements: ["JST-GOV-010"],
          gate: "Original gate",
          next: "Review",
          evidence: [original.id],
        },
      ],
      entries: [original, record],
      source: "",
    },
    source: "",
    read: () => "",
    requirements: new Set(["JST-GOV-010"]),
  });
  assert.ok(issues.includes("M0: Done without passing gate evidence"));
});
