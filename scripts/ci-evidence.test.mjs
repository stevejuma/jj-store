import test from "node:test";
import assert from "node:assert/strict";
import {
  selectProducerInventory,
  verifyOrdinaryEvidence,
} from "./ci-evidence.mjs";
import { nightlyHealth } from "./ci-health.mjs";
import { verifyProducer } from "./ci-artifacts.mjs";
import { verifyUiSource } from "./ci-ui-producer.mjs";
const expected = {
  source: "a".repeat(40),
  run: "12",
  attempt: "2",
  repository: "stevejuma/jj-store",
  archive_sha256: "b".repeat(64),
  packages: ["package"],
  partitions: Array.from({ length: 4 }, (_, index) => [
    JSON.stringify(["package", "binary", `test-${index}`, false]),
  ]),
};
function reports() {
  return Array.from({ length: 4 }, (_, index) => ({
    inventory: {
      source: expected.source,
      cohort: "ordinary",
      profile: "ci",
      packages: expected.packages,
      archive_sha256: expected.archive_sha256,
      shard: index + 1,
      expected: 1,
      tests: [JSON.stringify(["package", "binary", `test-${index}`, false])],
    },
    report: {
      ...expected,
      suite: `ordinary-${index + 1}`,
      commands: [{ label: "rust-tests", exit_code: 0, signal: null }],
      results: {
        junit: [
          {
            path: "target/nextest/ci/junit.xml",
            cases: [
              { name: `test-${index}`, class: "binary", result: "passed" },
            ],
            counts: { passed: 1, failed: 0, skipped: 0 },
          },
        ],
      },
    },
  }));
}
test("ordinary gate rejects absent, duplicate, foreign and incomplete shard evidence", () => {
  assert.equal(verifyOrdinaryEvidence(reports(), expected), 4);
  assert.throws(() => verifyOrdinaryEvidence(reports().slice(1), expected));
  assert.throws(() =>
    verifyOrdinaryEvidence([...reports(), reports()[0]], expected),
  );
  for (const edit of [
    (r) => (r.report.source = "b".repeat(40)),
    (r) => (r.report.attempt = "1"),
    (r) => (r.report.run = "13"),
    (r) => (r.report.commands = []),
    (r) => (r.report.commands[0].signal = "SIGTERM"),
    (r) => (r.report.commands[0].exit_code = 1),
    (r) => (r.report.results.junit = []),
    (r) => (r.report.results.junit[0].counts.passed = 0),
    (r) => (r.report.results.junit[0].counts.failed = 1),
    (r) => (r.inventory.cohort = "heavy"),
    (r) =>
      (r.inventory.tests = [
        JSON.stringify(["package", "binary", "test-1", false]),
      ]),
  ]) {
    const data = reports();
    edit(data[0]);
    assert.throws(() => verifyOrdinaryEvidence(data, expected));
  }
  const empty = reports();
  for (const r of empty) {
    r.inventory.expected = 0;
    r.inventory.tests = [];
    r.report.results.junit[0].counts.passed = 0;
    r.report.results.junit[0].cases = [];
  }
  assert.throws(
    () =>
      verifyOrdinaryEvidence(empty, {
        ...expected,
        partitions: [[], [], [], []],
      }),
    /no tests/,
  );
  const oneEmpty = reports();
  oneEmpty[0] = empty[0];
  assert.equal(
    verifyOrdinaryEvidence(oneEmpty, {
      ...expected,
      partitions: [[], ...expected.partitions.slice(1)],
    }),
    3,
  );
});
test("identical producer inventory copies are accepted but conflicts fail", () => {
  const inventory = {
    source: expected.source,
    cohort: "ordinary",
    archive_sha256: expected.archive_sha256,
  };
  assert.deepEqual(
    selectProducerInventory([inventory, structuredClone(inventory)]),
    inventory,
  );
  assert.throws(() => selectProducerInventory([]), /Missing/);
  assert.throws(
    () =>
      selectProducerInventory([
        inventory,
        { ...inventory, archive_sha256: "c".repeat(64) },
      ]),
    /Conflicting/,
  );
});
const now = Date.parse("2026-11-02T12:00:00Z");
const run = {
  id: 12,
  run_attempt: 2,
  repository: { full_name: expected.repository },
  path: ".github/workflows/ci-heavy.yml",
  event: "schedule",
  head_branch: "main",
  head_sha: expected.source,
  created_at: "2026-11-02T02:17:00Z",
  updated_at: "2026-11-02T03:17:00Z",
  status: "completed",
  conclusion: "success",
  complete: true,
  html_url: "https://github.com/stevejuma/jj-store/actions/runs/12",
};
test("freshness distinguishes absent attempts, failed, partial and complete scheduled runs", () => {
  assert.equal(nightlyHealth([run], now).healthy, true);
  for (const changed of [
    { event: "workflow_dispatch" },
    { head_branch: "feature" },
    { repository: { full_name: "fork/repo" } },
    { complete: false },
    { conclusion: "failure" },
    { conclusion: "cancelled" },
    { created_at: "2026-10-30T02:17:00Z", updated_at: "2026-10-30T03:17:00Z" },
    { created_at: "2026-11-03T02:17:00Z" },
  ])
    assert.equal(nightlyHealth([{ ...run, ...changed }], now).healthy, false);
  assert.equal(nightlyHealth([], now).healthy, false);
  assert.equal(
    nightlyHealth(
      [
        run,
        {
          ...run,
          id: 13,
          created_at: "2026-11-02T10:00:00Z",
          conclusion: "failure",
        },
      ],
      now,
    ).healthy,
    false,
  );
});
test("schedule admission is confined to canonical main and exact source; early same-run consumers retain producer checks", () => {
  const jobs = [
    "Resolve qualification source",
    "Standard checks / required",
    "build artifact producer",
  ].map((name) => ({ name, conclusion: "success" }));
  assert.doesNotThrow(() => verifyProducer(run, jobs, expected, ["build"], {}));
  const env = { GITHUB_RUN_ID: "12", GITHUB_RUN_ATTEMPT: "2" };
  assert.doesNotThrow(() => verifyUiSource(run, expected, env));
  const early = jobs.filter((job) => !job.name.includes("Standard"));
  assert.doesNotThrow(() =>
    verifyProducer(run, early, expected, ["build"], env),
  );
  assert.throws(() => verifyProducer(run, early, expected, ["build"], {}));
  assert.throws(() =>
    verifyProducer(run, early.slice(0, 1), expected, ["build"], env),
  );
  for (const change of [
    { head_branch: "feature" },
    { path: ".github/workflows/ci.yml" },
    { head_sha: "b".repeat(40) },
    { repository: { full_name: "fork/repo" } },
    { run_attempt: 1 },
  ]) {
    assert.throws(() =>
      verifyProducer({ ...run, ...change }, jobs, expected, ["build"], env),
    );
    assert.throws(() => verifyUiSource({ ...run, ...change }, expected, env));
  }
});

test("ordinary reports and archive consumers retain their required evidence and avoid dependency fetching", async () => {
  const { readFileSync } = await import("node:fs");
  const { parse } = await import("yaml");
  const ci = parse(readFileSync(".github/workflows/ci.yml", "utf8"));
  for (const name of ["build", "rust"]) {
    const report = ci.jobs[name].steps.find(
      (step) => step.uses === "./.github/actions/qualification-report",
    );
    assert.equal(report.if, "always()");
  }
  assert.equal(
    ci.jobs.rust.steps.find((step) => step.uses === "./.github/actions/setup")
      .with.capability,
    "archive",
  );
  assert.equal(
    ci.jobs.build.steps.find((step) =>
      step.uses?.startsWith("actions/upload-artifact@"),
    ).with["compression-level"],
    0,
  );
  const cache = ci.jobs.build.steps.findIndex((step) =>
    step.uses?.includes("rust-cache@"),
  );
  assert.ok(
    ci.jobs.build.steps.findIndex(
      (step) => step.run === "cargo fetch --locked",
    ) > cache,
  );
  assert.equal(
    ci.jobs.build.steps.find((step) => step.uses === "./.github/actions/setup")
      .with["fetch-rust"],
    "false",
  );
  assert.ok(
    ci.jobs.required.steps.some(
      (step) => step.run === "node scripts/ci-evidence.mjs",
    ),
  );
  const setup = parse(readFileSync(".github/actions/setup/action.yml", "utf8"));
  assert.match(
    setup.runs.steps.find((step) => step.run === "cargo fetch --locked").if,
    /capability == 'full'.*fetch-rust/,
  );
});

test("nightly diagnostics preserve the four original quarantine identities without waiving full qualification", async () => {
  const { readFileSync } = await import("node:fs");
  const { parse } = await import("yaml");
  const registry = JSON.parse(
    readFileSync(".config/ci-quarantine.json", "utf8"),
  );
  assert.deepEqual(registry.cases.map((entry) => entry.id).sort(), [
    "Q-BROWSER-001",
    "Q-RUST-002",
    "Q-STORY-001",
    "Q-STORY-002",
  ]);
  const heavy = parse(readFileSync(".github/workflows/ci-heavy.yml", "utf8"));
  assert.equal(heavy.jobs.diagnostics["continue-on-error"], true);
  assert.deepEqual(heavy.jobs.diagnostics.strategy.matrix.kind, [
    "stories",
    "browser",
    "rust",
  ]);
  assert.ok(!heavy.jobs.complete.needs.includes("diagnostics"));
  assert.ok(heavy.jobs.complete.needs.includes("standard"));
  const runner = readFileSync("scripts/ci-quarantine.mjs", "utf8");
  assert.match(runner, /repetition <= 3/);
  assert.match(runner, /--retries=0/);
  assert.match(runner, /--retry=0/);
  assert.match(runner, /if \(failed\) process.exitCode = 1/);
  for (const entry of registry.cases.filter(
    (entry) => entry.kind === "stories",
  )) {
    const source = readFileSync(entry.file, "utf8");
    assert.ok(source.includes(entry.name));
    assert.ok(source.includes('tags={["quarantined"]}'));
  }
});

test("cold CI workspace preparation is isolated and repeatable without changing tracked source", async (t) => {
  const { ensureCiWorkspace } = await import("./ci-jj-workspace.mjs");
  const { mkdtempSync, rmSync, writeFileSync } = await import("node:fs");
  const { tmpdir } = await import("node:os");
  const { join } = await import("node:path");
  const { execFileSync } = await import("node:child_process");
  const root = mkdtempSync(join(tmpdir(), "ci-jj-workspace-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const git = (...args) =>
    execFileSync("git", args, { cwd: root, encoding: "utf8" });
  git("init", "--initial-branch=main");
  writeFileSync(join(root, "fixture.txt"), "retained source\n");
  git("add", ".");
  git(
    "-c",
    "user.name=CI",
    "-c",
    "user.email=ci@localhost",
    "commit",
    "-m",
    "fixture",
  );
  const before = git("rev-parse", "HEAD");
  assert.throws(() => ensureCiWorkspace(root, {}), /disposable CI/);
  ensureCiWorkspace(root, { CI: "true" });
  ensureCiWorkspace(root, { CI: "true" });
  assert.equal(git("rev-parse", "HEAD"), before);
  assert.equal(git("status", "--porcelain"), "");
});

test("diagnostic repetitions keep the first failure and select only original quarantines", async (t) => {
  const fs = await import("node:fs");
  const { tmpdir } = await import("node:os");
  const { join, resolve } = await import("node:path");
  const { spawnSync } = await import("node:child_process");
  const root = fs.mkdtempSync(join(tmpdir(), "ci-quarantine-driver-"));
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));
  fs.mkdirSync(join(root, ".config"));
  fs.mkdirSync(join(root, "bin"));
  fs.copyFileSync(
    ".config/ci-quarantine.json",
    join(root, ".config/ci-quarantine.json"),
  );
  fs.writeFileSync(
    join(root, "bin/pnpm"),
    `#!${process.execPath}
const fs = require('node:fs');
const file = process.env.DIAGNOSTIC_LOG;
const previous = fs.existsSync(file) ? fs.readFileSync(file, 'utf8').trim().split('\\n').length : 0;
fs.appendFileSync(file, JSON.stringify({args:process.argv.slice(2), selection:process.env.JJ_STORE_QUARANTINE, report:process.env.PLAYWRIGHT_JUNIT_OUTPUT_FILE})+'\\n');
if (previous === 0) process.exit(7);
`,
    { mode: 0o755 },
  );
  for (const kind of ["stories", "browser"]) {
    const log = join(root, `${kind}.jsonl`);
    const result = spawnSync(
      process.execPath,
      [resolve("scripts/ci-quarantine.mjs"), kind],
      {
        cwd: root,
        encoding: "utf8",
        env: {
          ...process.env,
          PATH: `${join(root, "bin")}:${process.env.PATH}`,
          DIAGNOSTIC_LOG: log,
        },
      },
    );
    assert.equal(result.status, 1, result.stderr);
    const calls = fs
      .readFileSync(log, "utf8")
      .trim()
      .split("\n")
      .map(JSON.parse);
    assert.equal(calls.length, 3);
    for (const call of calls) {
      assert.equal(
        call.selection,
        kind === "stories" ? "stories" : "Q-BROWSER-001",
      );
      assert.ok(
        call.args.includes(kind === "stories" ? "--retry=0" : "--retries=0"),
      );
    }
    if (kind === "browser") {
      assert.equal(new Set(calls.map((call) => call.report)).size, 3);
      assert.ok(
        calls[0].args.includes(
          "comparison parent and workspace controls stay ordered after viewport resize$",
        ),
      );
    }
  }
});
