import test from "node:test";
import assert from "node:assert/strict";
import {
  validateLedger,
  ledgerPath,
  needsEvidence,
} from "./check-progress.mjs";
import { dependencyErrors } from "./check-boundaries.mjs";

const entry = {
  id: "E-TEST-001",
  tasks: ["Bootstrap"],
  date: "2026-09-26",
  kind: "governance",
  files: ["test.rs"],
  tests: ["test.rs#proof"],
  runs: [{ command: "cargo test proof", result: "pass", failures: 0 }],
  result: "pass",
  remaining: [],
  versions: "test",
  formats: "none",
};
function fixture(
  rows = "| Bootstrap | Done | - | JST-GOV-001 | gate | continue | E-TEST-001 |",
  entries = [entry],
  extra = {},
) {
  return {
    source: `${rows}\n\n\`\`\`json\n${JSON.stringify(entries)}\n\`\`\``,
    read: (path) => {
      if (path === "test.rs") return "#[test] fn proof() {}";
      throw Error("missing");
    },
    requirements: new Set(["JST-GOV-001"]),
    ...extra,
  };
}
test("accepts_verified_task", () =>
  assert.deepEqual(validateLedger(fixture()), []));
test("rejects_missing_evidence", () =>
  assert.match(
    validateLedger(fixture(undefined, [])).join(),
    /missing evidence/,
  ));
test("rejects_completed_parent_with_incomplete_child", () =>
  assert.match(
    validateLedger(
      fixture(
        "| Bootstrap | Done | - | JST-GOV-001 | gate | next | E-TEST-001 |\n| Bootstrap.1 | Planned | - | JST-GOV-001 | child gate | next | - |",
      ),
    ).join(),
    /incomplete child/,
  ));
test("rejects_missing_named_test", () =>
  assert.match(
    validateLedger(
      fixture(undefined, [{ ...entry, tests: ["test.rs#absent"] }]),
    ).join(),
    /missing named test/,
  ));
test("rejects_invented_test_mention", () =>
  assert.match(
    validateLedger(
      fixture(undefined, undefined, { read: () => "// proof is not a test" }),
    ).join(),
    /missing named test/,
  ));
test("rejects_dependency_cycle", () =>
  assert.match(
    validateLedger(
      fixture(
        "| Bootstrap | Planned | M0 | JST-GOV-001 | gate | next | - |\n| M0 | Planned | Bootstrap | JST-GOV-001 | gate | next | - |",
        [],
      ),
    ).join(),
    /cycle/,
  ));
test("rejects_unchanged_ledger_for_code", () =>
  assert.match(
    validateLedger(
      fixture(undefined, undefined, {
        changed: ["crates/a/src/lib.rs"],
        previousEntries: [entry],
      }),
    ).join(),
    /same-change ledger/,
  ));
test("rejects_touch_without_new_evidence", () =>
  assert.match(
    validateLedger(
      fixture(undefined, undefined, {
        changed: ["crates/a/src/lib.rs", ledgerPath],
        previousEntries: [entry],
      }),
    ).join(),
    /new or updated evidence/,
  ));
test("allows_documentation_without_execution_ledger", () =>
  assert.equal(needsEvidence("docs/cli/tutorial.md"), false));
test("rejects_false_passing_results", () =>
  assert.match(
    validateLedger(
      fixture(undefined, [
        { ...entry, runs: [{ command: "test", result: "pass", failures: 1 }] },
      ]),
    ).join(),
    /invalid run/,
  ));
test("rejects_fake_rustfs_qualification", () =>
  assert.match(
    validateLedger(
      fixture(
        "| M0.3 | Done | - | JST-GOV-001 | real storage | next | E-TEST-001 |",
        [{ ...entry, tasks: ["M0.3"], kind: "fake" }],
      ),
    ).join(),
    /real RustFS/,
  ));
test("rejects_unknown_requirement", () =>
  assert.match(
    validateLedger(
      fixture(undefined, undefined, { requirements: new Set() }),
    ).join(),
    /unknown requirement/,
  ));
test("rejects_layer_and_native_dependency_leaks", () =>
  assert.equal(
    dependencyErrors([
      {
        name: "jj-store-wal",
        dependencies: [{ name: "jj-lib" }, { name: "jj-store-tantivy" }],
      },
    ]).length,
    2,
  ));
test("accepts_owned_dependencies", () =>
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-store-jj",
        dependencies: [{ name: "jj-lib" }, { name: "jj-store-wal" }],
      },
    ]),
    [],
  ));
