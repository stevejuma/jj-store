import assert from "node:assert/strict";
import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { cohortProfile } from "./ci-packages.mjs";

export function verifyCohortEvidence(reports, expected) {
  const tests = new Set();
  let executed = 0;
  for (let shard = 1; shard <= expected.shards; shard++) {
    const matches = reports.filter(
      (entry) => entry.report.suite === `${expected.cohort}-${shard}`,
    );
    assert.equal(
      matches.length,
      1,
      `Missing or duplicate ${expected.cohort} shard ${shard}`,
    );
    const { report, inventory } = matches[0];
    for (const key of ["source", "run", "attempt"])
      assert.equal(report[key], expected[key], `Report ${key} mismatch`);
    assert.equal(inventory.source, expected.source);
    assert.equal(inventory.cohort, expected.cohort);
    assert.equal(inventory.profile, expected.profile);
    assert.equal(
      inventory.archive_sha256,
      expected.archive_sha256,
      "Shard archive digest mismatch",
    );
    assert.deepEqual(
      inventory.packages,
      expected.packages,
      "Shard package selection mismatch",
    );
    assert.deepEqual(
      inventory.tests,
      expected.partitions[shard - 1],
      "Shard selection differs from producer inventory",
    );
    assert.equal(inventory.shard, shard);
    assert.equal(inventory.expected, inventory.tests.length);
    for (const identity of inventory.tests) {
      assert.ok(!tests.has(identity), "Duplicate test across shards");
      tests.add(identity);
    }
    const junit = report.results.junit.filter((entry) =>
      entry.path.startsWith("target/nextest/ci/"),
    );
    assert.equal(
      junit.length,
      1,
      `Missing or duplicate JUnit for shard ${shard}`,
    );
    assert.equal(junit[0].counts.failed, 0, `Failed tests in shard ${shard}`);
    assert.equal(
      junit[0].counts.passed,
      inventory.expected,
      `Incomplete execution in shard ${shard}`,
    );
    const expectedCases = inventory.tests
      .map((identity) => {
        const [, binary, name] = JSON.parse(identity);
        return JSON.stringify([binary, name]);
      })
      .sort();
    const actualCases = junit[0].cases
      .filter((entry) => entry.result === "passed")
      .map((entry) => JSON.stringify([entry.class, entry.name]))
      .sort();
    assert.deepEqual(
      actualCases,
      expectedCases,
      `Wrong test identities in shard ${shard}`,
    );
    assert.ok(
      report.commands?.some(
        (command) =>
          command.label ===
            (expected.cohort === "targeted"
              ? "targeted-tests"
              : "rust-tests") &&
          command.exit_code === 0 &&
          !command.signal,
      ),
      "Missing successful test command",
    );
    executed += junit[0].counts.passed;
  }
  assert.ok(executed > 0, "Nonempty Rust selection executed no tests");
  return executed;
}
export const verifyOrdinaryEvidence = (reports, expected) =>
  verifyCohortEvidence(reports, {
    ...expected,
    cohort: "ordinary",
    profile: "ci",
    shards: 4,
  });

export function selectProducerInventory(inventories) {
  assert.ok(inventories.length > 0, "Missing producer inventory");
  const producer = inventories[0];
  for (const inventory of inventories.slice(1))
    assert.deepEqual(
      inventory,
      producer,
      "Conflicting producer inventory copies",
    );
  return producer;
}

function main() {
  const cohort = process.argv[2] || "ordinary";
  if (!["ordinary", "targeted"].includes(cohort))
    throw Error("Unsupported evidence cohort");
  const shards = cohort === "targeted" ? 2 : 4;
  const root = ".release/downloaded";
  assert.ok(existsSync(root), `Missing ${cohort} reports`);
  const reports = readdirSync(root).flatMap((name) => {
    const dir = join(root, name);
    const paths = readdirSync(dir, { recursive: true }).filter((path) =>
      /(?:^|\/)ci-results\/timing\.json$/.test(path),
    );
    if (!paths.length) return [];
    assert.equal(paths.length, 1, `Ambiguous report in ${name}`);
    const reportPath = join(dir, paths[0]);
    const inventoryPath = reportPath.replace(/timing\.json$/, "inventory.json");
    const report = JSON.parse(readFileSync(reportPath, "utf8"));
    if (!new RegExp(`^${cohort}-[1-${shards}]$`).test(report.suite)) return [];
    assert.ok(
      existsSync(inventoryPath),
      `Missing inventory for ${report.suite}`,
    );
    return [
      { report, inventory: JSON.parse(readFileSync(inventoryPath, "utf8")) },
    ];
  });
  const inventories = readdirSync(root, { recursive: true }).filter((path) =>
    path.endsWith(`/tests-${cohort}.tar.zst.json`),
  );
  const producer = selectProducerInventory(
    inventories.map((path) =>
      JSON.parse(readFileSync(join(root, path), "utf8")),
    ),
  );
  assert.equal(producer.source, process.env.CI_SOURCE);
  assert.equal(producer.cohort, cohort);
  assert.deepEqual(
    producer.packages,
    JSON.parse(process.env.CI_PACKAGES).sort(),
  );
  const count = verifyCohortEvidence(reports, {
    cohort,
    profile: cohortProfile(cohort),
    shards,
    archive_sha256: producer.archive_sha256,
    packages: producer.packages,
    partitions: producer.partitions,
    source: process.env.CI_SOURCE,
    run: process.env.GITHUB_RUN_ID,
    attempt: process.env.GITHUB_RUN_ATTEMPT,
  });
  console.log(
    `Verified complete ${cohort} evidence: ${count} executions across ${shards} disjoint shards.`,
  );
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  main();
