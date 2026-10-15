import test from "node:test";
import assert from "node:assert/strict";
import { verifyInventory, testsFromList } from "./ci-inventory.mjs";
import { readFileSync } from "node:fs";
import { parse } from "yaml";
const fixture = () => ({
  format: 1,
  source: "source",
  cohort: "ordinary",
  archive_sha256: "hash",
  packages: ["p"],
  tests: ["a", "b", "ignored"],
  runnable: ["a", "b"],
  partitions: [["a"], [], ["b"], []],
});
const expected = {
  source: "source",
  cohort: "ordinary",
  hash: "hash",
  packages: ["p"],
};
test("verified empty shards are allowed only with exact source packages digest and complete disjoint inventory", () => {
  assert.doesNotThrow(() => verifyInventory(fixture(), expected));
  for (const changed of [
    { source: "other" },
    { packages: ["other"] },
    { archive_sha256: "bad" },
    { partitions: [["a"], ["a"], ["b"], []] },
    { partitions: [["a"], [], [], []] },
    { partitions: [] },
    { runnable: ["a", "b", "unknown"] },
  ])
    assert.throws(() =>
      verifyInventory({ ...fixture(), ...changed }, expected),
    );
});
test("Nextest inventories retain ignored tests without requiring their standalone execution", () => {
  const data = {
    "test-count": 2,
    "rust-suites": {
      binary: {
        "package-name": "p",
        status: "listed",
        testcases: {
          active: { ignored: false, "filter-match": { status: "matches" } },
          child: { ignored: true, "filter-match": { status: "matches" } },
        },
      },
    },
  };
  const list = testsFromList(data);
  assert.equal(list.tests.length, 2);
  assert.equal(list.runnable.length, 1);
  assert.throws(() => testsFromList({ ...data, "test-count": 3 }));
  data["rust-suites"].binary.status = "skipped";
  assert.throws(() => testsFromList(data));
});
test("selected packages reach both build and shard consumers and all archive uploads carry inventory", () => {
  for (const file of ["scripts/ci-build.mjs", "scripts/ci-tests.mjs"])
    assert.match(readFileSync(file, "utf8"), /selectedPackages\(cohort\)/);
  const ci = parse(readFileSync(".github/workflows/ci.yml", "utf8"));
  assert.equal(
    ci.jobs.plan.outputs.packages,
    "${{ steps.plan.outputs.packages }}",
  );
  for (const name of ["build", "rust"])
    assert.equal(
      ci.jobs[name].env.CI_PACKAGES,
      "${{ needs.plan.outputs.packages }}",
    );
  for (const file of ["ci.yml", "ci-heavy.yml"]) {
    const { jobs } = parse(readFileSync(`.github/workflows/${file}`, "utf8"));
    for (const job of Object.values(jobs))
      for (const step of job.steps ?? []) {
        if (step.uses?.startsWith("Swatinem/rust-cache@")) {
          assert.equal(step.with["cache-workspace-crates"], "false");
          assert.equal(step.with["cache-on-failure"], "false");
          assert.match(
            step.with["save-if"],
            /needs\.(plan|resolve)\.outputs.cache_write/,
          );
        }
        if (
          step.uses?.startsWith("actions/upload-artifact@") &&
          step.with.name.startsWith("tests-")
        )
          assert.match(step.with.path, /\.tar\.zst\*$/);
      }
  }
});
