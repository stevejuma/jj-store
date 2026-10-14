import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  validateLedger,
  ledgerPath,
  needsEvidence,
  run,
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
test("service_owned_review_reader_is_within_layer_boundary", () => {
  assert.deepEqual(
    dependencyErrors([
      { name: "jj-store-service", dependencies: [{ name: "jj-review" }] },
    ]),
    [],
  );
});
test("server_composes_pure_collaboration_types_without_reverse_runtime_dependencies", () => {
  assert.deepEqual(
    dependencyErrors([
      { name: "jj-store-server", dependencies: [{ name: "jj-collab" }] },
    ]),
    [],
  );
  assert.equal(
    dependencyErrors([
      { name: "jj-collab", dependencies: [{ name: "jj-store-server" }] },
      { name: "jj-store-server", dependencies: [{ name: "jj-cli" }] },
    ]).length,
    2,
  );
});
test("accepts_m9_delivery_tasks", () =>
  assert.deepEqual(
    validateLedger(
      fixture(
        "| M9 | Planned | - | JST-GOV-001 | collaboration | continue | - |",
        [],
      ),
    ),
    [],
  ));
test("accepts_multi_digit_milestone_tasks", () =>
  assert.deepEqual(
    validateLedger(
      fixture(
        "| M10 | Planned | - | JST-GOV-001 | replication | continue | - |",
        [],
      ),
    ),
    [],
  ));
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
test("accepts_evidence_superseded by tracked removals", () => {
  const retired = {
    ...entry,
    id: "E-TEST-002",
    files: ["removed.js", "test.rs"],
    tests: ["removed.js#old_proof", "test.rs#old_proof"],
  };
  const replacement = {
    ...entry,
    removed_files: ["removed.js"],
    removed_tests: ["test.rs#old_proof"],
  };
  assert.deepEqual(
    validateLedger(
      fixture(
        "| Bootstrap | Done | - | JST-GOV-001 | gate | continue | E-TEST-001,E-TEST-002 |",
        [retired, replacement],
      ),
    ),
    [],
  );
});
test("rejects_removed_evidence_that_still_exists", () =>
  assert.match(
    validateLedger(
      fixture(undefined, [{ ...entry, removed_files: ["test.rs"] }]),
    ).join(),
    /removed evidence file still exists/,
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
test("progress_checks_merge_against_every_parent", () => {
  const root = mkdtempSync(join(tmpdir(), "jj-store-progress-merge-"));
  const jj = (...args) =>
    execFileSync("jj", ["--no-pager", ...args], {
      cwd: root,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    }).trim();
  const ledger = (entries) =>
    writeFileSync(join(root, ledgerPath), fixture(undefined, entries).source);
  try {
    jj("git", "init");
    mkdirSync(join(root, "spec/src"), { recursive: true });
    writeFileSync(
      join(root, "spec/src/boundary.md"),
      "| JST-GOV-001 | rule |\n",
    );
    writeFileSync(join(root, "test.rs"), "#[test] fn proof() {}\n");
    ledger([entry]);
    jj("commit", "-m", "baseline");
    const base = jj("log", "-r", "@-", "--no-graph", "-T", "commit_id");
    const left = { ...entry, id: "E-TEST-002" },
      right = { ...entry, id: "E-TEST-003" };
    mkdirSync(join(root, "web"));
    writeFileSync(join(root, "web/left.ts"), "export {};\n");
    ledger([entry, left]);
    jj("commit", "-m", "left");
    const leftId = jj("log", "-r", "@-", "--no-graph", "-T", "commit_id");
    jj("new", base, "-m", "right");
    mkdirSync(join(root, "web"), { recursive: true });
    writeFileSync(join(root, "web/right.ts"), "export {};\n");
    ledger([entry, right]);
    jj("commit", "-m", "right");
    const rightId = jj("log", "-r", "@-", "--no-graph", "-T", "commit_id");
    jj("new", leftId, rightId, "-m", "merge");
    ledger([entry, left, right]);
    assert.throws(() => run(root), /new or updated evidence/);
    ledger([entry, left, right, { ...entry, id: "E-TEST-004" }]);
    assert.doesNotThrow(() => run(root));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
test("rejects_existing_parent_variant_as_new_merge_evidence", () =>
  assert.match(
    validateLedger(
      fixture(undefined, undefined, {
        changed: ["web/merged.ts", ledgerPath],
        previousEntries: [entry, { ...entry, kind: "other-parent" }],
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
      {
        name: "jj-store-rpc",
        dependencies: [{ name: "jj-lib" }, { name: "jj-store-wal" }],
      },
    ]).length,
    4,
  ));
test("accepts_owned_dependencies", () =>
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-store-jj",
        dependencies: [{ name: "jj-lib" }, { name: "jj-store-wal" }],
      },
      {
        name: "jj-store-rpc",
        dependencies: [
          { name: "jj-store-model" },
          { name: "jj-store-object" },
          { name: "jj-store-service" },
        ],
      },
    ]),
    [],
  ));

test("collaboration_targets_use_owned_work_types_without_reverse_or_runtime_edges", () => {
  assert.deepEqual(
    dependencyErrors([
      { name: "jj-collab", dependencies: [{ name: "jj-work" }] },
    ]),
    [],
  );
  assert.equal(
    dependencyErrors([
      { name: "jj-work", dependencies: [{ name: "jj-collab" }] },
      {
        name: "jj-collab",
        dependencies: [{ name: "jj-store-wal" }, { name: "jj-lib" }],
      },
    ]).length,
    3,
  );
});

test("agent_models_stay_below_publication_and_native_layers", () => {
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-agent",
        dependencies: [{ name: "jj-store-model" }],
      },
      {
        name: "jj-store-wal",
        dependencies: [
          { name: "jj-agent" },
          { name: "jj-store-model" },
          { name: "jj-store-object" },
        ],
      },
    ]),
    [],
  );
  assert.match(
    dependencyErrors([
      {
        name: "jj-agent",
        dependencies: [{ name: "jj-store-wal" }],
      },
    ]).join(),
    /invalid layer dependency jj-store-wal/,
  );
});

test("server_composition_cannot_depend_on_cli_or_native_upstream", () => {
  assert.equal(
    dependencyErrors([
      {
        name: "jj-store-server",
        dependencies: [{ name: "jj-store" }, { name: "jj-lib" }],
      },
    ]).length,
    2,
  );
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-store-server",
        dependencies: [
          { name: "jj-store-jj" },
          { name: "jj-store-rpc" },
          { name: "jj-store-service" },
          { name: "jj-store-http" },
          { name: "jj-store-adapters" },
          { name: "jj-store-tantivy" },
        ],
      },
    ]),
    [],
  );
});

test("capture_runtime_consumes_owned_interfaces_without_reverse_dependencies", () => {
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-store-capture",
        dependencies: [
          "jj-agent",
          "jj-collab",
          "jj-store-model",
          "jj-store-wal",
          "jj-store-service",
          "jj-store-rpc",
        ].map((name) => ({ name })),
      },
      { name: "jj-store-server", dependencies: [{ name: "jj-store-capture" }] },
    ]),
    [],
  );
  const rejected = dependencyErrors([
    {
      name: "jj-store-capture",
      dependencies: [
        "jj-lib",
        "jj-store",
        "jj-store-server",
        "jj-store-jj",
      ].map((name) => ({ name })),
    },
    ...["jj-agent", "jj-store-model", "jj-store-object", "jj-store-wal"].map(
      (name) => ({ name, dependencies: [{ name: "jj-store-capture" }] }),
    ),
  ]);
  assert.equal(rejected.length, 8);
});

test("http_consumes_owned_service_contracts_without_storage_or_upstream", () => {
  assert.equal(
    dependencyErrors([
      {
        name: "jj-store-http",
        dependencies: [
          { name: "jj-lib" },
          { name: "jj-store-object" },
          { name: "jj-store-jj" },
        ],
      },
    ]).length,
    3,
  );
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-store-http",
        dependencies: [
          { name: "jj-store-model" },
          { name: "jj-store-service" },
        ],
      },
    ]),
    [],
  );
});

test("requires_catalog_evidence", () => {
  for (const path of [
    "catalog/components/Example.svelte",
    ".storybook/main.ts",
  ]) {
    assert.equal(needsEvidence(path), true);
  }
});

test("service_review_models_preserve_core_layer_boundaries", () => {
  assert.deepEqual(
    dependencyErrors([
      { name: "jj-store-service", dependencies: [{ name: "jj-review" }] },
      {
        name: "jj-review",
        dependencies: [{ name: "jj-agent" }, { name: "jj-store-model" }],
      },
    ]),
    [],
  );
  assert.equal(
    dependencyErrors([
      { name: "jj-store-model", dependencies: [{ name: "jj-review" }] },
      { name: "jj-review", dependencies: [{ name: "jj-store-wal" }] },
    ]).length,
    2,
  );
});
